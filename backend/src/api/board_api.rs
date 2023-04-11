use crate::{
    models::{
        board_model::*,
        general_model::GeneralStatus
    },
    repository::{board_repo::BoardRepo, hist_repo::HistRepo, user_repo::UserRepo}
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

use rand::seq::SliceRandom;

extern crate argon2;

const COMPUTER_STR: &str = "*";
const DRAW_STR: &str = "^";

#[post("/board/create", data = "<new_board>")]
pub fn create_board(db: &State<BoardRepo>, new_board: Json<Board>) -> Result<Json<GeneralBoardResponse>, Status> {

    let board_var = Board::new(
        new_board.width.clone(),
        new_board.height.clone(),
        new_board.player_1.clone(),
        new_board.player_2.clone(),
        new_board.mode.clone(),
        new_board.difficulty.clone(),
        new_board.p1_remain.clone(),
        new_board.p2_remain.clone()
    );
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

    let (board, col, reverse) = (
        move_req.board_info.clone(),
        move_req.col.clone(),
        move_req.reverse.clone()
    );

    match db.get_board(&board) {
        
        // there is a matched active board in database
        Some(mut b) => {

            // give up case
            if col == -1 {
                let winner = b.last_player.clone();
                db.delete_board(&b);
                HistRepo::init().push_hist(&b, &winner);
                UserRepo::init().add_score(&winner, b.difficulty.clone());
                UserRepo::init().add_score(&b.opponent(&winner), -b.difficulty.clone()*10);

                return Ok(Json(PerformMoveResponse::new(
                    (true, ""),
                    (-1, -1),
                    (-1, -1),
                    (false, false),
                    winner.clone(),
                    b.last_player.clone(),
                    &b.clone()
                )));
            }

            println!("{:?}-{:?}-{:?}", col, b.get_next_player().clone(), reverse);
            if b.allows_move(&col, &b.get_next_player().clone(), &reverse) {
                let next_player = b.get_next_player();
                b.perform_move(col.clone(), next_player.clone(), &reverse);
            } else {
                // case when move is invalid
                return Ok(Json(PerformMoveResponse::new(
                    (false, "Invalid move."),
                    (-1, -1),
                    (-1, -1),
                    (false, false),
                    "".to_owned(),
                    "".to_owned(),
                    &Board::empty()
                )));
            }
            let human_move = (b.last_row.clone(), b.last_col.clone());

            // case when human wins, draw, or lose
            match b._has_winner() {
                (true, winner) => {

                    db.delete_board(&b);
                    HistRepo::init().push_hist(&b, &winner);
                    UserRepo::init().add_score(&winner, b.difficulty.clone());
                    UserRepo::init().add_score(&b.opponent(&winner), -b.difficulty.clone());

                    return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        (-1, -1),
                        (reverse.clone(), false),
                        winner.clone(),
                        b.last_player.clone(),
                        &b.clone()
                    )));
                },
                (false, _) => ()
            };
            match b.is_draw() {
                true => {

                    db.delete_board(&b);
                    HistRepo::init().push_hist(&b, &DRAW_STR.to_owned());

                    return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        (-1, -1),
                        (reverse.clone(), false),
                        DRAW_STR.to_owned(),
                        b.last_player.clone(),
                        &b.clone()
                    )));
                },
                false => (),
            };
            assert!(!b.is_terminal());

            // case when the opposite is computer
            let mut cmput_rev = false;
            if b.get_next_player() == COMPUTER_STR {
                let mut b_sim = b.clone();

                // add a mutation possibility
                // let (a, bc, c) = (
                //     b_sim.difficulty.clone(),
                //     b_sim.difficulty.clone() - 2,
                //     b_sim.difficulty.clone() - 4
                // );
                // b_sim.difficulty = vec![
                //     a, a, a, a, a, a, a,
                //     bc, bc,
                //     c
                // ].choose(&mut rand::thread_rng()).unwrap().clone();
                // if b_sim.difficulty < 1 {
                //     b_sim.difficulty = 1;
                // }

                // get the best move
                println!("---------- 1st ----------");
                let (sc1, mut best_move) = b_sim.alpha_beta(
                    b.get_next_player(),
                    i64::MIN,
                    i64::MAX,
                    b.difficulty,
                    &false,
                    &(b.p1_remain.len() == 2)
                );

                // if TOOT rule, then make a reverse case as well
                if b.p1_remain.len() == 2 {
                    println!("---------- 2nd ----------");
                    let (sc2, best_move_2) = b.clone().alpha_beta(
                        b.get_next_player(),
                        i64::MIN,
                        i64::MAX,
                        b.difficulty,
                        &true,
                        &true
                    );

                    println!("b1: {:?}, b2: {:?}", best_move, best_move_2);

                    // choose optimum one
                    if sc1 > sc2 {
                        println!("Reverse is better");
                        best_move = best_move_2.clone();
                        cmput_rev = true;
                    } else if sc1 == sc2 {
                        println!("Either is OK");
                        cmput_rev = vec![false, true].choose(&mut rand::thread_rng()).unwrap().clone();
                        println!("{:?}", cmput_rev);
                        if cmput_rev {
                            best_move = best_move_2.clone();
                        }
                        // best_move = vec![best_move.clone(), best_move_2.clone()]
                        //     .choose(&mut rand::thread_rng()).unwrap().clone();
                    } else {
                        println!("Normal is better");
                    }
                }

                let next_player = b.get_next_player();
                println!("Move to {:?}", best_move.clone());
                b.perform_move(best_move.clone(), next_player.clone(), &cmput_rev);
            } else {
                b.last_row = -1;
                b.last_col = -1;
            }
            let cmput_move = (b.last_row.clone(), b.last_col.clone());

            // case when computer wins, draw, or lose
            match b._has_winner() {
                (true, winner) => {

                    db.delete_board(&b);
                    HistRepo::init().push_hist(&b, &winner);
                    UserRepo::init().add_score(&winner, b.difficulty.clone());
                    UserRepo::init().add_score(&b.opponent(&winner), -b.difficulty.clone());

                    return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        cmput_move.clone(),
                        (reverse.clone(), cmput_rev.clone()),
                        winner.clone(),
                        b.last_player.clone(),
                        &b.clone()
                    )))
                },
                (false, _) => ()
            };
            match b.is_draw() {
                true => {

                    db.delete_board(&b);
                    HistRepo::init().push_hist(&b, &DRAW_STR.to_owned());
                    
                    return Ok(Json(PerformMoveResponse::new(
                        (true, ""),
                        human_move.clone(),
                        cmput_move.clone(),
                        (reverse.clone(), cmput_rev.clone()),
                        DRAW_STR.to_owned(),
                        b.last_player.clone(),
                        &b.clone()
                    )))
                },
                false => (),
            };
            assert!(!b.is_terminal());

            // update the board into mongodb & send the computer move if there is one
            match db.update_board(&b) {
                true => return Ok(Json(PerformMoveResponse::new(
                    (true, ""),
                    human_move.clone(),
                    cmput_move.clone(),
                    (reverse.clone(), cmput_rev.clone()),
                    "".to_owned(),
                    b.last_player.clone(),
                    &b.clone()
                ))),
                false => return Ok(Json(PerformMoveResponse::new(
                    (false, "Database not connected."),
                    human_move.clone(),
                    cmput_move.clone(),
                    (reverse.clone(), cmput_rev.clone()),
                    "".to_owned(),
                    b.last_player.clone(),
                    &b.clone()
                ))),
            }
        },

        // no matched board found in database
        None => return Ok(Json(PerformMoveResponse::new(
            (false, "Board does not exist or database not connected."),
            (-1, -1),
            (-1, -1),
            (false, false),
            "".to_owned(),
            "".to_owned(),
            &Board::empty()
        ))),
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