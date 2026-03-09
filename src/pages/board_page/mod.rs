mod server;

use jiff::Timestamp;
use leptos::{Params, prelude::*};
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_params},
    params::Params,
};
use stylance::import_style;
use uuid::Uuid;

use crate::{
    components::{AddThought, BoardTitle, Thought, thought::ThoughtCategory},
    types::Board,
};

import_style!(style, "style.module.scss");

#[derive(PartialEq, Params)]
pub struct BoardParams {
    id: Option<Uuid>,
}

#[component]
pub fn BoardPage() -> impl IntoView {
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

    let fallback_board = move || match params.read().as_ref().ok().and_then(|params| params.id) {
        Some(id) => Board {
            id,
            team_id: None,
            admin_id: None,
            name: "Loading…".to_string(),
            status: Default::default(),
            created_at: Timestamp::now(),
            updated_at: None,
        },
        None => Board {
            id: Uuid::now_v7(),
            team_id: None,
            admin_id: None,
            name: "Untitled Board".to_string(),
            status: Default::default(),
            created_at: Timestamp::now(),
            updated_at: None,
        },
    };

    // Keep the rendered board reactive so fallback data is replaced as soon as the server responds.
    let board = Memo::new(move |_| match board_resource.get() {
        Some(Ok(board)) => board,
        _ => fallback_board(),
    });

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

    let error = move || board_resource.get().and_then(|result| result.err());

    view! {
        {move || -> Result<_, ServerFnError> {
            if let Some(error) = error() {
                return Err(error);
            }
            Ok(view! { <BoardPageInner board=board.into() /> })
        }}
    }
}

#[component]
fn BoardPageInner(board: Signal<Board>) -> impl IntoView {
    let initial_board = board.get();
    let board_name = RwSignal::new(initial_board.name);
    let status = RwSignal::new(initial_board.status);
    let team_id = RwSignal::new(initial_board.team_id);
    let admin_id = RwSignal::new(initial_board.admin_id);
    let updated_at = RwSignal::new(initial_board.updated_at);
    let created_at = RwSignal::new(initial_board.created_at);

    Effect::new(move |_| {
        let board = board.get();
        board_name.set(board.name);
        status.set(board.status);
        team_id.set(board.team_id);
        admin_id.set(board.admin_id);
        updated_at.set(board.updated_at);
        created_at.set(board.created_at);
    });

    let board_id = Signal::derive(move || board.get().id);

    view! {
        <BoardTitle id=board_id title=board_name.clone() />
        <div class=style::layout>
            <section class=style::column>
                <h2 class=style::title>Went well</h2>
                <Thought
                    category=ThoughtCategory::WentWell
                    thought="Great teamwork!"
                    author="Alice"
                    emoji="😁"
                    blurred=false
                />
                <Thought
                    category=ThoughtCategory::WentWell
                    thought="Completed all tasks on time."
                    author="Bob"
                    blurred=true
                />
                <AddThought id="abc" category=ThoughtCategory::WentWell />
            </section>

            <section class=style::column>
                <h2 class=style::title>To Improve</h2>
                <AddThought id="def" category=ThoughtCategory::ToImprove />
            </section>

            <section class=style::column>
                <h2 class=style::title>Actions</h2>
                <AddThought id="ghi" category=ThoughtCategory::Action />
            </section>
        </div>
    }
}
