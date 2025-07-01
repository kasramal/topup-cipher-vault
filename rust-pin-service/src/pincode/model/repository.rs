use std::io;

use crate::{
    application::AppContext,
    pincode::model::{PinCode, PinCodeReservation, PinStatus},
};
use bson::{DateTime, doc, oid::ObjectId, to_bson};
use mongodb::{Collection, error::Error};

#[derive(Debug, Clone)]
pub struct PinCodeRepository {
    collection: Collection<PinCode>,
}

impl PinCodeRepository {
    pub fn new(context: &AppContext) -> Self {
        Self {
            collection: context.db_client.db().collection("pincodes"),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Option<PinCode> {
        let object_id = ObjectId::parse_str(id).ok()?;
        let filter = doc! { "_id": object_id };
        self.collection.find_one(filter, None).await.ok().flatten()
    }

    pub async fn find_available(&self) -> Option<PinCode> {
        let now = DateTime::now();

        let filter = doc! {
            "$or": [
                { "status": to_bson(&PinStatus::Active).unwrap() },
                {
                    "$and": [
                        { "status": to_bson(&PinStatus::Reserved).unwrap() },
                        { "expiresAt": { "$lte": now } }
                    ]
                }
            ]
        };
        self.collection.find_one(filter, None).await.ok().flatten()
    }

    pub async fn find_by_reservation_id(&self, reservation_id: &str) -> Option<PinCode> {
        let now = DateTime::now();
        let object_id = ObjectId::parse_str(reservation_id).ok()?;

        let filter = doc! {
                "status": to_bson(&PinStatus::Reserved).unwrap(),
                "reservationId": object_id ,
                "expiresAt": { "$gte": now } 
        };
        self.collection.find_one(filter, None).await.ok().flatten()
    }

    pub async fn reserve_pincode(
        &self,
        id: &str,
        reservation_id: &str,
        now: DateTime,
        expires_at: DateTime,
    ) -> Result<ObjectId, Error> {
        let object_id = ObjectId::parse_str(id).map_err(|e| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid ObjectId string: {}", e),
            ))
        })?;
        let rev_object_id = ObjectId::parse_str(reservation_id).map_err(|e| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid ObjectId string: {}", e),
            ))
        })?;
        let filter = doc! {
            "_id": object_id
        };

        let update = doc! {
            "$set": {
                "status": to_bson(&PinStatus::Reserved)?,
                "reservedAt": to_bson(&now)?,
                "reservationId": rev_object_id,
                "expiresAt": to_bson(&expires_at)?
            }
        };

        let result = self.collection.update_one(filter, update, None).await?;

        if result.matched_count == 1 {
            Ok(object_id)
        } else {
            Err(Error::from(io::Error::new(
                io::ErrorKind::NotFound,
                "No matching document found to update",
            )))
        }
    }

    pub async fn purchase_pincode(&self, id: &str, now: DateTime) -> Result<ObjectId, Error> {
        let object_id = ObjectId::parse_str(id).map_err(|e| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid ObjectId string: {}", e),
            ))
        })?;
        let filter = doc! {
            "_id": object_id
        };

        let update = doc! {
            "$set": {
                "status": to_bson(&PinStatus::Purchased)?,
                "purchasedAt": to_bson(&now)?,
            }
        };

        let result = self.collection.update_one(filter, update, None).await?;

        if result.matched_count == 1 {
            Ok(object_id)
        } else {
            Err(Error::from(io::Error::new(
                io::ErrorKind::NotFound,
                "No matching document found to update",
            )))
        }
    }

    pub async fn insert_one(&self, mut pincode: PinCode) -> mongodb::error::Result<ObjectId> {
        if pincode.id.is_none() {
            pincode.id = Some(ObjectId::new());
        }

        let result = self.collection.insert_one(pincode, None).await?;

        match result.inserted_id {
            bson::Bson::ObjectId(oid) => Ok(oid),
            _ => Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Expected ObjectId in insert result",
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PinCodeReservationRepository {
    collection: Collection<PinCodeReservation>,
}

impl PinCodeReservationRepository {
    pub fn new(context: &AppContext) -> Self {
        Self {
            collection: context.db_client.db().collection("reserved-pins"),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<PinCodeReservation, Error> {
        let filter = doc! { "_id": ObjectId::parse_str(id).unwrap() };
        match self.collection.find_one(filter, None).await? {
            Some(reservation) => {
                println!("{:?}", reservation);
                Ok(reservation)
            }
            None => Err(Error::from(io::Error::new(
                io::ErrorKind::NotFound,
                "PinCode not found",
            ))),
        }
    }

    pub async fn insert_one(
        &self,
        mut pincode: PinCodeReservation,
    ) -> mongodb::error::Result<ObjectId> {
        if pincode.id.is_none() {
            pincode.id = Some(ObjectId::new());
        }

        let result = self.collection.insert_one(pincode, None).await?;

        match result.inserted_id {
            bson::Bson::ObjectId(oid) => Ok(oid),
            _ => Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Expected ObjectId in insert result",
            ))),
        }
    }
}
