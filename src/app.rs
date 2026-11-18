use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::{
    components::Navbar,
    pages::{BoardPage, ErrorPage, HomePage},
    state::AppState,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Manrope:wght@200..800&family=Pacifico&display=swap"
                    rel="stylesheet"
                />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let state = Store::new(AppState::default());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/retreau.css" />
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route
                        path=path!("/boards/new")
                        view=move || view! { <BoardRoute state create=true /> }
                    />
                    <Route
                        path=path!("/boards/:id")
                        ssr=SsrMode::Async
                        view=move || view! { <BoardRoute state /> }
                    />
                    <Route path=path!("/boards") view=HomePage />
                    <Route path=path!("/about") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn BoardRoute(
    state: Store<AppState>,
    /// Whether to create a new board upon loading the page.
    #[prop(optional)]
    create: bool,
) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            for error in errors.get().into_iter() {
                leptos::logging::error!("{error:?}");
            }
            view! { <ErrorPage /> }
        }>
            <BoardPage state create />
        </ErrorBoundary>
    }
}
