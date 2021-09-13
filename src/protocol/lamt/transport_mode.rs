// TransportMode is encoded using 2 bits, allowing 4 total possible modes
#[derive(Copy, Clone)]
pub enum TransportMode {
    Unknown = 0x0,
    Unicast,
    Multicast,
    Dynamic
}

impl TransportMode {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for TransportMode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for TransportMode {
    fn from(orig: u8) -> Self {
        match orig {
            0x1 => return Self::Unicast,
            0x2 => return Self::Multicast,
            0x3 => return Self::Dynamic,
            _ => return Self::default(),
        };
    }
}

// TransportMode::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for TransportMode {
    fn from(orig: &Vec<u8>) -> Self {
        Self::from((orig[5] & 0xc0) >> 6)
    }
}
