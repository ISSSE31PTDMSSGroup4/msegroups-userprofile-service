mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
// use rocket::{get, http::Status, serde::json::Json};
use api::user_api::{create_user_profile, get_user_profile, update_user_profile, delete_user_profile, get_all_users, get_user_by_substring};
use repository::mongodb_repo::MongoRepo;
// use rocket::http::Method;
// use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

// #[get("/")]
// fn hello() -> Result<Json<String>, Status> {
    // Ok(Json(String::from("Hello from rust and mongoDB")))
// }

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post, rocket::http::Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "X-USER"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::build()
    .manage(db)
    .mount("/", routes![create_user_profile])
    .mount("/", routes![get_user_profile])
    .mount("/", routes![update_user_profile])
    .mount("/", routes![delete_user_profile])
    .mount("/", routes![get_all_users])
    .mount("/", routes![get_user_by_substring])
    .manage(cors.to_cors())
}