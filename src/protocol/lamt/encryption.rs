use std::ops::Deref;

use blake2::Blake2b;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use whirlpool::Whirlpool;

use crate::lamt::encryption_mode::*;

// This trait duplicates three functions from digest::Digest,
// but uses primitive data types for arguments so the dyn trait can be
// boxed into a return value.
pub trait Digest {
    fn finalize(self: Box<Self>) -> Hash;
    fn update(&mut self, data: &[u8]);
    fn reset(&mut self);
}

#[derive(Copy, Clone)]
pub struct DigestAdapter<D: digest::Digest>(D);

impl<D: digest::Digest> DigestAdapter<D> {
    pub fn new() -> Self {
        Self(D::new())
    }
}

impl<D: digest::Digest> Digest for DigestAdapter<D> {
    fn finalize(self: Box<Self>) -> Hash {
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

#[cfg(test)]
mod tests {
    use ring::signature::*;

    #[test]
    pub fn ed25519_sign_and_verify() {
        let input = [
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
        ];
        let pkcs8_key_pair =
            Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(&pkcs8_key_pair.as_ref()).unwrap();
        let signature = key_pair.sign(input.as_ref());
        let vec = Vec::from(signature.as_ref());
        ED25519.verify(key_pair.public_key(), input, signature);
    }

    #[test]
    pub fn ecdsa_signature() {
        let input = [
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
        ];
        let pkcs8_key_pair = EcdsaKeyPair::generate_pkcs8(
            &ECDSA_P384_SHA384_FIXED_SIGNING,
            &ring::rand::SystemRandom::new(),
        )
        .unwrap();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P384_SHA384_FIXED_SIGNING, &pkcs8_key_pair.as_ref());
    }
}
