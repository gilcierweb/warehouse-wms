use chrono::{NaiveDate, NaiveDateTime};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::profiles;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: Uuid,
    pub first_name_enc: Option<Vec<u8>>,
    pub last_name_enc: Option<Vec<u8>>,
    pub phone_enc: Option<Vec<u8>>,
    pub full_name: Option<Vec<u8>>,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub avatar: Option<String>,
    pub phone: Option<i64>,
    pub social_network: serde_json::Value,
    pub status: bool,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub first_name_enc: Option<Vec<u8>>,
    pub last_name_enc: Option<Vec<u8>>,
    pub phone_enc: Option<Vec<u8>>,
    pub full_name: Option<Vec<u8>>,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub avatar: Option<String>,
    pub phone: Option<i64>,
    pub social_network: serde_json::Value,
    pub status: bool,
    pub user_id: Uuid,
}

impl NewProfile {
    pub fn for_user(user_id: Uuid) -> Self {
        Self {
            first_name_enc: None,
            last_name_enc: None,
            phone_enc: None,
            full_name: None,
            nickname: None,
            bio: None,
            birthday: None,
            avatar: None,
            phone: None,
            social_network: serde_json::json!({}),
            status: true,
            user_id,
        }
    }
}
