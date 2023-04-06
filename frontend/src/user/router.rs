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
    #[at("/user")]
    UserHome,

    #[at("/user/login")]
    UserLogin,

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

    #[at("/user/play-computer-TOOT")]
    UserPlayComputerTOOT,

    #[at("/user/play-human-TOOT")]
    UserPlayHumanTOOT,

    #[at("/user/history")]
    UserGameHistory,

    #[at("/user/scoreboard")]
    UserScoreBoard,

    #[at("/user/settings")]
    UserSettings,
}

#[function_component(UserHome)]
fn user_home() -> Html {
    let navigator = use_navigator().unwrap();
    let (login_nav, register_nav) = (navigator.clone(), navigator.clone());

    let login_onclick = Callback::from(move |_| login_nav.push(&UserRoute::UserLogin));
    let register_onclick = Callback::from(move |_| register_nav.push(&UserRoute::UserRegister));

    html! {
        <div class="sidenavpadding">
            <div>
                <h1>{ "Login/Register" }</h1>
                <button onclick={login_onclick}>{ "Login" }</button>
                <button onclick={register_onclick}>{ "Register" }</button>
            </div>
        </div>
    }
}

#[function_component(UserLogin)]
fn user_login() -> Html {
    let navigator = use_navigator().unwrap();

    let login_nav = navigator.clone();
    let login_onclick = Callback::from(move |_event: MouseEvent| {
        let name_input = document()
            .get_element_by_id("login-name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let pwd_input = document()
            .get_element_by_id("login-pwd")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let verify_user_uri = format!("{}/user/verify", BACKEND_URI);
        let get_user_uri = format!("{}/user/info/{}", BACKEND_URI, name_input.replace(" ", "_"));

        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let validity = client
                .post(verify_user_uri)
                .json(&json!({
                    "name": name_input,
                    "location": "",
                    "title": "",
                    "pwd": pwd_input
                }))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();
            if !validity["exists"].as_bool().unwrap() {
                document()
                    .get_element_by_id("login-err-msg")
                    .unwrap()
                    .dyn_into::<HtmlHeadingElement>()
                    .unwrap()
                    .set_inner_html("Login failed! Check your user name & password.");
                //.set_node_value(Some("Login failed! Check your user name & password."));
            } else {
                let user = reqwest_wasm::get(get_user_uri)
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();
            }
        });
    });

    html! {
        <div>
            <h5>{ "Enter name & pwd aa:" }</h5>
            <input id="login-name" placeholder="Name" /><br />
            <input id="login-pwd" placeholder="Password" /><br />
            <button class="dif" onclick={login_onclick}>{ "Login" }</button><br />
            <h1 id="login-err-msg">{ "" }</h1>
        </div>
    }
}

#[function_component(UserRegister)]

fn user_register() -> Html {
    let navigator = use_navigator().unwrap();

    let register_onclick = Callback::from(move |_event: MouseEvent| {
        let name_input = document()
            .get_element_by_id("register-name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let pwd_input = document()
            .get_element_by_id("register-pwd")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        if name_input.contains("_") {
            let _ = document()
                .get_element_by_id("register-msg")
                .unwrap()
                .dyn_into::<HtmlHeadingElement>()
                .unwrap()
                .set_inner_html("Register failed! Do not include \"_\" in your username.");
        } else {
            let create_user_uri = format!("{}/user/create", BACKEND_URI);
            let get_user_uri =
                format!("{}/user/info/{}", BACKEND_URI, name_input.replace(" ", "_"));

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

                    document()
                        .get_element_by_id("register-msg")
                        .unwrap()
                        .dyn_into::<HtmlHeadingElement>()
                        .unwrap()
                        .set_inner_html(format!("Register failed! {}", errormessage).as_str());
                    //.set_node_value(Some("Login failed! Check your user name & password."));
                } else {
                    let _ = document()
                        .get_element_by_id("register-msg")
                        .unwrap()
                        .dyn_into::<HtmlHeadingElement>()
                        .unwrap()
                        .set_inner_html("Register success!");

                    let _ = sleep(Duration::new(5, 0));
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
        let name_input = document()
            .get_element_by_id("player-name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let pwd_input = document()
            .get_element_by_id("player-pwd")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        if name_input.contains("_") {
            let _ = document()
                .get_element_by_id("login-msg")
                .unwrap()
                .dyn_into::<HtmlHeadingElement>()
                .unwrap()
                .set_inner_html("Login failed! Do not include \"_\" in your username.");
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
                    document()
                        .get_element_by_id("login-msg")
                        .unwrap()
                        .dyn_into::<HtmlHeadingElement>()
                        .unwrap()
                        .set_inner_html("Login failed! User password combination does not exist!");
                } else {
                    let _ = document()
                        .get_element_by_id("login-msg")
                        .unwrap()
                        .dyn_into::<HtmlHeadingElement>()
                        .unwrap()
                        .set_attribute("style", "display: none");

                    let _ = document()
                        .get_element_by_id("dimension-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: ");

                    let _ = document()
                        .get_element_by_id("difficulty-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: ");

                    let _ = document()
                        .get_element_by_id("mode-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: ");

                    let _ = document()
                        .get_element_by_id("login-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: none");
                }
            });
        }
    });

    let generateBoard = Callback::from(move |_event: MouseEvent| {
        let width = document()
            .get_element_by_id("board-width")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let height = document()
            .get_element_by_id("board-height")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let mode = document()
            .get_element_by_id("board-mode")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

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

        let _ = document()
            .get_element_by_id("board")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .set_inner_html((&finalString).as_str());

        let _ = document()
            .get_element_by_id("column-prompt")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .set_attribute("style", "display: ");

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
                "player_1": document()
                            .get_element_by_id("player-name")
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap()
                            .value(),
                "player_2": "*",
                "mode": pattern,
                "difficulty": document()
                            .get_element_by_id("board-difficulty")
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap()
                            .value()
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
                let _ = document()
                    .get_element_by_id("dimension-prompt")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_attribute("style", "display: none");

                let _ = document()
                    .get_element_by_id("difficulty-prompt")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_attribute("style", "display: none");

                let _ = document()
                    .get_element_by_id("mode-prompt")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_attribute("style", "display: none");
            }
        });
    });

    let makeMove = Callback::from(move |_event: MouseEvent| {
        let column = document()
            .get_element_by_id("column-number")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<i64>()
            .unwrap()
            - 1;

        let mode = document()
            .get_element_by_id("board-mode")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let pattern: Vec<bool> = mode.chars().map(|c| c.eq(&'O')).collect();
        // Make the move
        let create_board_uri = format!("{}/board/move", BACKEND_URI);
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let response = client
                .post(create_board_uri)
                .json(&json!({
                    "board_info": {
                        "width": document()
                                .get_element_by_id("board-width")
                                .unwrap()
                                .dyn_into::<HtmlInputElement>()
                                .unwrap()
                                .value()
                                .parse::<i64>()
                                .unwrap(),
                        "height": document()
                                .get_element_by_id("board-height")
                                .unwrap()
                                .dyn_into::<HtmlInputElement>()
                                .unwrap()
                                .value()
                                .parse::<i64>()
                                .unwrap(),
                        "board": [],
                        "last_row": 0,
                        "last_col": 0,
                        "last_player": "",
                        "player_1": document()
                                    .get_element_by_id("player-name")
                                    .unwrap()
                                    .dyn_into::<HtmlInputElement>()
                                    .unwrap()
                                    .value(),
                        "player_2": "*",
                        "mode": pattern,
                        "difficulty": document()
                                    .get_element_by_id("board-difficulty")
                                    .unwrap()
                                    .dyn_into::<HtmlInputElement>()
                                    .unwrap()
                                    .value()
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

                    document()
                        .get_element_by_id("winner-msg")
                        .unwrap()
                        .dyn_into::<HtmlHeadingElement>()
                        .unwrap()
                        .set_inner_html(winner.as_str());

                    let _ = document()
                        .get_element_by_id("restart-button-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: ");

                    let _ = document()
                        .get_element_by_id("column-prompt")
                        .unwrap()
                        .dyn_into::<HtmlDivElement>()
                        .unwrap()
                        .set_attribute("style", "display: none");
                }
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
                        <input id="player-pwd" placeholder="Password" style="margin-left: 0px" type = "password" readonly=false/>
                        <button class="button" onclick={login_onclick}>{ "Start game" }</button>
                    </div>
                </div>

                <h5 id="login-msg" style="color: red; font-weight: normal">{ "" }</h5>

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

            <div id = "board">
                // Board goes here
            </div><br/>
        </div>
    }
}

#[function_component(UserPlayHuman)]
fn user_play_human() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Enter players name"}</h5><br />
                <div class="flex-container">
                    <input id="player-name1" placeholder="Player 1's name" style="margin-left: 0px"/><br />
                    <input id="player-name2" placeholder="Player 2's name"/><br />
                    <button class="button">{ "Start game" }</button><br />
                </div>
            </div>
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

#[function_component(UserPlayComputerTOOT)]
fn user_play_computer_TOOT() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Enter your name"}</h5><br />
                <div class="flex-container">
                    <input id="player-name" placeholder="Your name" style="margin-left: 0px"/><br />
                    <button class="button">{ "Start game" }</button><br />
                </div>
            </div>
        </div>
    }
}

#[function_component(UserPlayHumanTOOT)]
fn user_play_human_TOOT() -> Html {
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Enter players name"}</h5><br />
                <div class="flex-container">
                    <input id="player-name1" placeholder="Player 1's name" style="margin-left: 0px"/><br />
                    <input id="player-name2" placeholder="Player 2's name"/><br />
                    <button class="button">{ "Start game" }</button><br />
                </div>
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

#[function_component(UserSettings)]
fn user_settings() -> Html {
    let protanopia_onclick = Callback::from(move |_event: MouseEvent| {});

    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Color bind"}</h5><br />
                <div class="flex-container">
                    <button class="button" style="margin-left: 0px">{ "Normal" }</button><br />
                    <button class="button" onclick={protanopia_onclick}>{ "Protanopia" }</button><br />
                    <button class="button">{ "Deuteranopia" }</button><br />
                    <button class="button">{ "Tritanopia" }</button><br />
                </div>
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
        UserRoute::UserHome => html! {
            <div>
                <UserHome />
                <SideBar />
            </div>
        },

        UserRoute::UserLogin => html! {
            <UserLogin />
        },

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

        UserRoute::UserPlayComputerTOOT => html! {
        <div>
            <UserPlayComputerTOOT />
            <SideBar />
        </div>},

        UserRoute::UserPlayHumanTOOT => html! {
        <div>
            <UserPlayHumanTOOT />
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

        UserRoute::UserSettings => html! {
        <div>
            <UserSettings />
            <SideBar />
        </div>},
    }
}
