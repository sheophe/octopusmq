use std::io::{Error, ErrorKind};
use std::ops::Deref;

use digest::Digest;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use blake2::Blake2b;
use whirlpool::Whirlpool;

use crate::lamt::encryption_mode::*;

pub trait HasherTrait {
    fn finalize(self: Box<Self>) -> HashDigest;
    fn update(&mut self, data: &Vec<u8>);
    fn reset(&mut self);
}

#[derive(Copy, Clone)]
pub struct Hasher<T: Digest>(T);

impl<T: Digest> Hasher<T> {
    pub fn new() -> Self {
        Self(T::new())
    }
}

impl<T: Digest> HasherTrait for Hasher<T> {
    fn finalize(self: Box<Self>) -> HashDigest {
        HashDigest::from(self.0.finalize().deref())
    }

    fn update(&mut self, data: &Vec<u8>) {
        self.0.update(data)
    }

    fn reset(&mut self) {
        self.0.reset()
    }
}

pub struct HasherFactory();

impl HasherFactory {
    pub fn create(algo: HashAlgorithm) -> Result<Box<dyn HasherTrait>, Error> {
        match algo {
            HashAlgorithm::Ripemd256 => Ok(Box::new(Hasher::<Ripemd256>::new())),
            HashAlgorithm::Ripemd320 => Ok(Box::new(Hasher::<Ripemd320>::new())),
            HashAlgorithm::Sha224 => Ok(Box::new(Hasher::<Sha224>::new())),
            HashAlgorithm::Sha256 => Ok(Box::new(Hasher::<Sha256>::new())),
            HashAlgorithm::Sha384 => Ok(Box::new(Hasher::<Sha384>::new())),
            HashAlgorithm::Sha512 => Ok(Box::new(Hasher::<Sha512>::new())),
            HashAlgorithm::Blake2b => Ok(Box::new(Hasher::<Blake2b>::new())),
            HashAlgorithm::Whirlpool => Ok(Box::new(Hasher::<Whirlpool>::new())),
            _ => Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}
