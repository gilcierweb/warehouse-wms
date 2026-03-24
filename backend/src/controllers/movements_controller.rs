use actix_web::{delete, get, HttpResponse, post, put, web, Error};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use uuid::Uuid;
use rust_i18n::t;

use crate::db::database::Database;
use crate::db::schema::{movements, slots};
use crate::models::{movement::Movement, slot::Slot};
use crate::repositories::base_repository::BaseRepository;
use crate::repositories::movements_repository::MovementRepository;

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
pub async fn get_movements(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let result = MovementRepository::new(db, None).all();
    match result {
        Ok(movements) => {
            let response = serde_json::to_string(&movements).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

/// GET /api/movements/filtered
#[get("/movements/filtered")]
pub async fn list_movements(
    db: web::Data<Database>,
    filter: web::Query<MovementFilter>,
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.connection_error").to_string()))?;
    let limit = filter.limit.unwrap_or(50).min(200);
    let offset = filter.offset.unwrap_or(0);
    let slot_address = filter.slot_address.clone();
    let mov_type = filter.mov_type.clone();
    let from = filter.from;
    let to = filter.to;

    let result = web::block(move || {
        let mut query = movements::table.into_boxed();

        if let Some(ref addr) = slot_address {
            // Use a subquery approach to avoid type issues
            let slot_ids: Vec<Uuid> = slots::table
                .filter(slots::address.eq(addr))
                .select(slots::id)
                .load::<Uuid>(&mut conn)?;
            
            if !slot_ids.is_empty() {
                query = query.filter(movements::slot_id.eq_any(slot_ids));
            }
        }
        if let Some(ref t) = mov_type {
            // Convert string to i32 for movement_type
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
            .load::<Movement>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.error").to_string()))?;

    let movements = result.map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.query_error").to_string()))?;
    Ok(HttpResponse::Ok().json(movements))
}

/// POST /api/movements/undo
#[post("/movements/undo")]
pub async fn undo_movement(
    db: web::Data<Database>,
    body: web::Json<UndoRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db.pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;
    let addr = body.slot_address.to_uppercase();

    let updated_slot = web::block(move || {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 1. Resolve o endereço para o UUID interno do slot
            let slot: Slot = slots::table
                .filter(slots::address.eq(&addr))
                .first::<Slot>(conn)?;

            // 2. Encontra o último movimento deste slot pela FK UUID
            let last_movement: Movement = movements::table
                .filter(movements::slot_id.eq(slot.id))
                .order(movements::created_at.desc())
                .first::<Movement>(conn)?;

            // 3. Inverte a ação
            let new_status = if last_movement.movement_type == 1 { "free" } else { "occupied" };

            diesel::update(slots::table.find(slot.id))
                .set((
                    slots::status.eq(new_status),
                    slots::sku.eq(Option::<String>::None),
                    slots::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            // 4. Remove o movimento desfeito
            diesel::delete(movements::table.find(last_movement.id))
                .execute(conn)?;

            // 5. Registra o undo como novo movimento
            use crate::models::movement::NewMovement;
            diesel::insert_into(movements::table)
                .values(&NewMovement {
                    slot_id: Some(slot.id),
                    movement_type: if new_status == "free" { 2 } else { 1 }, // 2=exit, 1=entry
                    operator_id: last_movement.operator_id,
                    operator_name: Some(format!("[UNDO] {:?}", last_movement.operator_name)),
                    sku: None,
                    note: Some("Operação desfeita".to_string()),
                })
                .execute(conn)?;

            Ok(slot)
        })
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.error").to_string()))?;

    match updated_slot {
        Ok(slot) => Ok(HttpResponse::Ok().json(&slot)),
        Err(_) => Ok(HttpResponse::NotFound().body(t!("movements.undo.not_found").to_string())),
    }
}

/// POST /api/movements
#[post("/movements")]
pub async fn create_movement(db: web::Data<Database>, new_movement: web::Json<Movement>) -> HttpResponse {
    let movement = MovementRepository::new(db, None).create(&mut new_movement.into_inner());

    match movement {
        Ok(movement) => HttpResponse::Ok().json(movement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/movements/{id}")]
pub async fn get_movement_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let movement = MovementRepository::new(db, None).find(&id);

    match movement {
        Some(movement) => HttpResponse::Ok().json(movement),
        None => HttpResponse::NotFound().body(t!("movements.get.not_found").to_string()),
    }
}

#[put("/movements/{id}")]
pub async fn update_movement_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    updated_movement: web::Json<Movement>,
) -> HttpResponse {
    let movement = MovementRepository::new(db, None).update(&id, &mut updated_movement.into_inner());

    match movement {
        Some(movement) => HttpResponse::Ok().json(movement),
        None => HttpResponse::NotFound().body(t!("movements.get.not_found").to_string()),
    }
}

#[delete("/movements/{id}")]
pub async fn delete_movement_by_id(db: web::Data<Database>, id: web::Path<Uuid>) -> HttpResponse {
    let movement = MovementRepository::new(db, None).delete(&id);

    match movement {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body(t!("movements.get.not_found").to_string()),
    }
}
