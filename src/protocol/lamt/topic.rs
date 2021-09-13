use std::mem;

use crate::lamt::Header;
use crate::lamt::util;

#[derive(Clone)]
pub struct Topic {
    name: Vec<u8>,
    id: u32
}

impl Topic {
    pub fn new(name: Vec<u8>, id: u32) -> Self {
        Self {
            name: name,
            id: id
        }
    }

    pub fn new_text(name: Vec<u8>) -> Self {
        Self::new(name, 0)
    }

    pub fn new_numeric(id: u32) -> Self {
        Self::new(Vec::new(), id)
    }
    
    pub fn from(orig: &Vec<u8>, header: &mut Header) -> Self {
        if header.get_message_flags().get_text_topic() {
            return Self::named_from(orig, header.get_mut_offset())
        }
        Self::numbered_from(orig, header.get_mut_offset())
    }

    pub fn raw_id(&self) -> Vec<u8> {
        Vec::from(util::u32_as_slice(self.id))
    }

    pub fn raw_name(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from(util::u32_as_slice(self.name.len() as u32)));
        vec.append(&mut self.name.clone());
        vec
    }

    fn named_from(orig: &Vec<u8>, offset: &mut usize) -> Self {
        let length = orig[*offset] as usize;
        *offset += mem::size_of::<u8>();
        let topic = Self {
            name: Vec::from(&orig[*offset..*offset+length]),
            id: 0
        };
        *offset += length as usize;
        topic
    }

    fn numbered_from(orig: &Vec<u8>, offset: &mut usize) -> Self {
        let length = mem::size_of::<u32>();
        let id_slice = &orig[*offset..*offset+length];
        *offset += length;
        Self {
            name: Vec::new(),
            id: util::slice_as_u32(id_slice)
        }
    }
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            name: Vec::new(),
            id: 0
        }
    }
}
