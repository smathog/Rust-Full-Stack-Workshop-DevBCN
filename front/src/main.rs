mod components;
mod models;

use components::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Logger init failed!");
    launch(App);
}
