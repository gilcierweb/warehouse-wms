use std::fmt::Error;
use actix_web::{web, HttpRequest};

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::alert_configs::dsl::*;
use crate::models::alert_config::AlertConfig;
use crate::repositories::base_repository::BaseRepository;

pub struct AlertConfigRepository {
    connection: web::Data<Database>,
    request: Option<HttpRequest> // Optional HttpRequest

}

impl BaseRepository<AlertConfig> for AlertConfigRepository {

    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self {
        Self { connection, request }
    }

    fn all(&self) -> Result<Vec<AlertConfig>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = alert_configs.load::<AlertConfig>(&mut conn).expect("Error loading all alert_configs");
        Ok(items)
    }


    fn find(&self, alert_config_id: &Uuid) -> Option<AlertConfig> {
        let alert_config = alert_configs.find(alert_config_id)
            .get_result::<AlertConfig>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading AlertConfig by id");
        Some(alert_config)
    }

    fn create(&mut self, entity: &mut AlertConfig) -> Result<AlertConfig, Error> {
        let alert_config = AlertConfig {        
            ..entity.to_owned()
        };
        diesel::insert_into(alert_configs)
            .values(&alert_config)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new AlertConfig");
        Ok(alert_config)
    }

    fn update(&mut self, alert_config_id: &Uuid, entity: &mut AlertConfig) -> Option<AlertConfig> {
        let connection = &mut self.connection.pool.get().unwrap();
        let alert_config = diesel::update(alert_configs.find(alert_config_id))
            .set(entity.to_owned())
            .get_result::<AlertConfig>(connection)
            .expect("Error updating AlertConfig by id");
        Some(alert_config)
    }

    fn delete(&mut self, alert_config_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(alert_configs.find(alert_config_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting AlertConfig by id");
        Some(count)
    }
}