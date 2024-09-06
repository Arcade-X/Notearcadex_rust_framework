use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::Files;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;
use std::fs;
use crate::ws_login::login_ws_route;

mod ws_login;
mod login_handler;

async fn start_actix_web_server(pool: web::Data<SqlitePool>) -> std::io::Result<()> {
    let pool_clone = pool.clone(); // Clone once and reuse
    HttpServer::new(move || {
        App::new()
            .app_data(pool_clone.clone()) // Use the cloned pool
            .service(Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/ws/login", web::get().to(login_ws_route)) // WebSocket route for login
            .route("/", web::get().to(index)) // Serve the index.html file
            .route("/login", web::get().to(login_page)) // Serve the login.html file
            .route("/projects", web::get().to({
                let pool = pool_clone.clone(); // Clone inside the closure
                move |req| projects_page(req, pool.clone())
            })) // Serve the projects.html file
            .route("/sandbox", web::get().to({
                let pool = pool_clone.clone(); // Clone inside the closure
                move |req| sandbox_page(req, pool.clone())
            })) // Serve the sandbox.html file
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
    validate_token_and_serve_page(req, pool, "../static/pages/projects.html").await
}

async fn sandbox_page(req: HttpRequest, pool: web::Data<SqlitePool>) -> HttpResponse {
    validate_token_and_serve_page(req, pool, "../static/pages/sandbox.html").await
}

async fn validate_token_and_serve_page(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    page_path: &'static str,
) -> HttpResponse {
    if let Some(token) = req.headers().get("Authorization") {
        let token_str = token.to_str().unwrap_or("");
        println!("Authorization token: {}", token_str); // Debugging output

        let user = sqlx::query!(
            "SELECT id FROM users WHERE session_token = ? LIMIT 1",
            token_str
        )
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

        if let Some(_) = user {
            // Token is valid, serve the requested page
            println!("Valid token. Serving page: {}", page_path); // Debugging output
            let page_content = fs::read_to_string(page_path).unwrap_or_else(|_| String::from("Error loading page"));
            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(page_content);
        } else {
            println!("Invalid token. Redirecting to login."); // Debugging output
        }
    } else {
        println!("No token provided. Redirecting to login."); // Debugging output
    }

    // Token is invalid or not provided, redirect to login
    HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}