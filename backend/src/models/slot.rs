use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, Associations, Selectable};
use uuid::Uuid;
use diesel::pg::Pg;

use crate::db::schema::slots;

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Associations, Insertable, AsChangeset, Selectable)]
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