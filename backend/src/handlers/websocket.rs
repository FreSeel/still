use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use tokio::sync::broadcast;

pub async fn images_ws(
    State(tx): State<broadcast::Sender<&'static str>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let rx = tx.subscribe();

    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<&'static str>) {
    loop {
        tokio::select! {
            event = rx.recv() => {
                if matches!(event, Err(broadcast::error::RecvError::Closed)) {
                    break;
                }

                if socket.send(Message::Text("image_uploaded".into())).await.is_err() {
                    break;
                }
            }

            message = socket.recv() => {
                if matches!(message, Some(Ok(Message::Close(_))) | Some(Err(_)) | None) {
                    break;
                }
            }
        }
    }
}
