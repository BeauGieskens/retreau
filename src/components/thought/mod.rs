use leptos::prelude::*;
use stylance::{classes, import_style};

import_style!(style, "style.module.scss");

pub enum ThoughtCategory {
    WentWell,
    ToImprove,
    Action,
}

#[component]
pub fn Thought(
    /// Category this thought belongs to.
    category: ThoughtCategory,
    /// The actual body/text of the thought.
    #[prop(into)]
    thought: String,
    /// Emoji which best represents the sentiment of the thought.
    #[prop(into, optional)]
    emoji: Option<String>,
    /// Author of the thought.
    #[prop(into)]
    author: String,
    /// Whether to blur the contents of the thought.
    #[prop(into, optional)]
    blurred: bool,
) -> impl IntoView {
    view! {
        <div class=style::thought_card>
            <div class=classes!(style::thought, blurred.then_some(style::blurred))>{thought}</div>
            <div class=style::emoji>
                // TODO: emoji picker if one not present and if you're the author/admin
                {if let Some(emoji) = emoji { emoji.to_string() } else { "🫥".to_string() }}
            </div>
            <div class=style::meta>{author}</div>
        </div>
    }
}
