pub mod server;

use leptos::{Params, prelude::*};
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_params},
    params::Params,
};
use reactive_stores::{ArcStore, Store};
use stylance::import_style;
use uuid::Uuid;

use crate::{
    components::{BoardTitle, ThoughtColumn},
    state::{AppState, AppStateStoreFields},
    types::{Board, ThoughtCategory},
};

import_style!(style, "style.module.scss");

#[derive(PartialEq, Params)]
struct BoardParams {
    id: Option<Uuid>,
}

#[component]
pub fn BoardPage(state: Store<AppState>, create: bool) -> impl IntoView {
    let params = use_params::<BoardParams>();
    let navigate = use_navigate();

    let board_resource = Resource::new_rkyv(
        move || {
            if create {
                None
            } else {
                params.read().as_ref().ok().and_then(|params| params.id)
            }
        },
        move |maybe_id| async move {
            if create {
                server::create_board().await.map(Some)
            } else if let Some(id) = maybe_id {
                server::get_board(id).await
            } else {
                Ok(None)
            }
        },
    );

    if create {
        Effect::new(move |_| {
            if let Some(Ok(Some(board))) = board_resource.get() {
                navigate(
                    &format!("/boards/{}", board.id),
                    NavigateOptions {
                        replace: true,
                        ..Default::default()
                    },
                );
            }
        });
    }

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading board…"</p> }
        }>
            {move || {
                board_resource
                    .get()
                    .map(|result| {
                        result
                            .map(|board| {
                                match board {
                                    Some(board) => {
                                        let board = ArcStore::new(board);
                                        state.board().set(Some(board.clone()));
                                        let board = Store::from(board);
                                        view! { <BoardPageInner board /> }.into_any()
                                    }
                                    None => {
                                        state.board().set(None);
                                        view! { <BoardNotFound /> }.into_any()
                                    }
                                }
                            })
                    })
            }}
        </Suspense>
    }
}

#[component]
fn BoardNotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
        response.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"404: Board not found"</h1> }
}

#[component]
fn BoardPageInner(board: Store<Board>) -> impl IntoView {
    view! {
        <BoardTitle board />
        <div class=style::layout>
            <ThoughtColumn board category=ThoughtCategory::WentWell />
            <ThoughtColumn board category=ThoughtCategory::ToImprove />
            <ThoughtColumn board category=ThoughtCategory::Action />
        </div>
    }
}
