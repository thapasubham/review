use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query, MySqlPool};
#[derive(Serialize, Deserialize)]
pub struct newReview {
    reviewed_by: i16,
    movie_id: i8,
    review_msg: String,
    star: i8,
}

pub async fn insert(
    State(pool): State<MySqlPool>,
    Json(payload): Json<newReview>,
) -> impl IntoResponse {
    let result = sqlx::query!("INSERT INTO `review` (`reviewed_by`, `movie_id`, `review_msg`, `star`) VALUES ( ?, ?, ?, ?)", 
    payload.reviewed_by,
    payload.movie_id,
    payload.review_msg,
    payload.star
).execute(&pool).await;

    let reply = match result {
        Ok(_) => String::from("Added your review"),
        Err(_) => String::from("Failed to add review"),
    };

    Json(reply)
}
