use std::fmt;
use std::fmt::{Display, Formatter};
use jiff::Timestamp;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as De, Serialize as Ser};
use uuid::Uuid;

use crate::rkyv::*;

// Note about timestamps: they are serialized as milliseconds because that's the resolution we get
// from the wasm-bindgen API.

/// Status of a board, which progresses through Thinking → Grouping → Voting → Completed.
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

impl Display for BoardStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thinking => write!(f, "thinking"),
            Self::Grouping => write!(f, "grouping"),
            Self::Voting => write!(f, "voting"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

/// A retro board.
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

/// Category a thought belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ser, De, Archive, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThoughtCategory {
    WentWell,
    ToImprove,
    Action,
}

impl Display for ThoughtCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::WentWell => write!(f, "went-well"),
            Self::ToImprove => write!(f, "to-improve"),
            Self::Action => write!(f, "action"),
        }
    }
}

/// A thought that someone has added to a retro board.
#[derive(Debug, Clone, PartialEq, Eq, Ser, De, Archive, Deserialize, Serialize)]
pub struct Thought {
    #[rkyv(with = UuidRkyv)]
    pub id: Uuid,
    #[rkyv(with = UuidRkyv)]
    pub board_id: Uuid,
    #[rkyv(with = UuidRkyv)]
    pub author_id: Uuid,
    pub content: String,
    pub emoji: String,
    pub category: ThoughtCategory,
    pub votes: i32,
    pub idx: usize,
    /// Other thought that this thought is connected to. Connecting thoughts together is done in a
    /// linked list style, with the first thought linking to the next one, and so on.
    #[rkyv(with = OptionUuidRkyv)]
    pub connected_thought_id: Option<Uuid>,
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
    #[rkyv(with = TimestampRkyv)]
    pub created_at: Timestamp,
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::optional")]
    #[rkyv(with = OptionTimestampRkyv)]
    pub updated_at: Option<Timestamp>,
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::optional")]
    #[rkyv(with = OptionTimestampRkyv)]
    pub deleted_at: Option<Timestamp>,
}
