use still_backend::image_storage::{
    ImageMetadata,
    process_upload,
};

use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};
use std::path::Path;
use tokio::fs;

fn create_test_png(color: [u8; 4]) -> Vec<u8> {
    let mut bytes = Vec::new();

    PngEncoder::new(&mut bytes)
        .write_image(&color, 1, 1, ExtendedColorType::Rgba8)
        .expect("Failed to generate test PNG");

    bytes
}

async fn count_files(dir: &Path) -> usize {
    let mut entries = fs::read_dir(dir).await.expect("Failed to read directory");

    let mut count = 0;

    while entries
        .next_entry()
        .await
        .expect("Failed to inspect directory")
        .is_some()
    {
        count += 1;
    }

    count
}

#[tokio::test]
async fn uploads_are_isolated_between_storage_directories() {
    // Arrange: independent temporary directories
    let dir_a = tempfile::tempdir().expect("Failed to create directory A");

    let dir_b = tempfile::tempdir().expect("Failed to create directory B");

    let image_a = create_test_png([255, 0, 0, 255]);
    let image_b = create_test_png([0, 0, 255, 255]);

    let metadata_a = ImageMetadata {
        title: "Image A".into(),
        tags: vec!["red".into()],
    };

    let metadata_b = ImageMetadata {
        title: "Image B".into(),
        tags: vec!["blue".into()],
    };

    // Act: upload to separate storage directories
    let path_a = process_upload(&image_a, metadata_a, dir_a.path())
        .await
        .expect("Upload A failed");

    let path_b = process_upload(&image_b, metadata_b, dir_b.path())
        .await
        .expect("Upload B failed");

    // Assert: images stored in the correct directories
    assert!(path_a.starts_with(dir_a.path()));
    assert!(path_b.starts_with(dir_b.path()));

    // Assert: uploaded contents are independent
    assert_eq!(fs::read(&path_a).await.unwrap(), image_a);

    assert_eq!(fs::read(&path_b).await.unwrap(), image_b);

    // Assert: metadata belongs to the correct image
    let saved_a: ImageMetadata =
        serde_json::from_slice(&fs::read(path_a.with_extension("json")).await.unwrap()).unwrap();

    let saved_b: ImageMetadata =
        serde_json::from_slice(&fs::read(path_b.with_extension("json")).await.unwrap()).unwrap();

    assert_eq!(saved_a.title, "Image A");
    assert_eq!(saved_a.tags, vec!["red"]);

    assert_eq!(saved_b.title, "Image B");
    assert_eq!(saved_b.tags, vec!["blue"]);

    // Assert: each directory contains only its own
    // image and corresponding metadata file
    assert_eq!(count_files(dir_a.path()).await, 2);
    assert_eq!(count_files(dir_b.path()).await, 2);
}
