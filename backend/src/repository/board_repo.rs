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

        match self.get_board(&new_board) {
            Some(_) => return false,
            None => (),
        };

        let board = self.col
            .insert_one(new_board, None)
            .ok();

        match board {
            Some(_) => true,
            None => false,
        }
    }

    // delete a board from mongodb
    pub fn delete_board(&self, board_info: &Board) -> bool {

        let filter = doc! {
            "player_1": board_info.player_1.replace("_", " "),
            "player_2": board_info.player_2.replace("_", " "),
            "mode": board_info.mode.clone(),
            "difficulty": board_info.difficulty.clone(),
            "width": board_info.width.clone(),
            "height": board_info.height.clone(),
        };

        match self.col.delete_one(filter, None).ok() {
            Some(_) => true,
            None => false,
        }
    }
    
    // update a board from mongodb
    pub fn update_board(&self, board_info: &Board) -> bool {
        self.delete_board(board_info) && self.create_board(board_info.clone())
    }

    // get a board from mongodb
    pub fn get_board(&self, board_info: &Board) -> Option<Board> {

        let filter = doc! {
            "player_1": board_info.player_1.replace("_", " "),
            "player_2": board_info.player_2.replace("_", " "),
            "mode": board_info.mode.clone(),
            "difficulty": board_info.difficulty.clone(),
            "width": board_info.width.clone(),
            "height": board_info.height.clone(),
        };
        let board_detail = self.col
            .find_one(filter, None)
            .ok();
        
        match board_detail {
            Some(x) => x,
            None => None
        }

        //return board_detail.unwrap();
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