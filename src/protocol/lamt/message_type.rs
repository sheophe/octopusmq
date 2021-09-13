// MessageType is encoded using 6 bits, allowing 64 total possible types
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MessageType {
    Unknown = 0x0,
    Publish,
    PublishAck,     
    PublishNack,
    PublishStored,
    PublishReleased,
    Request,
    Subscribe,
    SubscribeAck,
    Unsubscribe,
    UnsubscribeAck
}

impl MessageType {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for MessageType {
    fn from(orig: u8) -> Self {
        return match orig {
            0x01 => Self::Publish,
            0x02 => Self::PublishAck,
            0x03 => Self::PublishNack,
            0x04 => Self::PublishStored,
            0x05 => Self::PublishReleased,
            0x06 => Self::Request,
            0x07 => Self::Subscribe,
            0x08 => Self::SubscribeAck,
            0x09 => Self::Unsubscribe,
            0x0a => Self::UnsubscribeAck,
            _ => Self::default(),
        };
    }
}

// MessageType::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for MessageType {
    fn from(orig: &Vec<u8>) -> Self {
        Self::from(orig[5] & 0x3f)
    }
}