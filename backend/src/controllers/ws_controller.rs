use actix_web::{HttpResponse, Result, get};
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct WsTokenResponse {
    pub token: String,
}

#[get("/ws/token")]
pub async fn get_ws_token() -> Result<HttpResponse> {
    // Generate a temporary token for WebSocket connection
    let token = Uuid::new_v4().to_string();
    Ok(HttpResponse::Ok().json(WsTokenResponse { token }))
}
