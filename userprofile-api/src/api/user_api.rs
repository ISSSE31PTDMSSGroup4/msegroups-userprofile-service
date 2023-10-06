use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user/profile", data = "<new_user>")]
pub fn create_user_profile(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        avatar: new_user.avatar.to_owned(),
        email: new_user.email.to_owned(),
        about: new_user.about.to_owned(),
    };

    let user_detail = db.create_user_profile(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/user/profile/<path>")]
pub fn get_user_profile(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    // let id = path;
    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user_profile(&name);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[put("/user/profile/<path>", data = "<new_user>")]
pub fn update_user_profile(
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        // id: Some(ObjectId::parse_str(&id).unwrap()),
        id: None,
        name: new_user.name.to_owned(),
        avatar: new_user.avatar.to_owned(),
        email: new_user.email.to_owned(),
        about: new_user.about.to_owned(),
    };
    let update_result = db.update_user_profile(&name, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user_profile(&new_user.name);
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
pub fn get_user_by_substring(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    // let id = path;
    let name_substr = path;
    if name_substr.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_list = db.get_user_by_substring(&name_substr);
    match user_list {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::BadRequest),
    }
}