use actix::{Actor, StreamHandler, WrapFuture, ContextFutureSpawner};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use sqlx::SqlitePool;
use serde_json::Value;
use crate::login_handler::{handle_login, handle_token_verification, handle_projects_request, handle_sandbox_request};

pub async fn login_ws_route(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    ws::start(LoginWebSocket { pool: pool.get_ref().clone() }, &req, stream)
}

pub struct LoginWebSocket {
    pool: SqlitePool,
}

impl Actor for LoginWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LoginWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let parsed_msg: Value = serde_json::from_str(&text).unwrap();
            let pool_clone = self.pool.clone();

            if parsed_msg["action"] == "login" {
                let ctx_ref = ctx as *mut _; // Convert to raw pointer
                let fut = async move {
                    let ctx_ref = unsafe { &mut *ctx_ref }; // Convert back to reference
                    handle_login(ctx_ref, parsed_msg, pool_clone).await;
                };
                fut.into_actor(self).spawn(ctx);
            } else if parsed_msg["action"] == "verify_token" {
                let ctx_ref = ctx as *mut _; // Convert to raw pointer
                let fut = async move {
                    let ctx_ref = unsafe { &mut *ctx_ref }; // Convert back to reference
                    handle_token_verification(ctx_ref, parsed_msg, pool_clone).await;
                };
                fut.into_actor(self).spawn(ctx);
            } else if parsed_msg["action"] == "request_projects" {
                let ctx_ref = ctx as *mut _; // Convert to raw pointer
                let fut = async move {
                    let ctx_ref = unsafe { &mut *ctx_ref }; // Convert back to reference
                    handle_projects_request(ctx_ref, parsed_msg, pool_clone).await;
                };
                fut.into_actor(self).spawn(ctx);
            } else if parsed_msg["action"] == "request_sandbox" {  // Handle sandbox request
                let ctx_ref = ctx as *mut _; // Convert to raw pointer
                let fut = async move {
                    let ctx_ref = unsafe { &mut *ctx_ref }; // Convert back to reference
                    handle_sandbox_request(ctx_ref, parsed_msg, pool_clone).await;
                };
                fut.into_actor(self).spawn(ctx);
            }
        }
    }
}