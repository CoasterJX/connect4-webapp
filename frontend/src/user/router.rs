use super::general_elements::SideBar;
use super::settings::BACKEND_URI;
use futures::future::Lazy;
use gloo::timers::future::sleep;
use gloo::{console::log, utils::document};
use serde_json::json;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use std::{sync::Mutex, thread};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Element, Event, HtmlDivElement, HtmlHeadingElement, HtmlImageElement, HtmlInputElement,
    MouseEvent,
};
use yew::{function_component, html, Callback, Html};
use yew_router::{navigator, prelude::*};

#[derive(Clone, Routable, PartialEq)]
enum UserRoute {
    #[at("/user/register")]
    UserRegister,

    #[at("/user/guide")]
    UserGuide,

    #[at("/user/play-computer")]
    UserPlayComputer,

    #[at("/user/play-human")]
    UserPlayHuman,

    #[at("/user/guide-TOOT")]
    UserGuideTOOT,

    #[at("/user/history")]
    UserGameHistory,

    #[at("/user/scoreboard")]
    UserScoreBoard,
}

fn get_input_value(element: &str) -> String {
    return document()
        .get_element_by_id(element)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
}

fn set_heading_message(element: &str, message: &str) {
    let _ = document()
        .get_element_by_id(element)
        .unwrap()
        .dyn_into::<HtmlHeadingElement>()
        .unwrap()
        .set_inner_html(message);
}

fn set_Div_display(element: &str, display: bool) {
    let mut style = "";
    match display {
        true => style = "display: ",
        false => style = "display: none",
    }
    let _ = document()
        .get_element_by_id(element)
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap()
        .set_attribute("style", style);
}

#[function_component(UserRegister)]

fn user_register() -> Html {
    let navigator = use_navigator().unwrap();

    let register_onclick = Callback::from(move |_event: MouseEvent| {
        let name_input = get_input_value("register-name");
        let pwd_input = get_input_value("register-pwd");

        if name_input.contains("_") {
            set_heading_message(
                "register-msg",
                "Register failed! Do not include \"_\" in your username.",
            );
        } else {
            let create_user_uri = format!("{}/user/create", BACKEND_URI);

            let registernav = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest_wasm::Client::new();
                let response = client
                    .post(create_user_uri)
                    .json(&json!({
                        "name": name_input,
                        "pwd": pwd_input,
                        "score": 0
                    }))
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                if !response["status"]["success"].as_bool().unwrap() {
                    let errormessage = response["status"]["msg"]
                        .to_string()
                        .replace("\\", "")
                        .replace("\"", "");

                    set_heading_message(
                        "register-msg",
                        format!("Register failed! {}", errormessage).as_str(),
                    );
                } else {
                    registernav.push(&UserRoute::UserGuide);
                }
            });
        }
    });

    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{ "Enter name and password:" }</h5>
                <div class="flex-container">
                    <input id="register-name" placeholder="Name" style="margin-left: 0px"/>
                    <input id="register-pwd" placeholder="Password"/>
                    <button class="button" onclick={register_onclick}>{ "Register" }</button><br />
                </div>
                <h5 id="register-msg" style="color: red; font-weight: normal">{ "" }</h5>
            </div>
        </div>
    }
}

#[function_component(UserGuide)]
fn user_guide() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"How to Play Connect 4"}</h5><br />
                <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p><br />
                <h5>{"To play Connect 4 follow the following steps:"}</h5>
                <ul>
                    <li>{"A new game describes discs of which color belongs to which player"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
                </ul><br /><br />
                {"For More information on Connect 4 click "} <a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
            </div>
        </div>
    }
}

#[function_component(UserPlayComputer)]
fn user_play_computer() -> Html {
    let login_onclick = Callback::from(move |_event: MouseEvent| {
        let name_input = get_input_value("player-name");
        let pwd_input = get_input_value("player-pwd");

        if name_input.contains("_") {
            set_heading_message(
                "login-msg",
                "Login failed! Do not include \"_\" in your username.",
            );
        } else {
            let verify_user_uri = format!("{}/user/verify", BACKEND_URI);

            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest_wasm::Client::new();
                let response = client
                    .post(verify_user_uri)
                    .json(&json!({
                        "name": name_input,
                        "pwd": pwd_input,
                        "score": 1
                    }))
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                if !response["exists"].as_bool().unwrap() {
                    set_heading_message(
                        "login-msg",
                        "Login failed! User password combination does not exist!",
                    );
                } else {
                    set_Div_display("dimension-prompt", true);
                    set_Div_display("difficulty-prompt", true);
                    set_Div_display("mode-prompt", true);
                    set_Div_display("login-prompt", false);
                }
            });
        }
    });

    let generateBoard = Callback::from(move |_event: MouseEvent| {
        let width = get_input_value("board-width");
        let height = get_input_value("board-height");
        let mode = get_input_value("board-mode");
        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();

        let imgprefix = "<img ";
        let imgsuffix = "src= \"https:\\/\\/i.ibb.co/GFk3XzG/cell-empty.png\" alt=\"Cell\" />";

        let mut finalString = String::from("");

        for j in 0..height.parse().unwrap() {
            let flexprefix = "<div class=\"flex-container\">";
            let flexsuffix = "</div>";
            let mut row = String::from("");
            let mut bundle = String::from("");
            for i in 0..width.parse().unwrap() {
                let id = format!("id = \"{}-{}\"", j, i);
                row += format!("{}{}{}", imgprefix, id, imgsuffix).as_str();
            }
            bundle = format!("{}{}{}", flexprefix, row, flexsuffix);
            let rowFinal = bundle.as_str();
            finalString += rowFinal;
        }

        let create_board_uri = format!("{}/board/create", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(create_board_uri)
                .json(&json!({
                "width": width.parse::<i64>().unwrap(),
                "height": height.parse::<i64>().unwrap(),
                "board": [],
                "last_row": 0,
                "last_col": 0,
                "last_player": "",
                "player_1": get_input_value("player-name"),
                "player_2": "*",
                "mode": pattern,
                "difficulty": get_input_value("board-difficulty")
                            .parse::<i64>()
                            .unwrap(),
                    }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Board generation failed");
            } else {
                set_Div_display("dimension-prompt", false);
                set_Div_display("difficulty-prompt", false);
                set_Div_display("mode-prompt", false);
                set_Div_display("column-prompt", true);
                set_Div_display("giveup-button-prompt", true);

                let _ = document()
                    .get_element_by_id("board")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_inner_html((&finalString).as_str());
            }
        });
    });

    let makeMove = Callback::from(move |_event: MouseEvent| {
        let column = get_input_value("column-number").parse::<i64>().unwrap() - 1;

        let mode = get_input_value("board-mode");

        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();
        // Make the move
        let make_move_uri = format!("{}/board/move", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(make_move_uri)
                .json(&json!({
                    "board_info": {
                        "width": get_input_value("board-width")
                                .parse::<i64>()
                                .unwrap(),
                        "height": get_input_value("board-height")
                                .parse::<i64>()
                                .unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": get_input_value("player-name"),
                        "player_2": "*",
                        "mode": pattern,
                        "difficulty": get_input_value("board-difficulty")
                                    .parse::<i64>()
                                    .unwrap()
                    },
                        "col": column}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            log!("{:#?}", response["winner"].to_string());

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                let human_row = response["human_row"].clone().to_string();
                let human_column = response["human_col"].clone().to_string();
                let _ = document()
                    .get_element_by_id(
                        format!("{}-{}", human_row.clone(), human_column.clone()).as_str(),
                    )
                    .unwrap()
                    .dyn_into::<HtmlImageElement>()
                    .unwrap()
                    .set_attribute("src", "https://i.ibb.co/3z2fDPN/player1-fill.png");

                let cmput_row = response["cmput_row"].clone().to_string();
                let cmput_column = response["cmput_col"].clone().to_string();

                if cmput_row != "-1" {
                    let _ = document()
                        .get_element_by_id(
                            format!("{}-{}", cmput_row.clone(), cmput_column.clone()).as_str(),
                        )
                        .unwrap()
                        .dyn_into::<HtmlImageElement>()
                        .unwrap()
                        .set_attribute("src", "https://i.ibb.co/dgzxtqp/player2-fill.png");
                }

                if response["winner"].as_str().unwrap().to_owned().len() != 0 {
                    let mut winner = String::new();
                    if response["winner"].as_str().unwrap().to_owned() == "*".to_owned() {
                        winner = "Computer won the game!".to_owned();
                    } else if response["winner"].as_str().unwrap().to_owned() == "^".to_owned() {
                        winner = "Draw".to_owned();
                    } else {
                        winner = format!(
                            "{} won the game!",
                            response["winner"].as_str().unwrap().to_owned().clone()
                        );
                    }

                    set_heading_message("winner-msg", winner.as_str());
                    set_Div_display("restart-button-prompt", true);
                    set_Div_display("column-prompt", false);
                    set_Div_display("giveup-button-prompt", false);
                }
            }
        });
    });

    let giveup = Callback::from(move |_event: MouseEvent| {
        let giveup_board_uri = format!("{}/board/move", BACKEND_URI);
        let mode = get_input_value("board-mode");

        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();

        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(giveup_board_uri)
                .json(&json!({
                    "board_info": {
                        "width": get_input_value("board-width")
                                .parse::<i64>()
                                .unwrap(),
                        "height": get_input_value("board-height")
                                .parse::<i64>()
                                .unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": get_input_value("player-name"),
                        "player_2": "*",
                        "mode": pattern,
                        "difficulty": get_input_value("board-difficulty")
                                    .parse::<i64>()
                                    .unwrap()
                    },
                        "col": -1}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            log!("{:#?}", response["winner"].to_string());

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                set_heading_message("winner-msg", "Computer won the game!");
                set_Div_display("restart-button-prompt", true);
                set_Div_display("column-prompt", false);
                set_Div_display("giveup-button-prompt", false);
            }
        });
    });

    html! {
        <div class="sidenavpadding">
            <div>
                <div id="login-prompt">
                    <h5 style="padding-top: 72px">{"Enter your name"}</h5>
                    <div class="flex-container">
                        <input id="player-name" placeholder="Your name" style="margin-left: 0px" readonly=false/>
                        <input id="player-pwd" placeholder="Password" type = "password" readonly=false/>
                        <button class="button" onclick={login_onclick}>{ "Start game" }</button>
                    </div>
                    <h5 id="login-msg" style="color: red; font-weight: normal">{ "" }</h5>
                </div>

                <div class="flex-container">
                    <div id="dimension-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter board dimensions"}</h5>
                        <div class="flex-container">
                            <input id="board-width" placeholder="Width" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                            <input id="board-height" placeholder="Height" type = "number" min = "1" readonly=false/>
                        </div>
                    </div>

                    <div id="difficulty-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter difficulty"}</h5>
                        <div class="flex-container">
                            <input id="board-difficulty" placeholder="Difficulty" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                        </div>
                    </div>

                    <div id="mode-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter pattern"}</h5>
                        <div class="flex-container">
                            <input id="board-mode" placeholder="Mode" style="margin-left: 0px" type="text" pattern="[OT]" maxlength="4" readonly=false/>
                            <button class="button" onclick={generateBoard}>{ "Generate" }</button>
                        </div>
                    </div>
                </div>

                <div id="column-prompt" style="display: none">
                    <h5 style="padding-top: 72px">{"Enter column number to place a checker"}</h5>
                    <div class="flex-container" >
                        <input id="column-number" placeholder="Column number" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                        <button id = "column-button" class="button" onclick={makeMove}>{ "Confirm" }</button>
                    </div>
                </div>
            </div><br />

            <div id = "restart-button-prompt"  style="display: none">
                <a href="/user/play-computer">{"Restart"}</a>
            </div>

            <h5 id="winner-msg" style="color: green; font-weight: normal">{ "" }</h5>

            <div id = "giveup-button-prompt"  style="display: none">
                <button id = "giveup-button" style="margin-left: 0px" class="button" onclick={giveup}>{ "Giveup" }</button>
            </div><br/>

            <div id = "board">
                // Board goes here
            </div><br/>
        </div>
    }
}

#[function_component(UserPlayHuman)]
fn user_play_human() -> Html {
    let mut player = 1;

    let login_onclick = Callback::from(move |_event: MouseEvent| {
        let name_input1 = get_input_value("player-name1");
        let pwd_input1 = get_input_value("player-pwd1");
        let name_input2 = get_input_value("player-name2");
        let pwd_input2 = get_input_value("player-pwd2");

        if name_input1.contains("_") || name_input2.contains("_") {
            set_heading_message(
                "login-msg",
                "Login failed! Do not include \"_\" in your username.",
            );
        } else {
            let verify_user_uri = format!("{}/user/verify", BACKEND_URI);

            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest_wasm::Client::new();
                let response1 = client
                    .post(verify_user_uri.clone())
                    .json(&json!({
                        "name": name_input1,
                        "pwd": pwd_input1,
                        "score": 1
                    }))
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                let response2 = client
                    .post(verify_user_uri)
                    .json(&json!({
                        "name": name_input2,
                        "pwd": pwd_input2,
                        "score": 1
                    }))
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                if !response1["exists"].as_bool().unwrap()
                    || !response2["exists"].as_bool().unwrap()
                {
                    set_heading_message(
                        "login-msg",
                        "Login failed! User password combination does not exist!",
                    );
                } else {
                    set_Div_display("dimension-prompt", true);
                    set_Div_display("difficulty-prompt", true);
                    set_Div_display("mode-prompt", true);
                    set_Div_display("login-prompt", false);
                }
            });
        }
    });

    let generateBoard = Callback::from(move |_event: MouseEvent| {
        let width = get_input_value("board-width");
        let height = get_input_value("board-height");
        let mode = get_input_value("board-mode");
        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();

        let imgprefix = "<img ";
        let imgsuffix = "src= \"https:\\/\\/i.ibb.co/GFk3XzG/cell-empty.png\" alt=\"Cell\" />";

        let mut finalString = String::from("");

        for j in 0..height.parse().unwrap() {
            let flexprefix = "<div class=\"flex-container\">";
            let flexsuffix = "</div>";
            let mut row = String::from("");
            let mut bundle = String::from("");
            for i in 0..width.parse().unwrap() {
                let id = format!("id = \"{}-{}\"", j, i);
                row += format!("{}{}{}", imgprefix, id, imgsuffix).as_str();
            }
            bundle = format!("{}{}{}", flexprefix, row, flexsuffix);
            let rowFinal = bundle.as_str();
            finalString += rowFinal;
        }

        let create_board_uri = format!("{}/board/create", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(create_board_uri)
                .json(&json!({
                    "width": width.parse::<i64>().unwrap(),
                    "height": height.parse::<i64>().unwrap(),
                    "board": [],
                    "last_row": 0,
                    "last_col": 0,
                    "last_player": "",
                    "player_1": get_input_value("player-name1"),
                    "player_2": get_input_value("player-name2"),
                    "mode": pattern,
                    "difficulty": 1,
                }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Board generation failed");
            } else {
                set_Div_display("dimension-prompt", false);
                set_Div_display("difficulty-prompt", false);
                set_Div_display("mode-prompt", false);
                set_Div_display("column-prompt", true);
                set_Div_display("giveup-button-prompt", true);

                let _ = document()
                    .get_element_by_id("board")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_inner_html((&finalString).as_str());
            }
        });
    });

    let makeMove = Callback::from(move |_event: MouseEvent| {
        let column = get_input_value("column-number").parse::<i64>().unwrap() - 1;

        let mode = get_input_value("board-mode");

        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();
        // Make the move
        let make_move_uri = format!("{}/board/move", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(make_move_uri)
                .json(&json!({
                    "board_info": {
                        "width": get_input_value("board-width")
                                .parse::<i64>()
                                .unwrap(),
                        "height": get_input_value("board-height")
                                .parse::<i64>()
                                .unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": get_input_value("player-name1"),
                        "player_2": get_input_value("player-name2"),
                        "mode": pattern,
                        "difficulty": 1
                    },
                        "col": column}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            log!("{:#?}", response["winner"].to_string());

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                let human_row = response["human_row"].clone().to_string();
                let human_column = response["human_col"].clone().to_string();
                if player == 1 {
                    let _ = document()
                        .get_element_by_id(
                            format!("{}-{}", human_row.clone(), human_column.clone()).as_str(),
                        )
                        .unwrap()
                        .dyn_into::<HtmlImageElement>()
                        .unwrap()
                        .set_attribute("src", "https://i.ibb.co/3z2fDPN/player1-fill.png");
                    player = 2;
                } else if player == 2 {
                    let _ = document()
                        .get_element_by_id(
                            format!("{}-{}", human_row.clone(), human_column.clone()).as_str(),
                        )
                        .unwrap()
                        .dyn_into::<HtmlImageElement>()
                        .unwrap()
                        .set_attribute("src", "https://i.ibb.co/dgzxtqp/player2-fill.png");
                    player = 1;
                }

                if response["winner"].as_str().unwrap().to_owned().len() != 0 {
                    let mut winner = String::new();
                    if response["winner"].as_str().unwrap().to_owned() == "*".to_owned() {
                        winner = "Computer won the game!".to_owned();
                    } else if response["winner"].as_str().unwrap().to_owned() == "^".to_owned() {
                        winner = "Draw".to_owned();
                    } else {
                        winner = format!(
                            "{} won the game!",
                            response["winner"].as_str().unwrap().to_owned().clone()
                        );
                    }

                    set_heading_message("winner-msg", winner.as_str());
                    set_Div_display("restart-button-prompt", true);
                    set_Div_display("column-prompt", false);
                    set_Div_display("giveup-button-prompt", false);
                }
            }
        });
    });

    let giveup = Callback::from(move |_event: MouseEvent| {
        let giveup_board_uri = format!("{}/board/move", BACKEND_URI);
        let mode = get_input_value("board-mode");

        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();

        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(giveup_board_uri)
                .json(&json!({
                    "board_info": {
                        "width": get_input_value("board-width")
                                .parse::<i64>()
                                .unwrap(),
                        "height": get_input_value("board-height")
                                .parse::<i64>()
                                .unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": get_input_value("player-name1"),
                        "player_2": get_input_value("player-name2"),
                        "mode": pattern,
                        "difficulty": 1
                    },
                        "col": -1}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            log!("{:#?}", response["winner"].to_string());

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                set_heading_message("winner-msg", "Computer won the game!");
                set_Div_display("restart-button-prompt", true);
                set_Div_display("column-prompt", false);
                set_Div_display("giveup-button-prompt", false);
            }
        });
    });

    html! {
        <div class="sidenavpadding">
            <div>
                <div id="login-prompt">
                    <h5 style="padding-top: 72px">{"Enter your name"}</h5>
                    <div class="flex-container">
                        <input id="player-name1" placeholder="Your name" style="margin-left: 0px" readonly=false/>
                        <input id="player-pwd1" placeholder="Password" type = "password" readonly=false/>
                        <input id="player-name2" placeholder="Your name" style="margin-left: 0px" readonly=false/>
                        <input id="player-pwd2" placeholder="Password" type = "password" readonly=false/>
                        <button class="button" onclick={login_onclick}>{ "Start game" }</button>
                    </div>
                    <h5 id="login-msg" style="color: red; font-weight: normal">{ "" }</h5>
                </div>

                <div class="flex-container">
                    <div id="dimension-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter board dimensions"}</h5>
                        <div class="flex-container">
                            <input id="board-width" placeholder="Width" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                            <input id="board-height" placeholder="Height" type = "number" min = "1" readonly=false/>
                        </div>
                    </div>

                    <div id="difficulty-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter difficulty"}</h5>
                        <div class="flex-container">
                            <input id="board-difficulty" placeholder="Difficulty" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                        </div>
                    </div>

                    <div id="mode-prompt" style="display: none">
                        <h5 style="padding-top: 72px">{"Enter pattern"}</h5>
                        <div class="flex-container">
                            <input id="board-mode" placeholder="Mode" style="margin-left: 0px" type="text" pattern="[OT]" maxlength="4" readonly=false/>
                            <button class="button" onclick={generateBoard}>{ "Generate" }</button>
                        </div>
                    </div>
                </div>

                <div id="column-prompt" style="display: none">
                    <h5 style="padding-top: 72px">{"Enter column number to place a checker"}</h5>
                    <div class="flex-container" >
                        <input id="column-number" placeholder="Column number" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                        <button id = "column-button" class="button" onclick={makeMove}>{ "Confirm" }</button>
                    </div>
                </div>
            </div><br />

            <div id = "restart-button-prompt"  style="display: none">
                <a href="/user/play-human">{"Restart"}</a>
            </div>

            <h5 id="winner-msg" style="color: green; font-weight: normal">{ "" }</h5>

            <div id = "giveup-button-prompt"  style="display: none">
                <button id = "giveup-button" style="margin-left: 0px" class="button" onclick={giveup}>{ "Giveup" }</button>
            </div><br/>

            <div id = "board">
                // Board goes here
            </div><br/>
        </div>
    }
}

#[function_component(UserGuideTOOT)]
fn user_guide_TOOT() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"How to Play TOOT-OTTO"}</h5><br />
                <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}</p><br />
                <h5>{"To play TOOT-OTTO follow the following steps:"}</h5>
                <ul>
                    <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                    <li>{"Select the disc type T or O that you want to place"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
                </ul><br /><br />
                {"For More information on TOOT-OTTO click "} <a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
            </div>
        </div>
    }
}

#[function_component(UserScoreBoard)]
fn user_score_board() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Score board"}</h5><br />

            </div>
        </div>
    }
}

#[function_component(UserGameHistory)]
fn user_game_history() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Game history"}</h5><br />

            </div>
        </div>
    }
}

#[function_component(User)]
pub fn user_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<UserRoute> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: UserRoute) -> Html {
    match routes {
        UserRoute::UserRegister => html! {
            <div>
                <UserRegister />
                <SideBar />
            </div>
        },

        UserRoute::UserGuide => html! {
        <div>
            <UserGuide />
            <SideBar />
        </div>},

        UserRoute::UserPlayComputer => html! {
        <div>
            <UserPlayComputer />
            <SideBar />
        </div>},

        UserRoute::UserPlayHuman => html! {
        <div>
            <UserPlayHuman />
            <SideBar />
        </div>},

        UserRoute::UserGuideTOOT => html! {
        <div>
            <UserGuideTOOT />
            <SideBar />
        </div>},

        UserRoute::UserGameHistory => html! {
        <div>
            <UserGameHistory />
            <SideBar />
        </div>},

        UserRoute::UserScoreBoard => html! {
        <div>
            <UserScoreBoard />
            <SideBar />
        </div>},
    }
}
