use super::auth::AuthenticatedUser;
use super::verificationrole::user_has_role;
use crate::services::users::models::JsonWebTokenClaims;
use crate::AppState;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

pub struct AdminGuard(pub JsonWebTokenClaims);

impl FromRequest for AdminGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let state = req
            .app_data::<web::Data<AppState>>()
            .expect("AppState missing");

        let fut = AuthenticatedUser::from_request(req, payload).into_inner();

        match fut {
            Ok(AuthenticatedUser(claims)) => {
                let db = &state.postgres_client;
                let user_id = &claims.sub;
                let db = db.clone();

                // Atenção: aqui estamos simulando a chamada async com `ready`, mas ideal seria um wrapper async extractor personalizado
                match futures::executor::block_on(user_has_role(&db, user_id, "ADMIN")) {
                    Ok(true) => ready(Ok(AdminGuard(claims))),
                    _ => ready(Err(actix_web::error::ErrorForbidden("Access denied"))),
                }
            }
            Err(e) => ready(Err(e)),
        }
    }
}
