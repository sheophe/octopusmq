// HashAlgorithm is encoded with 4 bits
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum HashAlgorithm {
    Unknown = 0x0,
    Ripemd256,
    Ripemd320,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Blake2b256,
    Blake2b512,
    Whirpool
}

impl HashAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    pub fn digest_size(&self) -> usize {
        match self {
            Self::Sha224 => 0x0e0,
            Self::Ripemd256 | Self::Sha256 | Self::Blake2b256 => 0x100,
            Self::Ripemd320 => 0x140,
            Self::Sha384 => 0x180,
            Self::Sha512 |  Self::Blake2b512 | Self::Whirpool => 0x200,
            _ => 0x0
        }
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for HashAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::Ripemd256,
            0x2 => Self::Ripemd320,
            0x3 => Self::Sha224,
            0x4 => Self::Sha256,
            0x5 => Self::Sha384,
            0x6 => Self::Sha512,
            0x7 => Self::Blake2b256,
            0x8 => Self::Blake2b512,
            0x9 => Self::Whirpool,
            _ => Self::default()
        };
    }
}

// AsymEncryptionAlgorithm is encoded with 4 bits
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AsymEncryptionAlgorithm {
    Unknown = 0x0,
    Rsa,
    Ecc
}

impl AsymEncryptionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for AsymEncryptionAlgorithm {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for AsymEncryptionAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::Rsa,
            0x3 => Self::Ecc,
            _ => Self::default()
        };
    }
}

// SymEncryptionAlgorithm is encoded with 4 bits
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SymEncryptionAlgorithm {
    Unknown = 0x0,
    Aes,
    Blowfish,
    Twofish,
    Threefish,
    Idea,
    Cast5,
    Cast6,
    Serpent
}

impl SymEncryptionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for SymEncryptionAlgorithm {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for SymEncryptionAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::Aes,
            0x2 => Self::Blowfish,
            0x3 => Self::Twofish,
            0x4 => Self::Threefish,
            0x5 => Self::Idea,
            0x6 => Self::Cast5,
            0x7 => Self::Cast6,
            0x8 => Self::Serpent,
            _ => Self::default()
        };
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EncryptionMode {
    hash_algo: HashAlgorithm,
    asym_crypt_algo: AsymEncryptionAlgorithm,
    sym_crypt_algo: SymEncryptionAlgorithm
}

impl EncryptionMode {
    pub fn new(
        hash_algo: HashAlgorithm,
        asym_crypt_algo: AsymEncryptionAlgorithm,
        sym_crypt_algo: SymEncryptionAlgorithm
    ) -> Self {
        Self {
            hash_algo: hash_algo,
            asym_crypt_algo: asym_crypt_algo,
            sym_crypt_algo: sym_crypt_algo
        }
    }

    pub fn raw(&self) -> u8 {
        ((self.hash_algo.raw() & 0x0f) << 4) | (self.asym_crypt_algo.raw() & 0x0f)
    }

    pub fn hash_algo(&self) -> HashAlgorithm {
        self.hash_algo
    }

    pub fn asym_crypt_algo(&self) -> AsymEncryptionAlgorithm {
        self.asym_crypt_algo
    }

    pub fn sym_crypt_algo(&self) -> SymEncryptionAlgorithm {
        self.sym_crypt_algo
    }
}

impl Default for EncryptionMode {
    fn default() -> Self {
        Self {
            hash_algo: HashAlgorithm::default(),
            asym_crypt_algo: AsymEncryptionAlgorithm::default(),
            sym_crypt_algo: SymEncryptionAlgorithm::default()
        }
    }
}

impl From<u8> for EncryptionMode {
    fn from(orig: u8) -> Self {
        Self {
            hash_algo: HashAlgorithm::from((orig >> 4) & 0x0f),
            asym_crypt_algo: AsymEncryptionAlgorithm::from(orig & 0x1f),
            sym_crypt_algo: SymEncryptionAlgorithm::from(orig & 0x1f)
        }
    }
}
