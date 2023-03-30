use super::general_elements::SideBar;
use super::settings::BACKEND_URI;
use gloo::{console::log, utils::document};
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlHeadingElement, HtmlInputElement, MouseEvent};
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
}

#[function_component(UserHome)]
fn user_home() -> Html {
    let navigator = use_navigator().unwrap();
    let (login_nav, register_nav) = (navigator.clone(), navigator.clone());

    let login_onclick = Callback::from(move |_| login_nav.push(&UserRoute::UserLogin));
    let register_onclick = Callback::from(move |_| register_nav.push(&UserRoute::UserRegister));

    html! {
        <div>
            <h1>{ "Login/Register" }</h1>
            <button onclick={login_onclick}>{ "Login" }</button>
            <button onclick={register_onclick}>{ "Register" }</button>
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
            <h1>{ "Enter name & pwd aa:" }</h1>
            <input id="login-name" placeholder="Name" /><br />
            <input id="login-pwd" placeholder="Password" /><br />
            <button class="dif" onclick={login_onclick}>{ "Login" }</button><br />
            <h1 id="login-err-msg">{ "" }</h1>
        </div>
    }
}

#[function_component(UserRegister)]
fn user_register() -> Html {
    html! {
        <div />
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
        UserRoute::UserRegister => html! { <UserRegister /> },
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
