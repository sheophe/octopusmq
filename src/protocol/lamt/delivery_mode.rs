// DeliveryMode is encoded using 4 bits, allowing 16 total possible modes
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DeliveryMode {
    Unknown = 0x0,
    PublishAndForget,
    AtLeastOnce,
    ExactlyOnce,
}

impl DeliveryMode {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for DeliveryMode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for DeliveryMode {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::PublishAndForget,
            0x2 => Self::AtLeastOnce,
            0x3 => Self::ExactlyOnce,
            _ => Self::default(),
        };
    }
}

// DeliveryMode::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for DeliveryMode {
    fn from(orig: &Vec<u8>) -> Self {
        // DeliveryMode is located at the first 4 bits of 7-th byte of Header
        Self::from((orig[6] & 0xf0) >> 4)
    }
}
