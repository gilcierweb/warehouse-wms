use std::sync::Arc;

use crate::db::database::DBPool;
use crate::repositories::base::BaseRepo;
use crate::repositories::users_repository::IUserRepository;
use crate::repositories::profiles_repository::IProfileRepository;
use crate::repositories::movements_repository::IMovementRepository;
use crate::repositories::slots_repository::ISlotRepository;
use crate::repositories::alert_configs_repository::IAlertConfigRepository;

/// Central dependency injection container.
/// Groups all repositories behind trait objects (`Arc<dyn Trait>`) for:
/// - Thread-safe sharing across Actix workers
/// - Easy swapping with mock implementations in tests
pub struct AppContainer {
    pub users: Arc<dyn IUserRepository>,
    pub profiles: Arc<dyn IProfileRepository>,
    pub movements: Arc<dyn IMovementRepository>,
    pub slots: Arc<dyn ISlotRepository>,
    pub alert_configs: Arc<dyn IAlertConfigRepository>,
}

impl AppContainer {
    pub fn new(pool: DBPool) -> Self {
        Self {
            users: Arc::new(BaseRepo::new(pool.clone())),
            profiles: Arc::new(BaseRepo::new(pool.clone())),
            movements: Arc::new(BaseRepo::new(pool.clone())),
            slots: Arc::new(BaseRepo::new(pool.clone())),
            alert_configs: Arc::new(BaseRepo::new(pool)),
        }
    }
}
