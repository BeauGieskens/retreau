use leptos::prelude::*;
use reactive_stores::Field;
use stylance::{classes, import_style};

use crate::types::{Thought as ThoughtData, ThoughtStoreFields};

import_style!(style, "style.module.scss");

#[component]
pub fn Thought(
    #[prop(into)] thought: Field<ThoughtData>,
    /// Whether to blur the contents of the thought.
    #[prop(optional)]
    blurred: bool,
) -> impl IntoView {
    view! {
        <div class=style::thought_card>
            <div class=classes!(
                style::thought, blurred.then_some(style::blurred)
            )>{move || thought.content().get()}</div>
            <div class=style::emoji>{move || thought.emoji().get()}</div>
            <div class=style::meta>
                {move || {
                    thought
                        .author_id()
                        .get()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "Deleted User".to_string())
                }}
            </div>
        </div>
    }
}
