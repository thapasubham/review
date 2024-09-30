use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query, MySqlPool};
#[derive(Serialize, Deserialize)]
pub struct Review {
    reviewed_by: i16,
    movie_id: i8,
    review_msg: String,
    star: i8,
}

pub async fn insert(
    State(pool): State<MySqlPool>,
    Json(payload): Json<Review>,
) -> impl IntoResponse {
    let result = sqlx::query!("INSERT INTO `review` (`reviewed_by`, `movie_id`, `review_msg`, `star`) VALUES ( ?, ?, ?, ?)", 
    payload.reviewed_by,
    payload.movie_id,
    payload.review_msg,
    payload.star
).execute(&pool).await;

    let reply = match result {
        Ok(query_result) => {
            let affected = query_result.rows_affected();
            if affected > 0 {
                String::from("Added your review")
            } else {
                String::from("Couldnt add")
            }
        }
        Err(_) => String::from("Failed to add review"),
    };

    Json(reply)
}

pub async fn edit(State(pool): State<MySqlPool>, Json(payload): Json<Review>) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE review SET review_msg=?, star=?
             WHERE movie_id=? and reviewed_by=?",
        payload.review_msg,
        payload.star,
        payload.movie_id,
        payload.reviewed_by,
    )
    .execute(&pool)
    .await;

    let reply = match result {
        Ok(query_result) => {
            let affected = query_result.rows_affected();
            if affected > 0 {
                String::from("Edited your review")
            } else {
                String::from("No review was updated (check movie_id or review_id)")
            }
        }
        Err(_) => String::from("Failed to edit review"),
    };

    // println!("{}", review_id);
    Json(reply)
}

pub async fn delete_review(
    State(pool): State<MySqlPool>,
    Path(r_id): Path<i16>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM review WHERE review_id = ?", r_id)
        .execute(&pool)
        .await;

    let reply = match result {
        Ok(_) => String::from("Deleted the review successfully"),
        Err(_) => String::from("Failed to delete the review"),
    };

    Json(reply)
}
