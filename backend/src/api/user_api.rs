use std::path::PathBuf;

use crate::{models::user_model::*, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, Response, fs::NamedFile};

extern crate argon2;


#[post("/user/create", data = "<new_user>")]
pub fn create_user(db: &State<MongoRepo>, new_user: Json<User>) -> Result<Json<User>, Status> {

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
        pwd: argon2::hash_encoded(
            new_user.pwd.as_bytes(),
            b"randomsalt",
            &argon2::Config::default()
        ).unwrap(),
    };

    match db.create_user(data.clone()) {
        Ok(_user) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[post("/user/verify", data = "<user>")]
pub fn verify_pwd(db: &State<MongoRepo>, user: Json<User>) -> Result<Json<PwdVerify>, Status> {

    let data = PwdVerify {
        exists: db.verify_pwd(&user.name, &user.pwd)
    };
    Ok(Json(data))
}


#[options("/<p..>")]
pub fn options_verify_user(p: PathBuf) -> Result<(), ()> {
    Ok(())
}


#[get("/user/info/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {

    let name = path;
    if name.is_empty() {
        return Err(Status::BadRequest);
    };
    
    match db.get_user(&name) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/user/all")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    
    match db.get_all_users() {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}