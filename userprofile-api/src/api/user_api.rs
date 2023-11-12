use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use rocket::request::{self, Request, FromRequest, Outcome};

#[derive(Debug)]
pub struct UserEmail(String);


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
    let user_detail = db.get_user_profile(&user_email.0);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => {
            let empty_user = User {
                id: None,
                name: String::new(),
                avatar: String::new(),
                email: String::new(),
                about: String::new(),
            };
            Ok(Json(empty_user))
        },
    }
    // let empty_data = User {
    //     id: None,
    //     name: '',
    //     avatar: '',
    //     email: '',
    //     about: '',
    // };

    // if let Ok(user) = db.get_user_profile(&user_email.0) {
    //     Ok(Json(user))
    // } else {
    //     Ok(Json(empty_data))
    // }
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

#[delete("/user/profile/delete")]
pub fn delete_user_profile(db: &State<MongoRepo>, user_email: UserEmail) -> Result<Json<&str>, Status> {
    let result = db.delete_user_profile(&user_email.0);
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

    // let id = path;
    // if id.is_empty() {
    //     return Err(Status::BadRequest);
    // };
    // let result = db.delete_user_profile(&id);
    // match result {
    //     Ok(res) => {
    //         if res.deleted_count == 1 {
    //             return Ok(Json("User successfully deleted!"));
    //         } else {
    //             return Err(Status::NotFound);
    //         }
    //     }
    //     Err(_) => Err(Status::BadRequest),
    // }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rocket;
    use rocket::http::{Status, Header};
    use rocket::http::ContentType;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;
    use mongodb::results::InsertOneResult;

    // #[test]
    // fn test_user_profile(){
    //     test_create_user_profile();
    //     test_get_user_profile();
    //     test_update_user_profile();
    //     test_get_user_by_substring();
    //     test_delete_user_profile();
    //     test_get_all_users();
    // }

    #[test]
    fn test_create_user_profile() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/user/profile")
            .header(Header::new("X-USER", "test@test.com"))
            .header(ContentType::JSON)
            .body(json!({
                "name": "Test User",
                "avatar": "test_avatar",
                "email": "test@test.com",
                "about": "Test About"
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client.delete("/api/user/profile/delete")
            .header(Header::new("X-USER", "test@test.com"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_user_profile() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        
        let response = client.get("/api/user/profile")
            .header(Header::new("X-USER", "zenablade@gmail.com"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_update_user_profile() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/user/profile")
            .header(Header::new("X-USER", "test@test.com"))
            .header(ContentType::JSON)
            .body(json!({
                "name": "Test User",
                "avatar": "test_avatar",
                "email": "test@test.com",
                "about": "Test About"
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client.put("/api/user/profile/update")
            .header(ContentType::JSON)
            .header(Header::new("X-USER", "test@test.com"))
            .body(json!({
                "name": "Updated User",
                "avatar": "updated_avatar",
                "email": "test@test.com",
                "about": "Updated About"
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client.delete("/api/user/profile/delete")
            .header(Header::new("X-USER", "test@test.com"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_delete_user_profile() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/user/profile")
            .header(Header::new("X-USER", "test@test.com"))
            .header(ContentType::JSON)
            .body(json!({
                "name": "Test User",
                "avatar": "test_avatar",
                "email": "test@test.com",
                "about": "Test About"
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response = client.delete("/api/user/profile/delete")
            .header(Header::new("X-USER", "test@test.com"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_all_users() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/profiles").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_user_by_substring() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/profile/search/zena").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}