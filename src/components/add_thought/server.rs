#[cfg(feature = "ssr")]
use jiff::Timestamp;
use leptos::prelude::*;
use server_fn::codec::Rkyv;
use uuid::Uuid;

use crate::types::{Thought, ThoughtCategory};
#[cfg(feature = "ssr")]
use crate::util::timestamp_millis_number;

/// Create a new thought and return it.
#[server(AddThought, output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn add_thought(
    board_id: Uuid,
    author_id: Option<Uuid>,
    category: ThoughtCategory,
) -> Result<Thought, ServerFnError> {
    let env = crate::get_worker_env().await.inspect_err(|error| {
        worker::console_error!("{error}");
    })?;

    let id = Uuid::now_v7();
    let thought = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .prepare(
            "\
            INSERT INTO thought ( \
                id, board_id, author_id, content, emoji, category, idx, created_at) \
            VALUES ($1, $2, $3, '', '🫥', $4, \
                (SELECT COALESCE(MAX(idx), -1) + 1000000 FROM thought \
                 WHERE board_id = $2 AND category = $4 AND deleted_at IS NULL), $5) \
            RETURNING id, board_id, author_id, content, emoji, category, votes, idx, \
                connected_thought_id, created_at, updated_at, deleted_at \
            ",
        )
        .bind(&[
            id.as_simple().to_string().into(),
            board_id.as_simple().to_string().into(),
            author_id
                .map(|id| id.as_simple().to_string().into())
                .unwrap_or(wasm_bindgen::JsValue::NULL),
            category.db_value().into(),
            timestamp_millis_number(Timestamp::now()).into(),
        ])
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Thought not created"))?;

    Ok(thought)
}
