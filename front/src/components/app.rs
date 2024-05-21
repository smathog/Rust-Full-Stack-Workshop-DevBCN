use crate::components::film_modal::FilmModal;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::models::FilmModalVisibility;

use super::{Footer, Header};

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(FilmModalVisibility(false)));
    let mut modal_visibility = use_context::<Signal<FilmModalVisibility>>();
    info!("Rendering App component!");
    rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
            Footer {}
            FilmModal {
                on_create_or_update: move |_| {},
                on_cancel: move |_| {
                    modal_visibility.write().0 = false;
                }
           }
        }
    }
}
