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
    board_id: String,
    author_id: String,
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
            VALUES ($1, $2, $3, '', '🫥', $4, 0, $5) \
            RETURNING id, board_id, author_id, content, emoji, category, votes, idx, created_at, \
                updated_at, deleted_at \
            ",
        )
        .bind(&[
            id.as_simple().to_string().into(),
            board_id.into(),
            author_id.as_simple().to_string().into(),
            category.to_string().into(),
            timestamp_millis_number(Timestamp::now()).into(),
        ])?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Thought not created"))?;

    Ok(thought)
}
