use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use tokio::sync::broadcast;
use tracing::{info, warn};

pub async fn images_ws(
    State(tx): State<broadcast::Sender<&'static str>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let rx = tx.subscribe();

    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<&'static str>) {
    info!("WebSocket client connected");

    loop {
        tokio::select! {
            event = rx.recv() => {
                if matches!(event, Err(broadcast::error::RecvError::Closed)) {
                    warn!("WebSocket broadcast channel closed");
                    break;
                }

                if socket.send(Message::Text("image_uploaded".into())).await.is_err() {
                    warn!("Failed to send WebSocket notification!");
                    break;
                }

                info!("Image upload notification sent");
            }

            message = socket.recv() => {
                if matches!(message, Some(Ok(Message::Close(_))) | Some(Err(_)) | None) {
                    info!("WebSocket client closed conenction");
                    break;
                }

                if let Some(Err(err)) = message {
                    warn!("WebSocket connection error: {err}");
                    break;
                }
            }
        }
    }

    info!("WebSocket client disconnectedd");
}
