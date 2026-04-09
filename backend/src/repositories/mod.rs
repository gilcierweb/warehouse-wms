pub mod base;
pub mod base_repository;
#[macro_use]
pub mod macros;
pub mod container;
pub mod traits;

pub mod users_repository;
pub mod profiles_repository;
pub mod movements_repository;
pub mod slots_repository;
pub mod alert_configs_repository;
pub mod refresh_tokens_repository;
pub mod audit_logs_repository;

// Re-export key types for convenient access
pub use container::AppContainer;
pub use users_repository::IUserRepository;
pub use profiles_repository::IProfileRepository;
pub use movements_repository::IMovementRepository;
pub use slots_repository::ISlotRepository;
pub use alert_configs_repository::IAlertConfigRepository;
pub use refresh_tokens_repository::IRefreshTokenRepository;
pub use audit_logs_repository::IAuditLogRepository;

use crate::db::schema::{users, profiles, movements, slots, alert_configs, refresh_tokens, audit_logs};
use crate::models::user::{User, NewUser};
use crate::models::profile::{Profile, NewProfile};
use crate::models::movement::{Movement, NewMovement};
use crate::models::slot::{Slot, NewSlot};
use crate::models::alert_config::{AlertConfig, NewAlertConfig};
use crate::models::refresh_token::{RefreshToken, NewRefreshToken};
use crate::models::audit_log::{AuditLog, NewAuditLog};
use crate::repositories::base::BaseRepo;
use uuid::Uuid;
use chrono::NaiveDateTime;
use ipnet::IpNet;
use diesel::ExpressionMethods;

impl_crud!(User, NewUser, users::table);
impl_crud!(Movement, NewMovement, movements::table);
impl_crud!(Slot, NewSlot, slots::table);
impl_crud!(AlertConfig, NewAlertConfig, alert_configs::table);

impl<T> IMovementRepository for T where T: traits::IRepository<Movement, NewMovement> + Send + Sync {}
impl<T> ISlotRepository for T where T: traits::IRepository<Slot, NewSlot> + Send + Sync {}
impl<T> IAlertConfigRepository for T where T: traits::IRepository<AlertConfig, NewAlertConfig> + Send + Sync {}

// IUserRepository implementation
#[async_trait::async_trait]
impl IUserRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<User>> {
        use diesel::RunQueryDsl;
        self.run(|conn| users::table.load::<User>(conn)).await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<User> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| users::table.find(id).first::<User>(conn))
            .await
    }

    async fn create(&self, item: &NewUser) -> diesel::QueryResult<User> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewUser) -> diesel::QueryResult<User> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(users::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let search = username_or_email.to_string();
        self.run(move |conn| {
            dsl::users
                .filter(dsl::email.eq(&search))
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn find_by_email(&self, email: &str) -> diesel::QueryResult<Option<User>> {
        self.find_by_username_or_email(email).await
    }

    async fn find_by_reset_token(&self, token: &str) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let token = token.to_string();
        self.run(move |conn| {
            dsl::users
                .filter(dsl::reset_password_token.eq(&token))
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn update_login_info(
        &self,
        id: &Uuid,
        current_sign_in_at: Option<NaiveDateTime>,
        last_sign_in_at: Option<NaiveDateTime>,
        current_sign_in_ip: Option<IpNet>,
        last_sign_in_ip: Option<IpNet>,
    ) -> diesel::QueryResult<User> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::current_sign_in_at.eq(current_sign_in_at),
                    dsl::last_sign_in_at.eq(last_sign_in_at),
                    dsl::current_sign_in_ip.eq(current_sign_in_ip.map(|ip| ip.to_string())),
                    dsl::last_sign_in_ip.eq(last_sign_in_ip.map(|ip| ip.to_string())),
                ))
                .get_result(conn)
        })
        .await
    }

    async fn update_password(
        &self,
        id: &Uuid,
        encrypted_password: &str,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *id;
        let pwd = encrypted_password.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::password_hash.eq(pwd))
                .execute(conn)
        })
        .await
    }

    async fn update_reset_token(
        &self,
        id: &Uuid,
        token: Option<String>,
        sent_at: Option<NaiveDateTime>,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::reset_password_token.eq(token),
                    dsl::reset_password_sent_at.eq(sent_at),
                ))
                .execute(conn)
        })
        .await
    }

    async fn confirm_email(&self, token: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::dsl::now;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let token = token.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.filter(dsl::confirmation_token.eq(&token)))
                .set((
                    dsl::confirmed_at.eq(now),
                    dsl::confirmation_token.eq(None::<String>),
                ))
                .execute(conn)
        })
        .await
    }

    async fn record_failed_login(
        &self,
        user_id: &Uuid,
        max_attempts: i32,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use chrono::Utc;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
        let id = *user_id;
        let max_attempts = max_attempts.max(1);
        self.run(move |conn| {
            let user = dsl::users
                .find(id)
                .select(User::as_select())
                .first::<User>(conn)
                .optional()?;

            if let Some(user) = user {
                let next_failed_attempts = user.failed_attempts.saturating_add(1);
                let next_locked_at = if next_failed_attempts >= max_attempts {
                    Some(Utc::now())
                } else {
                    user.locked_at
                };

                diesel::update(dsl::users.find(id))
                    .set((
                        dsl::failed_attempts.eq(next_failed_attempts),
                        dsl::locked_at.eq(next_locked_at),
                    ))
                    .execute(conn)
            } else {
                Ok(0)
            }
        })
        .await
    }

    async fn record_successful_login(
        &self,
        user_id: &Uuid,
        ip: Option<IpNet>,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        let ip_str = ip.map(|i| i.to_string());
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::failed_attempts.eq(0),
                    dsl::locked_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                    dsl::current_sign_in_ip.eq(ip_str),
                ))
                .execute(conn)
        })
        .await
    }

    async fn clear_lockout(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::failed_attempts.eq(0),
                    dsl::locked_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                ))
                .execute(conn)
        })
        .await
    }

    async fn get_user_roles(&self, user_id: &Uuid) -> diesel::QueryResult<Vec<String>> {
        use crate::db::schema::{roles, user_roles};
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            user_roles::table
                .inner_join(roles::table)
                .filter(user_roles::user_id.eq(id))
                .select(roles::name)
                .load::<String>(conn)
        })
        .await
    }

    async fn create_password_reset_token(
        &self,
        user_id: &Uuid,
        token: &str,
        sent_at: NaiveDateTime,
    ) -> diesel::QueryResult<usize> {
        self.update_reset_token(user_id, Some(token.to_string()), Some(sent_at))
            .await
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use chrono::{Duration, Utc};
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
        const RESET_TOKEN_TTL_HOURS: i64 = 1;
        let token = token.to_string();
        let pwd = new_password.to_string();
        self.run(move |conn| {
            let user: Option<User> = dsl::users
                .filter(dsl::reset_password_token.eq(&token))
                .first(conn)
                .optional()?;
            match user {
                Some(user) => {
                    let min_valid_sent_at = Utc::now() - Duration::hours(RESET_TOKEN_TTL_HOURS);
                    let is_valid = user
                        .reset_password_sent_at
                        .map(|sent_at| sent_at >= min_valid_sent_at)
                        .unwrap_or(false);

                    if !is_valid {
                        return Ok(0);
                    }

                    diesel::update(dsl::users.find(user.id))
                        .set((
                            dsl::password_hash.eq(pwd),
                            dsl::reset_password_token.eq(None::<String>),
                            dsl::reset_password_sent_at.eq(None::<NaiveDateTime>),
                        ))
                        .execute(conn)
                }
                None => Ok(0),
            }
        })
        .await
    }

    async fn set_otp_secret(&self, user_id: &Uuid, secret: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        let secret = secret.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::totp_secret.eq(secret))
                .execute(conn)
        })
        .await
    }

    async fn enable_2fa(
        &self,
        user_id: &Uuid,
        _backup_codes: &[String],
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::totp_enabled.eq(true))
                .execute(conn)
        })
        .await
    }

    async fn disable_2fa(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::totp_secret.eq(None::<String>),
                    dsl::totp_enabled.eq(false),
                ))
                .execute(conn)
        })
        .await
    }
}

// IProfileRepository implementation
#[async_trait::async_trait]
impl IProfileRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Profile>> {
        use diesel::RunQueryDsl;
        self.run(|conn| profiles::table.load::<Profile>(conn)).await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<Profile> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| profiles::table.find(id).first::<Profile>(conn))
            .await
    }

    async fn create(&self, item: &NewProfile) -> diesel::QueryResult<Profile> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(profiles::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewProfile) -> diesel::QueryResult<Profile> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(profiles::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(profiles::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> diesel::QueryResult<Option<Profile>> {
        use crate::db::schema::profiles::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            dsl::profiles
                .filter(dsl::user_id.eq(id))
                .first::<Profile>(conn)
                .optional()
        })
        .await
    }
}
// IRefreshTokenRepository implementation
#[async_trait::async_trait]
impl IRefreshTokenRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<RefreshToken>> {
        use diesel::RunQueryDsl;
        self.run(|conn| refresh_tokens::table.load::<RefreshToken>(conn))
            .await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<RefreshToken> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| refresh_tokens::table.find(id).first::<RefreshToken>(conn))
            .await
    }

    async fn create(&self, item: &NewRefreshToken) -> diesel::QueryResult<RefreshToken> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(refresh_tokens::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewRefreshToken) -> diesel::QueryResult<RefreshToken> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(refresh_tokens::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(refresh_tokens::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> diesel::QueryResult<Option<RefreshToken>> {
        use crate::db::schema::refresh_tokens::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let hash = token_hash.to_string();
        self.run(move |conn| {
            dsl::refresh_tokens
                .filter(dsl::token_hash.eq(&hash))
                .first::<RefreshToken>(conn)
                .optional()
        })
        .await
    }

    async fn revoke(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::refresh_tokens::dsl;
        use chrono::Utc;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        let now = Utc::now();
        self.run(move |conn| {
            diesel::update(dsl::refresh_tokens.find(id))
                .set(dsl::revoked_at.eq(now))
                .execute(conn)
        })
        .await
    }

    async fn revoke_all_for_user(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::refresh_tokens::dsl;
        use chrono::Utc;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        let now = Utc::now();
        self.run(move |conn| {
            diesel::update(dsl::refresh_tokens.filter(dsl::user_id.eq(id)))
                .set(dsl::revoked_at.eq(now))
                .execute(conn)
        })
        .await
    }
}

// IAuditLogRepository implementation
#[async_trait::async_trait]
impl IAuditLogRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<AuditLog>> {
        use diesel::RunQueryDsl;
        self.run(|conn| audit_logs::table.load::<AuditLog>(conn))
            .await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<AuditLog> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| audit_logs::table.find(id).first::<AuditLog>(conn))
            .await
    }

    async fn create(&self, item: &NewAuditLog) -> diesel::QueryResult<AuditLog> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(audit_logs::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewAuditLog) -> diesel::QueryResult<AuditLog> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(audit_logs::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(audit_logs::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_user(&self, user_id: &Uuid) -> diesel::QueryResult<Vec<AuditLog>> {
        use crate::db::schema::audit_logs::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            dsl::audit_logs
                .filter(dsl::user_id.eq(id))
                .load::<AuditLog>(conn)
        })
        .await
    }

    async fn find_by_action(&self, action: &str) -> diesel::QueryResult<Vec<AuditLog>> {
        use crate::db::schema::audit_logs::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let act = action.to_string();
        self.run(move |conn| {
            dsl::audit_logs
                .filter(dsl::action.eq(&act))
                .load::<AuditLog>(conn)
        })
        .await
    }
}
