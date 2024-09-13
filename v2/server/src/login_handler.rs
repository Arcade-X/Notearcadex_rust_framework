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

    let user = sqlx::query!(
        "SELECT id, password FROM users WHERE username = ? LIMIT 1",
        username
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(user) = user {
        if user.password == password {
            let token = uuid::Uuid::new_v4().to_string();
            sqlx::query!(
                "UPDATE users SET session_token = ? WHERE id = ?",
                token,
                user.id
            )
            .execute(&pool)
            .await
            .unwrap();

            ctx.text(json!({"status": "login_successful", "token": token}).to_string());
        } else {
            ctx.text(json!({"status": "invalid_credentials"}).to_string());
        }
    } else {
        ctx.text(json!({"status": "user_not_found"}).to_string());
    }
}

pub async fn handle_token_verification(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let token = msg["token"].as_str().unwrap_or("");

    let user = sqlx::query!(
        "SELECT id FROM users WHERE session_token = ? LIMIT 1",
        token
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if user.is_some() {
        ctx.text(json!({"status": "token_valid"}).to_string());
    } else {
        ctx.text(json!({"status": "invalid_token"}).to_string());
    }
}

pub async fn handle_projects_request(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let token = msg["token"].as_str().unwrap_or("");

    let user = sqlx::query!(
        "SELECT id FROM users WHERE session_token = ? LIMIT 1",
        token
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if user.is_some() {
        ctx.text(json!({
            "status": "projects_page",
            "content": include_str!("../../static/pages/projects.html")
        }).to_string());
    } else {
        ctx.text(json!({
            "status": "unauthorized"
        }).to_string());
    }
}

pub async fn handle_sandbox_request(
    ctx: &mut ws::WebsocketContext<super::ws_login::LoginWebSocket>,
    msg: serde_json::Value,
    pool: SqlitePool,
) {
    let token = msg["token"].as_str().unwrap_or("");

    let user = sqlx::query!(
        "SELECT id FROM users WHERE session_token = ? LIMIT 1",
        token
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if user.is_some() {
        ctx.text(json!({
            "status": "sandbox_page",
            "content": include_str!("../../static/pages/sandbox.html")
        }).to_string());
    } else {
        ctx.text(json!({
            "status": "unauthorized"
        }).to_string());
    }
}