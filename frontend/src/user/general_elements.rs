use super::settings::BACKEND_URI;
use gloo::{console::log, utils::document};
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlHeadingElement, HtmlInputElement, MouseEvent};
use yew::{function_component, html, Callback, Html};
use yew_router::{navigator, prelude::*};

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
        <div>
            <style>
                {"body,h1,h2,h3,h4,h5 {font-family: 'Poppins', sans-serif}"}
            </style>
            <div class="sidenav" id="sidenav">
                <h1 >{"Play Connect4 / TOOT-OTTO"}</h1><br />

                <a href="/user/register">{"Register"}</a><br /><br />

                <a href="/user/guide">{"How to Play Connect4"}</a>
                <a href="/user/guide-TOOT">{"How to Play TOOT-OTTO"}</a><br /><br />

                <a href="/user/play-computer">{"Play With Computer"}</a>
                <a href="/user/play-human">{"Play With Another Player"}</a><br /><br />

                <a href="/user/history">{"View Game History"}</a>
                <a href="/user/scoreboard">{"Score Board"}</a><br /><br />
            </div>
        </div>
    }
}
