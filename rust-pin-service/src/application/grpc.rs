use std::net::SocketAddr;
use tonic::transport::Server;
use crate::vault::pin_code_vault_service_server::PinCodeVaultServiceServer;
use crate::pincode::service::RustPinCodeVault;
use crate::application::AppContext;


pub async fn run_grpc_server(context: &AppContext) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket_addr_str = format!("0.0.0.0:{}", context.env.grpc.port);
    let addr: SocketAddr = socket_addr_str.parse()?;

    let service = RustPinCodeVault::new(context);

    println!("Vault gRPC server running at {}", addr);

    Server::builder()
        .add_service(PinCodeVaultServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}

pub fn run_grpc_server_bl(context: &AppContext) -> tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    let context = context.clone();
    tokio::spawn(async move {
        run_grpc_server(&context).await
    }) 
}
