#![feature(iter_intersperse)]

use rocket::{Build, Rocket, Config, figment::Figment};
use token_auth::TokenAuth;
use gaia_auth::GaiaAuth;
use timeseries::Timeseries;

mod endpoints;
mod gaia_auth;
mod timeseries;
mod token_auth;
mod bearer_token;

pub struct ServerState {
    pub auth: GaiaAuth,
    pub token: TokenAuth,
    pub timeseries: Timeseries,
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {    
    dotenv::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    let gaia_auth_url = dotenv::var("GAIAAUTH_URL")
        .expect("GAIAAUTH_URL env var not set!");

    let api_key = dotenv::var("API_KEY")
        .expect("API_KEY env var not set!");

    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL env var not set!");

    let gaia_auth = GaiaAuth::new(&gaia_auth_url);
    let token_auth = TokenAuth::new(&api_key);
    let timeseries = Timeseries::new(&database_url).await
        .expect("Couldn't connect to the database!");

    println!("Starting rocket on 0.0.0.0:5000");

    let figment = Figment::from(Config::default())
        .merge((Config::PORT, 5000))
        .merge((Config::ADDRESS, "0.0.0.0"));

    rocket::custom(figment)
        .mount("/", endpoints::get_routes())
        .manage(ServerState {
            auth: gaia_auth,
            token: token_auth,
            timeseries: timeseries,
        })
}