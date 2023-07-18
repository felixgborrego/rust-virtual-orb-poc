use log::info;
use rocket::response::status::Conflict;
use rocket::serde::json::Json;
use rocket::{post, routes, Config, State};
use serde::Deserialize;

use in_memory_db::InmemoryDB;

mod in_memory_db;
mod signup;

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    id: String,
    image_base64: String,
}

// Allow to reset the already seen Images
// Usage curl -X POST localhost:4242/api/reset
#[post("/reset")]
fn reset(state: &State<InmemoryDB>) -> &'static str {
    state.clear();
    "reset completed!"
}

// Handle the REST API sign up
#[post("/signup", format = "json", data = "<request>")]
fn http_signup(
    state: &State<InmemoryDB>,
    request: Json<SignupRequest>,
) -> Result<String, Conflict<String>> {
    signup::process_signup(state, &request.0).map_err(|err| Conflict(Option::from(err.to_string())))
}

// Start the Rocket server mounting the orb API
pub async fn start_server(orb_server_port: u16) {
    info!(
        "🚀 Virtual Orb Test Server at http:/localhost:{}",
        orb_server_port
    );

    let figment = Config::figment().merge(("port", orb_server_port));

    let _ = rocket::custom(figment)
        .mount("/api", routes![reset, http_signup])
        .manage(InmemoryDB::new())
        .launch()
        .await;
}
