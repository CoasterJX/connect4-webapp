use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use crate::models::user_model::User;
use super::db_type::*;

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {

    // initialize a mongodb repo with collection of users
    pub fn init() -> Self {

        dotenv().ok();
        let uri = match env::var(ENV_MONGODB) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        println!("{}", uri);

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(DB_NAME);
        let col: Collection<User> = db.collection(COL_USER);
        UserRepo { col }
    }

    // add a user into mongodb
    pub fn create_user(&self, new_user: User) -> bool {

        match self.get_user(&new_user.name) {
            Some(_) => return false,
            None => (),
        };

        let user = self.col
            .insert_one(new_user, None)
            .ok();
        
        match user {
            Some(_) => true,
            None => false,
        }
    }

    // get a user from mongodb
    pub fn get_user(&self, name: &String) -> Option<User> {

        let filter = doc! {"name": name.replace("_", " ")};
        let user_detail = self.col
            .find_one(filter, None)
            .ok();
        
        return user_detail.unwrap();
    }

    // get all users from mongodb
    pub fn get_all_users(&self) -> Option<Vec<User>> {

        let cursors = self.col
            .find(None, None)
            .ok();
        
        match cursors {
            Some(c) => Some(c.map(|doc| doc.unwrap()).collect()),
            None => None,
        }
    }

    // verify user password
    pub fn verify_pwd(&self, name: &String, pwd: &String) -> bool {

        match self.get_user(name) {
            Some(user) => argon2::verify_encoded(user.pwd.as_str(), pwd.as_bytes()).unwrap(),
            None => false,
        }
    }
}