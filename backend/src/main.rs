mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::*;
use repository::user_repo::UserRepo;
use rocket::{
    http::Header,
    routes,
    Request,
    Response
};

use rocket::fairing::{
    Fairing,
    Info,
    Kind
};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

    }
}


#[launch]
fn rocket() -> _ {

    // let allowed_origins = AllowedOrigins::all();
    // let cors = CorsOptions {
    //     allowed_origins,
    //     // allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
    //     allow_credentials: true,
    //     ..Default::default()
    // }.to_cors().unwrap();

    let db = UserRepo::init();
    rocket::build()
        .attach(Cors)
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![verify_pwd])
        .mount("/", routes![placeholder])
}
