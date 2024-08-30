use actix_web_actors::ws;
use serde_json::json;
use sqlx::SqlitePool;
use rand::Rng;
use chrono::Utc;
use crate::ws_login::LoginWebSocket;

pub async fn handle_login(
    ctx: &mut ws::WebsocketContext<LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let username = msg["username"].as_str().unwrap().to_string();
    let code = generate_code();
    
    // Get the current timestamp (Unix time)
    let now = Utc::now().timestamp();
    
    // Insert code into the database asynchronously
    let pool_clone = pool.clone();
    let username_clone = username.clone();
    let code_clone = code.clone();

    sqlx::query("INSERT INTO login_codes (username, code, created_at) VALUES (?, ?, ?)")
        .bind(&username_clone)
        .bind(&code_clone)
        .bind(now)
        .execute(&pool_clone)
        .await
        .unwrap();

    // Schedule a task to delete the code after 1 minute
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        sqlx::query("DELETE FROM login_codes WHERE username = ? AND code = ?")
            .bind(&username_clone)
            .bind(&code_clone)
            .execute(&pool_clone)
            .await
            .unwrap();
    });

    // Send the code via email (or log it for testing)
    send_code_to_email(&username, &code);

    // Immediately respond to the WebSocket
    ctx.text(json!({"status": "code_sent"}).to_string());
}

pub async fn handle_code_verification(
    ctx: &mut ws::WebsocketContext<LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let username = msg["username"].as_str().unwrap().to_string();
    let code = msg["code"].as_str().unwrap().to_string();

    let now = Utc::now().timestamp();

    let result = sqlx::query!(
        "SELECT created_at FROM login_codes WHERE username = ? AND code = ? LIMIT 1",
        username,
        code
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(record) = result {
        let created_at = record.created_at;

        if now - created_at <= 60 {
            ctx.text(json!({"status": "login_successful"}).to_string());
        } else {
            ctx.text(json!({"status": "invalid_or_expired_code"}).to_string());
        }
    } else {
        ctx.text(json!({"status": "invalid_or_expired_code"}).to_string());
    }
}

fn generate_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(1000..9999);
    code.to_string()
}

fn send_code_to_email(username: &str, code: &str) {
    // Implement email sending logic here
    // For now, log the code for testing
    println!("Code for {}: {}", username, code);
}