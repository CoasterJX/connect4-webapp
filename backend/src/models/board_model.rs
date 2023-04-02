use std::cmp::{max, min};

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::general_model::GeneralStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub width: i64,
    pub height: i64,
    pub board: Vec<Vec<String>>,
    pub last_row: i64,  // I initialize last_row to be -1 indicating that no move has been made so far.
    pub last_col: i64,  // I initialize last_col to be -1 indicating that no move has been made so far.
    pub last_player: String,
    pub player_1: String,
    pub player_2: String,
}

impl Board {
    pub fn available_moves(&self) -> Vec<i64> {
        let mut allowable_moves: Vec<i64> = Vec::new();
        for col in 0..self.width {
            if self.allows_move(&col){
                allowable_moves.push(col.clone());
            }
        }
        return allowable_moves;
    }

    pub fn allows_move(&self, col: &i64) -> bool {
        if *col < 0 && *col >= self.width {
            return false;
        } else {
            if self.board[0][(*col) as usize] == ' '.to_string() {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn perform_move(&mut self, col: i64, ox: String) {
        for row in self.height..-1 {
            if self.board[row as usize][col as usize] == ' '.to_string() {
                self.board[row as usize][col as usize] = ox.clone();
                self.last_row = row.clone();
                self.last_col = col.clone();
                self.last_player = ox.clone();
                return;
            }
        }
    }

    pub fn empty() -> Self {
        Board {
            width: 0,
            height: 0,
            board: vec![],
            last_row: -1,
            last_col: -1,
            last_player: 'O'.to_string(),
            player_1: ' '.to_string(),
            player_2: ' '.to_string(),
        }
    }

    pub fn new(w: i64, h: i64, p1: String, p2: String) -> Self {
        let mut board_init = vec![];
        for r in 0..h {
            board_init.push(vec![]);
            for c in 0..w {
                board_init[r as usize].push(' '.to_string());
            }
        }
        return Board {
            width: w,
            height: h,
            board: board_init.clone(),
            last_row: -1,
            last_col: -1,
            last_player: 'O'.to_string(),
            player_1: p1.to_owned(),
            player_2: p2.to_owned(),
        }
    }

    pub fn undo_move(&mut self, col: i64) {
        for row in 0..self.height {
            if self.board[row as usize][col as usize] != ' '.to_string() {
                self.board[row as usize][col as usize] = ' '.to_string();
                return;
            }
        }
    }

    pub fn is_terminal(&self) -> bool {
        return self.has_winner() || self.is_draw();
    }

    pub fn has_winner(&self) -> bool {
        let row = self.last_row;
        let col = self.last_col;
        let ox = self.last_player.clone();
        
        // No moves made on the board so far
        if row != -1 {
            return false;
        }
        // Checks to see if there is a horizontal win
        for c in max(0, col - 3)..min(self.width-3, col+1) {
            if self.board[row as usize][c as usize] == ox && self.board[row as usize][(c+1) as usize] == ox && self.board[row as usize][(c+2) as usize] == ox && self.board[row as usize][(c+3) as usize] == ox {
                return true;
            }
        }
        // Checks to see if there is a vertical win
        if row < self.height - 3 {
            if self.board[row as usize][col as usize] == ox && self.board[(row+1) as usize][col as usize] == ox && self.board[(row+2) as usize][col as usize] == ox && self.board[(row+3) as usize][col as usize] == ox {
                return true;
            }
        }
        // Checks to see if there is a win on the upper right diagonal
        for i in 0..4 {
            let r = row - i;
            let c = col - i;
            if 0 <= r && r < self.height-3 && 0 <= c && c < self.width-3 {
                if self.board[r as usize][c as usize] == ox && self.board[(r+1) as usize][(c+1) as usize] == ox && self.board[(r+2) as usize][(c+2) as usize] == ox && self.board[(r+3) as usize][(c+3) as usize] == ox {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn is_draw(&self) -> bool {
        return self.available_moves().len() == 0;
    }

    pub fn game_value(&self) -> i64 {
        if self.has_winner() {
            if self.last_player == 'X'.to_string() {
                return 1;
            } else {
                return -1;
            }
        } else if self.is_draw() {
            return 0;
        } else {
            return 0  // In this case the board does not represent a terminal state and will return 0.
        }
    }
}

// message sending model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralBoardResponse {
    pub status: GeneralStatus,
    pub board: Board,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAllBoardResponse {
    pub status: GeneralStatus,
    pub all_boards: Vec<Board>,
}