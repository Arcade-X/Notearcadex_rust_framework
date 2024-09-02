use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::Files;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;
use tokio::task;
use crate::ws_login::login_ws_route;
use crate::smtp_server::start_smtp_server; // Import the SMTP server function

mod ws_login;
mod login_handler;
mod mail;
mod smtp_server; // Ensure smtp_server module is included

async fn start_actix_web_server(pool: SqlitePool) -> std::io::Result<()> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    // Start the SMTP server as a background task
    task::spawn(start_smtp_server(pool.clone()));

    // Start the Actix Web server
    start_actix_web_server(pool).await
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