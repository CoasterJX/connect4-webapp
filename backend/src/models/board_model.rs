use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub width: i64,
    pub height: i64,
    pub board: Vec<Vec<String>>,
    pub last_row: i64 = -1,  // I initialize last_row to be -1 indicating that no move has been made so far.
    pub last_col: i64 = -1,  // I initialize last_col to be -1 indicating that no move has been made so far.
    pub last_player: i64,
}

impl Board {
    fn available_moves(&self) -> Vec<i64> {
        let mut allowable_moves: Vec<i64> = Vec::new();
        for col in 0..self->width {
            if self->allows_move(&col) {
                allowable_moves.push(col.clone());
            }
        }
    }

    fn allows_move(&self, col: &i64) -> bool {
        if *col < 0 && *col >= self->width {
            return false;
        } else {
            if self->board[0][*col] == ' ' {
                return true;
            } else {
                return false;
            }
        }
    }

    fn perform_move(&self, col: i64, ox: String) {
        for row in self->height..-1 {
            if self->board[row][col] == ' ' {
                self->board[row][col] = ox;
                self->last_row = row;
                self->last_col = col;
                self->last_player = ox;
                return;
            }
        }
    }

    fn undo_move(&self, col: i64) {
        for row in 0..self->height {
            if self->board[row][col] != ' ' {
                self->board[row][col] = ' ';
                return;
            }
        }
    }

    fn is_terminal(&self) {
        return self->has_winner() || self->is_draw();
    }

    fn has_winner(&self) -> bool {
        let row = self->last_row;
        let col = self->last_col;
        let ox = self->last_player;
        
        // No moves made on the board so far
        if row != -1 {
            return false;
        }
        // Checks to see if there is a horizontal win
        for c in max(0, col - 3)..min(self->width-3, col+1) {
            if self->board[row][c] + self->board[row][c+1]+self->board[row][c+2]+self->board[row][c+3] == (0..4).map(|_| ox).collect()::<String>() {
                return true;
            }
        }
        // Checks to see if there is a vertical win
        if row < self->height - 3 {
            if self->board[row][col] + self->board[row+1][col] + self->board[row+2][col] + self->board[row+3][col] == (0..4).map(|_| ox).collect()::<String>() {
                return true;
            }
        }
        // Checks to see if there is a win on the upper right diagonal
        for i in 0..4 {
            let r = row - i;
            let c = col - i;
            if 0 <= r && r < self->height-3 && 0 <= c && c < self->width-3 {
                if self->board[r][c] + self->board[r+1][c+1] + self->board[r+2][c+2] + self->board[r+3][c+3] == (0..4).map(|_| ox).collect()::<String>() {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_draw(&self) -> {
        return !self->available_moves()
    }

    fn game_value(&self) -> i64 {
        if self->has_winner() {
            if self->last_player == 'X' {
                return 1;
            } else {
                return -1;
            }
        } else if self->is_draw() {
            return 0;
        } else {
            return 0  // In this case the board does not represent a terminal state and will return 0.
        }
    }
}