use leptos::prelude::*;
use leptos_router::components::A;
use stylance::{classes, import_style};

import_style!(style, "style.module.scss");

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class=style::navbar>
            <div class=style::left>
                <A href="/" {..} class=classes!(style::brand, style::link)>
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
