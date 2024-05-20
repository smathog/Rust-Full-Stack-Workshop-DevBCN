mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use components::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Logger init failed!");
    launch(App);
}
