use axum::extract::Path;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
#[derive(Debug, Deserialize)]
pub struct UpdateUsername {
    username: String,
    user_id: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePassword {
    password: String,
    user_id: i16,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    message: String,
}

pub async fn update_username(
    State(pool): State<MySqlPool>,
    Json(payload): Json<UpdateUsername>,
) -> impl IntoResponse {
    // println!("Received payload: {:?}", payload); // Add this line

    let result = sqlx::query!(
        "UPDATE members SET username = ? WHERE m_id = ?",
        payload.username,
        payload.user_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(UpdateResponse {
            message: "Username updated successfully".into(),
        })
        .into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update username".to_string(),
        )
            .into_response(),
    }
}

pub async fn update_password(
    State(pool): State<MySqlPool>,
    Json(payload): Json<UpdatePassword>,
) -> impl IntoResponse {
    // println!("{:?}", payload);

    let result = sqlx::query!(
        "UPDATE members SET password = ? WHERE m_id = ?",
        payload.password,
        payload.user_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(UpdateResponse {
            message: "Password updated successfully".into(),
        })
        .into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update password".to_string(),
        )
            .into_response(),
    }
}
