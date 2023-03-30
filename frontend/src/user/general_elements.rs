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
        <div class="sidenav">
            <button>{"About"}</button>
            <a href="/login">{"Services"}</a>
            <a href="#">{"Clients"}</a>
            <a href="#">{"Contact"}</a>
        </div>
        </div>
    }
}
