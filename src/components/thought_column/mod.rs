use leptos::prelude::*;
use reactive_stores::Store;
use stylance::import_style;

use crate::{
    components::{AddThought, Thought},
    types::{Board, BoardStoreFields, ThoughtCategory, ThoughtStoreFields},
};

import_style!(style, "style.module.scss");

#[component]
pub fn ThoughtColumn(board: Store<Board>, category: ThoughtCategory) -> impl IntoView {
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
                children=move |thought| view! { <Thought thought /> }
            />
            <AddThought board category />
        </section>
    }
}
