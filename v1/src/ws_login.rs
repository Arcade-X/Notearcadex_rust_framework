use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, Error, web};
use actix_web_actors::ws;
use crate::login_handler::{handle_login, handle_code_verification};
use sqlx::SqlitePool;

pub async fn login_ws_route(req: HttpRequest, stream: web::Payload, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    ws::start(LoginWebSocket::new(pool.get_ref().clone()), &req, stream)
}

pub struct LoginWebSocket {
    pool: SqlitePool,
}

impl LoginWebSocket {
    pub fn new(pool: SqlitePool) -> Self {
        LoginWebSocket { pool }
    }
}

impl Actor for LoginWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LoginWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg: serde_json::Value = serde_json::from_str(&text).unwrap();
                let pool = self.pool.clone();

                if msg["action"] == "login" {
                    handle_login(ctx, msg, pool);
                } else if msg["action"] == "verify_code" {
                    handle_code_verification(ctx, msg, pool);
                }
            }
            _ => (),
        }
    }
}