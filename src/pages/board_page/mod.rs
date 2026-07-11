mod server;

use jiff::Timestamp;
use leptos::{Params, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use leptos_use::use_derive_signal;
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

#[component]
pub fn BoardPage() -> impl IntoView {
    let params = use_params::<BoardParams>();

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
            name: "Untitled board".to_string(),
            status: Default::default(),
            created_at: Timestamp::now(),
            updated_at: None,
        },
    };

    let board = {
      move || match board_resource.get() {
          Some(Ok(board)) => board,
          _ => fallback_board(),
      }
    };

    let (board_name, set_board_name) = create_signal(board().name.clone());

    let error = move || board_resource.get().and_then(|result| result.err());

    view! {
        {move || -> Result<_, ServerFnError> {
            if let Some(error) = error() {
                return Err(error);
            }
            Ok(
                view! {
                    <h2 class=style::header>
                        {move || {
                            let board = board();
                            format!("{}: {}", board.name, board.created_at)
                        }}
                    </h2>
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
                },
            )
        }}
    }
}
