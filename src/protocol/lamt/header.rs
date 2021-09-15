use crate::lamt::{
    ClientId, CompressionAlgorithm, CompressionMode, DeliveryMode, EncryptionMode, MessageFlags,
    MessageType, ProtocolVersion, Topic, TransportMode,
};

const LAMT_FIXED_OFFSET: usize = 7;

#[derive(Clone, Eq)]
pub struct Header {
    protocol_version: ProtocolVersion,
    transport_mode: TransportMode,
    message_type: MessageType,
    delivery_mode: DeliveryMode,
    message_flags: MessageFlags,
    compression_mode: Option<CompressionMode>,
    encryption_mode: Option<EncryptionMode>,
    client_id: ClientId,
    topic: Topic,
    offset: usize,
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
            encryption_mode: None,
            client_id: ClientId::default(),
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET,
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
            vec.push(self.encryption_mode.as_ref().unwrap().raw());
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
            None => CompressionMode::default(),
        }
    }

    pub fn encryption_mode(&self) -> EncryptionMode {
        match self.encryption_mode {
            Some(v) => v,
            None => EncryptionMode::default(),
        }
    }

    #[allow(dead_code)]
    pub fn transport_mode(&self) -> &TransportMode {
        &self.transport_mode
    }

    #[allow(dead_code)]
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
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

    #[allow(dead_code)]
    pub fn set_transport_mode<'a>(&'a mut self, transport_mode: TransportMode) -> &'a mut Self {
        self.transport_mode = transport_mode;
        self
    }

    #[allow(dead_code)]
    pub fn set_message_type<'a>(&'a mut self, message_type: MessageType) -> &'a mut Self {
        self.message_type = message_type;
        self
    }

    #[allow(dead_code)]
    pub fn set_delivery_mode<'a>(&'a mut self, delivery_mode: DeliveryMode) -> &'a mut Self {
        self.delivery_mode = delivery_mode;
        self
    }

    #[allow(dead_code)]
    pub fn set_message_flags<'a>(&'a mut self, message_flags: MessageFlags) -> &'a mut Self {
        self.message_flags = message_flags;
        self
    }

    #[allow(dead_code)]
    pub fn set_compression_mode<'a>(
        &'a mut self,
        compression_mode: CompressionMode,
    ) -> &'a mut Self {
        self.compression_mode = Some(compression_mode);
        if compression_mode.algorithm() != CompressionAlgorithm::NoCompression {
            self.message_flags.set_compression(true);
        }
        self
    }

    #[allow(dead_code)]
    pub fn set_encryption_mode<'a>(&'a mut self, encryption_mode: EncryptionMode) -> &'a mut Self {
        self.encryption_mode = Some(encryption_mode);
        self.message_flags.set_encryption(true);
        self
    }

    #[allow(dead_code)]
    pub fn set_string_topic<'a>(&'a mut self, text_topic: String) -> &'a mut Self {
        self.topic = Topic::new_text(text_topic.into_bytes());
        self.message_flags.set_text_topic(true);
        self
    }

    #[allow(dead_code)]
    pub fn set_vector_topic<'a>(&'a mut self, text_topic: Vec<u8>) -> &'a mut Self {
        self.topic = Topic::new_text(text_topic);
        self.message_flags.set_text_topic(true);
        self
    }

    #[allow(dead_code)]
    pub fn set_numeric_topic<'a>(&'a mut self, numeric_topic: u32) -> &'a mut Self {
        self.topic = Topic::new_numeric(numeric_topic);
        self.message_flags.set_text_topic(false);
        self
    }

    #[allow(dead_code)]
    pub fn set_client_id<'a>(&'a mut self, id: ClientId) -> &'a mut Self {
        self.client_id = id;
        self
    }
}

impl From<&Vec<u8>> for Header {
    fn from(orig: &Vec<u8>) -> Self {
        let mut header = Self {
            protocol_version: ProtocolVersion::from(orig),
            transport_mode: TransportMode::from(orig),
            message_type: MessageType::from(orig),
            delivery_mode: DeliveryMode::from(orig),
            message_flags: MessageFlags::from(orig),
            compression_mode: None,
            encryption_mode: None,
            client_id: ClientId::default(),
            topic: Topic::default(),
            offset: LAMT_FIXED_OFFSET,
        };
        if header.message_flags.compression() {
            header.compression_mode = Some(CompressionMode::from(orig[header.offset]));
            header.offset += 1;
        }
        if header.message_flags.encryption() {
            header.encryption_mode = Some(EncryptionMode::from(orig[header.offset]));
            header.offset += 1;
        }
        header.client_id = ClientId::from(orig, &mut header);
        header.topic = Topic::from(orig, &mut header);
        header
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.protocol_version == other.protocol_version
            && self.transport_mode == other.transport_mode
            && self.message_type == other.message_type
            && self.delivery_mode == other.delivery_mode
            && self.message_flags == other.message_flags
            && self.compression_mode == other.compression_mode
            && self.encryption_mode == other.encryption_mode
            && self.client_id == other.client_id
            && self.topic == other.topic
    }
}
