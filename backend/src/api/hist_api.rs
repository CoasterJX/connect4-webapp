use crate::{
    models::{
        board_model::*,
        general_model::GeneralStatus
    },
    repository::hist_repo::HistRepo,
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

#[get("/hist/get/<user>")]
pub fn get_hist(db: &State<HistRepo>, user: String) -> Result<Json<GetHistResponse>, Status> {

    let mut hist: Vec<HistBoard> = vec![];
    let user = user.replace("_", " ");

    match db.get_hist(&user, "player_1") {
        Some(h) => hist.append(&mut h.clone()),
        None => (),
    };

    match db.get_hist(&user, "player_2") {
        Some(h) => hist.append(&mut h.clone()),
        None => (),
    };

    hist.sort_by(|h1, h2| h1.date.cmp(&h2.date));
    hist.dedup_by(|h1, h2| h1.date.eq(&h2.date));

    Ok(Json(GetHistResponse {
        status: GeneralStatus::success(),
        hist
    }))
}