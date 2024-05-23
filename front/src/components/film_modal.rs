use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};
use dioxus::prelude::*;
use shared::models::FilmModel;
use uuid::Uuid;

#[component]
pub fn FilmModal(
    on_create_or_update: EventHandler<FilmModel>,
    on_cancel: EventHandler<MouseEvent>,
    film: Option<FilmModel>,
) -> Element {
    let modal_visibility = use_context::<Signal<FilmModalVisibility>>();
    if !modal_visibility.read().0 {
        return None;
    }
    let mut draft_film = use_signal(|| blank_film());
    {
        let mut draft_film = draft_film.clone();
        use_effect(move || {
            match &film {
                None => draft_film.set(blank_film()),
                Some(film) => draft_film.set(film.clone()),
            }
        });
    }
    rsx!(
        article {
            class: "z-50 w-full h-full fixed top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-auto bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl text-teal-950 font-semibold",
                        "🎬 Film"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Title"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film title",
                            value: "{draft_film.read().title}",
                            oninput: move |evt| {
                                draft_film.set(FilmModel {
                                    title: evt.value().clone(),
                                    ..draft_film()
                                })
                            },
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Director"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film director",
                            value: "{draft_film.read().director}",
                            oninput: move |evt| {
                                draft_film.set(FilmModel {
                                    director: evt.value().clone(),
                                    ..draft_film()
                                })
                            },
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Year"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "number",
                            placeholder: "Enter film year",
                            value: "{draft_film.read().year.to_string()}",
                            oninput: move |evt| {
                                draft_film.set(FilmModel {
                                    year: evt.value().clone().parse::<u16>().unwrap_or(1900),
                                    ..draft_film()
                                })
                            },
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Poster"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film poster URL",
                            value: "{draft_film.read().poster}",
                            oninput: move |evt| {
                                draft_film.set(FilmModel {
                                    poster: evt.value().clone(),
                                    ..draft_film()
                                })
                            },
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        on_click: move |evt| {
                            draft_film.set(blank_film());
                            on_cancel.call(evt);
                        },
                        "Cancel",
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        on_click: move |evt| {
                            on_create_or_update.call(draft_film());
                            draft_film.set(blank_film());
                        },
                        "Save film",
                    }
                }
            }

        }
    )
}

fn blank_film() -> FilmModel {
    FilmModel {
        id: Uuid::new_v4(),
        title: "".to_string(),
        director: "".to_string(),
        year: 1900,
        poster: "".to_string(),
        created_at: None,
        updated_at: None,
    }
}
