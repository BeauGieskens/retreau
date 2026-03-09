#[cfg(feature = "ssr")]
use jiff::Timestamp;
use leptos::prelude::*;
use server_fn::codec::Rkyv;
use uuid::Uuid;

use crate::types::Board;
#[cfg(feature = "ssr")]
use crate::util::timestamp_millis_number;

/// Create a new retro board and return its ID.
#[server(output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn create_board() -> Result<Board, ServerFnError> {
    let env = crate::get_worker_env().await.inspect_err(|error| {
        worker::console_error!("{error}");
    })?;

    let id = Uuid::now_v7();
    let board = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .prepare(
            "\
            INSERT INTO board (id, created_at) VALUES (?1, ?2) \
            RETURNING id, team_id, admin_id, name, status, created_at, updated_at \
            ",
        )
        .bind(&[
            id.as_simple().to_string().into(),
            timestamp_millis_number(Timestamp::now()).into(),
        ])?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Board not created"))?;

    Ok(board)
}

/// Get a retro board by its ID.
#[server(output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn get_board(id: Uuid) -> Result<Board, ServerFnError> {
    let env = crate::get_worker_env().await?;

    let board = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .prepare(
            "
            SELECT id, team_id, admin_id, name, status, created_at, updated_at \
            FROM board WHERE id = ?1 \
            ",
        )
        .bind(&[id.as_simple().to_string().into()])
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .first(None)
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .ok_or_else(|| ServerFnError::new("Board not found"))?;

    Ok(board)
}
