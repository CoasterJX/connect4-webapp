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
use api::hist_api::get_hist;
use api::user_api::*;
use models::board_model::Board;

use repository::hist_repo::HistRepo;
use repository::{
    user_repo::UserRepo,
    board_repo::BoardRepo,
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
            loop {
                welcome();
                // Initialize the variables that are needed //
                let mut player_1: String;
                let mut player_2: String;
                let mode: Vec<bool>;
                let difficulty: i64;
                let width: i64;
                let height: i64;

                // Get who is player 1 //
                loop {
                    player_1 = String::new();
                    println!("Enter player 1 (Press ENTER as computer): ");
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut player_1).unwrap();
                    player_1 = player_1.trim().to_string();
                    if player_1 == "*" || player_1 == "**" {
                        println!("You cannot use that name. Please choose another name.");
                    } else {
                        break;
                    }
                }
                

                // Get who is player 2 //
                loop {
                    player_2 = String::new();
                    println!("Enter player 2 (Press ENTER as computer): ");
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut player_2).unwrap();
                    player_2 = player_2.trim().to_string();
                    if player_2 == "*" || player_2 == "**" {
                        println!("You cannot use those names. Please choose another name.");
                        continue;
                    }
                    if player_1 == "" {
                        player_2 = "*".to_string();
                    }
                    break;
                }

                // Get the game mode //
                loop {
                    println!("Enter the game mode: ");
                    let mut input: String = String::new();
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    input = input.trim().to_string();
                    if input.len() != 4 {
                        println!("Input must contain exactly four characters.");
                        continue;
                    }
                    if !input.chars().all(|c| {
                        c == 'T' || c == 'O'
                    }) {
                        println!("Your input must contain either the characters T or O.");
                        continue;
                    }
                    if input.chars().nth(0) == input.chars().nth(3) && input.chars().nth(1) == input.chars().nth(2) {
                    } else {
                        println!("The gamemode must be symmetric. Please try again.");
                        continue;
                    }
                    mode = input.chars().map(|c| {
                        match c {
                            'T' => false,
                            'O' => true,
                            _ => false
                        }
                    })
                    .collect();
                    break;
                }

                // Get the difficulty //
                loop {
                    println!("Enter the difficulty level: ");
                    let mut input: String = String::new();
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse() {
                        Ok(i) => {
                            if i < 0 {
                                println!("The difficulty must be at least 0.");
                                continue;
                            }
                            difficulty = i;
                            break;
                        },
                        Err(_) => {
                            println!("Your input is invalid. Please try again.");
                            continue;
                        }
                    }
                }

                // Get the width of the board //
                loop {
                    println!("Enter the width of the board: ");
                    let mut input: String = String::new();
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse() {
                        Ok(i) => {
                            if i < 1 {
                                println!("The width of the board must be greater than 0.");
                                continue;
                            }
                            width = i;
                            break;
                        },
                        Err(_) => {
                            println!("Your input is invalid. Please try again.");
                            continue;
                        }
                    }
                }

                // Get the height of the board //
                loop {
                    println!("Enter the height of the board: ");
                    let mut input: String = String::new();
                    io::stdout().flush().unwrap();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse() {
                        Ok(i) => {
                            if i < 1 {
                                println!("The height of the board must be greater than 0.");
                                continue;
                            }
                            height = i;
                            break;
                        },
                        Err(_) => {
                            println!("Your input is invalid. Please try again.");
                            continue;
                        }
                    }
                }

                let db: BoardRepo = BoardRepo::init();
                let mut game_board: Board = match db.get_board(&Board::new(
                    width.clone(), 
                    height.clone(), 
                    player_1.clone(), 
                    player_2.clone(), 
                    mode.clone(), 
                    difficulty.clone(),
                    vec![], vec![]
                )) {
                    Some(board) => board,
                    None => Board::new(
                        width.clone(), 
                        height.clone(), 
                        player_1.clone(), 
                        player_2.clone(), 
                        mode.clone(), 
                        difficulty.clone(),
                        vec![], vec![]
                    ),
                };
                let winner: String = game_board.host_game();
                if player_2 == "*" {
                    if winner == "" {
                        println!("Computer 1 wins -- Congratulations!");
                    } else {
                        println!("Computer 2 wins -- Congratulations!");
                    }
                } else {
                    game_board.print_congrats();
                }
            }
        } else {
            println!("Environment variable not recognized. Launching backend instead.")
        }
    }

    let db_user = UserRepo::init();
    let db_board_active = BoardRepo::init();
    let db_board_hist = HistRepo::init();
    rocket::build()
        .attach(Cors)
        .manage(db_user)
        .manage(db_board_active)
        .manage(db_board_hist)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![verify_pwd])
        .mount("/", routes![placeholder])
        .mount("/", routes![create_board])
        .mount("/", routes![get_board])
        .mount("/", routes![get_all_boards])
        .mount("/", routes![perform_move])
        .mount("/", routes![get_hist])
}
