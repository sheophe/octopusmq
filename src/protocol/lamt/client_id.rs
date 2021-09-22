use std::mem::*;

use std::convert::TryInto;
use uuid::Uuid;

use crate::lamt::header::Header;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ClientId(u128);

impl ClientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().as_u128())
    }

    pub fn from(orig: &[u8], header: &mut Header) -> Self {
        let length = size_of::<u128>();
        let offset = header.offset_mut();
        let id_slice = &orig[*offset..*offset + length];
        let val: u128 = u128::from_be_bytes(id_slice.try_into().unwrap());
        *offset += length;
        Self(val)
    }

    pub fn raw(&self) -> Vec<u8> {
        Vec::from(self.0.to_be_bytes())
    }
}

impl From<u128> for ClientId {
    fn from(orig: u128) -> Self {
        ClientId(orig)
    }
}

impl Default for ClientId {
    fn default() -> Self {
        ClientId(0)
    }
}
