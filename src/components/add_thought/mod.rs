pub mod server;

use crate::types::ThoughtCategory;
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "style.module.scss");

#[component]
pub fn AddThought(
    /// ID of the board.
    #[prop(into)]
    board_id: String,
    /// Category that the created thought will belong to.
    #[prop(into)]
    category: ThoughtCategory,
) -> impl IntoView {
    let add_thought = Action::new(|()| {
        async move {
            server::add_thought(board_id.clone(), author_id, category).await
        }
    });

    view! {
        <div class=style::add_container>
            <button
                class=style::add
                on:click=move |_| {
                    add_thought.dispatch(());
                }
            >
                +
            </button>
        </div>
    }
}
