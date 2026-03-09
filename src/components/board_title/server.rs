#[cfg(feature = "ssr")]
use jiff::Timestamp;
use leptos::prelude::*;
use server_fn::codec::Rkyv;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::util::timestamp_millis_number;

/// Set the name of a retro board.
#[server(output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn set_board_name(id: Uuid, new_name: String) -> Result<(), ServerFnError> {
    let env = crate::get_worker_env().await?;

    let board = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?
        .prepare(
            "\
            UPDATE board SET name = ?1, updated_at = ?2 WHERE id = ?3 \
            ",
        )
        .bind(&[
            new_name.into(),
            timestamp_millis_number(Timestamp::now()).into(),
            id.as_simple().to_string().into(),
        ])?
        .run()
        .await
        .inspect_err(|error| {
            worker::console_error!("{error}");
        })?;
    Ok(())
}
