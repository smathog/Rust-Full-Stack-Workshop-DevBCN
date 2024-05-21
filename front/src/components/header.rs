use dioxus::prelude::*;
use shared::models::FilmModel;
use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

#[component]
pub fn Header() -> Element {
    let mut modal_visibility = use_context::<Signal<FilmModalVisibility>>();
    let films = use_signal::<Option<Vec<FilmModel>>>(|| None);
    let selected_film = use_signal::<Option<FilmModel>>(|| None);
    let force_get_films = use_signal(|| ());
    rsx! {
        header {
            class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
            div {
                class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
                a {
                    class: "flex title-font font-medium items-center text-teal-950 mb-4",
                    img {
                        class: "bg-transparent p-2 animate-jump",
                        alt: "ferris",
                        src: "img3.png", "loading": "lazy"
                    }
                    span {
                        class: "ml-3 text-2xl", "Rusty films"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        on_click: move |_| {
                            modal_visibility.write().0 = true;
                        },
                        "Add new film"
                    }
                }
            }
        }
    }
}
