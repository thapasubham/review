use axum::extract::Path;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{query_as, MySqlPool};
use std::env;
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    m_id: i32,
    redirect_to: Option<String>,
    error: Option<String>,
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let mut response = LoginResponse {
        success: false,
        m_id: 0,
        redirect_to: None,
        error: None,
    };

    let row = sqlx::query!(
        "SELECT m_id, password FROM members WHERE username = ?",
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(row) = row {
        if payload.password == row.password {
            response.success = true;
            response.m_id = row.m_id;
            response.redirect_to = Some("../".to_string());
            return ((StatusCode::OK, Json(response))).into_response();
        } else {
            response.error = Some("Invalid password".to_string());
        }
    } else {
        response.error = Some("User does not exist".to_string());
    }

    (StatusCode::UNAUTHORIZED, Json(response)).into_response()
}

#[derive(Serialize, Deserialize)]
struct User {
    firstname: String,
    lastname: String,
    phone: String,
    username: String,
}
pub async fn get_username(
    State(pool): State<MySqlPool>,
    Path(m_id): Path<i32>,
) -> impl IntoResponse {
    // Query to fetch the first name from the database
    let result = sqlx::query_as!(
        User,
        "SELECT firstname, lastname, phone, username FROM members WHERE m_id = ?",
        m_id
    )
    .fetch_one(&pool)
    .await
    .expect("Couldnt find user");

    Json(result)
}

//logout
pub async fn logout() -> impl IntoResponse {
    let logout = r#"{"message": "Logged out successfully"}"#;
    logout
}
