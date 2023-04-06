mod api;
mod command_line_interface;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use std::env;
use std::io;
use std::io::Write;

use api::board_api::*;
use api::user_api::*;
use models::board_model::Board;

use repository::{
    user_repo::UserRepo,
    board_repo::BoardRepo,
    db_type::*
};

use rocket::{
    http::Header,
    routes,
    Request,
    Response
};

use rocket::fairing::{Fairing, Info, Kind};

use command_line_interface::welcome;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    // let allowed_origins = AllowedOrigins::all();
    // let cors = CorsOptions {
    //     allowed_origins,
    //     // allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
    //     allow_credentials: true,
    //     ..Default::default()
    // }.to_cors().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd: &String = &args[1];
        if cmd == "debug" {
            welcome();

            // Initialize the variables that are needed //
            let mut player_1: String = String::new();
            let mut player_2: String = String::new();
            let mut mode: Vec<bool> = vec![];
            let mut difficulty: i64 = i64::from(1);
            let mut width: i64 = i64::from(1);
            let mut height: i64 = i64::from(1);

            // Get who is player 1 //
            println!("Enter player 1 (Press ENTER as computer): ");
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut player_1).unwrap();
            player_1 = player_1.trim().to_string();

            // Get who is player 2 //
            println!("Enter player 2 (Press ENTER as computer): ");
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut player_2).unwrap();
            player_2 = player_2.trim().to_string();

            // Get the game mode //
            println!("Enter the game mode: ");
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            mode = input.chars().map(|c| {
                match c {
                    'T' => false,
                    'O' => true,
                    _ => false
                }
            })
            .collect();

            // Get the difficulty //
            println!("Enter the difficulty level: ");
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut input).unwrap();
            difficulty = input.trim().parse().unwrap();

            // Get the width of the board //
            println!("Enter the width of the board: ");
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut input).unwrap();
            width = input.trim().parse().unwrap();

            // Get the height of the board //
            println!("Enter the height of the board: ");
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            let _ = io::stdin().read_line(&mut input).unwrap();
            height = input.trim().parse().unwrap();
            let db: BoardRepo = BoardRepo::init(COL_BOARD);
            let mut game_board: Board = match db.get_board(&Board::new(width.clone(), height.clone(), player_1.clone(), player_2.clone(), mode.clone(), difficulty.clone())) {
                Some(board) => board,
                None => Board::new(width.clone(), height.clone(), player_1.clone(), player_2.clone(), mode.clone(), difficulty.clone()),
            };
            game_board.host_game();
            game_board.print();
        } else {
            println!("Environment variable not recognized. Launching backend instead.")
        }
    }

    let db_user = UserRepo::init();
    let db_board_active = BoardRepo::init(COL_BOARD);
    rocket::build()
        .attach(Cors)
        .manage(db_user)
        .manage(db_board_active)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![verify_pwd])
        .mount("/", routes![placeholder])
        .mount("/", routes![create_board])
        .mount("/", routes![get_board])
        .mount("/", routes![get_all_boards])
        .mount("/", routes![perform_move])
}
