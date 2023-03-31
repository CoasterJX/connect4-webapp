use serde::{Serialize, Deserialize};
use super::general_model::GeneralStatus;

extern crate argon2;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub pwd: String,
    pub score: i64,
    pub last_grids: Vec<String>, // board id
}

impl User {

    pub fn empty() -> Self {
        User {
            name: "".to_owned(),
            pwd: "".to_owned(),
            score: 0,
            last_grids: vec![],
        }
    }

    pub fn new(name: String, pwd: String) -> Self {
        User {
            name: name.to_owned(),
            pwd: argon2::hash_encoded(
                pwd.as_bytes(),
                b"randomsalt",
                &argon2::Config::default()
            ).unwrap(),
            score: 0,
            last_grids: vec![]
        }
    }
}

// message sending model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralUserResponse {
    pub status: GeneralStatus,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PwdVerifyResponse {
    pub status: GeneralStatus,
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAllUserResonse {
    pub status: GeneralStatus,
    pub all_users: Vec<User>,
}