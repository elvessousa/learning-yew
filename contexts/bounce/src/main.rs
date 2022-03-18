pub mod app;
pub mod components;
pub mod context;
pub mod page;

use app::App;

fn main() {
    yew::start_app::<App>();
}
