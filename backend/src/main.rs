mod admin;
mod members;
mod reviews;
use axum::{
    body::{Body, BoxBody},
    error_handling::{HandleError, HandleErrorLayer},
    extract::Path,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Error, Router,
};

use dotenv;
use sqlx::MySqlPool;
use std::env;
use std::net::SocketAddr;
use tokio::fs::File;
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{fs::ServeDir, ServeFile},
};
#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let db = env::var("DATABASE_URL").expect("Failed to load the database url");
    let pool = MySqlPool::connect(&db)
        .await
        .expect("Failed to connect to the database");

    let serve_file = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_err| async {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }))
        .service(ServeDir::new("./src/images"));
    let app = Router::new()
        .nest_service("/image/", serve_file)
        .route("/get_movies", get(reviews::movie::movies))
        .route("/members/login", post(members::user_api::login))
        .route("/member/register", post(members::register::member_register))
        .route("/review/:movie_id", get(reviews::movie::get_details))
        .route("/review/:movie_id/review", get(reviews::movie::get_reviews))
        .route(
            "/member/username/:m_id",
            get(members::user_api::get_username),
        )
        .route("/member/logout", get(members::user_api::logout))
        .route("/admin/login", post(admin::login::login_admin))
        .route("/admin/upload", post(admin::movie::upload))
        .route("/upload", post(admin::movie::what))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            1024 * 1024 * 5,
        ));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// Define the fallible service
