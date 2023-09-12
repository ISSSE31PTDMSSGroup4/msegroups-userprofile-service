mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
// use rocket::{get, http::Status, serde::json::Json};
use api::user_api::{create_user_profile, get_user_profile};
use repository::mongodb_repo::MongoRepo;

// #[get("/")]
// fn hello() -> Result<Json<String>, Status> {
    // Ok(Json(String::from("Hello from rust and mongoDB")))
// }

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
    .manage(db)
    .mount("/", routes![create_user_profile])
    .mount("/", routes![get_user_profile])
}