use crate::serde::Deserialize;

#[derive(Deserialize)]
pub struct Metadata {
    create_index: u32,
    modify_index: u32,
    lock_index: u32,
    key: String,
    flags: u32,
    value: Vec<u8>,
    session: String,
}
