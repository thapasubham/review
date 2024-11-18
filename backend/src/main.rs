mod admin;
mod members;
mod reviews;
use axum::{
    body::{Body, BoxBody},
    error_handling::{HandleError, HandleErrorLayer},
    extract::Path,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Error, Router,
};

use dotenv;
use reviews::review;
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
        .route("/movie/id", get(reviews::movie::movies))
        .route("/members/login", post(members::user_api::login))
        .route("/member/register", post(members::register::member_register))
        .route(
            "/member/update/username",
            put(members::update::update_username),
        )
        .route(
            "/member/update/password",
            put(members::update::update_password),
        )
        .route("/review/:movie_id", get(reviews::movie::get_details))
        .route("/review/movies/:u_id", get(reviews::movie::get_movies))
        .route("/review/:movie_id/review", get(reviews::movie::get_reviews))
        .route(
            "/member/username/:m_id",
            get(members::user_api::get_username),
        )
        .route("/member/logout", get(members::user_api::logout))
        .route("/admin/login", post(admin::login::login_admin))
        .route("/admin/upload", post(admin::movie::upload))
        .route("/admin/movie/edit", put(admin::movie::edit_movie))
        .route("/admin/movie", get(admin::movie::get_movies))
        .route("/admin/movie/:m_id", get(admin::movie::get_movie))
        .route("/admin/members/user", get(admin::view::get_users))
        .route("/admin/view/movie", get(admin::view::get_movies))
        .route("/review/edit", put(reviews::review::edit))
        .route("/review/add", post(reviews::review::insert))
        .route(
            "/review/delete/:r_id",
            delete(reviews::review::delete_review),
        )
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
