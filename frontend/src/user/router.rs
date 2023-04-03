use super::general_elements::SideBar;
use super::settings::BACKEND_URI;
use futures::future::Lazy;
use gloo::{console::log, utils::document};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use std::{sync::Mutex, thread};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlDivElement, HtmlHeadingElement, HtmlInputElement, MouseEvent};
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
                log!(user["title"].to_string());
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
            log!("Error");
            let _ = document()
                .get_element_by_id("register-msg")
                .unwrap()
                .dyn_into::<HtmlHeadingElement>()
                .unwrap()
                .set_inner_html("Register failed! Do not include \"_\" in your username.");
        } else {
            let verify_user_uri = format!("{}/user/verify", BACKEND_URI);
            let get_user_uri =
                format!("{}/user/info/{}", BACKEND_URI, name_input.replace(" ", "_"));

            // wasm_bindgen_futures::spawn_local(async move {
            //     let client = reqwest_wasm::Client::new();
            //     let response = client
            //         .post(verify_user_uri)
            //         .json(&json!({
            //             "name": name_input,
            //             "location": "",
            //             "title": "",
            //             "pwd": pwd_input
            //         }))
            //         .send()
            //         .await
            //         .unwrap()
            //         .json::<serde_json::Value>()
            //         .await
            //         .unwrap();
            //     if !response["status"]["success"].as_bool().unwrap() {
            //         document()
            //             .get_element_by_id("login-err-msg")
            //             .unwrap()
            //             .dyn_into::<HtmlHeadingElement>()
            //             .unwrap()
            //             .set_inner_html("Register failed! {append error message}.");
            //         //.set_node_value(Some("Login failed! Check your user name & password."));
            //     } else {
            //         let user = reqwest_wasm::get(get_user_uri)
            //             .await
            //             .unwrap()
            //             .json::<serde_json::Value>()
            //             .await
            //             .unwrap();
            //         log!(user["title"].to_string());
            //     }
            // });
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
    // let _ = document()

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

        let singleCell = "<img src= \"https:\\/\\/i.ibb.co/H2CPYvY/fotor-2023-4-1-20-30-22.png\" alt=\"Cell\" />";
        let mut finalRow = String::from("");
        for i in 0..width.parse().unwrap() {
            finalRow += singleCell;
        }

        finalRow = "<div class=\"flex-container\">".to_owned() + finalRow.as_str() + "</div>";
        let mut finalString = String::from("");

        for j in 0..height.parse().unwrap() {
            finalString += finalRow.as_str();
        }

        let _ = document()
            .get_element_by_id("test")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .set_inner_html((&finalString).as_str());
    });

    let imagetest = Callback::from(move |_event: MouseEvent| {
        log! {"Success!"};
    });

    let showDimensionPrompt = Callback::from(move |_event: MouseEvent| {
        let username = document()
            .get_element_by_id("player-name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let userpassword = document()
            .get_element_by_id("player-password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let _ = document()
            .get_element_by_id("dimension-prompt")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .set_attribute("style", "display: ");
    });
    //<div class=\"flex-container\"><img src= \"https:\\/\\/i.ibb.co/H2CPYvY/fotor-2023-4-1-20-30-22.png\" alt=\"Cell\"/></div>
    html! {
        <div class="sidenavpadding">
            <div>
                <h5 style="padding-top: 72px">{"Enter your name"}</h5>
                <div class="flex-container">
                    <input id="player-name" placeholder="Your name" style="margin-left: 0px"/>
                    <input id="player-password" placeholder="Password" style="margin-left: 0px"/>
                    <button class="button" onclick={showDimensionPrompt}>{ "Start game" }</button>
                </div>


                <div id="dimension-prompt" style="display: none">
                    <h5>{"Enter board dimensions"}</h5>
                    <div class="flex-container">
                        <input id="board-width" placeholder="Width" style="margin-left: 0px"/>
                        <input id="board-height" placeholder="Height"/>
                        <button class="button" onclick={generateBoard}>{ "Generate" }</button>
                    </div>
                </div>
            </div><br />

            <div id="test">

            </div>
            // <div id = "board">
            //     <div class="flex-container">
            //         <img src= "https://i.ibb.co/H2CPYvY/fotor-2023-4-1-20-30-22.png" alt="Cell" onclick={imagetest}/>
            //     </div>
            // </div>
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
