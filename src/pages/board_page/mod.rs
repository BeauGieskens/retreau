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
pub struct BoardParams {
    id: Option<Uuid>,
}

#[component]
pub fn BoardPage(state: Store<AppState>) -> impl IntoView {
    let params = use_params::<BoardParams>();
    let navigate = use_navigate();

    let board_resource = Resource::new_rkyv(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        |maybe_id| async move {
            match maybe_id {
                Some(id) => server::get_board(id).await,
                None => server::create_board().await,
            }
        },
    );

    Effect::new(move |_| {
        if params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .is_some()
        {
            return;
        }
        if let Some(Ok(board)) = board_resource.get() {
            navigate(
                &format!("/boards/{}", board.id),
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

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
                                let board = ArcStore::new(board);
                                state.board().set(Some(board.clone()));
                                let board = Store::from(board);
                                view! { <BoardPageInner board /> }
                            })
                    })
            }}
        </Suspense>
    }
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
