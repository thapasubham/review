use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::{query_as, MySqlPool};
#[derive(Debug, Serialize)]
pub struct User {
    m_id: i32,
    firstname: String,
    lastname: String,
    username: String,
    email: String,
    total_reviews: i64,
}

pub async fn get_users(State(pool): State<MySqlPool>) -> impl IntoResponse {
    let result = query_as!(
        User,
        "SELECT 
    members.m_id, 
    members.firstname, 
    members.lastname, 
    members.username, 
    members.email, 
    COUNT(review.review_id) AS total_reviews
FROM 
    members
LEFT JOIN 
    review 
ON 
    members.m_id = review.reviewed_by
GROUP BY 
    members.m_id;",
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => {
            eprintln!("Error fetching data: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching data").into_response()
        }
    }
}

#[derive(Serialize)]
struct MovieWithReviews {
    movie_id: i32,
    movie_name: String,
    about_movie: String,
    img_name: String,
    genre: String,
    released: i32,
    total_reviews: i64,
}

pub async fn get_movies(State(pool): State<MySqlPool>) -> impl IntoResponse {
    let result = query_as!(
        MovieWithReviews,
        r#"
        SELECT 
            movie.movie_id, 
            movie.movie_name, 
            movie.about_movie, 
            movie.img_name, 
            movie.genre, 
            movie.released, 
            COUNT(review.review_id) AS total_reviews
        FROM 
            movie
        LEFT JOIN 
            review 
        ON 
            movie.movie_id = review.movie_id
        GROUP BY 
            movie.movie_id;
        "#
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(movies) => (StatusCode::OK, Json(movies)).into_response(),
        Err(err) => {
            eprintln!("Error fetching data: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching data").into_response()
        }
    }
}
