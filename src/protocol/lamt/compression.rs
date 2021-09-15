use std::io::{Error, ErrorKind, Read, Write};

use flate2::Compression as Flate2Compression;
use flate2::write::{GzEncoder, ZlibEncoder, DeflateEncoder};
use flate2::read::{GzDecoder, ZlibDecoder, DeflateDecoder};
use bzip2::Compression as Bzip2Compression;
use bzip2::write::BzEncoder;
use bzip2::read::BzDecoder;
use zstd::Encoder as ZstdEncoder;
use zstd::Decoder as ZstdDecoder;

use crate::lamt::{CompressionMode, CompressionAlgorithm};

pub fn compress(vec: &Vec<u8>, compression_mode: CompressionMode) -> Result<Vec<u8>, Error> {
    let level = compression_mode.level();
    match compression_mode.algorithm() {
        CompressionAlgorithm::Deflate => compress_deflate(vec, level),
        CompressionAlgorithm::Gzip => compress_gzip(vec, level),
        CompressionAlgorithm::Bzip2 => compress_bzip2(vec, level),
        CompressionAlgorithm::Zlib => compress_zlib(vec, level),
        CompressionAlgorithm::Zstd => compress_zstd(vec, level),
        CompressionAlgorithm::Brotli => compress_brotli(vec, level),
        _ => Err(Error::from(ErrorKind::InvalidInput))
    }
}

pub fn decompress(vec: &Vec<u8>, compression_mode: CompressionMode) -> Result<Vec<u8>, Error> {
    match compression_mode.algorithm() {
        CompressionAlgorithm::Deflate => decompress_deflate(vec),
        CompressionAlgorithm::Gzip => decompress_gzip(vec),
        CompressionAlgorithm::Bzip2 => decompress_bzip2(vec),
        CompressionAlgorithm::Zlib => decompress_zlib(vec),
        CompressionAlgorithm::Zstd => decompress_zstd(vec),
        CompressionAlgorithm::Brotli => decompress_brotli(vec),
        _ => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn compress_deflate(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut e = DeflateEncoder::new(
        Vec::new(),
        Flate2Compression::new(level_range(level, 0, 10) as u32)
    );
    match e.write_all(vec) {
        Ok(_) => e.finish(),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn compress_gzip(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut e = GzEncoder::new(
        Vec::new(),
        Flate2Compression::new(level_range(level, 0, 10) as u32)
    );
    match e.write_all(vec) {
        Ok(_) => e.finish(),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}


fn compress_zlib(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut e = ZlibEncoder::new(
        Vec::new(),
        Flate2Compression::new(level_range(level, 0, 10) as u32)
    );
    match e.write_all(vec) {
        Ok(_) => e.finish(),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn compress_zstd(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut e = ZstdEncoder::new(
        Vec::new(),
        level_range(level, 1, 21) as i32
    ).unwrap();
    match e.write_all(vec) {
        Ok(_) => e.finish(),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn compress_bzip2(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut e = BzEncoder::new(
        Vec::new(),
        Bzip2Compression::new(level_range(level, 1, 9) as u32)
    );
    match e.write_all(vec) {
        Ok(_) => e.finish(),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn compress_brotli(vec: &Vec<u8>, level: i8) -> Result<Vec<u8>, Error> {
    let mut out: Vec<u8> = Vec::new();
    let mut params = brotli::enc::BrotliEncoderParams::default();
    params.quality = level_range(level, 0, 11) as i32;
    match brotli::BrotliCompress(&mut vec.clone().as_slice(), &mut out, &params) {
        Ok(_) => Ok(out),
        Err(e) => Err(e)
    }
}

fn decompress_deflate(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut e = DeflateDecoder::new(&vec[..]);
    let mut decoded_vec: Vec<u8> = Vec::new();
    match e.read_to_end(&mut decoded_vec) {
        Ok(_) => Ok(decoded_vec),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn decompress_gzip(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut e = GzDecoder::new(&vec[..]);
    let mut decoded_vec: Vec<u8> = Vec::new();
    match e.read_to_end(&mut decoded_vec) {
        Ok(_) => Ok(decoded_vec),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn decompress_bzip2(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut e = BzDecoder::new(&vec[..]);
    let mut decoded_vec: Vec<u8> = Vec::new();
    match e.read_to_end(&mut decoded_vec) {
        Ok(_) => Ok(decoded_vec),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn decompress_zlib(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut e = ZlibDecoder::new(&vec[..]);
    let mut decoded_vec: Vec<u8> = Vec::new();
    match e.read_to_end(&mut decoded_vec) {
        Ok(_) => Ok(decoded_vec),
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn decompress_zstd(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let decoder = ZstdDecoder::new(&vec[..]);
    let mut decoded_vec: Vec<u8> = Vec::new();
    match decoder {
        Ok(mut e) => match e.read_to_end(&mut decoded_vec) {
            Ok(_) => Ok(decoded_vec),
            Err(_) => Err(Error::from(ErrorKind::InvalidInput))
        },
        Err(_) => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn decompress_brotli(vec: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut out: Vec<u8> = Vec::new();
    match brotli::BrotliDecompress(&mut vec.clone().as_slice(), &mut out) {
        Ok(_) => Ok(out),
        Err(e) => Err(e)
    }
}

fn level_range(level: i8, min: i8, max: i8) -> i8 {
    match level {
        x if x < min => min,
        x if x > max => max,
        x => x
    }
}