use std::sync::Arc;

use crate::config::app_config::AppConfig;
use crate::db::database::DBPool;
use crate::repositories::alert_configs_repository::IAlertConfigRepository;
use crate::repositories::audit_logs_repository::IAuditLogRepository;
use crate::repositories::base::BaseRepo;
use crate::repositories::movements_repository::IMovementRepository;
use crate::repositories::profiles_repository::IProfileRepository;
use crate::repositories::refresh_tokens_repository::IRefreshTokenRepository;
use crate::repositories::slots_repository::ISlotRepository;
use crate::repositories::users_repository::IUserRepository;
use diesel::QueryResult;

pub struct AppContainer {
    pub config: Arc<AppConfig>,
    pub users: Arc<dyn IUserRepository>,
    pub profiles: Arc<dyn IProfileRepository>,
    pub movements: Arc<dyn IMovementRepository>,
    pub slots: Arc<dyn ISlotRepository>,
    pub alert_configs: Arc<dyn IAlertConfigRepository>,
    pub refresh_tokens: Arc<dyn IRefreshTokenRepository>,
    pub audit_logs: Arc<dyn IAuditLogRepository>,
    base: BaseRepo,
}

impl AppContainer {
    pub fn new(pool: DBPool, config: Arc<AppConfig>) -> Self {
        let base = BaseRepo::new(pool);
        Self {
            config,
            users: Arc::new(base.clone()),
            profiles: Arc::new(base.clone()),
            movements: Arc::new(base.clone()),
            slots: Arc::new(base.clone()),
            alert_configs: Arc::new(base.clone()),
            refresh_tokens: Arc::new(base.clone()),
            audit_logs: Arc::new(base.clone()),
            base,
        }
    }

    /// Run a database query inline
    pub async fn run<F, T>(&self, f: F) -> QueryResult<T>
    where
        F: FnOnce(&mut diesel::PgConnection) -> QueryResult<T> + Send + 'static,
        T: Send + 'static,
    {
        self.base.run(f).await
    }
}
