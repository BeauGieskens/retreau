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
    /// ID of the user clicking the button.
    #[prop(optional)]
    author_id: Option<Uuid>,
    /// Category that the created thought will belong to.
    category: ThoughtCategory,
) -> impl IntoView {
    let add_thought = Action::new(move |_| {
        let board_id = board.id().get_untracked();
        async move {
            let thought = server::add_thought(board_id, author_id, category).await?;
            board.thoughts().update(|thoughts| thoughts.push(thought));
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div class=style::add_container>
            <button
                class=style::add
                disabled=move || add_thought.pending().get()
                on:click=move |_| {
                    add_thought.dispatch(());
                }
            >
                +
            </button>
            {move || {
                add_thought
                    .value()
                    .get()
                    .and_then(Result::err)
                    .map(|error| {
                        view! { <p role="alert">{format!("Could not add thought: {error}")}</p> }
                    })
            }}
        </div>
    }
}
