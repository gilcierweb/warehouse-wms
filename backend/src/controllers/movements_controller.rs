use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use rust_i18n::t;

use crate::db::database::Database;
use crate::db::schema::{movements, slots};
use crate::models::{movement::Movement, movement::NewMovement, slot::Slot};
use crate::repositories::AppContainer;

#[derive(Debug, Deserialize)]
pub struct MovementFilter {
    pub slot_address: Option<String>,   // filtro pelo endereço legível "A-5-N2"
    #[serde(rename = "type")]
    pub mov_type: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UndoRequest {
    pub slot_address: String,   // operador informa o endereço legível
}

/// GET /api/movements
#[get("/movements")]
pub async fn get_movements(
    container: web::Data<AppContainer>,
) -> impl Responder {
    super::generic_controller::get_all(container.movements.clone()).await
}

/// GET /api/movements/filtered
#[get("/movements/filtered")]
pub async fn list_movements(
    container: web::Data<AppContainer>,
    filter: web::Query<MovementFilter>,
) -> Result<HttpResponse, Error> {
    let limit = filter.limit.unwrap_or(50).min(200);
    let offset = filter.offset.unwrap_or(0);
    let slot_address = filter.slot_address.clone();
    let mov_type = filter.mov_type.clone();
    let from = filter.from;
    let to = filter.to;

    let movements = container
        .run(move |conn| {
            let mut query = movements::table.into_boxed();

            if let Some(ref addr) = slot_address {
                let slot_ids: Vec<Uuid> = slots::table
                    .filter(slots::address.eq(addr))
                    .select(slots::id)
                    .load::<Uuid>(conn)?;
                
                if !slot_ids.is_empty() {
                    query = query.filter(movements::slot_id.eq_any(slot_ids));
                }
            }
            if let Some(ref t) = mov_type {
                if let Ok(mov_type_int) = t.parse::<i32>() {
                    query = query.filter(movements::movement_type.eq(mov_type_int));
                }
            }
            if let Some(f) = from {
                query = query.filter(movements::created_at.ge(f.naive_utc()));
            }
            if let Some(t) = to {
                query = query.filter(movements::created_at.le(t.naive_utc()));
            }

            query
                .order(movements::created_at.desc())
                .limit(limit)
                .offset(offset)
                .load::<Movement>(conn)
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.error").to_string()))?;

    Ok(HttpResponse::Ok().json(movements))
}

/// POST /api/movements/undo
#[post("/movements/undo")]
pub async fn undo_movement(
    container: web::Data<AppContainer>,
    body: web::Json<UndoRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let addr = body.slot_address.to_uppercase();

    let result: Result<Slot, _> = container
        .run(move |conn| {
            conn.transaction::<_, diesel::result::Error, _>(|conn| {
                let slot: Slot = slots::table
                    .filter(slots::address.eq(&addr))
                    .first::<Slot>(conn)?;

                let last_movement: Movement = movements::table
                    .filter(movements::slot_id.eq(slot.id))
                    .order(movements::created_at.desc())
                    .first::<Movement>(conn)?;

                let new_status = if last_movement.movement_type == 1 { "free" } else { "occupied" };

                diesel::update(slots::table.find(slot.id))
                    .set((
                        slots::status.eq(new_status),
                        slots::sku.eq(Option::<String>::None),
                        slots::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;

                diesel::delete(movements::table.find(last_movement.id))
                    .execute(conn)?;

                diesel::insert_into(movements::table)
                    .values(&NewMovement {
                        slot_id: Some(slot.id),
                        movement_type: if new_status == "free" { 2 } else { 1 },
                        operator_id: last_movement.operator_id,
                        operator_name: Some(format!("[UNDO] {:?}", last_movement.operator_name)),
                        sku: None,
                        note: Some(t!("movements.undo.note").to_string()),
                    })
                    .execute(conn)?;

                Ok(slot)
            })
        })
        .await;

    match result {
        Ok(slot) => Ok(HttpResponse::Ok().json(&slot)),
        Err(_) => Ok(HttpResponse::NotFound().body(t!("movements.undo.not_found"))),
    }
}

/// POST /api/movements
#[post("/movements")]
pub async fn create_movement(
    container: web::Data<AppContainer>,
    new_movement: web::Json<NewMovement>,
) -> impl Responder {
    super::generic_controller::create(container.movements.clone(), new_movement).await
}

#[get("/movements/{id}")]
pub async fn get_movement_by_id(
    container: web::Data<AppContainer>,
    id: web::Path<Uuid>,
) -> impl Responder {
    super::generic_controller::get_by_id(container.movements.clone(), id).await
}

#[put("/movements/{id}")]
pub async fn update_movement_by_id(
    container: web::Data<AppContainer>,
    id: web::Path<Uuid>,
    updated_movement: web::Json<NewMovement>,
) -> impl Responder {
    super::generic_controller::update(container.movements.clone(), id, updated_movement).await
}

#[delete("/movements/{id}")]
pub async fn delete_movement_by_id(
    container: web::Data<AppContainer>,
    id: web::Path<Uuid>,
) -> impl Responder {
    super::generic_controller::delete(container.movements.clone(), id).await
}
