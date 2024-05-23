use crate::components::{FilmCard, FilmModal};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use uuid::Uuid;
use shared::models::FilmModel;
use crate::models::FilmModalVisibility;

use super::{Footer, Header};

const API_ENDPOINT: &str = "api/v1";

fn films_endpoint() -> String {
    let window = web_sys::window().expect("No global 'window' exists");
    let location = window.location();
    let host = location.host().expect("Host should be present");
    let protocol = location.protocol().expect("Should have a protocol");
    let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
    format!("{}/films", endpoint)
}

async fn get_films() -> Vec<FilmModel> {
    let endpoint = films_endpoint();
    info!("Fetching films from {}", endpoint);
    reqwest::get(&endpoint)
        .await
        .expect(format!("Fetch from {} failed", endpoint).as_str())
        .json::<Vec<FilmModel>>()
        .await
        .expect("Conversion to JSON from fetch failed")
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(FilmModalVisibility(false)));
    let mut modal_visibility = use_context::<Signal<FilmModalVisibility>>();
    let films = use_signal::<Option<Vec<FilmModel>>>(|| None);
    let mut selected_film = use_signal::<Option<FilmModel>>(|| None);
    let force_get_films = use_signal(|| ());
    {
        let force_get_films = force_get_films.clone();
        let mut films = films.clone();
        use_effect(move || {
            force_get_films.read();
            let closure = move || async move {
                let current_films = get_films().await;
                if current_films.is_empty() {
                    films.set(None);
                } else {
                    films.set(Some(current_films));
                }
            };
            spawn(closure());
        });
    }
    let delete_film = move |filmId: Uuid| {
        let mut force_get_films = force_get_films.clone();
        spawn({
            async move {
                let response = reqwest::Client::new()
                    .delete(&format!("{}/{}", &films_endpoint(), filmId))
                    .send()
                    .await;
                match response {
                    Ok(data) => {
                        info!("Film deleted");
                        force_get_films.set(());
                    }
                    Err(err) => {
                        info!("Error detecting film: {:?}", err);
                    }
                }
            }
        });
    };

    let create_or_update_film = move |film: FilmModel| {
        let mut force_get_films = force_get_films.clone();
        let mut current_selected_film = selected_film.clone();
        let is_modal_visible = modal_visibility.clone();
        spawn({
           async move {
               let response = if current_selected_film.read().is_some() {
                   reqwest::Client::new()
                       .put(&films_endpoint())
                       .json(&film)
                       .send()
                       .await
               } else {
                   reqwest::Client::new()
                       .post(&films_endpoint())
                       .json(&film)
                       .send()
                       .await
               };
               match response {
                   Ok(_) => {
                       info!("Film created");
                       current_selected_film.set(None);
                       modal_visibility.write().0 = false;
                       force_get_films.set(());
                   }
                   Err(err) => {
                       info!("Error creating film: {:?}", err);
                   }
               }
           }
        });
    };

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
                                        on_delete: move |_| {
                                            delete_film(signal.read().id);
                                        }
                                    }
                                }
                            })}
                        }
                 }
            }
            Footer {}
            FilmModal {
                film: selected_film(),
                on_create_or_update: move |new_film| {
                    create_or_update_film(new_film);
                },
                on_cancel: move |_| {
                    selected_film.set(None);
                    modal_visibility.write().0 = false;
                }
           }
        }
    }
}
