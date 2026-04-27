use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// Shared data models go here. Both app_client and app_server depend on this crate.
// Types that cross the network boundary must derive Serialize + Deserialize.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technician {
    pub id: i32,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: i32,
    pub service_type: HvacServiceType,
    pub price: i32, // Stored as cents.
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HvacServiceType {
    Maintenance,
    Installation,
    Repair,
    Emergency
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String, //ToDo: validate email regex: '^[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}$'
    pub phone_number: String, //ToDo: format as 10 digit code
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub id: i32,
    pub technician_id: i32,
    pub service_id: i32,
    pub customer_id: i32,
    // ISO8601:RFC3339 DateTime for compatibility outside Rust
    #[serde(with = "time::serde::rfc3339")]
    pub scheduled_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime
}
