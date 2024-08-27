use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};

async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

async fn projects(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/pages/projects.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/", web::get().to(index)) // Serve the index.html file
            .route("/projects", web::get().to(projects)) // Serve the projects.html file
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}