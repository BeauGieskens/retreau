use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as De, Serialize as Ser};
use time::UtcDateTime;
use uuid::Uuid;

use crate::rkyv::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Ser, De, Archive, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum BoardStatus {
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
    // #[rkyv(with = UtcDateTimeRkyv)]
    // pub created_at: UtcDateTime,
    // #[rkyv(with = OptionUtcDateTimeRkyv)]
    // pub updated_at: Option<UtcDateTime>,
}
