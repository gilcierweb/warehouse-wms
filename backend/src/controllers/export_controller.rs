use crate::{
    models::{movement::Movement, slot::Slot},
    db::schema::{movements, slots},
    repositories::AppContainer,
};
use actix_web::{web, HttpResponse, Error, get};
use chrono::Utc;
use diesel::prelude::*;
use rust_i18n::t;

// -- GET /api/export/excel

#[get("/export/excel")]
pub async fn export_excel(container: web::Data<AppContainer>) -> Result<HttpResponse, Error> {
    let (all_slots, all_movements) = container
        .run(|conn| {
            let s: Vec<Slot> = slots::table
                .order((slots::street.asc(), slots::lane.asc(), slots::position.asc()))
                .select(Slot::as_select())
                .load(conn)?;

            let m: Vec<Movement> = movements::table
                .order(movements::created_at.desc())
                .limit(1000)
                .select(Movement::as_select())
                .load(conn)?;

            Ok::<_, diesel::result::Error>((s, m))
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError(t!("database.error").to_string()))?;

    // Criar CSV simples como fallback
    let csv_data = build_csv(&all_slots, &all_movements);
    let filename = format!("wms_relatorio_{}.csv", Utc::now().format("%Y%m%d_%H%M%S"));

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .body(csv_data))
}

// -- CSV builder 

fn build_csv(slot_data: &[Slot], mov_data: &[Movement]) -> String {
    let mut csv = String::new();
    
    csv.push_str(&format!("{}\n", t!("export.csv.type")));
    csv.push_str(&format!("{}\n", t!("export.csv.slots_title")));
    csv.push_str(&format!("{}\n", t!("export.csv.slots_header")));
    
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
    
    csv.push_str(&format!("{}\n", t!("export.csv.movements_title")));
    csv.push_str(&format!("{}\n", t!("export.csv.movements_header")));
    
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