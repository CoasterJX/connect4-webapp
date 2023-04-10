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

fn verify_board_setting(mode: &str) -> bool {
    let mut pass = true;

    if get_input_value("board-width") == ""
        || get_input_value("board-height") == ""
        || get_input_value("board-mode") == ""
    {
        pass = false;
    }

    if mode == "computer" {
        if get_input_value("board-difficulty") == "" {
            pass = false;
        }
    }

    if pass == false {
        set_heading_message(
            "info-msg",
            "Please provide all information, leave no box empty!",
        );
        return pass;
    }

    if get_input_value("board-width").parse::<i64>().unwrap() < 1
        || get_input_value("board-height").parse::<i64>().unwrap() < 1
        || !get_input_value("board-mode")
            .chars()
            .all(|c| "OT".contains(c))
        || get_input_value("board-mode").len() < 4
    {
        pass = false;
    }

    if mode == "computer" {
        if get_input_value("board-difficulty").parse::<i64>().unwrap() < 1
            || get_input_value("board-difficulty").parse::<i64>().unwrap() > 5
        {
            pass = false;
        }
    }

    if pass == false {
        set_heading_message("info-msg", "Invalid input!");
        return pass;
    }

    return pass;
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
                    set_Div_display("info-prompt", true);
                    set_Div_display("login-prompt", false);
                }
            });
        }
    });

    let generateBoard = Callback::from(move |_event: MouseEvent| {
        let width = get_input_value("board-width");
        let height = get_input_value("board-height");
        if !verify_board_setting("computer") {
            return;
        }

        set_heading_message("info-msg", "");

        let difficulty = get_input_value("board-difficulty").parse::<i64>().unwrap() * 2 - 1;

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

        let recover_board_uri = format!("{}/board/info", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(recover_board_uri)
                .json(&json!({
                    "width": get_input_value("board-width").parse::<i64>().unwrap(),
                    "height": get_input_value("board-height").parse::<i64>().unwrap(),
                    "board": [],
                    "last_row": 0,
                    "last_col": 0,
                    "last_player": "",
                    "player_1": get_input_value("player-name"),
                    "player_2": "*",
                    "mode": get_input_value("board-mode").chars().map(|c| c.eq(&'O')).collect::<Vec<_>>(),
                    "difficulty": difficulty.clone(),
                }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Recover board failed");

                let create_board_uri = format!("{}/board/create", BACKEND_URI);
                wasm_bindgen_futures::spawn_local(async move {
                    let client = reqwest_wasm::Client::new();
                    let response = client
                        .post(create_board_uri)
                        .json(&json!({
                        "width": get_input_value("board-width").parse::<i64>().unwrap(),
                        "height": get_input_value("board-height").clone().parse::<i64>().unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": get_input_value("player-name"),
                        "player_2": "*",
                        "mode": get_input_value("board-mode").chars().map(|c| c.eq(&'O')).collect::<Vec<_>>(),
                        "difficulty": difficulty,
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
                        set_Div_display("info-prompt", false);
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
            } else {
                set_Div_display("info-prompt", false);
                set_Div_display("column-prompt", true);
                set_Div_display("giveup-button-prompt", true);

                let _ = document()
                    .get_element_by_id("board")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_inner_html((&finalString).as_str());

                let board = response["board"]["board"].as_array().unwrap();
                for j in 0..get_input_value("board-height")
                    .clone()
                    .parse::<i64>()
                    .unwrap()
                {
                    for i in 0..get_input_value("board-width").parse::<i64>().unwrap() {
                        if board[j as usize][i as usize].as_str().unwrap()
                            == response["board"]["player_1"].as_str().unwrap()
                        {
                            let _ = document()
                                .get_element_by_id(format!("{}-{}", j, i).as_str())
                                .unwrap()
                                .dyn_into::<HtmlImageElement>()
                                .unwrap()
                                .set_attribute("src", "https://i.ibb.co/3z2fDPN/player1-fill.png");
                        } else if board[j as usize][i as usize].as_str().unwrap()
                            == response["board"]["player_2"].as_str().unwrap()
                        {
                            let _ = document()
                                .get_element_by_id(format!("{}-{}", j, i).as_str())
                                .unwrap()
                                .dyn_into::<HtmlImageElement>()
                                .unwrap()
                                .set_attribute("src", "https://i.ibb.co/dgzxtqp/player2-fill.png");
                        }
                    }
                }
            }
        });
    });

    let makeMove = Callback::from(move |_event: MouseEvent| {
        let column = get_input_value("column-number").parse::<i64>().unwrap() - 1;

        if column < 0 {
            set_heading_message("winner-msg", "Invalid column number!");
            return;
        }

        let difficulty = get_input_value("board-difficulty").parse::<i64>().unwrap() * 2 - 1;

        set_heading_message("winner-msg", "");

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
                        "difficulty": difficulty
                    },
                        "col": column}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

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
        let difficulty = get_input_value("board-difficulty").parse::<i64>().unwrap() * 2 - 1;

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
                        "difficulty": difficulty
                    },
                        "col": -1}))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

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


                <div id = "info-prompt" style="display: none">
                    <div class="flex-container">
                        <div id="dimension-prompt">
                            <h5 style="padding-top: 72px">{"Enter board dimensions"}</h5>
                            <div class="flex-container">
                                <input id="board-width" placeholder="Width" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                                <input id="board-height" placeholder="Height" type = "number" min = "1" readonly=false/>
                            </div>
                        </div>

                        <div id="difficulty-prompt">
                            <h5 style="padding-top: 72px">{"Enter difficulty"}</h5>
                            <div class="flex-container">
                                <input id="board-difficulty" placeholder="Difficulty" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                            </div>
                        </div>

                        <div id="mode-prompt">
                            <h5 style="padding-top: 72px">{"Enter pattern"}</h5>
                            <div class="flex-container">
                                <input id="board-mode" placeholder="Mode" style="margin-left: 0px" type="text" pattern="[OT]" maxlength="4" readonly=false/>
                                <button class="button" onclick={generateBoard}>{ "Generate" }</button>
                            </div>
                        </div>
                    </div>
                    <ul>
                        <li>{"Height and width: A positive interger."}</li>
                        <li>{"Difficulty: An integer between 1 and 5, inclusive."}</li>
                        <li>{"Mode: 4 letters consist of only \"O\" and \"T\"."}</li>
                    </ul>
                    <h5 id="info-msg" style="color: red; font-weight: normal">{ "" }</h5>
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

            <h5 id="winner-msg" style="font-weight: normal">{ "" }</h5>

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
                    set_Div_display("info-prompt", true);
                    set_Div_display("login-prompt", false);
                }
            });
        }
    });

    let generateBoard = Callback::from(move |_event: MouseEvent| {
        let width = get_input_value("board-width");
        let height = get_input_value("board-height");

        if !verify_board_setting("human") {
            return;
        }

        set_heading_message("info-msg", "");

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

        let recover_board_uri = format!("{}/board/info", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(recover_board_uri)
                .json(&json!({
                    "width": get_input_value("board-width").parse::<i64>().unwrap(),
                    "height": get_input_value("board-height").clone().parse::<i64>().unwrap(),
                    "board": [],
                    "last_row": 0,
                    "last_col": 0,
                    "last_player": "",
                    "player_1": get_input_value("player-name1"),
                    "player_2": get_input_value("player-name2"),
                    "mode": get_input_value("board-mode").chars().map(|c| c.eq(&'O')).collect::<Vec<_>>(),
                    "difficulty": 1,
                }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Recover board failed");

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
                            "mode": get_input_value("board-mode").chars().map(|c| c.eq(&'O')).collect::<Vec<_>>(),
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
                        set_Div_display("info-prompt", false);
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
            } else {
                set_Div_display("info-prompt", false);
                set_Div_display("column-prompt", true);
                set_Div_display("giveup-button-prompt", true);

                let _ = document()
                    .get_element_by_id("board")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_inner_html((&finalString).as_str());

                let board = response["board"]["board"].as_array().unwrap();
                for j in 0..get_input_value("board-height")
                    .clone()
                    .parse::<i64>()
                    .unwrap()
                {
                    for i in 0..get_input_value("board-width").parse::<i64>().unwrap() {
                        if board[j as usize][i as usize].as_str().unwrap()
                            == response["board"]["player_1"].as_str().unwrap()
                        {
                            let _ = document()
                                .get_element_by_id(format!("{}-{}", j, i).as_str())
                                .unwrap()
                                .dyn_into::<HtmlImageElement>()
                                .unwrap()
                                .set_attribute("src", "https://i.ibb.co/3z2fDPN/player1-fill.png");
                        } else if board[j as usize][i as usize].as_str().unwrap()
                            == response["board"]["player_2"].as_str().unwrap()
                        {
                            let _ = document()
                                .get_element_by_id(format!("{}-{}", j, i).as_str())
                                .unwrap()
                                .dyn_into::<HtmlImageElement>()
                                .unwrap()
                                .set_attribute("src", "https://i.ibb.co/dgzxtqp/player2-fill.png");
                        }
                    }
                }
            }
        });
    });

    let makeMove = Callback::from(move |_event: MouseEvent| {
        let column = get_input_value("column-number").parse::<i64>().unwrap() - 1;

        if column < 0 {
            set_heading_message("winner-msg", "Invalid column number!");
            return;
        }

        set_heading_message("winner-msg", "");

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

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                let human_row = response["human_row"].clone().to_string();
                let human_column = response["human_col"].clone().to_string();
                if response["player"].as_bool().unwrap() == false {
                    let _ = document()
                        .get_element_by_id(
                            format!("{}-{}", human_row.clone(), human_column.clone()).as_str(),
                        )
                        .unwrap()
                        .dyn_into::<HtmlImageElement>()
                        .unwrap()
                        .set_attribute("src", "https://i.ibb.co/3z2fDPN/player1-fill.png");
                } else {
                    let _ = document()
                        .get_element_by_id(
                            format!("{}-{}", human_row.clone(), human_column.clone()).as_str(),
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

            if !response["status"]["success"].as_bool().unwrap() {
                log!("Make move failed");
            } else {
                set_heading_message(
                    "winner-msg",
                    format!(
                        "{} won the game!",
                        response["winner"].as_str().unwrap().to_owned().clone()
                    )
                    .as_str(),
                );
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

                <div id = "info-prompt" style="display: none">
                    <div class="flex-container">
                        <div id="dimension-prompt">
                            <h5 style="padding-top: 72px">{"Enter board dimensions"}</h5>
                            <div class="flex-container">
                                <input id="board-width" placeholder="Width" style="margin-left: 0px" type = "number" min = "1" readonly=false/>
                                <input id="board-height" placeholder="Height" type = "number" min = "1" readonly=false/>
                            </div>
                        </div>

                        <div id="mode-prompt">
                            <h5 style="padding-top: 72px">{"Enter pattern"}</h5>
                            <div class="flex-container">
                                <input id="board-mode" placeholder="Mode" style="margin-left: 0px" type="text" pattern="[OT]" maxlength="4" readonly=false/>
                                <button class="button" onclick={generateBoard}>{ "Generate" }</button>
                            </div>
                        </div>
                    </div>
                    <ul>
                        <li>{"Height and width: A positive interger."}</li>
                        <li>{"Mode: 4 letters consist of only \"O\" and \"T\"."}</li>
                    </ul>
                    <h5 id="info-msg" style="color: red; font-weight: normal">{ "" }</h5>
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

            <h5 id="winner-msg" style="font-weight: normal">{ "" }</h5>

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
    let score_board_uri = format!("{}/user/all", BACKEND_URI);

    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest_wasm::Client::new();
        let response = client
            .get(score_board_uri)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        if !response["status"]["success"].as_bool().unwrap() {
            log!("Get score board failed!");
        } else {
            let scoreboardprefix = "<table><tr><th>User</th><th>Score</th></tr>";
            let scoreboardsuffix = "</table>";
            let mut content = String::new();
            for i in 0..response["all_users"].as_array().unwrap().len() {
                content += format!(
                    "<tr><td>{}</td><td>{}</td></tr>",
                    response["all_users"][i]["name"].as_str().unwrap(),
                    response["all_users"][i]["score"].as_i64().unwrap()
                )
                .as_str();
            }
            let _ = document()
                .get_element_by_id("scoreboard")
                .unwrap()
                .dyn_into::<HtmlDivElement>()
                .unwrap()
                .set_inner_html(
                    format!("{}{}{}", scoreboardprefix, content, scoreboardsuffix).as_str(),
                );
        }
    });

    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Score board"}</h5><br />
            </div>

            <div id = "scoreboard">
            </div><br/>

        </div>
    }
}

#[function_component(UserGameHistory)]
fn user_game_history() -> Html {
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
                        "name": name_input.clone(),
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
                    set_Div_display("login-prompt", false);
                    set_Div_display("game-history", true);

                    let game_history_uri =
                        format!("{}/hist/get/{}", BACKEND_URI, name_input.replace(" ", "_"));

                    wasm_bindgen_futures::spawn_local(async move {
                        let client = reqwest_wasm::Client::new();
                        let response = client
                            .get(game_history_uri)
                            .send()
                            .await
                            .unwrap()
                            .json::<serde_json::Value>()
                            .await
                            .unwrap();

                        if !response["status"]["success"].as_bool().unwrap() {
                            log!("Get game history failed!");
                        } else {
                            let scoreboardprefix = "<table><tr><th>Player 1</th><th>Player 2</th><th>Date</th><th>Width</th><th>Height</th><th>Mode</th><th>Difficulty</th><th>Winner</th></tr>";
                            let scoreboardsuffix = "</table>";
                            let mut content = String::new();

                            for i in 0..response["hist"].as_array().unwrap().len() {
                                let mut pattern = String::new();
                                for j in 0..4 {
                                    if response["hist"][i]["board"]["mode"].as_array().unwrap()[j]
                                        == false
                                    {
                                        pattern += "T";
                                    } else if response["hist"][i]["board"]["mode"]
                                        .as_array()
                                        .unwrap()[i]
                                        == true
                                    {
                                        pattern += "O";
                                    }
                                }

                                let mut winner = response["hist"][i]["winner"].as_str().unwrap();
                                if winner == "*" {
                                    winner = "Computer";
                                }

                                let mut player2 =
                                    response["hist"][i]["board"]["player_2"].as_str().unwrap();

                                let mut difficulty_raw =
                                    response["hist"][i]["board"]["difficulty"].as_i64().unwrap();

                                let mut difficulty = ((difficulty_raw + 1) / 2).to_string();

                                if player2 == "*" {
                                    player2 = "Computer";
                                } else {
                                    difficulty = String::from("N/A");
                                }

                                content += format!(
                                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                                    response["hist"][i]["board"]["player_1"].as_str().unwrap(),
                                    player2,
                                    response["hist"][i]["date"].as_str().unwrap(),
                                    response["hist"][i]["board"]["width"].as_i64().unwrap(),
                                    response["hist"][i]["board"]["height"].as_i64().unwrap(),
                                    pattern,
                                    difficulty,
                                    winner,
                                )
                                .as_str();
                            }
                            let _ = document()
                                .get_element_by_id("game-history-table")
                                .unwrap()
                                .dyn_into::<HtmlDivElement>()
                                .unwrap()
                                .set_inner_html(
                                    format!("{}{}{}", scoreboardprefix, content, scoreboardsuffix)
                                        .as_str(),
                                );
                        }
                    });
                }
            });
        }
    });

    html! {
        <div class="sidenavpadding">

            <div id="login-prompt">
                <h5 style="padding-top: 72px">{"Enter your name"}</h5>
                    <div class="flex-container">
                        <input id="player-name" placeholder="Your name" style="margin-left: 0px" readonly=false/>
                        <input id="player-pwd" placeholder="Password" type = "password" readonly=false/>
                        <button class="button" onclick={login_onclick}>{ "Check my history" }</button>
                    </div>
                <h5 id="login-msg" style="color: red; font-weight: normal">{ "" }</h5>
            </div>


            <div id = "game-history" style = "display: none">
                <div>
                    <h5 style="padding-top: 72px">{"Game history"}</h5><br />
                </div>

                <div id = "game-history-table">
                </div><br/>
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
