use leptos::prelude::*;
use stylance::{import_crate_style, import_style};

use crate::components::{board_card::BoardCard, user_card::UserCard};

import_crate_style!(
    #[allow(dead_code)]
    shared,
    "src/style.module.scss"
);
import_style!(style, "style.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class=style::hero>
            <h1>Welcome</h1>
        </div>
        <div class=style::layout>
            <section class=style::column>
                <h2 class=style::title>Start retrospecting</h2>
                <BoardCard primary=true name="New Board" />
            </section>

            <section class=style::column>
                <h2 class=style::title>Recent</h2>
                <BoardCard name="Sprint 123" timestamp="3 days ago" />
                <BoardCard name="Sprint 122" timestamp="a month ago" />
                <BoardCard name="Sprint 121" timestamp="2 months ago" />
            </section>

            <section class=style::column>
                <h2 class=style::title>Your team</h2>
                <UserCard name="Beau Gieskens" role="Senior Software Engineer" />
                <UserCard name="Billy-Bob McGee" role="Product Manager" />
                <UserCard name="John Smith" role="Full Stack Developer" />
                <UserCard name="Jane Doe" role="UX Designer" />
                <UserCard name="Jimmy" role="DevOps Engineer" />
            </section>
        </div>
    }
}
