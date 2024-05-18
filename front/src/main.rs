use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_logger::tracing::info;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Logger init failed!");
    launch(App);
}

#[component]
fn App() -> Element {
    info!("Rendering App component!");
    rsx! {
        div {
            "Hello, Dioxus!"
        }
    }
}
