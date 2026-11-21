use std::{path::PathBuf, time::Duration};

use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};
use serde_json::Value;
use tokio::time::timeout;

use still_backend::routes::router_with_images_dir;

fn create_test_server(storage_dir: PathBuf) -> TestServer {
    TestServer::builder()
        .http_transport()
        .build(router_with_images_dir(storage_dir))
}

fn create_upload_form(title: &str) -> MultipartForm {
    let mut image_bytes = Vec::new();

    PngEncoder::new(&mut image_bytes)
        .write_image(&[255, 0, 0, 255], 1, 1, ExtendedColorType::Rgba8)
        .expect("Failed to generate test image");

    MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(image_bytes)
                .file_name("test.png")
                .mime_type("image/png"),
        )
        .add_text("title", title)
        .add_text("tags", "testing")
}

#[tokio::test]
async fn websocket_notifies_multiple_clients_after_upload() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let server = create_test_server(temp_dir.path().to_path_buf());

    let mut client_a = server
        .get_websocket("/ws/images")
        .await
        .into_websocket()
        .await;
    let mut client_b = server
        .get_websocket("/ws/images")
        .await
        .into_websocket()
        .await;

    let response = server
        .post("/uploads")
        .multipart(create_upload_form("Multiple Clients"))
        .await;

    response.assert_status(StatusCode::CREATED);

    let (event_a, event_b) = timeout(Duration::from_secs(5), async {
        tokio::join!(client_a.receive_text(), client_b.receive_text())
    })
    .await
    .expect("Timed out waiting for WebSocket notifications");

    assert_eq!(event_a, "image_uploaded");
    assert_eq!(event_b, "image_uploaded");

    client_a.close().await;
    client_b.close().await;
}

#[tokio::test]
async fn websocket_handles_disconnect_and_reconnect() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let server = create_test_server(temp_dir.path().to_path_buf());

    let initial_client = server
        .get_websocket("/ws/images")
        .await
        .into_websocket()
        .await;
    initial_client.close().await;

    let disconnected_response = server
        .post("/uploads")
        .multipart(create_upload_form("Uploaded While Disconnected"))
        .await;
    disconnected_response.assert_status(StatusCode::CREATED);

    let mut reconnected_client = server
        .get_websocket("/ws/images")
        .await
        .into_websocket()
        .await;

    let reconnected_response = server
        .post("/uploads")
        .multipart(create_upload_form("Uploaded After Reconnect"))
        .await;
    reconnected_response.assert_status(StatusCode::CREATED);

    let event = timeout(Duration::from_secs(5), reconnected_client.receive_text())
        .await
        .expect("Timed out waiting for notification after reconnect");

    assert_eq!(event, "image_uploaded");
    reconnected_client.close().await;
}

#[tokio::test]
async fn websocket_notification_matches_uploaded_feed_entry() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let server = create_test_server(temp_dir.path().to_path_buf());
    let mut client = server
        .get_websocket("/ws/images")
        .await
        .into_websocket()
        .await;

    let upload_response = server
        .post("/uploads")
        .multipart(create_upload_form("Feed Consistency"))
        .await;
    upload_response.assert_status(StatusCode::CREATED);

    let uploaded_path = PathBuf::from(upload_response.text());
    let uploaded_filename = uploaded_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("Invalid uploaded filename");

    let event = timeout(Duration::from_secs(5), client.receive_text())
        .await
        .expect("Timed out waiting for upload notification");
    assert_eq!(event, "image_uploaded");

    let feed_response = server.get("/images").await;
    feed_response.assert_status(StatusCode::OK);

    let feed: Value = serde_json::from_str(&feed_response.text()).expect("Invalid feed JSON");
    let entries = feed.as_array().expect("Feed response should be an array");

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["title"], "Feed Consistency");
    assert_eq!(entries[0]["tags"], serde_json::json!(["testing"]));
    assert_eq!(entries[0]["url"], format!("/images/{uploaded_filename}"));

    client.close().await;
}
