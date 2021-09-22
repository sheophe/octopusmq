use std::ops::Deref;

use blake2::Blake2b;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use whirlpool::Whirlpool;

use crate::lamt::encryption_mode::*;

// This trait is a lightweight version of digest::Digest,
// but it uses primitive data types for arguments so the dyn trait can be
// boxed into a return value without specifying the values of associaated types.
pub trait Digest {
    fn finalize(self: Self) -> Hash;
    fn update(&mut self, data: &[u8]);
    fn reset(&mut self);
}

#[derive(Copy, Clone, Debug)]
pub struct DigestAdapter<D: digest::Digest>(D);

impl<D: digest::Digest> DigestAdapter<D> {
    pub fn new() -> Self {
        Self(D::new())
    }
}

impl<D: digest::Digest> Digest for DigestAdapter<D> {
    fn finalize(self: Self) -> Hash {
        Hash::from(self.0.finalize().deref())
    }

    fn update(&mut self, data: &[u8]) {
        self.0.update(data)
    }

    fn reset(&mut self) {
        self.0.reset()
    }
}

pub struct Hasher;
impl Hasher {
    fn new_with_algo(algo: HashAlgorithm) -> Box<dyn Digest> {
        match algo {
            HashAlgorithm::Ripemd256 => Box::new(DigestAdapter::<Ripemd256>::new()),
            HashAlgorithm::Ripemd320 => Box::new(DigestAdapter::<Ripemd320>::new()),
            HashAlgorithm::Sha224 => Box::new(DigestAdapter::<Sha224>::new()),
            HashAlgorithm::Sha256 => Box::new(DigestAdapter::<Sha256>::new()),
            HashAlgorithm::Sha384 => Box::new(DigestAdapter::<Sha384>::new()),
            HashAlgorithm::Sha512 => Box::new(DigestAdapter::<Sha512>::new()),
            HashAlgorithm::Blake2b => Box::new(DigestAdapter::<Blake2b>::new()),
            HashAlgorithm::Whirlpool => Box::new(DigestAdapter::<Whirlpool>::new()),
            _ => panic!("unknown algorithm"),
        }
    }
}
