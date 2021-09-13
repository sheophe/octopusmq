// CompressionMode is encoded using 8 bits
#[derive(Copy, Clone)]
pub enum EncryptionAlgorithm {
    Unknown = 0x0,
    AesGCM,
    AesCCM,
    AesCBC,
    OpenPGP,
    ChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for EncryptionAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::AesGCM,
            0x2 => Self::AesCCM,
            0x3 => Self::AesCBC,
            0x4 => Self::OpenPGP,
            0x5 => Self::ChaCha20Poly1305,
            _ => Self::default()
        };
    }
}
