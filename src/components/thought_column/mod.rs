use leptos::prelude::*;
use reactive_stores::Store;
use stylance::import_style;

use crate::{
    components::{AddThought, Thought},
    types::{Board, BoardStoreFields, ThoughtCategory, ThoughtStoreFields},
};

import_style!(style, "style.module.scss");

/// Container for a set of thoughts in a single category.
#[component]
pub fn ThoughtColumn(board: Store<Board>, category: ThoughtCategory) -> impl IntoView {
    let focus_thought = RwSignal::new(None);

    view! {
        <section class=style::column>
            <h2 class=style::title>{category.column_title()}</h2>
            <For
                each=move || {
                    board
                        .thoughts()
                        .into_iter()
                        .filter(move |thought| {
                            thought.category().get() == category
                                && thought.deleted_at().get().is_none()
                        })
                        .collect::<Vec<_>>()
                }
                key=|thought| thought.id().get_untracked()
                children=move |thought| {
                    let focus_on_mount = focus_thought
                        .get_untracked()
                        .is_some_and(|id| id == thought.id().get_untracked());
                    if focus_on_mount {
                        focus_thought.set(None);
                    }
                    view! { <Thought thought focus_on_mount /> }
                }
            />
            <AddThought board category focus_thought />
        </section>
    }
}
