use rocket::futures::TryFutureExt;
use rocket::{post, State, Responder};
use crate::ServerState;
use rocket::serde::{Deserialize, json::Json};
use crate::bearer_token::BearerToken;
use crate::timeseries::snapshot::InsertSnapshot;

#[derive(Debug, Responder)]
pub enum CreateResponse {
    #[response(status = 200)]
    Created(String),
    #[response(status = 500)]
    Error(String),
    #[response(status = 403)]
    Forbidden(()),
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct PostSnapshot {
    pub illumination: f32,
    pub humidity: f32,
    pub temperature: f32,
    pub voltage: f32,
    pub soil_humidity: f32,
    pub soil_salt: f32,
}

#[post("/log/<mac>", data = "<snapshot>")]
pub async fn log_with_token(state: &State<ServerState>, token: BearerToken, mac: String, snapshot: Json<PostSnapshot>) -> CreateResponse {
    match state
        .auth
        .verify_token(&token)
        .await
    {
        Err(_) => CreateResponse::Forbidden(()),
        Ok(db_mac) => {
            println!("Request with mac {} and db mac {}", mac.to_uppercase(), db_mac);
            if db_mac == mac.to_uppercase() {
                match state.timeseries.add(InsertSnapshot {
                    sensor_mac: mac,
                    illumination: snapshot.illumination,
                    humidity: snapshot.humidity,
                    temperature: snapshot.temperature,
                    voltage: snapshot.voltage,
                    soil_humidity: snapshot.soil_humidity,
                    soil_salt: snapshot.soil_salt,
                }).await {
                    Err(e) => CreateResponse::Error(e.to_string()),
                    Ok(id) => CreateResponse::Created(format!("Inserted with id {}", id)),
                }
            } else {
                CreateResponse::Error("MAC address incorrect.".to_string())
            }
        }
    }
}