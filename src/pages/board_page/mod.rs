use leptos::{Params, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use server_fn::codec::Rkyv;
use stylance::import_style;
use uuid::Uuid;

use crate::{
    components::{AddThought, Thought, thought::ThoughtCategory},
    types::Board,
};

import_style!(style, "style.module.scss");

#[derive(PartialEq, Params)]
pub struct BoardParams {
    id: Option<Uuid>,
}

/// Create a new retro board and return its ID.
#[server(output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn create_board() -> Result<Board, ServerFnError> {
    let env = crate::get_worker_env().await?;

    let id = Uuid::now_v7();
    let board = env
        .d1(crate::DB_NAME)?
        .prepare(
            "
            INSERT INTO board (id) VALUES (?1) \
            RETURNING id, team_id, admin_id, name, status
            ",
        )
        .bind(&[id.as_simple().to_string().into()])?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Board not created"))?;

    Ok(board)
}

/// Get a retro board by its ID.
#[server(output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn get_board(id: Uuid) -> Result<Board, ServerFnError> {
    let env = crate::get_worker_env().await?;

    let board = env
        .d1(crate::DB_NAME)?
        .prepare(
            "
            SELECT id, team_id, admin_id, name, status, created_at, updated_at \
            FROM board WHERE id = ?1
            ",
        )
        .bind(&[id.as_simple().to_string().into()])?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Board not found"))?;

    Ok(board)
}

#[component]
pub fn BoardPage() -> impl IntoView {
    let params = use_params::<BoardParams>();

    let id = Resource::new_rkyv(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        |maybe_id| async move {
            match maybe_id {
                Some(id) => id.as_hyphenated().to_string(),
                None => create_board()
                    .await
                    .map(|board| board.id.as_hyphenated().to_string())
                    .unwrap_or_default(),
            }
        },
    );

    // TODO: if id is new, do like what it says in the todo file

    view! {
        <Suspense fallback=move || {
            view! { <h2 class=style::header>"Loading…"</h2> }
        }>{move || id.get().map(|id| view! { <h2 class=style::header>{}</h2> })}</Suspense>
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
