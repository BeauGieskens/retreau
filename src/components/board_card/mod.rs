use leptos::prelude::*;
use leptos_router::components::A;
use stylance::{classes, import_style};

import_style!(style, "style.module.scss");

#[component]
pub fn BoardCard(
    /// ID of the board.
    #[prop(into)]
    id: String,
    /// Name to display in the middle of the card.
    #[prop(into)]
    name: String,
    /// Timestamp to show in the bottom right of the card.
    #[prop(into, optional)]
    timestamp: Option<String>, // TODO: make it an actual timestamp?
) -> impl IntoView {
    let new = id == "new";
    let timestamp = if let Some(timestamp) = timestamp {
        view! { <div class=style::meta>{timestamp.to_string()}</div> }.into_any()
    } else {
        ().into_any()
    };

    view! {
        <A href="/boards/new" {..}>
            <div class=classes!(style::board_card, new.then_some(style::primary))>
                <div class=style::title>{name}</div>
                {timestamp}
            </div>
        </A>
    }
}
