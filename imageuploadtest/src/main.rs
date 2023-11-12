#![feature(decl_macro)]

use rocket::{Data, Rocket, serde::json::Json};
use rocket_multipart_form_data::{
    mime,
    MultipartFormData,
    MultipartFormDataOptions,
    MultipartFormDataField,
    Repetition,
};

use rocket::http::{Status, ContentType};
use std::io::prelude::*;
use chrono::Utc;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;


// Create a structure to hold upload image data.
// #[derive(FromData)]
// struct FileUpload {
//     data: Vec<u8>,
//     content_type: String,
// }

// Create a simple response structure.
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
}

// Define an endpoint to upload an image
#[post("/upload", data = "<data>")]
async fn upload_image(content_type: &ContentType, data: Data<'_>) -> Result<Json<ApiResponse>, Status> {
    // Define the options for parsing the multipart form data
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![MultipartFormDataField::raw("image")
                                                                                .size_limit(10 * 1024 * 1024)                         // Set the size limit for the image file
                                                                                .content_type_by_string(Some(mime::IMAGE_STAR))     // Set the content type for the image file
                                                                                .unwrap(),
                                                                            ]);
    // Define the directory where you want to store uploaded images.
    let upload_dir = "uploads";
    // Create the upload directory if it doesn't exist
    // std::fs::create_dir_all(upload_dir).map_err(|_| Status::InternalServerError)?;

    // Parse the multipart form data
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    // Get the image file from the form data
    let image = multipart_form_data.raw.remove("image");    // Use the remove method to move raw fields out of the MultipartFormData instance (recommended)

    // Check if the image file exists and is valid
    if let Some(mut image) = image {
        // Get the first image file (ignore multiple files), the max length of this raw_fields is 1.
        let image = image.remove(0);
        // Get the file name and content type of the image file
        let file_name = image.file_name;
        let content_type = image.content_type;
        // Get the raw bytes of the image file
        let raw_data = image.raw;
        println!("Filename: {:?}", file_name);
        println!("Content type: {:?}", content_type);
        println!("Raw data length: {}", raw_data.len());

        // Get image type and save the file
        let png_ct: mime::Mime = "image/png".parse().unwrap();
        // let jpeg_ct: mime::Mime = "image/jpeg".parse().unwrap();
        let mut filename = String::from("");
        if content_type.unwrap() == png_ct{
            filename = format!("{}/image_{}.png", upload_dir, Utc::now().timestamp().to_string());
            // filename = format!("image_{}.png", Utc::now().timestamp().to_string());
        } 
        // else if content_type.unwrap() == jpeg_ct {
        //     filename = format!("{}/image_{}.jpeg", upload_dir.clone(), Utc::now().timestamp().to_string());
        // }
        println!("File name {:?}", filename);
        let new_file = Path::new(&filename);
        // let path = env::current_dir().unwrap().join(new_file);
        fs::write(new_file, raw_data).expect("Unable to write file");
        // if let Ok(mut file) = std::fs::File::create(new_file) {
        //     file.write_all(&raw_data).unwrap();
        // }
        // Return a success response with a message
        Ok(Json(ApiResponse {
            message: "Image uploaded successfully.".to_string(),
        }))
    } else {
        // Return an error response with a message
        Err(Status::BadRequest)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload_image])
}