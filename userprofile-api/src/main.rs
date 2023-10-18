mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
// use rocket::{get, http::Status, serde::json::Json};
use api::user_api::{create_user_profile, get_user_profile, update_user_profile, delete_user_profile, get_all_users, get_user_by_substring, upload_user_avatar};
use repository::{mongodb_repo::MongoRepo, s3bucket_repo::S3BucketService};
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
    let s3 = S3BucketService::init();
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post, rocket::http::Method::Put].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "X-USER"]),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    // let cors = Cors {
    //     allowed_origins: AllowedOrigins::all(),
    //     allowed_methods: vec![Method::Get, Method::Post, Method::Options, Method::Put],
    //     allowed_headers: AllowedHeaders::all(),
    //     allowed_credentials: true,
    //     ..Default::default()
    // };

    rocket::build()
    .attach(cors.to_cors().unwrap())
    .manage(db)
    .manage(s3)
    .mount("/", routes![create_user_profile])
    .mount("/", routes![get_user_profile])
    .mount("/", routes![update_user_profile])
    .mount("/", routes![delete_user_profile])
    .mount("/", routes![get_all_users])
    .mount("/", routes![get_user_by_substring])
    .mount("/", routes![upload_user_avatar])
}