use std::{cmp::{max, min, Ordering}, collections::HashMap};

use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

use super::general_model::GeneralStatus;

use std::io;
//use std::io::Write;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistBoard {
    pub board: Board,
    pub date: String,
    pub winner: String,
}

impl HistBoard {

    pub fn new(board: Board, winner: String) -> Self {
        HistBoard {
            board,
            date: Local::now().to_string(),
            winner
        }
    }
}

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
    pub mode: Vec<bool>,
    pub difficulty: i64,
}

impl Board {
    pub fn print(&self) -> String {
        // Start with the empty string
        let mut board_str = String::new();

        for row in 0..self.height {
            // Initialize each row with a |
            board_str.push('|');
            for col in 0..self.width {
                if self.board[row as usize][col as usize] == self.player_1 {
                    board_str.push_str("T");
                } else if self.board[row as usize][col as usize] == self.player_2 {
                    board_str.push_str("O");
                } else {
                    board_str.push_str(&self.board[row as usize][col as usize]);
                }
                board_str.push_str("|");
            }
            board_str.push_str("\n");
        }
        // Line of dashes
        for _ in 0..(self.width*2+1) {
            board_str.push_str("-");
        }
        board_str.push_str("\n");
        for col in 0..self.width {
            board_str.push_str(&format!(" {}", col));
        }
        return board_str;
    }

    /*
    Get a vector of all movable columns that can be performed on the board.
     */
    pub fn available_moves(&self) -> Vec<i64> {
        let mut allowable_moves: Vec<i64> = Vec::new();
        for col in 0..self.width {
            if self.allows_move(&col){
                allowable_moves.push(col.clone());
            }
        }
        allowable_moves.shuffle(&mut rand::thread_rng());
        return allowable_moves;
    }

    /*
    Check if player can perform move on the specified column.
     */
    pub fn allows_move(&self, col: &i64) -> bool {
        if *col < 0 || *col >= self.width {
            return false;
        } else {
            if self.board[0][(*col) as usize] == ' '.to_string() {
                return true;
            } else {
                return false;
            }
        }
    }

    /*
    Perform a move at specified column for specified player.
     */
    pub fn perform_move(&mut self, col: i64, ox: String) {
        for row in (0..self.height).rev() {
            if self.board[row as usize][col as usize] == ' '.to_string() {
                self.board[row as usize][col as usize] = ox.clone();
                self.last_row = row.clone();
                self.last_col = col.clone();
                self.last_player = ox.clone();
                return;
            }
        }
    }

    /*
    This should only be used as a dummy board for error cases.
     */
    pub fn empty() -> Self {
        Board {
            width: 0,
            height: 0,
            board: vec![],
            last_row: -1,
            last_col: -1,
            last_player: ' '.to_string(),
            player_1: ' '.to_string(),
            player_2: ' '.to_string(),
            mode: vec![],
            difficulty: 1,
        }
    }

    /*
    Create a new board with specified parameters:
    - width & height of board.
    - 2 players, treat empty string as computer.
    - mode: whether TOOT or OTTO or TTTT, etc.
    - difficulty: computer difficulty level, only useful when computer is involved.
     */
    pub fn new(w: i64, h: i64, p1: String, p2: String, m: Vec<bool>, d: i64) -> Self {
        let mut board_init: Vec<Vec<String>> = vec![];
        for r in 0..h {
            board_init.push(vec![]);
            for _ in 0..w {
                board_init[r as usize].push(' '.to_string());
            }
        }
        return Board {
            width: w,
            height: h,
            board: board_init.clone(),
            last_row: -1,
            last_col: -1,
            last_player: p2.to_string(),
            player_1: p1.to_string(),
            player_2: p2.to_string(),
            mode: m,
            difficulty: d,
        }
    }

    /*
    Should only be used in alpha-beta.
     */
    pub fn undo_move(&mut self, col: i64) {
        for row in 0..self.height {
            if self.board[row as usize][col as usize] != ' '.to_string() {
                self.board[row as usize][col as usize] = ' '.to_string();
                return;
            }
        }
    }

    /*
    Check if game over.
     */
    pub fn is_terminal(&self) -> bool {
        return self.has_winner() || self.is_draw();
    }

    pub fn get_next_player(&self) -> String {
        return self.pattern(&self.last_player, &true);
    }

    fn pattern(&self, ox: &String, bit: &bool) -> String {
        let rev = HashMap::from([
            (self.player_1.clone(), self.player_2.clone()),
            (self.player_2.clone(), self.player_1.clone()),
        ]);
        match bit {
            false => ox.clone(),
            true => rev.get(ox.as_str()).unwrap().to_string(),
        }
    }

    fn pattern_enemy(&self, ox: &String, bit: &bool) -> String {
        let rev = HashMap::from([
            (self.player_1.clone(), self.player_2.clone()),
            (self.player_2.clone(), self.player_1.clone()),
        ]);
        match bit {
            false => rev.get(ox.as_str()).unwrap().to_string(),
            true => ox.clone(),
        }
    }

    pub fn has_winner(&self) -> bool {
        self._has_winner().0
    }

    /*
    Check if there is a winner.
     */
    pub fn _has_winner(&self) -> (bool, String) {

        let row = self.last_row;
        let col = self.last_col;
        let ox = self.last_player.clone();
        
        // No moves made on the board so far
        if row == -1 && col == -1 {
            return (false, "".to_owned());
        }

        // Checks to see if there is a horizontal win
        for c in max(0, col - 3)..min(self.width-3, col+1) {

            let mut win = true;
            for i in 0..4 {
                win = win && self.board[row as usize][(c+i) as usize] == self.pattern(&ox, &self.mode[i as usize]);
            }
            if win { return (win, self.last_player.clone()); }

            let mut lose = true;
            for i in 0..4 {
                lose = lose && self.board[row as usize][(c+i) as usize] == self.pattern_enemy(&ox, &self.mode[i as usize]);
            }
            if lose { return (lose, self.get_next_player().clone()); }
        }

        // Checks to see if there is a vertical win
        if row < self.height - 3 {

            let mut win = true;
            for i in 0..4 {
                win = win && self.board[(row+i) as usize][col as usize] == self.pattern(&ox, &self.mode[i as usize]);
            }
            if win { return (win, self.last_player.clone()); }

            let mut lose = true;
            for i in 0..4 {
                lose = lose && self.board[(row+i) as usize][col as usize] == self.pattern_enemy(&ox, &self.mode[i as usize]);
            }
            if lose { return (lose, self.get_next_player().clone()); }
        }

        // Checks to see if there is a win on the upper right diagonal
        for i in 0..4 {
            let r = row - i;
            let c = col - i;
            if 0 <= r && r < self.height-3 && 0 <= c && c < self.width-3 {

                let mut win = true;
                for i in 0..4 {
                    win = win && self.board[(r+i) as usize][(c+i) as usize] == self.pattern(&ox, &self.mode[i as usize])
                }
                if win { return (win, self.last_player.clone()); }

                let mut lose = true;
                for i in 0..4 {
                    lose = lose && self.board[(r+i) as usize][(c+i) as usize] == self.pattern_enemy(&ox, &self.mode[i as usize])
                }
                if lose { return (lose, self.get_next_player().clone()); }
            }
        }

        // Check to see if there is a win on the upper left diagonal
        for i in 0..4 {
            let r = row - i;
            let c = col + i;
            if 0 <= r && r < self.height-3 && 3 <= c && c < self.width {

                let mut win = true;
                for i in 0..4 {
                    win = win && self.board[(r+i) as usize][(c-i) as usize] == self.pattern(&ox, &self.mode[i as usize])
                }
                if win { return (win, self.last_player.clone()); }

                let mut lose = true;
                for i in 0..4 {
                    lose = lose && self.board[(r+i) as usize][(c-i) as usize] == self.pattern_enemy(&ox, &self.mode[i as usize])
                }
                if lose { return (lose, self.get_next_player().clone()); }
            }
        }

        // no winner if none of the above is satisfied
        return (false, "".to_owned());
    }

    /*
    Check if it is a draw.
     */
    pub fn is_draw(&self) -> bool {
        return self.available_moves().len() == 0;
    }

    /*
    Input a player's checker piece and get a move for them.
     */
    pub fn get_player_move(&self, ox: String) -> i64 {
        loop {
            let mut input: String = String::new();
            println!("{}'s choice: ", ox);
            let _ = io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse() {
                Ok(p) => {
                    let player_move: i64 = p;
                    if self.allows_move(&player_move) {
                        return p;
                    } else {
                        println!("Move is not allowed. Please try again.");
                    }
                },
                Err(_) => {
                    println!("Invalid input. Please try again.")
                }
            }
        }
    }

    /*
    Prints out who won the game and the final game board.
     */
    pub fn print_congrats(&self) {
        if self.last_player == "" {
            println!("Computer wins -- Congratulations!");
        } else {
            println!("{} wins -- Congratulations!", self.last_player);
        }
        println!("{}", self.print());
    }

    /*
    Hosts a game which can be played between two players.
     */
    pub fn host_game(&mut self) -> String {
        println!("Welcome!");
        // let mut game_over: bool = false;
        let mut ox: String = self.player_1.clone();
        loop {
            println!("{}", self.print());
            if ox == "" || ox == "*" {
                let (_, col_move): (i64, i64) = self.alpha_beta(ox.clone(), i64::MIN, i64::MAX, self.difficulty);
                self.perform_move(col_move, ox.clone());
                if self.player_2 == "*" {  // This checks if we are playing a computer vs computer game.
                    if ox == "" {
                        println!("Computer 1 performed move {}.", col_move);
                    } else {
                        println!("Computer 2 performed move {}.", col_move);
                    }
                } else {  // We are playing a human vs computer game.
                    println!("Computer performed move {}.", col_move);
                }
            } else {
                let col_move: i64 = self.get_player_move(ox.clone());
                self.perform_move(col_move, ox.clone());
            }
            if self.has_winner() {
                return ox;
            }
            if self.is_draw() {
                return "**".to_string();
            }
            if ox == self.player_1 {
                ox = self.player_2.clone();
            } else {
                ox = self.player_1.clone();
            }
        }
    }

    /*
    Check who is the winner. Should only be called after is_terminal.
     */
    pub fn game_value(&self) -> i64 {

        let (hw, winner) = self._has_winner();
        if hw {
            if winner == self.player_1 {
                return 1;
            } else {
                return -1;
            }
        } else if self.is_draw() {
            return 0;
        } else {
            return 2  // In this case the board does not represent a terminal state and will return the dummy value 2.
        }
    }

    /*
        This function receives a player denoted as either 'X' or 'O' representing the current
        player who is to move. The parameters 'alpha' and 'beta' are used to prune the search
        tree. The parameter 'ply' represents the depth of the depth. Increasing the 'ply' value
        returns better moves but also takes longer.

        The function returns three values:
        1. the score of the optimal move for the player who is to act;
        2. the optimal move
    */
    pub fn alpha_beta(&mut self, player: String, mut alpha: i64, mut beta: i64, ply: i64) -> (i64, i64) {

        if self.is_terminal() {
            let game_value = self.game_value();
            if game_value < 0 {
                return (game_value - ply, 0);
            } else if game_value > 0 {
                return (game_value + ply, 0);
            }
            return (self.game_value(), 0);
        }

        let init_score = HashMap::from([
            (self.player_1.clone(), (-i64::MAX, self.player_2.clone())),
            (self.player_2.clone(), (i64::MAX, self.player_1.clone()))
        ]);

        if ply <= 0 {
            return (0, 0);
        }
        
        let ((mut score, next_player), mut mov) = (init_score.get(&player).unwrap(), -1);

        for m in self.available_moves() {
            self.perform_move(m.clone(), player.clone());
            let (m_score, _) = self.clone().alpha_beta(next_player.to_string(), alpha, beta, ply-1);

            if ply == self.difficulty {
                println!("{:?} - {:?}", m, m_score);
            }

            if player == self.player_1.clone() {

                //if score != max(score.clone(), m_score) {
                    //score = m_score.clone();
                    //mov = m.clone();
                //}
                score = max(score.clone(), m_score);
                if beta <= score {
                    self.undo_move(m.clone());
                    return (score.clone(), mov);
                }
                //alpha = max(alpha.clone(), score.clone());
                if score > alpha {
                    alpha = score.clone();
                    mov = m.clone();
                }
            }

            if player == self.player_2.clone() {

                //if score != min(score, m_score) {
                    //score = m_score.clone();
                    //mov = m.clone();
                //}
                score = min(score.clone(), m_score);
                if alpha >= score {
                    self.undo_move(m.clone());
                    return (score.clone(), mov);
                }
                //beta = min(beta.clone(), score.clone());
                if score < beta {
                    beta = score.clone();
                    mov = m.clone();
                }
            }

            self.undo_move(m.clone());
        }

        return (score.clone(), mov);
    }
}

// request model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformMoveRequest {
    pub board_info: Board,
    pub col: i64,
}

// response model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformMoveResponse {
    pub status: GeneralStatus,
    pub player: bool,
    pub human_row: i64,
    pub human_col: i64,
    pub cmput_row: i64,
    pub cmput_col: i64,
    pub winner: String,
}

impl PerformMoveResponse {

    pub fn new(status: (bool, &str), human_move: (i64, i64), cmput_move: (i64, i64), winner: String, player: String, board: &Board) -> Self {

        let s = if status.0 {
            GeneralStatus::success()
        } else { GeneralStatus::failure(status.1) };

        Self {
            status: s,
            player: player.eq(&board.player_2),
            human_row: human_move.0,
            human_col: human_move.1,
            cmput_row: cmput_move.0,
            cmput_col: cmput_move.1,
            winner
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetHistResponse {
    pub status: GeneralStatus,
    pub hist: Vec<HistBoard>,
}