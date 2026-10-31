mod handlers;
mod routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = routes::router();

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, app).await.expect("Server failed");
}
