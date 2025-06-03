mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod utils;

use config::Config;
use database::connection::DatabaseConnection;
use rocket::http::Method;
use rocket::{launch, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[launch]
async fn rocket() -> _ {
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Connect to database
    let db = DatabaseConnection::new(&config)
        .await
        .expect("Failed to connect to database");

    // Configure CORS
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), // In production, you should specify exact origins
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration error");

    rocket::build()
        .manage(config)
        .manage(db)
        .attach(cors)
        .mount(
            "/api/auth",
            routes![handlers::auth::signup, handlers::auth::login],
        )
        .mount(
            "/api",
            routes![
                handlers::todo::create_todo,
                handlers::todo::get_todos,
                handlers::todo::get_todo,
                handlers::todo::update_todo,
                handlers::todo::delete_todo
            ],
        )
}
