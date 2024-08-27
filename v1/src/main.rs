use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;

#[derive(Serialize, Deserialize)]
struct Project {
    id: i64,  // Change this from i32 to i64
    name: String,
}

async fn get_projects(pool: web::Data<SqlitePool>) -> impl Responder {
    println!("Attempting to fetch projects...");
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(pool.get_ref())
        .await;

    match projects {
        Ok(proj) => {
            println!("Successfully fetched projects.");
            HttpResponse::Ok().json(proj)
        },
        Err(e) => {
            eprintln!("Failed to fetch projects: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching projects")
        }
    }
}

async fn create_project(pool: web::Data<SqlitePool>, project: web::Json<Project>) -> impl Responder {
    println!("Attempting to create a new project...");
    let result = sqlx::query!("INSERT INTO projects (name) VALUES (?)", project.name)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            println!("Project created successfully.");
            HttpResponse::Ok().json(project.into_inner())
        },
        Err(e) => {
            eprintln!("Failed to create project: {:?}", e);
            HttpResponse::InternalServerError().body("Error creating project")
        }
    }
}

async fn index() -> Result<NamedFile, std::io::Error> {
    println!("Serving index.html...");
    NamedFile::open("./static/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);

    let pool = SqlitePool::connect(&database_url).await.unwrap_or_else(|e| {
        eprintln!("Failed to connect to database: {}", e);
        std::process::exit(1);
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/projects", web::get().to(get_projects))
            .route("/projects", web::post().to(create_project))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
