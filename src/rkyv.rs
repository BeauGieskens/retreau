//! Structs that tell rkyv how to serialize and deserialize foreign types.

use rkyv::{Archive, Deserialize, Serialize};
use time::UtcDateTime;
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
#[rkyv(remote = UtcDateTime, archived = ArchivedUtcDateTime)]
pub struct UtcDateTimeRkyv {
    #[rkyv(getter = unix_timestamp_nanos)]
    unix_timestamp_nanos: i128,
}

impl From<UtcDateTimeRkyv> for UtcDateTime {
    fn from(archived: UtcDateTimeRkyv) -> Self {
        UtcDateTime::from_unix_timestamp_nanos(archived.unix_timestamp_nanos)
            .unwrap_or(UtcDateTime::UNIX_EPOCH)
    }
}

fn unix_timestamp_nanos(dt: &UtcDateTime) -> i128 {
    dt.unix_timestamp_nanos()
}

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Option<UtcDateTime>, archived = ArchivedOptionUtcDateTime)]
pub struct OptionUtcDateTimeRkyv {
    #[rkyv(getter = opt_unix_timestamp_nanos)]
    unix_timestamp_nanos: Option<i128>,
}

fn opt_unix_timestamp_nanos(opt: &Option<UtcDateTime>) -> Option<i128> {
    opt.as_ref().map(|dt| dt.unix_timestamp_nanos())
}

impl From<OptionUtcDateTimeRkyv> for Option<UtcDateTime> {
    fn from(archived: OptionUtcDateTimeRkyv) -> Self {
        archived
            .unix_timestamp_nanos
            .and_then(|nanos| UtcDateTime::from_unix_timestamp_nanos(nanos).ok())
    }
}
