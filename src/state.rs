use reactive_stores::{ArcStore, Store};

use crate::types::Board;

/// Shared front-end application state.
#[derive(Default, Store)]
pub struct AppState {
    pub board: Option<ArcStore<Board>>,
}
