use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, repository::s3bucket_repo::S3BucketService};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, http::ContentType};
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::data::Data;
use rocket::response::status::BadRequest;
use rocket_multipart_form_data::{mime, multer, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, MultipartFormDataError};

#[derive(Debug)]
pub struct UserEmail(String);
#[derive(Debug)]
pub struct ImageData(String);
#[derive(Debug)]
pub struct UploadResponse{
    success: bool,
    message: String,
}

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

#[post("/api/user/profile", data = "<new_user>")]
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

#[get("/api/user/profile")]
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

#[put("/api/user/profile/update", data = "<new_user>")]
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

#[get("/api/user/profiles")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/api/user/profile/search/<path>")]
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

#[post("/api/user/profile/avatar/upload", data = "<image_data>")]
pub fn upload_user_avatar(
    s3: &State<S3BucketService>,
    content_type: &ContentType,
    image_data: Data<'_>,
) -> Result<Json<UploadResponse>, Status> {
    // Define the options for parsing the multipart form data
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![MultipartFormDataField::raw("image")
                                                                                .size_limit(10 * 1024 * 1024)                         // Set the size limit for the image file
                                                                                .content_type_by_string(Some(mime::IMAGE_STAR))     // Set the content type for the image file
                                                                                .unwrap(),
                                                                            ]);
    // Parse the multipart form data
    let mut multipart_form_data = match MultipartFormData::parse(content_type, image_data, options).await
    {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err(Status::BadRequest);
                },
                MultipartFormDataError::DataTypeError(_) => {
                    return Err(Status::BadRequest);
                },
                MultipartFormDataError::MulterError(multer::Error::IncompleteFieldData {
                    ..
                })
                | MultipartFormDataError::MulterError(multer::Error::IncompleteHeaders {
                    ..
                }) => {
                    // may happen when we set the max_data_bytes limitation
                    return Err(Status::BadRequest);
                },
                _ => panic!("{:?}", err),
            }
        }
    };

    // Get the image file from the form data
    let image = multipart_form_data.raw.remove("image");
    
    // Check if the image file exists and is valid
    if let Some(mut image) = image {
        // Get the first image file (ignore multiple files)
        let image = image.remove(0);
        // Get the file name and content type of the image file
        let file_name = image.file_name;
        let content_type = image.content_type;
        // Get the raw bytes of the image file
        let raw_data = image.raw;
        // Do something with the image file, such as saving it to disk or processing it
        // For simplicity, we just print some information here
        println!("Filename: {:?}", file_name);
        println!("Content type: {:?}", content_type);
        println!("Raw data length: {}", raw_data.len());

        // Return a success response with a message
        let resp: UploadResponse = UploadResponse {
            success: true,
            message: "image uploaded successfully".to_string(),

        };
        Ok(Json(resp))
    } else {
        // Return an error response with a message
        Err(Status::BadRequest)
    }
    
    // let object_key = "example.jpg"; // Replace with a dynamic key if needed

    // match aws_service.upload_image(&object_key, &image_data.image).await {
    //     Ok(_) => {
    //         let public_url = aws_service.generate_public_url(&object_key);
    //         Ok(content::Json(public_url))
    //     }
    //     Err(_) => Err(Status::InternalServerError),
    // }
}
