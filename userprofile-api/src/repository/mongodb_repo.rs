use std::env;
// extern crate dotenv;
// use dotenv::dotenv;
use regex;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        // dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user_profile(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            avatar: new_user.avatar,
            email: new_user.email,
            about: new_user.about,
        };
        let user = self.col.insert_one(new_doc, None).ok().expect("Error creating user");
        Ok(user)
    }

    pub fn get_user_profile(&self, user_email: &String) -> Result<User, Error> {
        // let obj_name = ObjectId::parse_str(name).unwrap();
        // let filter = doc! {"_id": obj_id};
        let empty_doc = User {
            id: None,
            name: String::new(),
            avatar: String::new(),
            email: String::new(),
            about: String::new(),
        };

        let filter = doc! {"email": user_email};

        let user_detail = self.col.find_one(filter, None).ok().expect("Error getting user");
        match user_detail {
            Some(user) => Ok(user),
            None => Ok(empty_doc),
        }
        // Ok(user_detail.unwrap())
    }

    pub fn update_user_profile(&self, user_email:&String, new_user: User) -> Result<UpdateResult, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();

        let filter_name = doc! {"email": user_email};
        // let user_detail = self.col.find_one(filter_name, None).ok().expect("Error getting user's profile");
        // let _user_id = user_detail.unwrap().id;
        // let filter = doc! {"_id": _user_id};

        let new_doc = doc! {
            "$set":
                {
                    "name": new_user.name,
                    "avatar": new_user.avatar,
                    "email": new_user.email,
                    "about": new_user.about
                },
        };

        let updated_doc = self.col.update_one(filter_name, new_doc, None).ok().expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user_profile(&self, user_email:&String,) -> Result<DeleteResult, Error> {
        let filter_name = doc! {"email": user_email};
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter_name, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).ok().expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_user_by_substring(&self, name: &String) -> Result<Vec<User>, Error> {
        let filter = doc! {
            "name": {
                "$regex": format!("^{}", regex::escape(name)),
                "$options": "i" // Case-insensitive
            }
        };

        // let filter = doc! {"name": name};
        // let cursors = self.col.find(filter, None).ok().expect("Error getting list of users");
        let cursors = self.col.find(filter, None)
            .ok().expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

}