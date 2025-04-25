use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, web};
use futures_util::future::{Ready, ready};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::services::users::models::JsonWebTokenClaims;

#[derive(Debug)]
pub struct AuthenticatedUser(pub JsonWebTokenClaims);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    // Note: Config was removed in newer versions of Actix-Web

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req
            .app_data::<web::Data<AppState>>()
            .expect("AppState not found");

        let auth_header = match req.headers().get("Authorization") {
            Some(header_value) => header_value.to_str().unwrap_or(""),
            None => return ready(Err(actix_web::error::ErrorUnauthorized("Missing token"))),
        };

        if !auth_header.starts_with("Bearer ") {
            return ready(Err(actix_web::error::ErrorUnauthorized(
                "Invalid token format",
            )));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        let decoded = decode::<JsonWebTokenClaims>(
            token,
            &DecodingKey::from_secret(state.json_web_token.as_ref()),
            &Validation::default(),
        );

        match decoded {
            Ok(token_data) => ready(Ok(AuthenticatedUser(token_data.claims))),
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
        }
    }
}
