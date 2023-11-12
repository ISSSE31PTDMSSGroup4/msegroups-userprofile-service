mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use api::user_api::{create_user_profile, get_user_profile, update_user_profile, delete_user_profile, get_all_users, get_user_by_substring};
use api::avatar_api::upload_image;
use repository::mongodb_repo::MongoRepo;
use rocket_cors::{AllowedHeaders, AllowedOrigins};


#[launch]
pub fn rocket() -> _ {
    let db = MongoRepo::init();
    // let s3 = S3BucketService::init();
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post, rocket::http::Method::Put].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "X-USER"]),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::build()
    .attach(cors.to_cors().unwrap())
    .manage(db)
    .mount("/api", routes![create_user_profile])
    .mount("/api", routes![get_user_profile])
    .mount("/api", routes![update_user_profile])
    .mount("/api", routes![delete_user_profile])
    .mount("/api", routes![get_all_users])
    .mount("/api", routes![get_user_by_substring])
    .mount("/api", routes![upload_image])
}