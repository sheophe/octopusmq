use std::mem;
use std::convert::TryInto;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const LAMT_DEFAULT_PROTOCOL_NAME: [u8; 4] = [0x4c, 0x41, 0x4d, 0x54];
const LAMT_EMPTY_PROTOCOL_NAME: [u8; 4] = [0x0, 0x0, 0x0, 0x0];
const LAMT_DEFAULT_PROTOCOL_VERSION: u8 = 0x01;
const LAMT_FIXED_OFFSET: usize = 7;

// ProtocolVersion is encoded using 5 bytes
#[derive(Copy, Clone)]
pub struct ProtocolVersion {
    name: [u8; 4],
    version: u8
}

impl ProtocolVersion {
    pub fn new(version: u8) -> Self {
        Self{
            name: LAMT_DEFAULT_PROTOCOL_NAME,
            version: version
        }
    }

    pub fn default() -> Self {
        Self{
            name: LAMT_DEFAULT_PROTOCOL_NAME,
            version: LAMT_DEFAULT_PROTOCOL_VERSION
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from(self.name));
        vec.push(self.version);
        vec
    }
}

// ProtocolVersion::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for ProtocolVersion {
    fn from(orig: &Vec<u8>) -> Self {
        Self{
            name: orig.chunks(4).next().unwrap().try_into().unwrap_or_default(),
            version: orig[4]
        }
    }
}

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

    pub fn default() -> Self {
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

// MessageType is encoded using 6 bits, allowing 64 total possible types
#[derive(Copy, Clone)]
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

    pub fn default() -> Self {
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

// DeliveryMode is encoded using 4 bits, allowing 16 total possible modes
#[derive(Copy, Clone)]
pub enum DeliveryMode {
    Unknown = 0x0,
    PublishAndForget,
    AtLeastOnce,
    ExactlyOnce
}

impl DeliveryMode {
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    pub fn default() -> Self {
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

#[derive(Copy, Clone)]
pub struct MessageFlags {
    compression: bool, 
    encryption: bool,
    text_topic: bool,
    payload: bool
}

impl MessageFlags {
    pub fn raw(&self) -> u8 {
        ((self.compression as u8) << 3) +
        ((self.encryption as u8) << 2) +
        ((self.text_topic as u8) << 1) +
        ((self.payload as u8) << 0)
    }

    pub fn default() -> Self {
        Self {
            compression: false,
            encryption: false,
            text_topic: false,
            payload: true
        }
    }
}

impl From<u8> for MessageFlags {
    fn from(orig: u8) -> Self {
        Self {
            compression: orig & (1 << 3) != 0,
            encryption: orig & (1 << 2) != 0,
            text_topic: orig & (1 << 1) != 0,
            payload: orig & (1 << 0) != 0
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

// CompressionAlgorithm is encoded using 3 bits, allowing 8 total possible algorithms
#[derive(Copy, Clone)]
pub enum CompressionAlgorithm {
    Unknown = 0x0,
    Gz,
    Bz2,
    Zlib,
    Zstd
}

impl CompressionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    pub fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for CompressionAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::Gz,
            0x2 => Self::Bz2,
            0x3 => Self::Zlib,
            0x4 => Self::Zstd,
            _ => Self::default()
        };
    }
}

// CompressionMode is encoded using 8 bits
#[derive(Copy, Clone)]
pub struct CompressionMode {
    compression_algorithm: CompressionAlgorithm,
    compression_level: u8
}

impl CompressionMode {
    pub fn new(compression_algorithm: CompressionAlgorithm, compression_level: u8) -> Self {
        Self {
            compression_algorithm: compression_algorithm,
            compression_level: compression_level
        }
    }

    pub fn raw(&self) -> u8 {
        return (self.compression_algorithm as u8 & 0x07 << 5) | (self.compression_level & 0x1f)
    }
}

impl From<u8> for CompressionMode {
    fn from(orig: u8) -> Self {
        Self {
            compression_algorithm: CompressionAlgorithm::from((orig & 0xe0) >> 5),
            compression_level: orig & 0x1f
        }
    }
}

// CompressionMode::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for CompressionMode {
    fn from(orig: &Vec<u8>) -> Self {
        Self::from(orig[7])
    }
}

// CompressionMode is encoded using 8 bits
#[derive(Copy, Clone)]
pub enum EncryptionAlgorithm {
    Unknown = 0x0,
    AesGCM,
    AesCCM,
    AesCBC,
    Gpg,
    ChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    pub fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for EncryptionAlgorithm {
    fn from(orig: u8) -> Self {
        return match orig {
            0x1 => Self::AesGCM,
            0x2 => Self::AesCCM,
            0x3 => Self::AesCBC,
            0x4 => Self::Gpg,
            0x5 => Self::ChaCha20Poly1305,
            _ => Self::default()
        };
    }
}

#[derive(Clone)]
pub struct Topic {
    name: Vec<u8>,
    id: u32
}

impl Topic {
    pub fn from(orig: &Vec<u8>, header: &mut Header) -> Self {
        if header.message_flags.text_topic {
            return Self::named_from(orig, &mut header.offset)
        }
        Self::numbered_from(orig, &mut header.offset)
    }

    pub fn default() -> Self {
        Self {
            name: Vec::new(),
            id: 0
        }
    }

    pub fn raw_id(&self) -> Vec<u8> {
        Vec::from(u32_as_slice(self.id))
    }

    pub fn raw_name(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from(u32_as_slice(self.name.len() as u32)));
        vec.append(&mut self.name.clone());
        vec
    }

    fn named_from(orig: &Vec<u8>, offset: &mut usize) -> Self {
        let length = orig[*offset] as usize;
        *offset += mem::size_of::<u8>();
        let topic = Self {
            name: Vec::from(&orig[*offset..*offset+length]),
            id: 0
        };
        *offset += length as usize;
        topic
    }

    fn numbered_from(orig: &Vec<u8>, offset: &mut usize) -> Self {
        let length = mem::size_of::<u32>();
        let id_slice = &orig[*offset..*offset+length];
        *offset += length;
        Self {
            name: Vec::new(),
            id: slice_as_u32(id_slice)
        }
    }
}

#[derive(Clone)]
pub struct Header {
    protocol_version: ProtocolVersion,
    transport_mode: TransportMode,
    message_type: MessageType,
    delivery_mode: DeliveryMode,
    message_flags: MessageFlags,
    compression_mode: Option<CompressionMode>, 
    encryption_algo: Option<EncryptionAlgorithm>,
    topic: Topic,
    offset: usize
}

impl Header {
    pub fn new() -> Self {
        Self {
            protocol_version: ProtocolVersion::default(),
            transport_mode: TransportMode::default(),
            message_type: MessageType::default(),
            delivery_mode: DeliveryMode::default(),
            message_flags: MessageFlags::default(),
            compression_mode: None,
            encryption_algo: None,
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.protocol_version.raw());
        vec.push(((self.transport_mode.raw() & 0x3) << 6) | (self.message_type.raw() & 0x3f));
        vec.push(((self.delivery_mode.raw() & 0xf) << 4) | (self.message_flags.raw() & 0xf));
        if self.message_flags.compression {
            vec.push(self.compression_mode.as_ref().unwrap().raw());
        }
        if self.message_flags.encryption {
            vec.push(self.encryption_algo.as_ref().unwrap().raw());
        }
        if self.message_flags.text_topic {
            vec.append(&mut self.topic.raw_name())
        } else {
            vec.append(&mut self.topic.raw_id())
        }
        vec
    }

    pub fn set_transport_mode<'a>(&'a mut self, transport_mode: TransportMode) -> &'a mut Self {
        self.transport_mode = transport_mode;
        self
    }

    pub fn set_message_type<'a>(&'a mut self, message_type: MessageType) -> &'a mut Self {
        self.message_type = message_type;
        self
    }

    pub fn set_delivery_mode<'a>(&'a mut self, delivery_mode: DeliveryMode) -> &'a mut Self {
        self.delivery_mode = delivery_mode;
        self
    }

    pub fn set_compression_mode<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        self.compression_mode = Some(compression_mode);
        self.message_flags.compression = true;
        self
    }

    pub fn set_encryption_algo<'a>(&'a mut self, encryption_algo: EncryptionAlgorithm) -> &'a mut Self {
        self.encryption_algo = Some(encryption_algo);
        self.message_flags.encryption = true;
        self
    }

    pub fn set_string_topic<'a>(&'a mut self, text_topic: String) -> &'a mut Self {
        self.topic = Topic {
            name: text_topic.into_bytes(),
            id: 0
        };
        self.message_flags.text_topic = true;
        self
    }

    pub fn set_vector_topic<'a>(&'a mut self, text_topic: Vec<u8>) -> &'a mut Self {
        self.topic = Topic {
            name: text_topic,
            id: 0
        };
        self.message_flags.text_topic = true;
        self
    }

    pub fn set_numeric_topic<'a>(&'a mut self, numeric_topic: u32) -> &'a mut Self {
        self.topic = Topic {
            name: Vec::new(),
            id: numeric_topic
        };
        self.message_flags.text_topic = false;
        self
    }
}

impl From<&Vec<u8>> for Header {
    fn from(orig: &Vec<u8>) -> Self {
        let mut header = Self{
            protocol_version: ProtocolVersion::from(orig),
            transport_mode: TransportMode::from(orig),
            message_type: MessageType::from(orig),
            delivery_mode: DeliveryMode::from(orig),
            message_flags: MessageFlags::from(orig),
            compression_mode: None,
            encryption_algo: None,
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET
        };
        if header.message_flags.compression {
            header.compression_mode = Some(CompressionMode::from(orig[header.offset]));
            header.offset += 1;
        }
        if header.message_flags.encryption {
            header.encryption_algo = Some(EncryptionAlgorithm::from(orig[header.offset]));
            header.offset += 1;
        }
        header.topic = Topic::from(orig, &mut header);
        header
    }
}

#[derive(Clone)]
pub struct Payload {
    current_part: u8,
    total_parts: u8,
    hash: u32,
    length: u32,
    data: Vec<u8>
}

impl Payload {
    pub fn new() -> Self {
        Self {
            current_part: 1,
            total_parts: 1,
            hash: 0,
            length: 0,
            data: Vec::new()
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(self.current_part);
        vec.push(self.total_parts);
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        vec.append(&mut Vec::from(u32_as_slice(hasher.finish() as u32)));
        vec.append(&mut Vec::from(u32_as_slice(self.length)));
        vec.append(&mut self.data.clone());
        vec
    }

    pub fn append<'a>(&'a mut self, other: &mut Vec<u8>) -> &'a mut Self {
        self.data.append(other);
        self.length = self.data.len() as u32;
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        self.hash = hasher.finish() as u32;
        self
    }
}

impl From<&Vec<u8>> for Payload {
    fn from(orig: &Vec<u8>) -> Self {
        Self {
            current_part: orig[0],
            total_parts: orig[1],
            hash: slice_as_u32(&orig[2..6]),
            length: slice_as_u32(&orig[6..10]),
            data: Vec::from(&orig[10..])
        }
    }
}

pub struct Message {
    header: Header,
    payload: Payload
}

impl Message {
    pub fn new(header: Header, payload: Payload) -> Message {
        Message {
            header: header,
            payload: payload
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.header.raw());
        vec.append(&mut self.payload.raw());
        vec
    }
}

fn slice_as_u32(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn u32_as_slice(val: u32) -> [u8; 4] {
    [
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        (val >> 0) as u8
    ]
}