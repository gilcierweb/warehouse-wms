use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::{AsChangeset, Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::slots;
use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SlotStatus {
    Free,
    Occupied,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Lane {
    N1,
    N2,
    N3,
}

impl SlotStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SlotStatus::Free => "free",
            SlotStatus::Occupied => "occupied",
        }
    }
}

impl Lane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Lane::N1 => "N1",
            Lane::N2 => "N2", 
            Lane::N3 => "N3",
        }
    }
}

impl From<String> for SlotStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "occupied" => SlotStatus::Occupied,
            _ => SlotStatus::Free,
        }
    }
}

impl From<String> for Lane {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N2" => Lane::N2,
            "N3" => Lane::N3,
            _ => Lane::N1,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Associations,
    Insertable,
    AsChangeset,
    Selectable,
)]
#[diesel(check_for_backend(Pg))]
#[diesel(belongs_to(User, foreign_key = updated_by))]
#[diesel(table_name = slots)]
pub struct Slot {
    #[serde(default)]
    pub id: Uuid,
    pub address: String,
    pub street: String,
    pub position: i16,
    pub lane: String,
    pub status: String,
    pub sku: Option<String>,
    pub updated_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Slot {
    pub fn street_char(&self) -> char {
        self.street.chars().next().unwrap_or('A')
    }
    
    pub fn status_enum(&self) -> SlotStatus {
        SlotStatus::from(self.status.clone())
    }
    
    pub fn lane_enum(&self) -> Lane {
        Lane::from(self.lane.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = slots)]
pub struct NewSlot {
    pub address: String,
    pub street: String,
    pub position: i16,
    pub lane: String,
    pub status: String,
    pub sku: Option<String>,
    pub updated_by: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSlotRequest {
    pub street: String,    // A-Z
    pub position: i16,      // 1-30
    pub lane: String,      // N1 | N2 | N3
}

impl CreateSlotRequest {
    pub fn validate(&self) -> Result<(), String> {
        // Validate street (A-Z)
        if self.street.len() != 1 || !self.street.chars().next().unwrap_or('A').is_ascii_uppercase() {
            return Err("Street must be a single uppercase letter (A-Z)".to_string());
        }
        
        // Validate position (1-30)
        if self.position < 1 || self.position > 30 {
            return Err("Position must be between 1 and 30".to_string());
        }
        
        // Validate lane (N1 | N2 | N3)
        if !matches!(self.lane.as_str(), "N1" | "N2" | "N3") {
            return Err("Lane must be N1, N2, or N3".to_string());
        }
        
        Ok(())
    }
    
    pub fn generate_address(&self) -> String {
        format!("{}-{}-{}", self.street, self.position, self.lane)
    }
}

impl From<CreateSlotRequest> for NewSlot {
    fn from(req: CreateSlotRequest) -> Self {
        Self {
            address: req.generate_address(),
            street: req.street,
            position: req.position,
            lane: req.lane,
            status: SlotStatus::Free.as_str().to_string(),
            sku: None,
            updated_by: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = slots)]
pub struct UpdateSlot {
    pub status: String,
    pub sku: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSlotRequest {
    pub status: Option<String>,
    pub sku: Option<String>,
}

impl UpdateSlotRequest {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref status) = self.status {
            if !matches!(status.as_str(), "free" | "occupied") {
                return Err("Status must be 'free' or 'occupied'".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreetStat {
    pub name: String,
    pub occupied: i64,
    pub free: i64,
    pub total: i64,
    pub pct: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarehouseStats {
    pub total: i64,
    pub occupied: i64,
    pub free: i64,
    pub pct: f64,
    pub streets: Vec<StreetStat>,
}
