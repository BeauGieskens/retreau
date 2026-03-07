//! Structs that tell rkyv how to serialize and deserialize foreign types.

use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Archive, Deserialize, Serialize)]
#[rkyv(remote = Uuid, archived = ArchivedUuid)]
pub struct UuidRkyv(#[rkyv(getter = Uuid::as_bytes)] [u8; 16]);

impl From<UuidRkyv> for Uuid {
    fn from(uuid: UuidRkyv) -> Self {
        Uuid::from_bytes(uuid.0)
    }
}
