use std::path::PathBuf;

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

#[get("/board/info/<path1>/<path2>/<path3>/<path4>/<path5>/<path6>")]
pub fn get_board(db: &State<BoardRepo>, path1: String, path2: String, path3: String, path4: String, path5: String, path6: String) -> Result<Json<GeneralBoardResponse>, Status> {
    if path1.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Player 1 cannot be empty."),
            board: Board::empty(),
        }));
    };

    if path2.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Player 2 cannot be empty."),
            board: Board::empty(),
        }));
    };

    if path3.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Mode cannot be empty."),
            board: Board::empty(),
        }));
    };

    if path4.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Difficulty cannot be empty."),
            board: Board::empty(),
        }));
    };

    if path5.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Width cannot be empty."),
            board: Board::empty(),
        }));
    };

    if path6.is_empty() {
        return Ok(Json(GeneralBoardResponse {
            status: GeneralStatus::failure("Height cannot be empty."),
            board: Board::empty(),
        }));
    };

    let p1: String = path1;
    let p2: String = path2;
    let mode: String = path3;
    let difficulty: i64 = path4.trim().parse().unwrap();
    let width: i64 = path5.trim().parse().unwrap();
    let height: i64 = path6.trim().parse().unwrap();

    match db.get_board(&p1, &p2, &mode, &difficulty, &width, &height) {

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