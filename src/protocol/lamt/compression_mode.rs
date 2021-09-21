const LAMT_DEFAULT_COMPRESSION: u8 = 6;

/// CompressionAlgorithm is encoded using 3 bits, allowing 8 total possible algorithms.
///
/// Supported algorithms with their `u8` representation:
/// * `0` — DEFLATE,
/// * `1` — GZ,
/// * `2` — BZ2,
/// * `3` — Zlib,
/// * `4` — Zstandard,
/// * `5` — brotli
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum CompressionAlgorithm {
    Deflate = 0x0,
    Gzip,
    Bzip2,
    Zlib,
    Zstd,
    Brotli,
    NoCompression = 0x8,
}

impl CompressionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::NoCompression
    }
}

impl From<u8> for CompressionAlgorithm {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => Self::Deflate,
            0x1 => Self::Gzip,
            0x2 => Self::Bzip2,
            0x3 => Self::Zlib,
            0x4 => Self::Zstd,
            0x5 => Self::Brotli,
            _ => Self::default(),
        }
    }
}

/// CompressionLevel is encoded using 5 bits, allowing values in range from 0 to 31
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CompressionLevel(u8);

impl CompressionLevel {
    pub fn new(level: u8) -> Self {
        Self(level)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self(0)
    }
}

// CompressionMode is encoded using 8 bits
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CompressionMode {
    algorithm: CompressionAlgorithm,
    level: CompressionLevel,
}

impl CompressionMode {
    pub fn new(
        compression_algorithm: CompressionAlgorithm,
        compression_level: CompressionLevel,
    ) -> Self {
        Self {
            algorithm: compression_algorithm,
            level: compression_level,
        }
    }

    #[allow(dead_code)]
    pub fn new_deflate() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Deflate)
    }

    #[allow(dead_code)]
    pub fn new_gzip() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Gzip)
    }

    #[allow(dead_code)]
    pub fn new_bzip2() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Bzip2)
    }

    #[allow(dead_code)]
    pub fn new_zlib() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Zlib)
    }

    #[allow(dead_code)]
    pub fn new_zstd() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Zstd)
    }

    #[allow(dead_code)]
    pub fn new_brotli() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Brotli)
    }

    pub fn raw(&self) -> u8 {
        ((self.algorithm.raw() & 0x07) << 5) | (self.level.raw() & 0x1f)
    }

    pub fn algorithm(&self) -> CompressionAlgorithm {
        self.algorithm
    }

    pub fn level(&self) -> u8 {
        self.level.0
    }

    fn new_with_algo(algo: CompressionAlgorithm) -> Self {
        Self {
            algorithm: algo,
            level: CompressionLevel::new(LAMT_DEFAULT_COMPRESSION),
        }
    }
}

impl Default for CompressionMode {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::default(),
            level: CompressionLevel::default(),
        }
    }
}

impl From<u8> for CompressionMode {
    fn from(orig: u8) -> Self {
        Self {
            algorithm: CompressionAlgorithm::from((orig >> 5) & 0x07),
            level: CompressionLevel::new(orig & 0x1f),
        }
    }
}

// CompressionMode::from(Vec<u8>) expects full original packet as an argument
impl From<&[u8]> for CompressionMode {
    fn from(orig: &[u8]) -> Self {
        Self::from(orig[7])
    }
}
