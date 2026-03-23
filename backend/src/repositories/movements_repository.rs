use std::fmt::Error;
use actix_web::{web, HttpRequest};

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::movements::dsl::*;
use crate::models::movement::Movement;
use crate::repositories::base_repository::BaseRepository;

pub struct MovementRepository {
    connection: web::Data<Database>,
    request: Option<HttpRequest> // Optional HttpRequest

}

impl BaseRepository<Movement> for MovementRepository {

    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self {
        Self { connection, request }
    }

    fn all(&self) -> Result<Vec<Movement>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = movements.load::<Movement>(&mut conn).expect("Error loading all movements");
        Ok(items)
    }


    fn find(&self, movement_id: &Uuid) -> Option<Movement> {
        let movement = movements.find(movement_id)
            .get_result::<Movement>(&mut self.connection.pool.get().unwrap())
            .expect("Error loading Movement by id");
        Some(movement)
    }

    fn create(&mut self, entity: &mut Movement) -> Result<Movement, Error> {
        let movement = Movement {        
            ..entity.to_owned()
        };
        diesel::insert_into(movements)
            .values(&movement)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new Movement");
        Ok(movement)
    }

    fn update(&mut self, movement_id: &Uuid, entity: &mut Movement) -> Option<Movement> {
        let connection = &mut self.connection.pool.get().unwrap();
        let movement = diesel::update(movements.find(movement_id))
            .set(entity.to_owned())
            .get_result::<Movement>(connection)
            .expect("Error updating Movement by id");
        Some(movement)
    }

    fn delete(&mut self, movement_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(movements.find(movement_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting Movement by id");
        Some(count)
    }
}