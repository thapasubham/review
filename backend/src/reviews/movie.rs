use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{mysql::MySqlPool, query, query_as, types::BigDecimal};
use tower::util::Optional;

#[derive(Serialize, Deserialize)]
struct Movie {
    movie_id: i32,
    movie_name: String,
    img_name: String,
    genre: String,
    released: i32,
}

#[derive(Serialize, Deserialize)]
struct Review {
    review_id: i32,
    review_msg: String,
    star: i32,
    reviewed_by: i32,
    firstname: String,
}
pub async fn get_reviews(
    State(pool): State<MySqlPool>,
    Path(movie_id): Path<i32>,
) -> impl IntoResponse {
    let review = query_as!(
        Review,
        "Select review.review_id, review.review_msg, review.star, review.reviewed_by, members.firstname from review  INNER JOIN members on review.reviewed_by = m_id where movie_id=?",
        movie_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to load data");

    Json(review)
}

pub async fn get_details(
    State(pool): State<MySqlPool>,
    Path(movie_id): Path<i32>,
) -> impl IntoResponse {
    let details = query_as!(
        Movie,
        "Select movie_id,  movie_name, img_name, genre, released from movie where movie_id =? ",
        movie_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to resolve");

    let avg_result = sqlx::query!(
        "SELECT AVG(star) AS average_star FROM review WHERE movie_id = ?",
        movie_id
    )
    .fetch_one(&pool)
    .await;

    let average_star_str = match avg_result.ok().and_then(|r| r.average_star) {
        Some(avg) => avg.to_string(),
        None => "0.0".to_string(),
    };

    let respond = json!({
        "movie": details,
        "star": average_star_str,
    });

    Json(respond)
}

#[derive(Serialize, Deserialize)]
struct Movie_ID {
    movie_id: i32,
}

pub async fn movies(State(pool): State<MySqlPool>) -> impl IntoResponse {
    let result = query_as!(Movie_ID, "SELECT movie_id FROM movie ")
        .fetch_all(&pool)
        .await
        .expect("Failed to get the movie details");
    Json(result)
}
