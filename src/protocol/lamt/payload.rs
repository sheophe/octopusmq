use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{Read, Write};

use flate2::Compression as Flate2Compression;
use bzip2::Compression as Bzip2Compression;
use flate2::write::{GzEncoder, ZlibEncoder, DeflateEncoder};
use flate2::read::{GzDecoder, ZlibDecoder, DeflateDecoder};
use bzip2::read::{BzEncoder, BzDecoder};

use crate::lamt::util;
use crate::lamt::{CompressionMode, CompressionAlgorithm};

#[derive(Clone)]
pub struct Payload {
    current_part: u8,
    total_parts: u8,
    hash: u32,
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
        vec.append(&mut Vec::from(util::u32_as_slice(hasher.finish() as u32)));
        vec.append(&mut Vec::from(util::u32_as_slice(self.length)));
        vec.append(&mut self.data.clone());
        vec
    }

    pub fn append<'a>(&'a mut self, other: &mut Vec<u8>) -> &'a mut Self {
        self.data.append(other);
        self.update_hash_and_length();
        self
    }

    pub fn into_compressed<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        let compression_result = Self::compress(&self.data, compression_mode);
        self.data = match compression_result {
            Ok(v) => v,
            Err(_) => self.data.clone()
        };
        self.update_hash_and_length();
        self.compressed = true;
        self
    }

    pub fn into_decompressed<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        let decompression_result = Self::decompress(&self.data, compression_mode);
        self.data = match decompression_result {
            Ok(v) => v,
            Err(_) => self.data.clone()
        };
        self.update_hash_and_length();
        self.compressed = false;
        self
    }

    pub fn is_compressed(&self) -> &bool {
        &self.compressed
    }

    pub fn is_encrypted(&self) -> &bool {
        &self.encrypted
    }

    fn update_hash_and_length(&mut self) {
        self.length = self.data.len() as u32;
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        self.hash = hasher.finish() as u32;
    }

    fn compress(vec: &Vec<u8>, compression_mode: CompressionMode) -> Result<Vec<u8>, io::Error> {
        let level = compression_mode.get_level();
        match compression_mode.get_algorithm() {
            CompressionAlgorithm::Deflate => Self::compress_deflate(vec, level),
            CompressionAlgorithm::Gzip => Self::compress_gzip(vec, level),
            CompressionAlgorithm::Bzip2 => Self::compress_bzip2(vec, level),
            CompressionAlgorithm::Zlib => Self::compress_zlib(vec, level),
            CompressionAlgorithm::Zstd => Self::compress_zstd(vec, level),
            CompressionAlgorithm::Brotli => Self::compress_brotli(vec, level),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn decompress(vec: &Vec<u8>, compression_mode: CompressionMode) -> Result<Vec<u8>, io::Error> {
        match compression_mode.get_algorithm() {
            CompressionAlgorithm::Deflate => Self::decompress_deflate(vec),
            CompressionAlgorithm::Gzip => Self::decompress_gzip(vec),
            CompressionAlgorithm::Bzip2 => Self::decompress_bzip2(vec),
            CompressionAlgorithm::Zlib => Self::decompress_zlib(vec),
            CompressionAlgorithm::Zstd => Self::decompress_zstd(vec),
            CompressionAlgorithm::Brotli => Self::decompress_brotli(vec),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn compress_deflate(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        let mut e = DeflateEncoder::new(Vec::new(), Flate2Compression::new(level as u32));
        match e.write_all(vec) {
            Ok(_) => e.finish(),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn compress_gzip(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        let mut e = GzEncoder::new(Vec::new(), Flate2Compression::new(level as u32));
        match e.write_all(vec) {
            Ok(_) => e.finish(),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }


    fn compress_zlib(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        let mut e = ZlibEncoder::new(Vec::new(), Flate2Compression::new(level as u32));
        match e.write_all(vec) {
            Ok(_) => e.finish(),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn compress_zstd(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        zstd::block::compress(vec, level as i32)
    }

    fn compress_bzip2(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        let mut e = BzEncoder::new(&vec[..], Bzip2Compression::new(level as u32));
        let mut encoded_vec: Vec<u8> = Vec::new();
        match e.read_to_end(&mut encoded_vec) {
            Ok(_) => Ok(encoded_vec),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn compress_brotli(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, io::Error> {
        let mut out: Vec<u8> = Vec::new();
        let mut params = brotli::enc::BrotliEncoderParams::default();
        params.quality = level as i32;
        match brotli::BrotliCompress(&mut vec.clone().as_slice(), &mut out, &params) {
            Ok(_) => Ok(out),
            Err(e) => Err(e)
        }
    }

    fn decompress_deflate(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        let mut e = DeflateDecoder::new(&vec[..]);
        let mut decoded_vec: Vec<u8> = Vec::new();
        match e.read_to_end(&mut decoded_vec) {
            Ok(_) => Ok(decoded_vec),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn decompress_gzip(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        let mut e = GzDecoder::new(&vec[..]);
        let mut decoded_vec: Vec<u8> = Vec::new();
        match e.read_to_end(&mut decoded_vec) {
            Ok(_) => Ok(decoded_vec),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn decompress_bzip2(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        let mut e = BzDecoder::new(&vec[..]);
        let mut decoded_vec: Vec<u8> = Vec::new();
        match e.read_to_end(&mut decoded_vec) {
            Ok(_) => Ok(decoded_vec),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn decompress_zlib(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        let mut e = ZlibDecoder::new(&vec[..]);
        let mut decoded_vec: Vec<u8> = Vec::new();
        match e.read_to_end(&mut decoded_vec) {
            Ok(_) => Ok(decoded_vec),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn decompress_zstd(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        zstd::block::decompress(vec,  std::i32::MAX as usize)
    }

    fn decompress_brotli(vec: &Vec<u8>) -> Result<Vec<u8>, io::Error> {
        let mut out: Vec<u8> = Vec::new();
        match brotli::BrotliDecompress(&mut vec.clone().as_slice(), &mut out) {
            Ok(_) => Ok(out),
            Err(e) => Err(e)
        }
    }
}

impl From<&Vec<u8>> for Payload {
    fn from(orig: &Vec<u8>) -> Self {
        Self {
            current_part: orig[0],
            total_parts: orig[1],
            hash: util::slice_as_u32(&orig[2..6]),
            length: util::slice_as_u32(&orig[6..10]),
            data: Vec::from(&orig[10..]),
            compressed: false,
            encrypted: false
        }
    }
}
