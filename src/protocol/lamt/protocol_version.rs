use std::convert::TryInto;

const LAMT_DEFAULT_PROTOCOL_NAME: [u8; 4] = [0x4c, 0x41, 0x4d, 0x54];
const LAMT_DEFAULT_PROTOCOL_VERSION: u8 = 0x01;

// ProtocolVersion is encoded using 5 bytes
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProtocolVersion {
    name: [u8; 4],
    version: u8,
}

impl ProtocolVersion {
    pub fn new(version: u8) -> Self {
        Self {
            name: LAMT_DEFAULT_PROTOCOL_NAME,
            version: version,
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from(self.name));
        vec.push(self.version);
        vec
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self {
            name: LAMT_DEFAULT_PROTOCOL_NAME,
            version: LAMT_DEFAULT_PROTOCOL_VERSION,
        }
    }
}

// ProtocolVersion::from(Vec<u8>) expects full original packet as an argument
impl From<&Vec<u8>> for ProtocolVersion {
    fn from(orig: &Vec<u8>) -> Self {
        Self {
            name: orig
                .chunks(4)
                .next()
                .unwrap()
                .try_into()
                .unwrap_or_default(),
            version: orig[4],
        }
    }
}
