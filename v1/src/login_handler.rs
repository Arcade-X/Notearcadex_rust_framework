use actix_web_actors::ws;
use serde_json::json;
use sqlx::SqlitePool;

pub async fn handle_login(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let username = msg["username"].as_str().unwrap().to_string();
    let password = msg["password"].as_str().unwrap().to_string();

    // Fetch the user's ID and password based on the username
    let user = sqlx::query!(
        "SELECT id, password FROM users WHERE username = ? LIMIT 1",
        username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(user) = user {
        if user.password == password {
            ctx.text(json!({"status": "login_successful"}).to_string());
        } else {
            ctx.text(json!({"status": "invalid_credentials"}).to_string());
        }
    } else {
        ctx.text(json!({"status": "user_not_found"}).to_string());
    }
}