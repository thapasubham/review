use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AdminResponse {
    admin_id: i32,
    sucess: bool,
}

pub async fn login_admin(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let mut response = AdminResponse {
        admin_id: 0,
        sucess: false,
    };

    // Debugging output
    println!("Received login attempt for user: {}", payload.username);

    let row = sqlx::query!(
        "SELECT admin_id, username, password FROM admin WHERE username = ?",
        payload.username,
    )
    .fetch_optional(&pool)
    .await;

    if let Ok(Some(row)) = row {
        println!("{}  {}", row.username, row.password);
        // Verify password
        if (&payload.password == &row.password) {
            println!("{}", row.admin_id);
            response.admin_id = row.admin_id;
            response.sucess = true;
        }
    }

    if response.sucess {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::UNAUTHORIZED, Json(response))
    }
}
