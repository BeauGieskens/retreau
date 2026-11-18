#[cfg(feature = "ssr")]
use jiff::Timestamp;
use leptos::prelude::*;
use server_fn::codec::Rkyv;
use uuid::Uuid;

use crate::types::ThoughtCategory;
#[cfg(feature = "ssr")]
use crate::util::timestamp_millis_number;

/// Update the text of an existing thought.
#[server(SetThoughtContent, output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn set_thought_content(id: Uuid, content: String) -> Result<(), ServerFnError> {
    let env = crate::get_worker_env().await.inspect_err(|error| {
        worker::console_error!("{error}");
    })?;
    let updated = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| worker::console_error!("{error}"))?
        .prepare(
            "UPDATE thought SET content = ?1, updated_at = ?2 \
             WHERE id = ?3 AND deleted_at IS NULL RETURNING id",
        )
        .bind(&[
            content.into(),
            timestamp_millis_number(Timestamp::now()).into(),
            id.as_simple().to_string().into(),
        ])
        .inspect_err(|error| worker::console_error!("{error}"))?
        .first::<String>(Some("id"))
        .await
        .inspect_err(|error| worker::console_error!("{error}"))?;
    updated.ok_or_else(|| ServerFnError::new("Thought no longer exists"))?;
    Ok(())
}

/// Save an emoji only if it is allowed for the thought's stored category.
#[server(SetThoughtEmoji, output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn set_thought_emoji(
    id: Uuid,
    category: ThoughtCategory,
    emoji: String,
) -> Result<(), ServerFnError> {
    if !category.emoji_choices().contains(&emoji.as_str()) {
        return Err(ServerFnError::new(
            "Emoji is not available for this category",
        ));
    }

    let env = crate::get_worker_env()
        .await
        .inspect_err(|error| worker::console_error!("{error}"))?;
    let updated = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| worker::console_error!("{error}"))?
        .prepare(
            "UPDATE thought SET emoji = ?1, updated_at = ?2 \
             WHERE id = ?3 AND category = ?4 AND deleted_at IS NULL RETURNING id",
        )
        .bind(&[
            emoji.into(),
            timestamp_millis_number(Timestamp::now()).into(),
            id.as_simple().to_string().into(),
            category.db_value().into(),
        ])
        .inspect_err(|error| worker::console_error!("{error}"))?
        .first::<String>(Some("id"))
        .await
        .inspect_err(|error| worker::console_error!("{error}"))?;
    updated.ok_or_else(|| ServerFnError::new("Thought no longer exists in this category"))?;
    Ok(())
}

#[server(DeleteThought, output = Rkyv)]
#[cfg_attr(feature = "ssr", worker::send)]
pub async fn delete_thought(id: Uuid) -> Result<(), ServerFnError> {
    let env = crate::get_worker_env().await.inspect_err(|error| {
        worker::console_error!("{error}");
    })?;
    let deleted = env
        .d1(crate::DB_NAME)
        .inspect_err(|error| worker::console_error!("{error}"))?
        .prepare(
            "UPDATE thought SET deleted_at = ?1, updated_at = ?1 \
             WHERE id = ?2 RETURNING id",
        )
        .bind(&[
            timestamp_millis_number(Timestamp::now()).into(),
            id.as_simple().to_string().into(),
        ])
        .inspect_err(|error| worker::console_error!("{error}"))?
        .first::<String>(Some("id"))
        .await
        .inspect_err(|error| worker::console_error!("{error}"))?;
    deleted.ok_or_else(|| ServerFnError::new("Thought no longer exists"))?;
    Ok(())
}
