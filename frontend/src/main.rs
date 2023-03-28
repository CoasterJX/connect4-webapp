mod user;

use user::router::User;

fn main() {
    yew::Renderer::<User>::new().render();
}
