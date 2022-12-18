use rocket::{post, State, Responder};
use crate::ServerState;
use rocket::serde::{Deserialize, json::Json};
use crate::bearer_token::BearerToken;
use crate::timeseries::snapshot::InsertSnapshot;
use rocket::request::FromSegments;
use rocket::http::uri::Segments;
use rocket::http::uri::fmt::Path;

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

pub struct Uid {
    pub uid: String,
}

impl FromSegments<'_> for Uid {
    type Error = String;

    fn from_segments(segments: Segments<'_, Path>) -> Result<Self, Self::Error> {
        let uid: String = segments.into_iter().intersperse("/").collect();
        if uid.len() > 0 {
            Ok(Self {
                uid: uid
            })
        } else {
            Err("uid is empty".to_string())
        }
    }
}

// uid is base64 encoded and can contain '/'
#[post("/log/<uid..>", data = "<snapshot>")]
pub async fn log_with_token(state: &State<ServerState>, token: BearerToken, uid: Uid, snapshot: Json<PostSnapshot>) -> CreateResponse {
    match state
        .auth
        .verify_token(&token)
        .await
    {
        Err(_) => CreateResponse::Forbidden(()),
        Ok(db_uid) => {
            let uid = uid.uid;
            println!("uid: {}, db uid: {}, token: {:?}", uid, db_uid, token);
            if db_uid != uid {
                return CreateResponse::Error("uid incorrect.".to_string());
            }

            match state.timeseries.add(InsertSnapshot {
                sensor_uid: uid,
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
        }
    }
}