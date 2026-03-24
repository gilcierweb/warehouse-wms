use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::alert_configs;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = alert_configs)]
pub struct AlertConfig {
    #[serde(default)]
    pub id: Uuid,
    pub threshold_pct: i16,
    pub notify_browser: bool,
    pub notify_email: bool,
    pub email_recipient: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = alert_configs)]
pub struct NewAlertConfig {
    pub threshold_pct: i16,
    pub notify_browser: bool,
    pub notify_email: bool,
    pub email_recipient: Option<String>,
}
