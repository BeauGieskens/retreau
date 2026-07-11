//! Structs that tell rkyv how to serialize and deserialize foreign types.

use jiff::Timestamp;
use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Uuid, archived = ArchivedUuid)]
pub struct UuidRkyv {
    #[rkyv(getter = Uuid::as_bytes)]
    bytes: [u8; 16],
}

impl From<UuidRkyv> for Uuid {
    fn from(archived: UuidRkyv) -> Self {
        Uuid::from_bytes(archived.bytes)
    }
}

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Option<Uuid>, archived = ArchivedOptionUuid)]
pub struct OptionUuidRkyv {
    #[rkyv(getter = opt_uuid)]
    bytes: Option<[u8; 16]>,
}

impl From<OptionUuidRkyv> for Option<Uuid> {
    fn from(archived: OptionUuidRkyv) -> Self {
        archived.bytes.map(Uuid::from_bytes)
    }
}

fn opt_uuid(opt: &Option<Uuid>) -> Option<[u8; 16]> {
    opt.as_ref().map(|uuid| *uuid.as_bytes())
}

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Timestamp, archived = ArchivedTimestamp)]
pub struct TimestampRkyv {
    #[rkyv(getter = as_millisecond)]
    unix_timestamp_millis: i64,
}

impl From<TimestampRkyv> for Timestamp {
    fn from(archived: TimestampRkyv) -> Self {
        Timestamp::from_millisecond(archived.unix_timestamp_millis).unwrap_or_else(|_| {
            if archived.unix_timestamp_millis.is_negative() {
                Timestamp::MIN
            } else {
                Timestamp::MAX
            }
        })
    }
}

fn as_millisecond(ts: &Timestamp) -> i64 {
    ts.as_millisecond()
}

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Option<Timestamp>, archived = ArchivedOptionTimestamp)]
pub struct OptionTimestampRkyv {
    #[rkyv(getter = opt_as_millisecond)]
    unix_timestamp_millis: Option<i64>,
}

impl From<OptionTimestampRkyv> for Option<Timestamp> {
    fn from(archived: OptionTimestampRkyv) -> Self {
        archived
            .unix_timestamp_millis
            .and_then(|millis| Timestamp::from_millisecond(millis).ok())
    }
}

fn opt_as_millisecond(opt: &Option<Timestamp>) -> Option<i64> {
    opt.as_ref().map(|ts| ts.as_millisecond())
}
