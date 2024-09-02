use actix_web_actors::ws;
use serde_json::json;
use sqlx::SqlitePool;
use chrono::Utc;
use crate::mail::send_code_to_email;
use rand::Rng; // Import the Rng trait

pub async fn handle_login(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let username = msg["username"].as_str().unwrap().to_string();

    // Generate a random 4-digit code
    let code = generate_code();

    // Get the current timestamp (Unix time)
    let now = Utc::now().timestamp();

    // Fetch the user's ID and email based on the username
    let user = sqlx::query!(
        "SELECT id, email FROM users WHERE username = ? LIMIT 1",
        username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(user) = user {
        let user_id = user.id;
        let email = user.email;

        // Insert or update the login code for the user
        sqlx::query!(
            "INSERT INTO login_codes (user_id, code, created_at) VALUES (?, ?, ?)
            ON CONFLICT(user_id) DO UPDATE SET code = excluded.code, created_at = excluded.created_at",
            user_id,
            code,
            now
        )
        .execute(&pool)
        .await
        .unwrap();

        // Send the code via email
        send_code_to_email(&email, &username, &code).await;

        // Send a response to the WebSocket
        ctx.text(json!({"status": "code_sent"}).to_string());

        // Schedule a task to delete the code after 1 minute
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            sqlx::query("DELETE FROM login_codes WHERE user_id = ? AND code = ?")
                .bind(&user_id)
                .bind(&code)
                .execute(&pool_clone)
                .await
                .unwrap();
        });
    } else {
        ctx.text(json!({"status": "user_not_found"}).to_string());
    }
}

pub async fn handle_code_verification(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let username = msg["username"].as_str().unwrap().to_string();
    let code = msg["code"].as_str().unwrap().to_string();

    let now = Utc::now().timestamp();

    // Fetch the user's ID based on the username
    let user = sqlx::query!(
        "SELECT id FROM users WHERE username = ? LIMIT 1",
        username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(user) = user {
        let user_id = user.id;

        let result = sqlx::query!(
            "SELECT created_at FROM login_codes WHERE user_id = ? AND code = ? LIMIT 1",
            user_id,
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
    } else {
        ctx.text(json!({"status": "user_not_found"}).to_string());
    }
}

fn generate_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(1000..9999);
    code.to_string()
}