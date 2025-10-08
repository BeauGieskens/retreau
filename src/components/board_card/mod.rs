use leptos::prelude::*;
use stylance::{classes, import_crate_style, import_style};

import_crate_style!(
    #[allow(dead_code)]
    shared,
    "src/style.module.scss"
);
import_style!(style, "style.module.scss");

#[component]
pub fn BoardCard(
    /// Whether this card should be styled as primary.
    #[prop(optional)]
    primary: bool,
    /// Name to display in the middle of the card.
    #[prop(into)]
    name: String,
    /// Timestamp to show in the bottom right of the card.
    #[prop(into, optional)]
    timestamp: Option<String>, // TODO: make it an actual timestamp?
) -> impl IntoView {
    view! {
        <div class=classes!(style::board_card, primary.then_some(style::primary))>
            <div class=style::title>{name}</div>
            {move || {
                if let Some(timestamp) = &timestamp {
                    view! { <div class=style::meta>{timestamp.to_string()}</div> }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
