use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::Navbar,
    pages::{BoardPage, HomePage},
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

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/retreau.css" />
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/boards/new") view=BoardPage />
                    <Route path=path!("/boards/:id") view=BoardPage />
                    <Route path=path!("/boards") view=HomePage />
                    <Route path=path!("/about") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
