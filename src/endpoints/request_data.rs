use rocket::{get, State, response::Responder, serde::json::Json};
use rocket::futures::TryFutureExt;
use crate::ServerState;
use crate::bearer_token::BearerToken;
use crate::timeseries::snapshot::Snapshot;

#[derive(Debug, Responder)]
pub enum VerifyResponse {
    #[response(status = 200)]
    Valid(Json<Vec<Snapshot>>),
    #[response(status = 403)]
    Forbidden(()),
    #[response(status = 500)]
    InternalServerError(String),
}

#[get("/request/all/last")]
pub async fn request_once(state: &State<ServerState>, token: BearerToken) -> VerifyResponse {
    match state
        .token
        .verify(&token)
        .await
    {
        Err(_) => VerifyResponse::Forbidden(()),
        Ok(_) => match state.timeseries.get_all_last().await {
            Ok(e) => VerifyResponse::Valid(Json(e)),
            Err(e) => VerifyResponse::InternalServerError(e.to_string()),
        }
    }
}