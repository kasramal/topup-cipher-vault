use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod repository;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PinStatus {
    Active,
    Reserved,
    Purchased,
}

impl fmt::Display for PinStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PinStatus::Active => "Active",
            PinStatus::Reserved => "Reserved",
            PinStatus::Purchased => "Purchased",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinCode {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub pincode: String,
    pub encrypted: String,
    pub status: PinStatus,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime>,

    #[serde(rename = "purchasedAt")]
    pub purchased_at: Option<DateTime>,

    #[serde(rename = "reservedAt")]
    pub reserved_at: Option<DateTime>,

    #[serde(rename = "reservationId")]
    pub reservation_id: Option<ObjectId>,

    #[serde(rename = "expiresAt")]
    pub expires_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinCodeReservation {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "pincodeId")]
    pub pincode_id: Option<ObjectId>,
    #[serde(rename = "reservedAt")]
    pub reserved_at: DateTime,
}
