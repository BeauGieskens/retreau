use leptos::prelude::*;
use stylance::{import_crate_style, import_style};

import_crate_style!(
    #[allow(dead_code)]
    shared,
    "src/style.module.scss"
);
import_style!(style, "style.module.scss");

#[component]
pub fn UserCard(#[prop(into)] name: String, #[prop(into)] role: String) -> impl IntoView {
    view! {
        <div class=style::user_card>
            <div class=style::avatar></div>
            <div class=style::details>
                <span class=style::name>{name}</span>
                <span class=style::role>{role}</span>
            </div>
        </div>
    }
}
