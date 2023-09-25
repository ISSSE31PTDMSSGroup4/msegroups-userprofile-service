use std::env;
extern crate dotenv;
use dotenv::dotenv;

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
        dotenv().ok();
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

    pub fn get_user_profile(&self, name: &String) -> Result<User, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        let obj_id = self.get_userid_by_username(name).id;
        println!("object id: {:?}", obj_id);
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.find_one(filter, None).ok().expect("Error getting user's profile");
        Ok(user_detail.unwrap())
    }

    pub fn update_user_profile(&self, name:&String, new_user: User) -> Result<UpdateResult, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        let obj_id = self.get_userid_by_username(name).id;
        println!("object id: {:?}", obj_id);
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "avatar": new_user.avatar,
                    "email": new_user.email,
                    "about": new_user.about
                },
        };
        let updated_doc = self.col.update_one(filter, new_doc, None).ok().expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user_profile(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).ok().expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_userid_by_username(&self, name: &String) -> User {
        let filter = doc! {"name": name};
        let user_obj = self.col.find_one(filter, None).ok().expect("User does not exists");
        user_obj.unwrap()
    }
}