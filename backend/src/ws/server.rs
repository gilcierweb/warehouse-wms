use actix_web::web;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::broadcast;
use futures_util::StreamExt;

// ── WsEvent ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsEventKind {
    SlotUpdated,
    StatsUpdated,
    Alert,
}

#[derive(Debug, Clone, Serialize)]
pub struct WsEvent {
    pub event:   WsEventKind,
    pub payload: Value,
}

impl WsEvent {
    pub fn slot_updated(slot: &impl Serialize) -> Self {
        Self { event: WsEventKind::SlotUpdated, payload: serde_json::to_value(slot).unwrap() }
    }

    pub fn stats_updated(stats: &impl Serialize) -> Self {
        Self { event: WsEventKind::StatsUpdated, payload: serde_json::to_value(stats).unwrap() }
    }

    pub fn alert(message: String, pct: f64) -> Self {
        Self {
            event: WsEventKind::Alert,
            payload: serde_json::json!({ "message": message, "pct": pct }),
        }
    }
}

// ── Hub ───────────────────────────────────────────────────────

#[derive(Clone)]
pub struct WsHub {
    sender: Arc<broadcast::Sender<String>>,
}

impl WsHub {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender: Arc::new(sender) }
    }

    /// Broadcast an event to all connected WebSocket clients.
    pub fn broadcast(&self, event: WsEvent) {
        if let Ok(json) = serde_json::to_string(&event) {
            let _ = self.sender.send(json);
        }
    }

    /// Subscribe to the broadcast channel.
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
}

pub type HubData = web::Data<WsHub>;

// ── WebSocket actor handler ───────────────────────────────────

use actix_web::{HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;

pub async fn ws_handler(
    req:  HttpRequest,
    body: web::Payload,
    hub:  HubData,
) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    let mut rx = hub.subscribe();

    // Spawn the WS task
    actix_rt::spawn(async move {
        loop {
            tokio::select! {
                // Forward broadcast messages to this client
                Ok(msg) = rx.recv() => {
                    if session.text(msg).await.is_err() {
                        break;
                    }
                }

                // Handle incoming WS frames (ping/close)
                Some(Ok(msg)) = stream.next() => {
                    match msg {
                        AggregatedMessage::Ping(bytes) => {
                            if session.pong(&bytes).await.is_err() { break; }
                        }
                        AggregatedMessage::Close(reason) => {
                            let _ = session.close(reason).await;
                            break;
                        }
                        _ => {}
                    }
                }

                else => break,
            }
        }
    });

    Ok(response)
}
