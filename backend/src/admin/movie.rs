use axum::{
    extract::{Multipart, Path, State},
    http::{Response, StatusCode},
    response::{ErrorResponse, IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, MySqlPool};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tracing_subscriber::fmt::format;

#[derive(Serialize)]
pub struct MovieResponse {
    message: String,
    success: bool,
}

// pub async fn what(mut multipart: Multipart) -> impl IntoResponse {
//     while let Some(mut field) = multipart.next_field().await.unwrap() {
//         println!("{}", field.file_name().unwrap().to_string());
//         let file_name = field.file_name().unwrap().to_string();
//         let file_path = format!("src/images/{}", file_name);
//         let mut file = File::create(file_path).await.unwrap();

//         while let Some(chunk) = field.chunk().await.unwrap() {
//             file.write_all(&chunk).await.unwrap();
//         }
//     }
//     (StatusCode::OK, "File uploaded successfully")
// }

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

#[derive(Serialize, Deserialize)]
pub struct EditMoviePayload {
    pub movie_id: i32,
    pub name: String,
    pub released: String,
    pub bio: String,
    pub genre: String,
    
}
pub async fn edit_movie(
    State(pool): State<MySqlPool>,
    Json(payload): Json<EditMoviePayload>,
) -> impl IntoResponse {
    let mut response = MovieResponse {
        message: String::new(),
        success: false,
    };

    // Update the movie in the database
    let result = sqlx::query!(
        "UPDATE movie SET movie_name = ?,  released = ?, about_movie = ?, genre = ? WHERE movie_id = ?",
        payload.name,   
        payload.released,
        payload.bio,
        payload.genre,
        payload.movie_id,
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            response.message = "Movie successfully updated".to_string();
            response.success = true;
        }
        Err(e) => {
            response.message = format!("Failed to update movie: {}", e);
            response.success = false;
        }
    }

    Json(response).into_response()
}

#[derive(Serialize)]
struct Movie {
    movie_id: i32,
    movie_name: String,
    about_movie: String,
    img_name: String,
    genre: String,
    released: i32,
}
pub async  fn get_movies(State(pool): State<MySqlPool>) -> impl IntoResponse{
    let details = query_as!(
        Movie,
        "Select movie_id,  movie_name, about_movie, img_name, genre, released from movie  "
        
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to resolve");
    Json(details)
}

pub async  fn get_movie(State(pool): State<MySqlPool>, Path(m_id): Path<i16>) -> impl IntoResponse{
    let details = query_as!(
        Movie,
        "Select movie_id,  movie_name, about_movie, img_name, genre, released from movie  where movie_id=?",
        m_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to resolve");
    Json(details)
}