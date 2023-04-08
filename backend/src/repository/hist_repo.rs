use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use crate::models::board_model::{HistBoard, Board};
use super::db_type::*;

pub struct HistRepo {
    col: Collection<HistBoard>,
}

impl HistRepo {

    // initialize a mongodb repo with a collection of hist boards
    pub fn init() -> Self {

        dotenv().ok();
        let uri = match env::var(ENV_MONGODB) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        println!("{}", uri);

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(DB_NAME);
        let col: Collection<HistBoard> = db.collection(COL_HIST);
        HistRepo { col }
    }

    // add board to hist record
    pub fn push_hist(&self, hist_board: &Board, winner: &String) -> bool {

        let hist = HistBoard::new(hist_board.clone(), winner.to_string());
        let res = self.col
            .insert_one(hist, None)
            .ok();

        match res {
            Some(_) => true,
            None => false,
        }
    }

    // get hist by user, * represents all
    pub fn get_hist(&self, user: &String, side: &str) -> Option<Vec<HistBoard>> {

        match user.clone().as_str() {
            "*" => {

                if side == "player_2" {
                    return None;
                }

                let cursors = self.col
                    .find(None, None)
                    .ok();

                match cursors {
                    Some(c) => Some(c.map(|doc| doc.unwrap()).collect()),
                    None => None,
                }
            },
            _ => {
                let filter = doc! {
                    format!("board.{}", side.clone()).as_str(): user.clone()
                };
                let cursors = self.col
                    .find(filter, None)
                    .ok();

                match cursors {
                    Some(c) => Some(c.map(|doc| doc.unwrap()).collect()),
                    None => None,
                }
            },
        }
    }
}