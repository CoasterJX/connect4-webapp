use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {

    // initialize a mongodb reo with collection of users
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

    // add a user into mongodb
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {

        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
            pwd: new_user.pwd,
        };

        let user = self.col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    // get a user from mongodb
    pub fn get_user(&self, name: &String) -> Result<User, ()> {

        let filter = doc! {"name": name.replace("_", " ")};
        let user_detail = self.col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        
        match user_detail {
            Some(user) => Ok(user),
            None => Err(()),
        }
    }

    // get all users from mongodb
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {

        let cursors = self.col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");

        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    // verify user password
    pub fn verify_pwd(&self, name: &String, pwd: &String) -> bool {

        match self.get_user(name) {
            Ok(user) => argon2::verify_encoded(user.pwd.as_str(), pwd.as_bytes()).unwrap(),
            Err(_) => false,
        }
    }
}