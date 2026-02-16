# Retreau

Frontend assets are built using `cargo leptos` by compiling the crate with the `hydrate` feature. The backend module uses `workers-rs` and is built by compiling the crate using `worker-build` with the `ssr` feature. This is done automatically when using `wrangler` with the custom build command specified in `wrangler.toml`.

Frontend assets are served using Workers Assets. Any request which matches an asset path will be served directly and not invoke the Worker. Requests which do not match an asset path will invoke the Worker. This includes requests to `index.html` (which will be server-side rendered) and any server function (API) routes.

## Setup

[Cargo Leptos](https://github.com/leptos-rs/cargo-leptos) is required to build the project, along with some other tools:

```bash
cargo install --locked cargo-leptos
cargo install --locked stylance-cli
cargo install --locked worker-build
cargo install --locked wasm-opt
```

Rust nightly and the wasm32 target are also required:

```bash
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Run Locally

```bash
wrangler dev
```

## Deploy

```bash
wrangler deploy
```
