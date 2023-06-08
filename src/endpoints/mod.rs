use rocket::{Route, routes};

mod log_data;
mod request_data;

pub fn get_routes() -> Vec<Route> {
    routes![
        log_data::debug_log_token,
        log_data::log_with_token,
        request_data::request_once,
    ]
}
