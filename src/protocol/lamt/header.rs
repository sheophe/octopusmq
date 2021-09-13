use crate::lamt::{
    ProtocolVersion,
    TransportMode,
    MessageType,
    DeliveryMode,
    MessageFlags,
    CompressionAlgorithm,
    CompressionMode,
    EncryptionAlgorithm,
    ClientId,
    Topic
};

const LAMT_FIXED_OFFSET: usize = 7;

#[derive(Clone, PartialEq, Eq)]
pub struct Header {
    protocol_version: ProtocolVersion,
    transport_mode: TransportMode,
    message_type: MessageType,
    delivery_mode: DeliveryMode,
    message_flags: MessageFlags,
    compression_mode: Option<CompressionMode>, 
    encryption_algo: Option<EncryptionAlgorithm>,
    client_id: ClientId,
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
            client_id: ClientId::default(),
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.protocol_version.raw());
        vec.push(((self.transport_mode.raw() & 0x3) << 6) | (self.message_type.raw() & 0x3f));
        vec.push(((self.delivery_mode.raw() & 0xf) << 4) | (self.message_flags.raw() & 0xf));
        if self.message_flags.compression() {
            vec.push(self.compression_mode.as_ref().unwrap().raw());
        }
        if self.message_flags.encryption() {
            vec.push(self.encryption_algo.as_ref().unwrap().raw());
        }
        vec.append(&mut self.client_id.raw());
        if self.message_flags.text_topic() {
            vec.append(&mut self.topic.raw_name())
        } else {
            vec.append(&mut self.topic.raw_id())
        }
        vec
    }

    pub fn compression_mode(&self) -> CompressionMode {
        match self.compression_mode {
            Some(v) => v,
            None => CompressionMode::default()
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn offset_mut<'a>(&'a mut self) -> &'a mut usize {
        &mut self.offset
    }

    pub fn message_flags(&self) -> &MessageFlags {
        &self.message_flags
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

    pub fn set_message_flags<'a>(&'a mut self, message_flags: MessageFlags) -> &'a mut Self {
        self.message_flags = message_flags;
        self
    }

    pub fn set_compression_mode<'a>(&'a mut self, compression_mode: CompressionMode) -> &'a mut Self {
        self.compression_mode = Some(compression_mode);
        if compression_mode.algorithm() != CompressionAlgorithm::NoCompression {
            self.message_flags.set_compression(true);
        }
        self
    }

    pub fn set_encryption_algo<'a>(&'a mut self, encryption_algo: EncryptionAlgorithm) -> &'a mut Self {
        self.encryption_algo = Some(encryption_algo);
        self.message_flags.set_encryption(true);
        self
    }

    pub fn set_string_topic<'a>(&'a mut self, text_topic: String) -> &'a mut Self {
        self.topic = Topic::new_text(text_topic.into_bytes());
        self.message_flags.set_text_topic(true);
        self
    }

    pub fn set_vector_topic<'a>(&'a mut self, text_topic: Vec<u8>) -> &'a mut Self {
        self.topic = Topic::new_text(text_topic);
        self.message_flags.set_text_topic(true);
        self
    }

    pub fn set_numeric_topic<'a>(&'a mut self, numeric_topic: u32) -> &'a mut Self {
        self.topic = Topic::new_numeric(numeric_topic);
        self.message_flags.set_text_topic(false);
        self
    }

    pub fn set_client_id<'a>(&'a mut self, id: ClientId) -> &'a mut Self {
        self.client_id = id;
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
            client_id: ClientId::default(),
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET
        };
        if header.message_flags.compression() {
            header.compression_mode = Some(CompressionMode::from(orig[header.offset]));
            header.offset += 1;
        }
        if header.message_flags.encryption() {
            header.encryption_algo = Some(EncryptionAlgorithm::from(orig[header.offset]));
            header.offset += 1;
        }
        header.client_id = ClientId::from(orig, &mut header);
        header.topic = Topic::from(orig, &mut header);
        header
    }
}
