use leptos::*;

pub mod app;
pub mod components;
pub mod route;
pub mod state;
mod layout;

fn main() {
    mount_to_body(app::App)
}
