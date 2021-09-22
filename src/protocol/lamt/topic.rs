use std::convert::TryInto;
use std::mem::*;

use crate::lamt::Header;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Topic {
    name: Vec<u8>,
    id: u32,
}

impl Topic {
    pub fn new(name: Vec<u8>, id: u32) -> Self {
        Self { name, id }
    }

    pub fn new_text(name: Vec<u8>) -> Self {
        Self::new(name, 0)
    }

    pub fn new_numeric(id: u32) -> Self {
        Self::new(Vec::new(), id)
    }

    pub fn from(orig: &[u8], header: &mut Header) -> Self {
        if header.message_flags().text_topic() {
            return Self::named_from(orig, header.offset_mut());
        }
        Self::numbered_from(orig, header.offset_mut())
    }

    pub fn raw_id(&self) -> Vec<u8> {
        Vec::from(self.id.to_be_bytes())
    }

    pub fn raw_name(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from((self.name.len() as u32).to_be_bytes()));
        vec.append(&mut self.name.clone());
        vec
    }

    fn named_from(orig: &[u8], offset: &mut usize) -> Self {
        let length = orig[*offset] as usize;
        *offset += size_of::<u8>();
        let topic = Self {
            name: Vec::from(&orig[*offset..*offset + length]),
            id: 0,
        };
        *offset += length as usize;
        topic
    }

    fn numbered_from(orig: &[u8], offset: &mut usize) -> Self {
        let length = size_of::<u32>();
        let id_slice = &orig[*offset..*offset + length];
        *offset += length;
        Self {
            name: Vec::new(),
            id: u32::from_be_bytes(id_slice.try_into().unwrap()),
        }
    }
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            name: Vec::new(),
            id: 0,
        }
    }
}
