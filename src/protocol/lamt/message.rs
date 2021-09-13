use crate::lamt::{Header, Payload};

#[derive(Clone, PartialEq, Eq)]
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
        if self.header.message_flags().payload() {
            vec.append(&mut self.payload.as_ref().unwrap().raw());
        }
        vec
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn payload(&self) -> &Option<Payload> {
        &self.payload
    }
}

impl From<&Vec<u8>> for Message {
    fn from(orig: &Vec<u8>) -> Self {
        let header = Header::from(orig);
        let payload = if header.message_flags().payload() {
            Some(Payload::from(&Vec::from(&orig[header.offset()..])))
        } else {
            None
        };
        Self {
            header: header,
            payload: payload
        }
    }
}