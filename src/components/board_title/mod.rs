pub mod server;

use leptos::{ev::KeyboardEvent, html, prelude::*, web_sys::HtmlElement};
use reactive_stores::Store;
use stylance::import_style;

use crate::types::{Board, BoardStoreFields};

import_style!(style, "style.module.scss");

#[component]
pub fn BoardTitle(board: Store<Board>) -> impl IntoView {
    let title = board.name();
    let set_board_name = Action::new(move |new_name: &String| {
        let board_id = board.id().get_untracked();
        let new_name = new_name.clone();
        async move { server::set_board_name(board_id, new_name).await }
    });

    let heading_ref = NodeRef::<html::H1>::new();

    // Keep browser-managed contenteditable text aligned with reactive title updates.
    Effect::new(move |_| {
        if let Some(heading) = heading_ref.get() {
            let new_title = title.get();
            if heading.inner_text() != new_title {
                heading.set_inner_text(&new_title);
            }
        }
    });

    let save_changes = move |input: HtmlElement| {
        let value = input.inner_text();
        title.set(value.clone());
        set_board_name.dispatch(value);
    };

    view! {
        <div class=style::header>
            <h1
                node_ref=heading_ref
                contenteditable
                on:blur=move |ev| {
                    let el = event_target(&ev);
                    save_changes(el);
                }
                on:keydown=move |ev: KeyboardEvent| {
                    let el: HtmlElement = event_target(&ev);
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        el.blur().ok();
                        save_changes(el);
                    } else if ev.key() == "Escape" {
                        ev.prevent_default();
                        el.blur().ok();
                    }
                }
            >
                {move || title.get()}
            </h1>
        </div>
    }
}
