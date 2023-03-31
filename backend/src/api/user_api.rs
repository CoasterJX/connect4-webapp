use std::path::PathBuf;

use crate::{
    models::{
        user_model::*,
        general_model::GeneralStatus
    },
    repository::user_repo::UserRepo
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

extern crate argon2;


#[options("/<_p..>")]
pub fn placeholder(_p: PathBuf) -> Result<(), ()> {
    Ok(())
}


#[post("/user/create", data = "<new_user>")]
pub fn create_user(db: &State<UserRepo>, new_user: Json<User>) -> Result<Json<GeneralUserResponse>, Status> {

    let user = User::new(new_user.name.clone(), new_user.pwd.clone());
    match db.create_user(user.clone()) {

        true => Ok(Json(GeneralUserResponse {
            status: GeneralStatus::success(),
            user: user.clone(),
        })),

        false => Ok(Json(GeneralUserResponse {
            status: GeneralStatus::failure("User already exists or database not connected."),
            user: user.clone(),
        }))
    }
}


#[post("/user/verify", data = "<user>")]
pub fn verify_pwd(db: &State<UserRepo>, user: Json<User>) -> Result<Json<PwdVerifyResponse>, Status> {

    Ok(Json(PwdVerifyResponse {
        status: GeneralStatus::success(),
        exists: db.verify_pwd(&user.name, &user.pwd),
    }))
}


#[get("/user/info/<path>")]
pub fn get_user(db: &State<UserRepo>, path: String) -> Result<Json<GeneralUserResponse>, Status> {

    let name = path;
    if name.is_empty() {
        return Ok(Json(GeneralUserResponse {
            status: GeneralStatus::failure("Name cannot be empty."),
            user: User::empty(),
        }));
    };
    
    match db.get_user(&name) {

        Some(user) => Ok(Json(GeneralUserResponse {
            status: GeneralStatus::success(),
            user
        })),

        None => Ok(Json(GeneralUserResponse {
            status: GeneralStatus::failure("User does not exists or database not connected."),
            user: User::empty()
        })),
    }
}


#[get("/user/all")]
pub fn get_all_users(db: &State<UserRepo>) -> Result<Json<GetAllUserResonse>, Status> {
    
    match db.get_all_users() {
        Some(all_users) => Ok(Json(GetAllUserResonse {
            status: GeneralStatus::success(),
            all_users
        })),
        None => Ok(Json(GetAllUserResonse {
            status: GeneralStatus::failure("Database not connected."),
            all_users: vec![]
        })),
    }
}