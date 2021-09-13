use std::mem;
use std::time::SystemTime;

use uuid::v1::{Timestamp, Context};
use uuid::Uuid;

use crate::lamt::header::Header;
use crate::protocol::util;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ClientId(u128);

impl ClientId {
    pub fn new() -> Self {
        let mac = mac_address::get_mac_address();
        match mac {
            Ok(v) => {
                let context = Context::new(42);
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
                let ts = Timestamp::from_unix(&context, now.as_secs(), now.as_nanos() as u32);
                let uuid = Uuid::new_v1(ts, &v.unwrap().bytes());
                match uuid {
                    Ok(v) => Self(v.as_u128()),
                    Err(_) => Self(Uuid::new_v4().as_u128())
                }
            },
            Err(_) => Self(Uuid::new_v4().as_u128())
        }
    }

    pub fn from(orig: &Vec<u8>, header: &mut Header) -> Self {
        let length = mem::size_of::<u128>();
        let offset = header.offset_mut();
        let id_slice = &orig[*offset..*offset+length];
        let val: u128 = util::slice_as_u128(id_slice);
        *offset += length;
        Self(val)
    }

    pub fn raw(&self) -> Vec<u8> {
        Vec::from(util::u128_as_slice(self.0))
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