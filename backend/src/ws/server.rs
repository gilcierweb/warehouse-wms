use actix_web::web;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::broadcast;
use futures_util::StreamExt;

// -- WsEvent 

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
    pub fn slot_updated(slot: &impl Serialize, action: &str) -> Self {
        let mut payload = serde_json::to_value(slot).unwrap();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("action".to_string(), serde_json::Value::String(action.to_string()));
        }
        Self { event: WsEventKind::SlotUpdated, payload }
    }

    pub fn slot_entry(slot: &impl Serialize) -> Self {
        Self::slot_updated(slot, "entry")
    }

    pub fn slot_exit(slot: &impl Serialize) -> Self {
        Self::slot_updated(slot, "exit")
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
use actix_ws::{Message};

pub async fn ws_handler(
    req:  HttpRequest,
    body: web::Payload,
    hub:  HubData,
) -> actix_web::Result<HttpResponse> {
    println!("DEBUG: WebSocket connection requested");
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;
    println!("DEBUG: WebSocket handshake successful");

    let mut rx = hub.subscribe();

    // WebSocket handler with broadcast
    actix_rt::spawn(async move {
        println!("DEBUG: WebSocket task started");
        
        // Send welcome message
        if session.text("Connected to WebSocket server").await.is_err() {
            println!("DEBUG: Failed to send welcome message");
            return;
        }
        println!("DEBUG: Welcome message sent");
        
        loop {
            tokio::select! {
                // Forward broadcast messages to this client
                Ok(msg) = rx.recv() => {
                    println!("DEBUG: Broadcasting message: {}", msg);
                    if session.text(msg).await.is_err() {
                        println!("DEBUG: Failed to send broadcast, breaking");
                        break;
                    }
                }

                // Handle incoming WS frames (ping/close)
                result = stream.next() => {
                    match result {
                        Some(Ok(msg)) => {
                            match msg {
                                Message::Ping(bytes) => {
                                    println!("DEBUG: Received ping, sending pong");
                                    if session.pong(&bytes).await.is_err() {
                                        println!("DEBUG: Failed to send pong");
                                        break;
                                    }
                                }
                                Message::Text(text) => {
                                    println!("DEBUG: Received text: {}", text);
                                    if session.text(format!("Echo: {}", text)).await.is_err() {
                                        break;
                                    }
                                }
                                Message::Close(reason) => {
                                    println!("DEBUG: Received close message: {:?}", reason);
                                    let _ = session.close(reason).await;
                                    break;
                                }
                                Message::Pong(_) => {
                                    println!("DEBUG: Received pong");
                                }
                                _ => {}
                            }
                        }
                        Some(Err(e)) => {
                            println!("DEBUG: WebSocket error: {:?}", e);
                            break;
                        }
                        None => {
                            println!("DEBUG: WebSocket stream ended");
                            break;
                        }
                    }
                }
            }
        }
        println!("DEBUG: WebSocket task ended");
    });

    Ok(response)
}