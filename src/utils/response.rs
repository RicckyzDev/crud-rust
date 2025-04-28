use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub status: u16,
}

impl SuccessResponse {
    pub fn new(message: &str, status: u16) -> Self {
        SuccessResponse {
            message: message.to_string(),
            status,
        }
    }
}

use actix_web::{HttpResponse, Responder};

pub fn success_response(message: &str, status: u16) -> HttpResponse {
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
        .json(SuccessResponse::new(message, status))
}
