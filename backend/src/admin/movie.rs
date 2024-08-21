use axum::{
    extract::{Multipart, State},
    http::{Response, StatusCode},
    response::{ErrorResponse, IntoResponse, Json},
};
use serde::Serialize;
use sqlx::MySqlPool;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tracing_subscriber::fmt::format;

#[derive(Serialize)]
pub struct MovieResponse {
    message: String,
    success: bool,
}

pub async fn what(mut multipart: Multipart) -> impl IntoResponse {
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

pub async fn upload(State(pool): State<MySqlPool>, mut multipart: Multipart) -> impl IntoResponse {
    let mut name = String::new();
    let mut released = String::new();
    let mut bio = String::new();
    let mut genre = String::new();
    let mut file_name = String::new();
    let mut response = MovieResponse {
        message: "Message".to_string(),
        success: false,
    };

    while let Some(mut field) = multipart
        .next_field()
        .await
        .expect("failed to get next field")
    {
        match field.name() {
            Some("name") => name = field.text().await.unwrap(),
            Some("released") => released = field.text().await.unwrap(),
            Some("bio") => bio = field.text().await.unwrap(),
            Some("genre") => genre = field.text().await.unwrap(),
            Some("image") => {
                println!("{}", name);
                println!("{}", field.file_name().unwrap().to_string());
                file_name = format!("{}.png", name);
                let file_path = format!("src/images/{}", &file_name);
                let mut file = File::create(&file_path).await.unwrap();
                println!("{}", file_path);
                while let Some(chunk) = field.chunk().await.unwrap() {
                    file.write_all(&chunk).await.unwrap();
                }
            }
            _ => {}
        }
    }

    println!("{}", file_name);
    let check = sqlx::query!("SELECT movie_name FROM movie WHERE movie_name = ?", name)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(existing)) = check {
        response.message = "Movie already exists".to_string();
        response.success = false;
    } else {
        // Insert new movie
        let sql = "INSERT INTO movie (movie_name, img_name, released, about_movie, genre) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(name)
            .bind(file_name)
            .bind(released)
            .bind(bio)
            .bind(genre)
            .execute(&pool)
            .await;

        match result {
            Ok(_) => {
                response.message = "Movie successfully added".to_string();
                response.success = true;
            }
            Err(e) => {
                response.message = format!("Failed to add movie: {}", e);
                response.success = false;
            }
        }
    }

    Json(response).into_response()
}
