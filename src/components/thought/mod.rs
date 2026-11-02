pub mod server;

use leptos::prelude::*;
use leptos_use::use_interval_fn;
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
    let draft = RwSignal::new(thought.content().get_untracked());

    let save = Action::new(move |content: &String| {
        let content = content.clone();
        let id = thought.id().get_untracked();
        async move {
            server::set_thought_content(id, content.clone()).await?;
            thought.content().set(content);
            Ok::<_, ServerFnError>(())
        }
    });
    let save_changes = move || {
        if !blurred
            && !save.pending().get_untracked()
            && draft.get_untracked() != thought.content().get_untracked()
        {
            save.dispatch(draft.get_untracked());
        }
    };
    // Serialize writes; edits made during a request remain dirty for the next tick.
    let _ = use_interval_fn(save_changes, 1_000);

    view! {
        <div class=style::thought_card>
            <Show
                when=move || !blurred
                fallback=move || {
                    view! {
                        <div class=classes!(
                            style::thought, style::blurred
                        )>{move || thought.content().get()}</div>
                    }
                }
            >
                <div class=style::editor data-content=move || format!("{} ", draft.get())>
                    <textarea
                        class=style::thought
                        placeholder="Write a thought…"
                        rows="1"
                        prop:value=move || draft.get()
                        on:input=move |ev| draft.set(event_target_value(&ev))
                    />

                </div>
                <div class=style::feedback>
                    {move || {
                        save.value()
                            .get()
                            .and_then(Result::err)
                            .map(|error| {
                                view! { {format!("Autosave failed: {error}. Retrying…")} }
                            })
                    }}
                </div>
            </Show>
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
