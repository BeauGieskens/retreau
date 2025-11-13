use leptos::{prelude::*, Params};
use leptos_router::{hooks::use_params, params::Params};
use stylance::import_style;

use crate::components::{thought::ThoughtCategory, AddThought, Thought};

import_style!(style, "style.module.scss");

#[derive(PartialEq, Params)]
pub struct BoardParams {
    id: Option<String>,
}

#[component]
pub fn BoardPage() -> impl IntoView {
    let params = use_params::<BoardParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default() // TODO: handle invalid ID
    };

    // TODO: if id is new, do like what it says in the todo file

    view! {
        <h2 class=style::header>"Sprint 123"</h2>
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
                <AddThought id=id() category=ThoughtCategory::WentWell />
            </section>

            <section class=style::column>
                <h2 class=style::title>To Improve</h2>
                <AddThought id=id() category=ThoughtCategory::ToImprove />
            </section>

            <section class=style::column>
                <h2 class=style::title>Actions</h2>
                <AddThought id=id() category=ThoughtCategory::Action />
            </section>
        </div>
    }
}
