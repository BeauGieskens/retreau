use leptos::prelude::*;
use stylance::import_style;

use crate::components::thought::ThoughtCategory;

import_style!(style, "style.module.scss");

#[component]
pub fn AddThought(
    /// ID of the board.
    #[prop(into)]
    id: String,
    /// Category that the created thought will belong to.
    #[prop(into)]
    category: ThoughtCategory,
) -> impl IntoView {
    view! {
        <div class=style::add_container>
            <button class=style::add>+</button>
        </div>
    }
}
