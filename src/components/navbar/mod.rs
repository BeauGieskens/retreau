use leptos::prelude::*;
use leptos_router::components::A;
use stylance::{classes, import_crate_style, import_style};

import_crate_style!(
    #[allow(dead_code)]
    shared,
    "src/style.module.scss"
);
import_style!(style, "style.module.scss");

#[component]
pub fn Navbar() -> impl IntoView {
    // Starter navbar:
    // - Brand logo
    // - Navigation links
    // - Spacer
    // - Placeholder for page-specific controls
    // - User avatar with dropdown menu
    view! {
        <nav class=style::navbar>
            <div class=style::left>
                <A href="/home" {..} class=classes!(style::brand, style::link)>
                    Retreau
                </A>
                <A href="/boards" {..} class=style::link>
                    Boards
                </A>
                <A href="/about" {..} class=style::link>
                    About
                </A>
            </div>
            <div class=style::right>
                <button class=style::avatar></button>
            </div>
        </nav>
    }
}
