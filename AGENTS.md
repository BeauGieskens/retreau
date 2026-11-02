# Retreau Agent Instructions

Tech stack: Leptos, Cloudflare workers (via WASM). Main libraries include `reactive_stores`, `leptos-use`, and `stylance`.

Format files with `leptosfmt **/*.rs` after editing, then follow that up with `cargo +nightly fmt --config=imports_granularity=Crate,group_imports=StdExternalCrate`.

## Style

- Use `<Component state />` shorthand instead of doing `<Component state=state />`, where possible.
- Use `inspect_err` on server-side functions to log errors with `worker::console_error!("{error}");`.
- Use 4-space tabs for SCSS files.

## State

Use `reactive_stores` to manage state in most cases. Prefer explicit props over implicit context when accessing higher-level state from lower-level components.

## Tests

NEVER write unit tests. This project is just for fun.

## Architecture

- `components`: all components live here, even when they're only used for a single thing. The main exception that rule is when a component has some boilerplate to handle suspense for a loading variant.
  - Each component is a directory which may contain:
    - `mod.rs`: contains the component definition for simple components.
    - `server.rs`: server-side code to support actions used by the component.
    - `style.module.scss`: styling code for the component.
- `durable_objects`: definitions for Cloudflare Durable Objects.
- `pages`: entry point for each page in the front-end.
