use std::fmt::Error;
use actix_web::{web, HttpRequest};

use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::database::Database;
use crate::db::schema::slots::dsl::*;
use crate::models::slot::Slot;
use crate::repositories::base_repository::BaseRepository;

pub struct SlotRepository {
    connection: web::Data<Database>,
    request: Option<HttpRequest> // Optional HttpRequest

}

impl BaseRepository<Slot> for SlotRepository {

    fn new(connection: web::Data<Database>, request: Option<HttpRequest>) -> Self {
        Self { connection, request }
    }

    fn all(&self) -> Result<Vec<Slot>, diesel::result::Error> {
        let mut conn = self.connection.pool.get().unwrap();
        let items = slots.load::<Slot>(&mut conn).expect("Error loading all slots");
        Ok(items)
    }


    fn find(&self, slot_id: &Uuid) -> Option<Slot> {
        let slot = slots.find(slot_id)
            .get_result(&mut self.connection.pool.get().unwrap())
            .expect("Error loading Slot by id");
        Some(slot)
    }

    fn create(&mut self, entity: &mut Slot) -> Result<Slot, Error> {
        let slot = Slot {        
            ..entity.to_owned()
        };
        diesel::insert_into(slots)
            .values(&slot)
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error creating new Slot");
        Ok(slot)
    }

    fn update(&mut self, slot_id: &Uuid, entity: &mut Slot) -> Option<Slot> {
        let connection = &mut self.connection.pool.get().unwrap();
        let slot = diesel::update(slots.find(slot_id))
            .set(entity.to_owned())
            .get_result::<Slot>(connection)
            .expect("Error updating Slot by id");
        Some(slot)
    }

    fn delete(&mut self, slot_id: &Uuid) -> Option<usize> {
        let count = diesel::delete(slots.find(slot_id))
            .execute(&mut self.connection.pool.get().unwrap())
            .expect("Error deleting Slot by id");
        Some(count)
    }
}