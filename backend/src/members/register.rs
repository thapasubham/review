use axum::{extract::State, response::IntoResponse, Form, Json};
use serde::{de, Deserialize, Serialize};
use sqlx::{pool, query, Error, MySqlPool};
use tower_http::classify::StatusInRangeFailureClass;
use tracing_subscriber::field::display::Messages;

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
    firstname: String,
    lastname: String,
    username: String,
    phone: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct message {
    response: Vec<String>,
    result: String,
}

pub async fn member_register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let mut reponse: String = "".to_owned();
    let result = query!(
        "Select username, email, phone from members where username =? or email =? or phone =?",
        payload.username,
        payload.email,
        payload.phone
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to query");

    let mut response = message {
        response: Vec::new(),
        result: "error".to_string(),
    };

    if let Some(existing) = result {
        if payload.username == existing.username {
            response
                .response
                .push("Username already exists".to_string());
        }
        if payload.email == existing.email {
            response
                .response
                .push("User with same email exists".to_string());
        }
        if payload.phone == existing.phone {
            response
                .response
                .push("User with same phone number exists".to_string());
        }
    }

    if response.response.is_empty() {
        let insert = sqlx::query!(
            "INSERT INTO members (firstname, lastname, username, phone, email, password)
             VALUES (?, ?, ?, ?, ?, ?)",
            payload.firstname,
            payload.lastname,
            payload.username,
            payload.phone,
            payload.email,
            payload.password
        )
        .execute(&pool)
        .await;

        let message = match insert {
            Ok(_) => String::from("Registered Successfully"),
            Err(_) => String::from("Couldn't register"),
        };

        response.response.push(message);
        response.result = "Sucess".to_string();
    }

    Json(response)
}
