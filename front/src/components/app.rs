use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use super::{Footer, Header};

#[component]
pub fn App() -> Element {
    info!("Rendering App component!");
    rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:containier md:mx-auto md:py-8 flex-1",
            }
            Footer {}
        }
    }
}
