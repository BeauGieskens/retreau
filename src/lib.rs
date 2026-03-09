#![recursion_limit = "256"]

pub mod app;
mod components;
#[cfg(feature = "ssr")]
mod durable_objects;
mod pages;
mod rkyv;
mod types;
mod util;

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
pub use durable_objects::Board;
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;
#[cfg(feature = "ssr")]
use worker::*;

pub use crate::app::*;

#[cfg(feature = "ssr")]
pub(crate) static DB_NAME: &str = "retreau";

#[cfg(feature = "ssr")]
pub(crate) async fn get_worker_env() -> Result<Arc<Env>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use worker::Env;

    let env: Extension<Arc<Env>> = extract().await?;
    Ok(env.0)
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    // Add all of your server functions here
    register_explicit::<components::show_data_from_api::SayHello>();
}

#[cfg(feature = "ssr")]
async fn router(env: Env) -> axum::Router {
    use axum::{Extension, Router};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    register_server_functions();

    // build our application with a route
    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env))) // <- Allow leptos server functions to access Worker stuff
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    console_error_panic_hook::set_once();

    Ok(router(env).await.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
