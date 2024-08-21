use axum::{extract::State, response::IntoResponse, Json};
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
    let row = sqlx::query!(
        "SELECT admin_id,username, password from admin where username =? ",
        payload.username,
    )
    .fetch_optional(&pool)
    .await;
    if let Ok(Some(row)) = row {
        if row.password == payload.password {
            println!("{}", row.admin_id);
            response.admin_id = row.admin_id;
            response.sucess = true;
        }
    }
    Json(response)
}
