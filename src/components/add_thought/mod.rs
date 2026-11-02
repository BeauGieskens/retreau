pub mod server;

use leptos::prelude::*;
use reactive_stores::Store;
use stylance::import_style;
use uuid::Uuid;

use crate::types::{Board, BoardStoreFields, ThoughtCategory};

import_style!(style, "style.module.scss");

#[component]
pub fn AddThought(
    board: Store<Board>,
    /// Creation requires an identified author; no identity is fabricated here.
    #[prop(optional)]
    author_id: Option<Uuid>,
    /// Category that the created thought will belong to.
    #[prop(into)]
    category: ThoughtCategory,
) -> impl IntoView {
    let add_thought = Action::new(move |author_id: &Uuid| {
        let board_id = board.id().get_untracked();
        let author_id = *author_id;
        async move { server::add_thought(board_id, author_id, category).await }
    });

    view! {
        <div class=style::add_container>
            <button
                class=style::add
                disabled=author_id.is_none()
                on:click=move |_| {
                    if let Some(author_id) = author_id {
                        add_thought.dispatch(author_id);
                    }
                }
            >
                +
            </button>
        </div>
    }
}
