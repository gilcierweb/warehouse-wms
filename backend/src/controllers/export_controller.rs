use crate::{
    db::database::Database,
    models::{movement::Movement, slot::Slot},
    db::schema::{movements, slots},
};
use actix_web::{web, HttpResponse, Error, get};
use chrono::Utc;
use diesel::prelude::*;

// ── GET /api/export/excel ─────────────────────────────────────

#[get("/export/excel")]
pub async fn export_excel(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().map_err(|_| actix_web::error::ErrorInternalServerError("Database connection error"))?;

    let Ok((all_slots, all_movements)) = web::block(move || -> Result<_, diesel::result::Error> {
        let s: Vec<Slot> = slots::table
            .order((slots::street.asc(), slots::lane.asc(), slots::position.asc()))
            .select(Slot::as_select())
            .load(&mut conn)?;

        let m: Vec<Movement> = movements::table
            .order(movements::created_at.desc())
            .limit(1000)
            .select(Movement::as_select())
            .load(&mut conn)?;

        Ok((s, m))
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))? else {
        return Err(actix_web::error::ErrorInternalServerError("Database error"));
    };

    // Criar CSV simples como fallback
    let csv_data = build_csv(&all_slots, &all_movements);
    let filename = format!("wms_relatorio_{}.csv", Utc::now().format("%Y%m%d_%H%M%S"));

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .body(csv_data))
}

// ── CSV builder ─────────────────────────────────────────────

fn build_csv(slot_data: &[Slot], mov_data: &[Movement]) -> String {
    let mut csv = String::new();
    
    // Header slots
    csv.push_str("TIPO,DADOS\n");
    csv.push_str("SLOTS,\n");
    csv.push_str("Endereço,Rua,Lane,Posição,Status,SKU,Atualizado em\n");
    
    for slot in slot_data {
        csv.push_str(&format!("{},{},{},{},{},{},{}\n",
            slot.address,
            slot.street.trim(),
            slot.lane,
            slot.position,
            slot.status,
            slot.sku.as_deref().unwrap_or(""),
            slot.updated_at.format("%d/%m/%Y %H:%M:%S")
        ));
    }
    
    // Header movements
    csv.push_str("MOVEMENTS,\n");
    csv.push_str("ID,Endereço,Tipo,Operador,SKU,Data/Hora,Obs.\n");
    
    for mov in mov_data {
        csv.push_str(&format!("{},{},{},{},{},{},{}\n",
            mov.id,
            mov.slot_id.map(|id| id.to_string()).unwrap_or_default(),
            mov.movement_type,
            mov.operator_name.as_deref().unwrap_or(""),
            mov.sku.as_deref().unwrap_or(""),
            mov.created_at.format("%d/%m/%Y %H:%M:%S"),
            mov.note.as_deref().unwrap_or("")
        ));
    }
    
    csv
}