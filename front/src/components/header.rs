use dioxus::prelude::*;
use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

#[component]
pub fn Header() -> Element {
    let mut modal_visibility = use_context::<Signal<FilmModalVisibility>>();

    rsx! {
        header {
            class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
            div {
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
