use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::Files;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;
use crate::ws_login::login_ws_route;

mod ws_login;
mod login_handler;

async fn start_actix_web_server(pool: web::Data<SqlitePool>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let pool_clone = pool.clone(); // Clone the pool here to avoid moving it
        App::new()
            .app_data(pool.clone()) // Clone the pool here to pass it to the App
            .service(Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/ws/login", web::get().to(login_ws_route)) // WebSocket route for login
            .route("/", web::get().to(index)) // Serve the index.html file
            .route("/login", web::get().to(login_page)) // Serve the login.html file
            .route("/projects", web::get().to(move |req| projects_page(req, pool_clone.clone()))) // Serve the projects.html file
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

    // Connect to the database and wrap it in web::Data
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    let pool_data = web::Data::new(pool); // Wrap the pool in web::Data

    // Start the Actix Web server
    start_actix_web_server(pool_data).await
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

async fn projects_page(req: HttpRequest, pool: web::Data<SqlitePool>) -> HttpResponse {
    if let Some(token) = req.headers().get("Authorization") {
        let token_str = token.to_str().unwrap_or("");

        // Check if the token is valid by querying the database
        let user = sqlx::query!(
            "SELECT id FROM users WHERE session_token = ? LIMIT 1",
            token_str
        )
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

        if user.is_some() {
            // Token is valid, serve the projects page
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(include_str!("../static/pages/projects.html"))
        } else {
            // Token is invalid, redirect to login
            HttpResponse::Found()
                .append_header(("Location", "/login")) // Updated to append_header
                .finish()
        }
    } else {
        // No token provided, redirect to login
        HttpResponse::Found()
            .append_header(("Location", "/login")) // Updated to append_header
            .finish()
    }
}