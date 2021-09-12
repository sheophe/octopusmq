use std::ptr;
use std::convert::TryInto;

const LAMT_DEFAULT_PROTOCOL_NAME: [u8; 4] = [0x4c, 0x41, 0x4d, 0x54];
const LAMT_EMPTY_PROTOCOL_NAME: [u8; 4] = [0x0, 0x0, 0x0, 0x0];

// ProtocolVersion is encoded using 5 bytes
pub struct ProtocolVersion {
    Name: [u8; 4],
    Version: u8
}

impl ProtocolVersion {
    pub fn new(version: u8) -> ProtocolVersion {
        ProtocolVersion{
            Name: LAMT_DEFAULT_PROTOCOL_NAME,
            Version: version
        }
    }
}

// ProtocolVersion::from(Vec<u8>) expects full original packet as an argument
impl From<Vec<u8>> for ProtocolVersion {
    fn from(orig: Vec<u8>) -> Self {
        ProtocolVersion{
            Name: orig.chunks(4).next().unwrap().try_into().unwrap_or_default(),
            Version: orig[4]
        }
    }
}

// DeliveryMode is encoded using 4 bits, allowing 16 total possible modes
pub enum DeliveryMode {
    PublishAndForget,
    AtLeastOnce,
    OnlyOnce,
    Unknown = 0xf
}

impl From<u8> for DeliveryMode {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return DeliveryMode::PublishAndForget,
            0x1 => return DeliveryMode::AtLeastOnce,
            0x2 => return DeliveryMode::OnlyOnce,
            _ => return DeliveryMode::Unknown,
        };
    }
}

// DeliveryMode::from(Vec<u8>) expects full original packet as an argument
impl From<Vec<u8>> for DeliveryMode {
    fn from(orig: Vec<u8>) -> Self {

    }
}

// TransportMode is encoded using 2 bits, allowing 4 total possible modes
pub enum TransportMode {
    Unicast,
    Multicast,
    Dynamic,
    Unknown = 0x3
}

impl From<u8> for TransportMode {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return TransportMode::Unicast,
            0x1 => return TransportMode::Multicast,
            0x2 => return TransportMode::Dynamic,
            _ => return TransportMode::Unknown,
        };
    }
}

// MessageType is encoded using 6 bits, allowing 64 total possible types
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

impl From<u8> for MessageType {
    fn from(orig: u8) -> Self {
        match orig {
            0x01 => return MessageType::Publish,
            0x02 => return MessageType::PublishAck,
            0x03 => return MessageType::PublishNack,
            0x04 => return MessageType::PublishStored,
            0x05 => return MessageType::PublishReleased,
            0x06 => return MessageType::Request,
            0x07 => return MessageType::Subscribe,
            0x08 => return MessageType::SubscribeAck,
            0x09 => return MessageType::Unsubscribe,
            0x0a => return MessageType::UnsubscribeAck,
            _ => return MessageType::Unknown,
        };
    }
}


// CompressionAlgorithm is encoded using 3 bits, allowing 8 total possible algorithms
pub enum CompressionAlgorithm {
    Gz,
    Bz2,
    Zlib,
    Zstd
}

impl From<u8> for CompressionAlgorithm {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return CompressionAlgorithm::Gz,
            0x1 => return CompressionAlgorithm::Bz2,
            0x2 => return CompressionAlgorithm::Zlib,
            0x3 => return CompressionAlgorithm::Zstd,
            _ => return CompressionAlgorithm::Gz
        };
    }
}

// CompressionMode is encoded using 8 bits
pub struct CompressionMode {
    compression_algorithm: CompressionAlgorithm, // bits 0..3
    compression_level: u8                        // bits 4..11
}

impl CompressionMode {
    pub fn new(compression_algorithm: CompressionAlgorithm, compression_level: u8) -> CompressionMode {
        CompressionMode{
            compression_algorithm: compression_algorithm,
            compression_level: compression_level
        }
    }

    pub fn raw(self) -> Vec<u8> {
        let mut raw_vec: Vec<u8> = Vec::new();
        raw_vec.push(((self.compression_algorithm as u8 & 0x0f) << 4) | (self.compression_level & 0x0f));
        raw_vec.push((self.compression_level & 0xf0) << 4);
        raw_vec
    }

    // Convert Vec<u8> to CompressionMode.
    pub fn from_raw(u: &Vec<u8>) -> *const CompressionMode {
        if u.len() < 2 {
            return ptr::null()
        }
        &CompressionMode{
            compression_algorithm: CompressionAlgorithm::from(u[0] & 0xf0),
            compression_level: (u[0] & 0x0f) | (u[1] & 0xf0)
        }
    }
}

pub struct MessageFlags {
    compression: bool, 
    encryption: bool,
    text_topic: bool,
    payload: bool
}

// CompressionMode is encoded using 8 bits
pub enum EncryptionAlgorithm {
    AesGCM = 0x0,
    AesCCM,
    AesCBC,
    ChaCha20Poly1305
}

pub struct PublishSettings {
    protocol_version: ProtocolVersion,
    transport_mode: TransportMode,
    message_type: MessageType,
    delivery_mode: DeliveryMode,
    message_flags: MessageFlags,

    compression_mode: *const CompressionMode, // bits 8..15
    encryption_algo: *const EncryptionAlgorithm // bits 16..23
}

impl PublishSettings {
    pub fn raw(self) -> u8 {
        (self.delivery_mode as u8 & 0x03) | ((self.transport_mode as u8 & 0x03) << 2)
    }

    pub fn from_raw(raw: Vec<u8>) -> PublishSettings {
        PublishSettings{
            delivery_mode: DeliveryMode::from(raw),
            transport_mode: TransportMode::from(raw),

            compression_mode: ptr::null(),
            encryption_algo: ptr::null()
        }
    }

    pub fn get_delivery_mode(&self) -> &DeliveryMode {
        &self.delivery_mode
    }

    pub fn get_transport_mode(&self) -> &TransportMode {
        &self.transport_mode
    }

    pub fn set_delivery_mode(&mut self, mode: DeliveryMode) {
        self.delivery_mode = mode
    }

    pub fn set_transport_mode(&mut self, mode: TransportMode) {
        self.transport_mode = mode
    }
}

pub struct Header {
    settings: PublishSettings,
    topic: String
}

impl Header {
    pub fn new(topic: String, settings: PublishSettings) -> Header {
        Header{
            settings: settings,
            topic: topic
        }
    }

    pub fn raw(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(self.settings.raw());
        buffer.push(self.topic.len() as u8);
        for byte in self.topic.bytes() {
            buffer.push(byte)
        }
        buffer
    }

    pub fn from_raw(raw: &Vec<u8>) -> *const Header {
        if raw.len() < 2 {
            return ptr::null();
        }
        let settings = PublishSettings::from_raw(raw[0] as u16);
        let topic_length = raw[1] as usize;
        let mut topic = String::new();
        for i in 0..topic_length {
            topic.push(raw[2+i] as char);
        }
        let header = Header::new(topic, settings);
        &header
    }

    pub fn get_topic(&self) -> &String {
        &self.topic
    }

    pub fn get_settings(&self) -> &PublishSettings {
        &self.settings
    }

    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic
    }

    pub fn set_settings(&mut self, settings: PublishSettings) {
        self.settings = settings
    }
}