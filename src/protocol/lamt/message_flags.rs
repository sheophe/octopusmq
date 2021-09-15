#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MessageFlags {
    compression: bool,
    encryption: bool,
    text_topic: bool,
    payload: bool,
}

impl MessageFlags {
    pub fn raw(&self) -> u8 {
        ((self.compression as u8) << 3)
            + ((self.encryption as u8) << 2)
            + ((self.text_topic as u8) << 1)
            + ((self.payload as u8) << 0)
    }

    #[allow(dead_code)]
    pub fn with_payload() -> Self {
        Self {
            compression: false,
            encryption: false,
            text_topic: false,
            payload: true,
        }
    }

    #[allow(dead_code)]
    pub fn with_compression() -> Self {
        Self {
            compression: true,
            encryption: false,
            text_topic: false,
            payload: true,
        }
    }

    #[allow(dead_code)]
    pub fn with_encryption() -> Self {
        Self {
            compression: false,
            encryption: true,
            text_topic: false,
            payload: true,
        }
    }

    #[allow(dead_code)]
    pub fn with_compression_and_encryption() -> Self {
        Self {
            compression: true,
            encryption: true,
            text_topic: false,
            payload: true,
        }
    }

    pub fn compression(&self) -> bool {
        self.compression
    }

    pub fn encryption(&self) -> bool {
        self.encryption
    }

    pub fn text_topic(&self) -> bool {
        self.text_topic
    }

    pub fn payload(&self) -> bool {
        self.payload
    }

    #[allow(dead_code)]
    pub fn set_compression(&mut self, compression: bool) {
        self.compression = compression
    }

    #[allow(dead_code)]
    pub fn set_encryption(&mut self, encryption: bool) {
        self.encryption = encryption
    }

    #[allow(dead_code)]
    pub fn set_text_topic(&mut self, text_topic: bool) {
        self.text_topic = text_topic
    }

    #[allow(dead_code)]
    pub fn set_payload(&mut self, payload: bool) {
        self.payload = payload
    }
}

impl Default for MessageFlags {
    fn default() -> Self {
        Self {
            compression: false,
            encryption: false,
            text_topic: false,
            payload: false,
        }
    }
}

impl From<u8> for MessageFlags {
    fn from(orig: u8) -> Self {
        Self {
            compression: orig & (1 << 3) != 0,
            encryption: orig & (1 << 2) != 0,
            text_topic: orig & (1 << 1) != 0,
            payload: orig & (1 << 0) != 0,
        }
    }
}

// MessageFlags::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for MessageFlags {
    fn from(orig: &Vec<u8>) -> Self {
        // MessageFlags is located at the last 4 bits of 7-th byte of Header
        Self::from(orig[6] & 0x0f)
    }
}
