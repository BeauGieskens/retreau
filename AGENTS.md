# Retreau Agent Instructions

Tech stack: Leptos, Cloudflare workers (via WASM). Main libraries include `reactive_stores`, `leptos-use`, and `stylance`.

Format files with `leptosfmt **/*.rs` after editing, then follow that up with `cargo +nightly fmt --config=imports_granularity=Crate,group_imports=StdExternalCrate`.

## Style

Use `<Component state />` shorthand instead of doing `<Component state=state />`, where possible.

## State

Use `reactive_stores` to manage state in most cases. Prefer explicit props over implicit context when accessing higher-level state from lower-level components.

## Tests

NEVER write unit tests. This project is just for fun.

## Architecture

TBD
