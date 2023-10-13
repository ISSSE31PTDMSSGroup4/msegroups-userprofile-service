use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, repository::s3bucket_repo::S3BucketService};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use rocket::request::{self, Request, FromRequest, Outcome};

#[derive(Debug)]
pub struct UserEmail(String);
pub struct ImageData(String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for UserEmail {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self,Self::Error> {
        match request.headers().get_one("X-USER") {
            Some(email) => Outcome::Success(UserEmail(email.to_string())),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[post("/user/profile", data = "<new_user>")]
pub fn create_user_profile(
    db: &State<MongoRepo>,
    new_user: Json<User>,
    user_email: UserEmail
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        avatar: new_user.avatar.to_owned(),
        email: user_email.0.to_owned(),
        about: new_user.about.to_owned(),
    };

    let user_detail = db.create_user_profile(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/user/profile")]
pub fn get_user_profile(db: &State<MongoRepo>, user_email: UserEmail) -> Result<Json<User>, Status> {
    // let id = path;
    // let name = path;
    // if name.is_empty() {
    //     return Err(Status::BadRequest);
    // };

    let user_detail = db.get_user_profile(&user_email.0);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[put("/user/profile/update", data = "<new_user>")]
pub fn update_user_profile(
    db: &State<MongoRepo>,
    new_user: Json<User>,
    user_email: UserEmail
) -> Result<Json<User>, Status> {
    // let name = path;
    // if name.is_empty() {
    //     return Err(Status::BadRequest);
    // };
    let data = User {
        // id: Some(ObjectId::parse_str(&id).unwrap()),
        id: None,
        name: new_user.name.to_owned(),
        avatar: new_user.avatar.to_owned(),
        email: user_email.0.to_owned(),
        about: new_user.about.to_owned(),
    };
    let update_result = db.update_user_profile(&user_email.0.clone(), data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user_profile(&user_email.0);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::BadRequest)
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::BadRequest),
    }
}

#[delete("/user/<path>")]
pub fn delete_user_profile(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user_profile(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/user/profiles")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/user/profile/search/<path>")]
pub fn get_user_by_substring(db: &State<MongoRepo>, path: String) -> Result<Json<Vec<User>>, Status> {
    // let id = path;
    let name_substr = path;
    if name_substr.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_list = db.get_user_by_substring(&name_substr);
    match user_list {
        Ok(user_list) => Ok(Json(user_list)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[post("/user/profile/avatar/upload", data = "<image_data>")]
pub fn upload_user_avatar(
    s3: &State<S3BucketService>,
    image_data: Json<ImageData>,
) -> Result<Json<String>, Status> {
    let object_key = "example.jpg"; // Replace with a dynamic key if needed

    match aws_service.upload_image(&object_key, &image_data.image).await {
        Ok(_) => {
            let public_url = aws_service.generate_public_url(&object_key);
            Ok(content::Json(public_url))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
