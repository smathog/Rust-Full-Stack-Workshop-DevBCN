use crate::components::{FilmCard, FilmModal};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::models::FilmModel;
use crate::models::FilmModalVisibility;

use super::{Footer, Header};


#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(FilmModalVisibility(false)));
    let mut modal_visibility = use_context::<Signal<FilmModalVisibility>>();
    let films = use_signal::<Option<Vec<FilmModel>>>(|| None);
    let mut selected_film = use_signal::<Option<FilmModel>>(|| None);
    let force_get_films = use_signal(|| ());
    info!("Rendering App component!");
    rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                 if let Some(films_vec) = films() {
                        ul {
                            class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                            { films_vec.into_iter().map(|film| {
                                let signal = ReadOnlySignal::new(Signal::new(film));
                                rsx! {
                                    FilmCard {
                                        key: "{signal.read().id}",
                                        film: signal,
                                        on_edit: move |_| {
                                            selected_film.set(Some(signal()));
                                            modal_visibility.write().0 = true;
                                        },
                                        on_delete: move |_| {}
                                    }
                                }
                            })}
                        }
                 }
            }
            Footer {}
            FilmModal {
                film: selected_film(),
                on_create_or_update: move |new_film| {},
                on_cancel: move |_| {
                    selected_film.set(None);
                    modal_visibility.write().0 = false;
                }
           }
        }
    }
}
