mod protocol_version;
mod transport_mode;
mod message_type;
mod delivery_mode;
mod message_flags;
mod compression_mode;
mod encryption_algorithm;
mod client_id;
mod topic;
mod header;
mod payload;
mod util;

pub use protocol_version::*;
pub use transport_mode::*;
pub use message_type::*;
pub use delivery_mode::*;
pub use message_flags::*;
pub use compression_mode::*;
pub use encryption_algorithm::*;
pub use client_id::*;
pub use topic::*;
pub use header::*;
pub use payload::*;
pub use util::*;

pub struct Message {
    header: Header,
    payload: Option<Payload>
}

impl Message {
    pub fn new(header: Header, payload: Option<Payload>) -> Self {
        Self {
            header: header,
            payload: payload
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.header.raw());
        if self.header.get_message_flags().get_payload() {
            vec.append(&mut self.payload.as_ref().unwrap().raw());
        }
        vec
    }
}

impl From<&Vec<u8>> for Message {
    fn from(orig: &Vec<u8>) -> Self {
        let header = Header::from(orig);
        let payload = if header.get_message_flags().get_payload() {
            Some(Payload::from(&Vec::from(&orig[header.get_offset()..])))
        } else {
            None
        };
        Self {
            header: header,
            payload: payload
        }
    }
}