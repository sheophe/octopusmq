use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::lamt::{CompressionMode, HashAlgorithm};
use crate::lamt::compression::*;
use crate::lamt::encryption::*;
use crate::protocol::util::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Payload {
    current_part: u8,
    total_parts: u8,
    hash: u64,
    length: u32,
    data: Vec<u8>,
    compressed: bool,
    encrypted: bool,
}

impl Payload {
    pub fn new() -> Self {
        Self {
            current_part: 1,
            total_parts: 1,
            hash: 0,
            length: 0,
            data: Vec::new(),
            compressed: false,
            encrypted: false
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(self.current_part);
        vec.push(self.total_parts);
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        vec.append(&mut Vec::from(u64_as_slice(hasher.finish())));
        vec.append(&mut Vec::from(u32_as_slice(self.length)));
        vec.append(&mut self.data.clone());
        vec
    }

    pub fn append<'a>(&'a mut self, other: &mut Vec<u8>) -> &'a mut Self {
        self.data.append(other);
        self.update_hash_and_length();
        self
    }

    pub fn into_compressed<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        let compression_result = compress(&self.data, compression_mode);
        self.data = match compression_result {
            Ok(v) => v,
            Err(_) => self.data.clone()
        };
        self.update_hash_and_length();
        self.compressed = true;
        self
    }

    pub fn into_decompressed<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        let decompression_result = decompress(&self.data, compression_mode);
        self.data = match decompression_result {
            Ok(v) => v,
            Err(_) => self.data.clone()
        };
        self.update_hash_and_length();
        self.compressed = false;
        self
    }

    pub fn compressed(&self) -> bool {
        self.compressed
    }

    pub fn encrypted(&self) -> bool {
        self.encrypted
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn set_compressed<'a>(&'a mut self, compressed: bool) -> &'a mut Self {
        self.compressed = compressed;
        self
    }

    pub fn set_encrypted<'a>(&'a mut self, encrypted: bool)  -> &'a mut Self {
        self.encrypted = encrypted;
        self
    }

    fn update_hash_and_length(&mut self) {
        self.length = self.data.len() as u32;
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        self.hash = hasher.finish();
    }
}

impl From<&Vec<u8>> for Payload {
    fn from(orig: &Vec<u8>) -> Self {
        Self {
            current_part: orig[0],
            total_parts: orig[1],
            hash: slice_as_u64(&orig[2..10]),
            length: slice_as_u32(&orig[10..14]),
            data: Vec::from(&orig[14..]),
            compressed: false,
            encrypted: false
        }
    }
}
