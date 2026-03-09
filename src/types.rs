use jiff::Timestamp;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as De, Serialize as Ser};
use uuid::Uuid;

use crate::rkyv::*;

// Note about timestamps: they are serialized as milliseconds because that's the resolution we get
// from the wasm-bindgen API.

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Ser,
    De,
    Archive,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum BoardStatus {
    #[default]
    Thinking,
    Grouping,
    Voting,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Ser, De, Archive, Deserialize, Serialize)]
pub struct Board {
    #[rkyv(with = UuidRkyv)]
    pub id: Uuid,
    #[rkyv(with = OptionUuidRkyv)]
    pub team_id: Option<Uuid>,
    #[rkyv(with = OptionUuidRkyv)]
    pub admin_id: Option<Uuid>,
    pub name: String,
    pub status: BoardStatus,
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
    #[rkyv(with = TimestampRkyv)]
    pub created_at: Timestamp,
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::optional")]
    #[rkyv(with = OptionTimestampRkyv)]
    pub updated_at: Option<Timestamp>,
}
