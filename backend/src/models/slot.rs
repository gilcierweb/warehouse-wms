use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::{AsChangeset, Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::slots;

use super::user::User;

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

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = slots)]
pub struct UpdateSlot {
    pub status: String,
    pub sku: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: Option<uuid::Uuid>,
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
