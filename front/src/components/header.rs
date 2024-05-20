use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
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
                }
            }
        }
    }
}
