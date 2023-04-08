use crate::{
    models::{
        board_model::*,
        general_model::GeneralStatus
    },
    repository::{board_repo::BoardRepo, hist_repo::HistRepo}
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

// #[get("/hist/get/<user>")]
// pub fn get_hist(db: &State<HistRepo>, user: String) -> Result<Json<GetHistResponse>, Status> {

// }