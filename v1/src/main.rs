use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::Files;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;
use crate::ws_login::login_ws_route;

mod ws_login;
mod login_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/ws/login", web::get().to(login_ws_route)) // WebSocket route for login
            .route("/", web::get().to(index)) // Serve the index.html file
            .route("/login", web::get().to(login_page)) // Serve the login.html file
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

async fn login_page(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/pages/login.html"))
}