use actix_web::{get, post, web, HttpResponse, Error, error::{ErrorInternalServerError, ErrorNotFound}};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::database::Database;
use crate::db::schema::{movements, slots};
use crate::models::{movement::NewMovement, slot::Slot, slot::StreetStat, slot::UpdateSlot, slot::WarehouseStats};
use crate::repositories::slots_repository::SlotRepository;
use crate::repositories::base_repository::BaseRepository;
use crate::ws::server::{HubData, WsEvent};

// ── Request / Response DTOs ───────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EntryRequest {
    pub sku:  Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExitRequest {
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SlotFilter {
    pub street: Option<String>,
    pub status: Option<String>,
}

// ── Handlers ──────────────────────────────────────────────────

/// GET /api/slots
#[get("/slots")]
pub async fn list_slots(
    db: web::Data<Database>,
    filter: web::Query<SlotFilter>,
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| ErrorInternalServerError("Database connection error"))?;

    let result = web::block(move || {
        let mut query = slots::table.into_boxed();

        if let Some(ref street) = filter.street {
            query = query.filter(slots::street.eq(street.as_str()));
        }
        if let Some(ref status) = filter.status {
            query = query.filter(slots::status.eq(status.as_str()));
        }

        query
            .order((slots::street.asc(), slots::lane.asc(), slots::position.asc()))
            .load::<Slot>(&mut conn)
    })
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;

    let slots = result.map_err(|_| ErrorInternalServerError("Database query error"))?;
    Ok(HttpResponse::Ok().json(slots))
}

/// GET /api/slots/:address  (ex: /api/slots/A-5-N2)
#[get("/slots/{address}")]
pub async fn get_slot(
    db: web::Data<Database>,
    address: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| ErrorInternalServerError("Database connection error"))?;
    let addr = address.into_inner().to_uppercase();
    let addr_clone = addr.clone();

    let result = web::block(move || {
        slots::table
            .filter(slots::address.eq(&addr))
            .first::<Slot>(&mut conn)
            .optional()
    })
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;

    let slot = result.map_err(|_| ErrorInternalServerError("Database query error"))?;
    match slot {
        Some(s) => Ok(HttpResponse::Ok().json(s)),
        None => Ok(HttpResponse::NotFound().body(format!("Slot '{}' não encontrado", addr_clone))),
    }
}

/// POST /api/slots/:address/entry  (ex: /api/slots/A-5-N2/entry)
#[post("/slots/{address}/entry")]
pub async fn entry(
    db: web::Data<Database>,
    hub: HubData,
    address: web::Path<String>,
    body: web::Json<EntryRequest>,
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| ErrorInternalServerError("Database connection error"))?;
    let addr = address.into_inner().to_uppercase();
    
    // TODO: Add authentication when middleware is implemented
    let user_id: Option<uuid::Uuid> = None;
    let operator_name = "System".to_string();
    let sku = body.sku.clone();
    let note = body.note.clone();
    
    let result = web::block(move || {
        // 1. Busca slot
        let slot = match slots::table
            .filter(slots::address.eq(&addr))
            .first::<Slot>(&mut conn) {
                Ok(slot) => slot,
                Err(e) => {
                    println!("DEBUG: Erro ao buscar slot: {}", e);
                    return Err(e);
                }
            };

        // 2. Verifica se está livre
        if slot.status != "free" {
            println!("DEBUG: Slot não está livre: {}", slot.status);
            return Err(diesel::result::Error::NotFound);
        }

        // 3. Atualiza status para occupied com SKU e timestamp
        println!("DEBUG: Atualizando slot ID: {}", slot.id);
        let updated: Slot = diesel::update(slots::table.find(slot.id))
            .set((
                slots::status.eq("occupied"),
                slots::sku.eq(&sku),
                slots::updated_at.eq(diesel::dsl::now),
                slots::updated_by.eq(user_id)
            ))
            .get_result(&mut conn)?;

        // 4. Insere movement
        println!("DEBUG: Inserindo movement para slot ID: {}", slot.id);
        let _ = diesel::insert_into(movements::table)
            .values(&NewMovement {
                slot_id: Some(slot.id),
                movement_type: 1,
                operator_id: user_id,
                operator_name: Some(operator_name),
                sku: sku,
                note: note,
            })
            .execute(&mut conn);

        Ok(updated)
    })
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;
    
    // Retorna resposta JSON simples
    let updated = result.map_err(|e| {
        match e {
            diesel::result::Error::NotFound => ErrorNotFound(serde_json::json!({
                "error": "Slot not found or already occupied",
                "code": "SLOT_OCCUPIED"
            })),
            _ => ErrorInternalServerError(serde_json::json!({
                "error": "Database error",
                "code": "DATABASE_ERROR"
            }))
        }
    })?;
    
    // Envia evento WebSocket
    let ws_event = WsEvent::slot_entry(&updated);
    hub.broadcast(ws_event);
    
    let response = SlotResponse::from(updated);
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/slots/:address/exit
#[post("/slots/{address}/exit")]
pub async fn exit(
    db:    web::Data<Database>,
    hub:     HubData,
    address: web::Path<String>,
    body:    web::Json<ExitRequest>,
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| ErrorInternalServerError("Database connection error"))?;
    let addr = address.into_inner().to_uppercase();
    
    // TODO: Add authentication when middleware is implemented
    let user_id: Option<uuid::Uuid> = None; // Usar None em vez de UUID aleatório para evitar foreign key violation
    let operator_name = "System".to_string(); // Placeholder
    let note = body.note.clone();

    let result = web::block(move || {
        // 1. Busca slot
        let slot = match slots::table
            .filter(slots::address.eq(&addr))
            .first::<Slot>(&mut conn) {
                Ok(slot) => slot,
                Err(_) => return Err(diesel::result::Error::NotFound),
            };

        // 2. Verifica se está occupied
        if slot.status != "occupied" {
            return Err(diesel::result::Error::NotFound);
        }

        // 3. Atualiza status para free
        let updated: Slot = diesel::update(slots::table.find(slot.id))
            .set(slots::status.eq("free"))
            .get_result(&mut conn)?;

        // 4. Insere movement
        diesel::insert_into(movements::table)
            .values(&NewMovement {
                slot_id: Some(slot.id),
                movement_type: 2, // 2=exit
                operator_id: user_id,
                operator_name: Some(operator_name),
                sku: slot.sku,
                note: note,
            })
            .execute(&mut conn)?;

        Ok(updated)
    })
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;

    let updated = result.map_err(|_| ErrorInternalServerError("Database transaction error"))?;
    
    // Envia evento WebSocket
    let ws_event = WsEvent::slot_exit(&updated);
    hub.broadcast(ws_event);
    
    let response = SlotResponse::from(updated);
    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/stats
#[get("/stats")]
pub async fn get_stats(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| ErrorInternalServerError("Database connection error"))?;

    let result = web::block(move || -> Result<_, diesel::result::Error> {
        compute_stats(&mut conn)
    })
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;

    let stats = result.map_err(|_| ErrorInternalServerError("Database query error"))?;
    Ok(HttpResponse::Ok().json(stats))
}

// ── compute_stats (interno) ───────────────────────────────────

pub fn compute_stats(
    conn: &mut diesel::PgConnection,
) -> Result<WarehouseStats, diesel::result::Error> {
    use diesel::dsl::count_star;

    let rows: Vec<(String, String, i64)> = slots::table
        .group_by((slots::street, slots::status))
        .select((slots::street, slots::status, count_star()))
        .order(slots::street.asc())
        .load(conn)?;

    let mut street_map: std::collections::BTreeMap<String, (i64, i64)> =
        std::collections::BTreeMap::new();

    for (street, status, cnt) in &rows {
        let slot_entry = street_map.entry(street.trim().to_string()).or_insert((0, 0));
        if status == "occupied" { slot_entry.0 += cnt; } else { slot_entry.1 += cnt; }
    }

    let streets: Vec<StreetStat> = street_map
        .into_iter()
        .map(|(name, (occ, free))| {
            let total = occ + free;
            let pct = if total > 0 { (occ as f64 / total as f64) * 100.0 } else { 0.0 };
            StreetStat { name, occupied: occ, free, total, pct }
        })
        .collect();

    let total    = streets.iter().map(|s| s.total).sum();
    let occupied = streets.iter().map(|s| s.occupied).sum();
    let free     = streets.iter().map(|s| s.free).sum();
    let pct      = if total > 0 { (occupied as f64 / total as f64) * 100.0 } else { 0.0 };

    Ok(WarehouseStats { total, occupied, free, pct, streets })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlotResponse {
    pub id: uuid::Uuid,
    pub address: String,
    pub street: String,
    pub position: i16,
    pub lane: String,
    pub status: String,
    pub sku: Option<String>,
    pub updated_by: Option<uuid::Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Slot> for SlotResponse {
    fn from(s: Slot) -> Self {
        Self {
            id: s.id,
            address: s.address,
            street: s.street,
            position: s.position,
            lane: s.lane,
            status: s.status,
            sku: s.sku,
            updated_by: s.updated_by,
            created_at: s.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_at: s.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}