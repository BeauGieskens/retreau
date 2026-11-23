//! Persist the local profile after OIDC identity verification.

use jiff::Timestamp;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use worker::Env;

use crate::util::timestamp_millis_number;

/// Action to perform when attempting to receive an avatar from a provider.
pub enum AvatarUpdate {
    /// Keep whatever value is currently in the database.
    Keep,
    /// Clear the avatar URL in the database.
    Clear,
    /// Replace the avatar URL in the database with the given value.
    Replace(String),
}

/// Sync the profile for a verified provider and subject.
pub async fn sync(
    env: &Env,
    provider: &str,
    subject: &str,
    name: Option<&str>,
    fallback_name: &str,
    photo: AvatarUpdate,
) -> worker::Result<()> {
    let (set_avatar, avatar) = match photo {
        AvatarUpdate::Keep => (0, JsValue::NULL),
        AvatarUpdate::Clear => (1, JsValue::NULL),
        AvatarUpdate::Replace(url) => (1, JsValue::from_str(&url)),
    };
    let name = name.map_or(JsValue::NULL, JsValue::from_str);
    let provider = JsValue::from_str(provider);
    let subject = JsValue::from_str(subject);
    let now = timestamp_millis_number(Timestamp::now());
    let user_id = Uuid::now_v7().as_simple().to_string();
    let identity_id = Uuid::now_v7().as_simple().to_string();
    let db = env.d1(crate::DB_NAME)?;

    db.batch(vec![
        db.prepare(
            "UPDATE user
             SET name = COALESCE(?, name), \
                 avatar_url = CASE WHEN ? THEN ? ELSE avatar_url END, updated_at = ? \
             WHERE id = (SELECT user_id FROM user_identity WHERE provider = ? AND subject = ?)",
        )
        .bind(&[
            name.clone(),
            set_avatar.into(),
            avatar.clone(),
            now.into(),
            provider.clone(),
            subject.clone(),
        ])?,
        db.prepare(
            "INSERT INTO user (id, name, avatar_url, created_at) \
             SELECT ?, COALESCE(?, ?), ?, ? \
             WHERE NOT EXISTS (SELECT 1 FROM user_identity WHERE provider = ? AND subject = ?)",
        )
        .bind(&[
            user_id.as_str().into(),
            name,
            fallback_name.into(),
            avatar,
            now.into(),
            provider.clone(),
            subject.clone(),
        ])?,
        db.prepare(
            "INSERT INTO user_identity (id, user_id, provider, subject) \
             SELECT ?1, ?2, ?3, ?4 \
             WHERE NOT EXISTS (SELECT 1 FROM user_identity WHERE provider = ?3 AND subject = ?4)",
        )
        .bind(&[
            identity_id.as_str().into(),
            user_id.as_str().into(),
            provider,
            subject,
        ])?,
    ])
    .await?;

    Ok(())
}
