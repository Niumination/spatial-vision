use axum::{extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::IntoResponse, routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Clone)] struct AppState { tx: Arc<broadcast::Sender<String>> }
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 tracing_subscriber::fmt::init(); let (tx,_)=broadcast::channel(64); let state=AppState{tx:Arc::new(tx)};
 let app=Router::new().route("/ws", get(ws)).fallback_service(ServeDir::new("frontend/dist")).layer(TraceLayer::new_for_http()).with_state(state);
 let addr=SocketAddr::from(([0,0,0,0],3000)); println!("Spatial Vision: http://localhost:3000"); let listener=tokio::net::TcpListener::bind(addr).await?; axum::serve(listener,app).await?; Ok(())
}
async fn ws(upgrade: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse { upgrade.on_upgrade(move |socket| handle(socket,state)) }
async fn handle(mut socket: WebSocket, state: AppState) { let mut rx=state.tx.subscribe(); loop { tokio::select! { msg=socket.recv()=>match msg {Some(Ok(Message::Text(t)))=>{let _=state.tx.send(t);}, _=>break}, msg=rx.recv()=>if let Ok(t)=msg {if socket.send(Message::Text(t)).await.is_err(){break}} } } }
