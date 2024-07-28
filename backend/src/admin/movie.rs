use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        println!("{}", field.file_name().unwrap().to_string());
        let file_name = field.file_name().unwrap().to_string();
        let file_path = format!("src/images/{}", file_name);
        let mut file = File::create(file_path).await.unwrap();

        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
        }
    }
    (StatusCode::OK, "File uploaded successfully")
}
