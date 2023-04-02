use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use crate::models::board_model::Board;
use super::db_type::*;

pub struct BoardRepo {
    col: Collection<Board>,
}

impl BoardRepo {

    // initialize a mongodb repo with a collection of boards
    pub fn init() -> Self {

        dotenv().ok();
        let uri = match env::var(ENV_MONGODB) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        println!("{}", uri);

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(DB_NAME);
        let col: Collection<Board> = db.collection(COL_BOARD);
        BoardRepo { col }
    }

    // add a board into mongodb
    pub fn create_board(&self, new_board: Board) -> bool {

        match self.get_board(&new_board.player_1, &new_board.player_2) {
            Some(_) => return false,
            None => (),
        }

        let board = self.col
            .insert_one(new_board, None)
            .ok();

        match board {
            Some(_) => true,
            None => false,
        }
    }

    // get a board from mongodb
    pub fn get_board(&self, player_1: &String, player_2: &String) -> Option<Board> {

        let filter = doc! {
            "player_1": player_1.replace("_", " "),
            "player_2": player_2.replace("_", " ")
        };
        let board_detail = self.col
            .find_one(filter, None)
            .ok();

        return board_detail.unwrap();
    }

    // get all boards from mongodb
    pub fn get_all_boards(&self) -> Option<Vec<Board>> {

        let cursors = self.col
            .find(None, None)
            .ok();

        match cursors {
            Some(c) => Some(c.map(|doc| doc.unwrap()).collect()),
            None => None,
        }
    }
}