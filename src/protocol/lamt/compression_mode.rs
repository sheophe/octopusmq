const LAMT_DEFAULT_COMPRESSION: i8 = 6;

// CompressionAlgorithm is encoded using 3 bits, allowing 8 total possible algorithms
#[derive(Copy, Clone, PartialEq)]
pub enum CompressionAlgorithm {
    Deflate = 0x0,
    Gzip,
    Bzip2,
    Zlib,
    Zstd,
    Brotli,
    NoCompression = 0x8
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
        return match orig {
            0x1 => Self::Deflate,
            0x2 => Self::Gzip,
            0x3 => Self::Bzip2,
            0x4 => Self::Zlib,
            0x5 => Self::Zstd,
            0x6 => Self::Brotli,
            _ => Self::default()
        };
    }
}

// CompressionMode is encoded using 8 bits
#[derive(Copy, Clone)]
pub struct CompressionMode {
    algorithm: CompressionAlgorithm,
    level: i8
}

impl CompressionMode {
    pub fn new(compression_algorithm: CompressionAlgorithm, compression_level: i8) -> Self {
        Self {
            algorithm: compression_algorithm,
            level: compression_level
        }
    }

    pub fn new_deflate() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Deflate)
    }

    pub fn new_gzip() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Gzip)
    }

    pub fn new_bzip2() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Bzip2)
    }

    pub fn new_zlib() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Zlib)
    }

    pub fn new_zstd() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Zstd)
    }

    pub fn new_brotli() -> Self {
        Self::new_with_algo(CompressionAlgorithm::Brotli)
    }

    pub fn raw(&self) -> u8 {
        (self.algorithm as u8 & 0x07 << 5) | (self.level as u8 & 0x1f)
    }

    pub fn get_algorithm(&self) -> CompressionAlgorithm {
        self.algorithm
    }

    pub fn get_level(&self) -> i8 {
        self.level
    }


    fn new_with_algo(algo: CompressionAlgorithm) -> Self {
        Self {
            algorithm: algo,
            level: LAMT_DEFAULT_COMPRESSION
        }
    }
}

impl Default for CompressionMode {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::default(),
            level: 0
        }
    }
}

impl From<u8> for CompressionMode {
    fn from(orig: u8) -> Self {
        Self {
            algorithm: CompressionAlgorithm::from((orig & 0xe0) >> 5),
            level: (orig & 0x1f) as i8
        }
    }
}

// CompressionMode::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for CompressionMode {
    fn from(orig: &Vec<u8>) -> Self {
        Self::from(orig[7])
    }
}
