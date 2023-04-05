use crate::{
    models::{
        board_model::*,
        general_model::GeneralStatus
    },
    repository::board_repo::BoardRepo
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

extern crate argon2;

const COMPUTER_STR: &str = "*";
const DRAW_STR: &str = "^";

#[post("/board/create", data = "<new_board>")]
pub fn create_board(db: &State<BoardRepo>, new_board: Json<Board>) -> Result<Json<GeneralBoardResponse>, Status> {

    let board_var = Board::new(new_board.width.clone(), new_board.height.clone(), new_board.player_1.clone(), new_board.player_2.clone(), new_board.mode.clone(), new_board.difficulty.clone());
    match db.create_board(board_var.clone()) {

        true => Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::success(),
            board: board_var.clone(),
        })),

        false => Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("User already exists or database not connected."),
            board: board_var.clone(),
        }))
    }
}

#[post("/board/move", data = "<move_req>")]
pub fn perform_move(db: &State<BoardRepo>, move_req: Json<PerformMoveRequest>) -> Result<Json<PerformMoveResponse>, Status> {

    let (board, col) = (
        move_req.board_info.clone(),
        move_req.col.clone()
    );

    match db.get_board(&board) {
        
        Some(mut b) => {
            if b.allows_move(&col) {
                let next_player = b.get_next_player();
                b.perform_move(col.clone(), next_player.clone());
            }
            let human_move = (b.last_row.clone(), b.last_col.clone());

            // case when human wins or draw
            if b.is_terminal() {
                match b.is_draw() {
                    true => return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        (-1, -1),
                        DRAW_STR.to_owned()))
                    ),
                    false => return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        (-1, -1),
                        b.last_player.clone()))),
                }
            }

            // case when the opposite is computer
            if b.get_next_player() == COMPUTER_STR {
                let (_, best_move) = b.alpha_beta(
                    b.get_next_player(),
                    i64::MIN,
                    i64::MAX,
                    b.difficulty
                );

                let next_player = b.get_next_player();
                b.perform_move(best_move.clone(), next_player.clone());
            } else {
                b.last_row = -1;
                b.last_col = -1;
            }
            let cmput_move = (b.last_row.clone(), b.last_col.clone());

            // case when computer wins or draw
            if b.is_terminal() {
                match b.is_draw() {
                    true => return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        cmput_move.clone(),
                        DRAW_STR.to_owned()))
                    ),
                    false => return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        cmput_move.clone(),
                        b.last_player.clone()))
                    ),
                }
            }

            // update the board into mongodb & send the computer move if there is one
            match db.update_board(&b) {
                true => return Ok(Json(PerformMoveResponse::new(
                    (true, ""),
                    human_move.clone(),
                    cmput_move.clone(),
                    "".to_owned()))
                ),
                false => return Ok(Json(PerformMoveResponse::new(
                    (false, "Database not connected."),
                    human_move.clone(),
                    cmput_move.clone(),
                    "".to_owned()))
                ),
            }
        },

        None => return Ok(Json(PerformMoveResponse::new(
            (false, "Board does not exist or database not connected."),
            (-1, -1),
            (-1, -1),
            "".to_owned()))
        ),
    }
}

#[post("/board/info", data = "<board>")]
pub fn get_board(db: &State<BoardRepo>, board: Json<Board>) -> Result<Json<GeneralBoardResponse>, Status> {

    match db.get_board(&board) {

        Some(board) => Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::success(),
            board
        })),

        None => Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Board does not exist or database not connected."),
            board: Board::empty()
        })),
    }
}

#[get("/board/all")]
pub fn get_all_boards(db: &State<BoardRepo>) -> Result<Json<GetAllBoardResponse>, Status> {

    match db.get_all_boards() {
        Some(all_boards) => Ok(Json(GetAllBoardResponse {
            status: GeneralStatus::success(),
            all_boards
        })),
        None => Ok(Json(GetAllBoardResponse {
            status: GeneralStatus::failure("Database not connected."),
            all_boards: vec![]
        })),
    }
}