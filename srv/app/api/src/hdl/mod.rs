pub mod sign;
pub mod user;
pub mod captcha;

use actix_web::{HttpResponse, web};

pub async fn ping() -> String {
	"pong".to_string()
}

pub async fn not_found() -> HttpResponse {
	HttpResponse::NotFound().body(r#"{"code":404,"msg":"not found","data":{}}"#)
}

pub async fn demo() -> HttpResponse {
	HttpResponse::Ok().body("Hello world!")
}
