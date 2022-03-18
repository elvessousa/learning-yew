pub mod app;
pub mod components;
pub mod context;

use app::App;

fn main() {
    yew::start_app::<App>();
}
