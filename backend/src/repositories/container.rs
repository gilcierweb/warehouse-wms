use std::sync::Arc;

use crate::db::database::DBPool;
use crate::models::alert_config::{AlertConfig, NewAlertConfig};
use crate::models::movement::{Movement, NewMovement};
use crate::models::profile::{NewProfile, Profile};
use crate::models::slot::{NewSlot, Slot};
use crate::models::user::{NewUser, User};
use crate::repositories::base::BaseRepo;
use crate::repositories::traits::IRepository;

pub struct AppContainer {
    pub users: Arc<dyn IRepository<User, NewUser>>,
    pub profiles: Arc<dyn IRepository<Profile, NewProfile>>,
    pub movements: Arc<dyn IRepository<Movement, NewMovement>>,
    pub slots: Arc<dyn IRepository<Slot, NewSlot>>,
    pub alert_configs: Arc<dyn IRepository<AlertConfig, NewAlertConfig>>,
}

impl AppContainer {
    pub fn new(pool: DBPool) -> Self {
        let base = BaseRepo::new(pool);
        Self {
            users: Arc::new(base.clone()),
            profiles: Arc::new(base.clone()),
            movements: Arc::new(base.clone()),
            slots: Arc::new(base.clone()),
            alert_configs: Arc::new(base),
        }
    }
}
