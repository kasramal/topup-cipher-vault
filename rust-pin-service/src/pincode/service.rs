use bson::DateTime;
use chrono::Duration;
use futures::future::join_all;
use tonic::{Request, Response, Status};

use crate::application::AppContext;
use crate::pincode::model::repository::{PinCodeRepository, PinCodeReservationRepository};
use crate::pincode::model::{PinCode, PinCodeReservation, PinStatus};
use crate::pincode::utils;
use crate::vault::pin_code_vault_service_server::PinCodeVaultService;

use crate::cipher::Cipher;
use crate::vault::{
    GenerationRequest, IdRequest, PinCodeChunk, PinCodeResponse, ReservationResponse,
    StatusResponse,
};

use std::sync::Arc;

pub struct RustPinCodeVault {
    pub cipher: Option<Arc<dyn Cipher + Send + Sync>>,
    pincode_repo: PinCodeRepository,
    reservation_repo: PinCodeReservationRepository,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}

impl RustPinCodeVault {
    pub fn new(context: &AppContext) -> Self {
        Self {
            cipher: context.cipher.clone(),
            pincode_repo: PinCodeRepository::new(context),
            reservation_repo: PinCodeReservationRepository::new(context),
        }
    }
}

#[tonic::async_trait]
impl PinCodeVaultService for RustPinCodeVault {
    async fn upload_pin_codes(
        &self,
        request: Request<tonic::Streaming<PinCodeChunk>>,
    ) -> Result<Response<StatusResponse>, Status> {
        let mut stream = request.into_inner();
        let mut tasks = Vec::new();

        let cipher = self
            .cipher
            .as_ref()
            .ok_or_else(|| Status::internal("Cipher not initialized"))?
            .clone();
        let repo = self.pincode_repo.clone();

        let mut line_buffer = String::new(); // stores leftover partial line

        while let Some(chunk) = stream.message().await? {
            println!(
                "Received chunk (filename: {:?}, size: {})",
                chunk.file_name,
                chunk.content.len()
            );

            let chunk_str = String::from_utf8(chunk.content)
                .map_err(|_| Status::invalid_argument("Chunk is not valid UTF-8"))?;

            // Combine leftover from previous chunk + current chunk
            line_buffer.push_str(&chunk_str);

            // Extract complete lines
            while let Some(idx) = line_buffer.find('\n') {
                let line = line_buffer[..idx].trim_end().to_string();
                line_buffer = line_buffer[idx + 1..].to_string(); // cut off processed line

                let cipher = cipher.clone();
                let repo = repo.clone();

                let task = tokio::spawn(async move {
                    println!("Line: {}", line);
                    let pin = cipher.enc_decrypt(line.clone());

                    let pin_code = PinCode {
                        pincode: pin,
                        encrypted: line,
                        status: PinStatus::Active,
                        created_at: Some(DateTime::now()),
                        expires_at: None,
                        id: None,
                        purchased_at: None,
                        reservation_id: None,
                        reserved_at: None,
                    };

                    if let Err(e) = repo.insert_one(pin_code).await {
                        println!("Insert failed: {:?}", e);
                    }
                });

                tasks.push(task);
            }
        }

        if !line_buffer.is_empty() {
            let line = line_buffer.trim_end().to_string();
            let cipher = cipher.clone();
            let repo = repo.clone();

            let task = tokio::spawn(async move {
                println!("Final Line: {}", line);
                let pin = cipher.enc_decrypt(line.clone());

                let pin_code = PinCode {
                    pincode: pin,
                    encrypted: line,
                    status: PinStatus::Active,
                    created_at: Some(DateTime::now()),
                    expires_at: None,
                    id: None,
                    purchased_at: None,
                    reservation_id: None,
                    reserved_at: None,
                };

                if let Err(e) = repo.insert_one(pin_code).await {
                    println!("Insert failed: {:?}", e);
                }
            });

            tasks.push(task);
        }

        let _ = futures::future::join_all(tasks).await;

        Ok(Response::new(StatusResponse {
            success: true,
            message: "Upload complete".into(),
        }))
    }

    async fn generate_pin_code(
        &self,
        request: Request<GenerationRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let count = request.into_inner().count;
        println!("Generating {} PIN codes", count);

        let cipher = match &self.cipher {
            Some(c) => c,
            None => {
                println!("No cipher available!");
                return Err(Status::internal("Cipher not initialized"));
            }
        };

        // Create a vector of futures
        let mut tasks = Vec::new();
        for _ in 0..count {
            let job = async move {
                let pin = utils::generate_random_pin(16);
                let encrypted = cipher.enc_encrypt(pin.clone());

                println!("{encrypted}");
                let repo = self.pincode_repo.clone(); // Make sure your repo is Arc<dyn ...>
                let pin_code = PinCode {
                    pincode: pin.clone(),
                    encrypted: encrypted.clone(),
                    status: PinStatus::Active,
                    created_at: Some(DateTime::now()),
                    expires_at: None,
                    id: None,
                    purchased_at: None,
                    reservation_id: None,
                    reserved_at: None,
                };
                if let Err(e) = repo.insert_one(pin_code).await {
                    println!("Insert failed: {:?}", e);
                }
            };

            tasks.push(job);
        }

        // Run all jobs in parallel
        join_all(tasks).await;

        Ok(Response::new(StatusResponse {
            success: true,
            message: format!("Generated {} PIN code(s)", count),
        }))
    }

    async fn get_pin_code(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<PinCodeResponse>, Status> {
        let id = request.into_inner().id;
        println!("Fetching PIN for ID: {}", id);
        match self.pincode_repo.find_by_id(&id).await {
            Some(pin_code) => Ok(Response::new(PinCodeResponse {
                success: true,
                message: "PIN found".into(),
                id,
                pin_code: pin_code.pincode,
            })),
            None => Ok(Response::new(PinCodeResponse {
                success: false,
                message: "PIN not found".into(),
                id,
                pin_code: "".into(),
            })),
        }
    }

    async fn reserve_pin_code(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ReservationResponse>, Status> {
        match self.pincode_repo.find_available().await {
            Some(pin_code) => {
                let now = DateTime::now();
                let expires_at = DateTime::from_chrono(now.to_chrono() + Duration::minutes(3));

                // Insert reservation
                let rev_id = self
                    .reservation_repo
                    .insert_one(PinCodeReservation {
                        pincode_id: pin_code.id.clone(),
                        reserved_at: now,
                        id: None,
                    })
                    .await
                    .map_err(|e| {
                        Status::internal(format!("Failed to insert reservation: {}", e))
                    })?;

                // Update the pin code's statusself.pincode_repo
                self.pincode_repo
                    .reserve_pincode(
                        &pin_code.id.as_ref().unwrap().to_hex(),
                        &rev_id.to_hex(),
                        now,
                        expires_at,
                    )
                    .await
                    .map_err(|e| Status::internal(format!("Failed to reserve pin code: {}", e)))?;

                Ok(Response::new(ReservationResponse {
                    success: true,
                    message: "PIN reserved".into(),
                    id: rev_id.to_hex(), // Convert ObjectId to hex string
                }))
            }
            None => Ok(Response::new(ReservationResponse {
                success: false,
                message: "No PIN Available!".into(),
                id: "".into(),
            })),
        }
    }

    async fn take_pin_code(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<PinCodeResponse>, Status> {
        let id = request.into_inner().id;
        println!("Purchasing PIN for Reservation ID: {}", id);
        match self.pincode_repo.find_by_reservation_id(&id).await {
            Some(pin_code) => {
                let now = DateTime::now();

                self.pincode_repo
                    .purchase_pincode(&pin_code.id.as_ref().unwrap().to_hex(), now)
                    .await
                    .map_err(|e| Status::internal(format!("Failed to reserve pin code: {}", e)))?;

                Ok(Response::new(PinCodeResponse {
                    success: true,
                    message: "PIN reserved".into(),
                    id: id.into(),
                    pin_code: pin_code.pincode.into(),
                }))
            }
            None => Ok(Response::new(PinCodeResponse {
                success: false,
                message: "Reservation does not exist anymore!".into(),
                id: "".into(),
                pin_code: "".into(),
            })),
        }
    }
}
