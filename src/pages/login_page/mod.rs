use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// Offers configured sign-in methods.
#[component]
pub fn LoginPage() -> impl IntoView {
    let query = use_query_map();
    let feedback = move || {
        let message = match query.get().get("status").as_deref() {
            Some("complete") => Some("Your profile was saved. App sessions aren't enabled yet."),
            Some("cancelled") => Some("Sign-in was cancelled. You can try again."),
            Some("failed") => Some("Sign-in failed. Please try again."),
            _ => None,
        };
        message.map(|message| view! { <p>{message}</p> })
    };

    view! {
        <section>
            <h1>Log in</h1>
            {feedback}
            <a href="/auth/entra/start" rel="external">
                "Continue with Microsoft Entra"
            </a>
        </section>
    }
}
