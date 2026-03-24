use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::{AsChangeset, Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::movements;

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
#[diesel(belongs_to(User, foreign_key = operator_id))]
#[diesel(table_name = movements)]
pub struct Movement {
    #[serde(default)]
    pub id: Uuid,
    pub slot_id: Option<Uuid>,
    pub movement_type: i32,
    pub operator_id: Option<Uuid>,
    pub operator_name: Option<String>,
    pub sku: Option<String>,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = movements)]
pub struct NewMovement {
    pub slot_id: Option<Uuid>,
    pub movement_type: i32,
    pub operator_id: Option<Uuid>,
    pub operator_name: Option<String>,
    pub sku: Option<String>,
    pub note: Option<String>,
}
