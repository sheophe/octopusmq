use crate::lamt::{Header, Payload};

#[derive(Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    payload: Option<Payload>,
}

impl Message {
    pub fn new(header: Header, mut payload: Option<Payload>) -> Self {
        Self::encrypt(&header, &mut payload);
        Self::compress(&header, &mut payload);
        Self {
            header: header,
            payload: payload,
        }
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut self.header.raw());
        if self.header.message_flags().payload() {
            vec.append(&mut self.payload.as_ref().unwrap().raw());
        }
        vec
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn payload(&self) -> &Option<Payload> {
        &self.payload
    }

    pub fn decode(&self) -> Vec<u8> {
        match &self.payload {
            Some(v) => self.decompress_and_decrypt(v.clone()),
            None => Vec::new(),
        }
    }

    fn decompress_and_decrypt(&self, mut v: Payload) -> Vec<u8> {
        if v.compressed() {
            v.into_decompressed(self.header().compression_mode())
                .data()
                .to_vec();
        }
        if v.encrypted() {
            v.into_decrypted(self.header().encryption_mode())
                .data()
                .to_vec();
        }
        v.data().to_vec()
    }

    fn encrypt(header: &Header, payload: &mut Option<Payload>) {
        match payload {
            Some(v) => {
                if header.message_flags().encryption() && !v.encrypted() {
                    v.into_encrypted(header.encryption_mode());
                }
            }
            None => return,
        }
    }

    fn compress(header: &Header, payload: &mut Option<Payload>) {
        match payload {
            Some(v) => {
                if header.message_flags().compression() && !v.compressed() {
                    v.into_compressed(header.compression_mode());
                }
            }
            None => return,
        }
    }
}

impl From<&Vec<u8>> for Message {
    fn from(orig: &Vec<u8>) -> Self {
        let header = Header::from(orig);
        let payload = if header.message_flags().payload() {
            let mut orig_payload = Payload::from(&Vec::from(&orig[header.offset()..]));
            orig_payload
                .set_compressed(header.message_flags().compression())
                .set_encrypted(header.message_flags().encryption());
            Some(orig_payload)
        } else {
            None
        };
        Self {
            header: header,
            payload: payload,
        }
    }
}
