mod client_id;
mod compression;
mod compression_mode;
mod delivery_mode;
mod encryption;
mod encryption_mode;
mod header;
mod message;
mod message_flags;
mod message_type;
mod payload;
mod protocol_version;
mod topic;
mod transport_mode;

pub use client_id::*;
pub use compression::*;
pub use compression_mode::*;
pub use delivery_mode::*;
pub use encryption::*;
pub use encryption_mode::*;
pub use header::*;
pub use message::*;
pub use message_flags::*;
pub use message_type::*;
pub use payload::*;
pub use protocol_version::*;
pub use topic::*;
pub use transport_mode::*;

#[cfg(test)]
mod tests {
    use crate::lamt;

    #[test]
    fn message_serde() {
        // Build header
        let mut header = lamt::Header::new();
        header
            .set_transport_mode(lamt::TransportMode::Unicast)
            .set_message_type(lamt::MessageType::Publish)
            .set_delivery_mode(lamt::DeliveryMode::ExactlyOnce)
            .set_message_flags(lamt::MessageFlags::with_payload())
            .set_compression_mode(lamt::CompressionMode::new(
                lamt::CompressionAlgorithm::Brotli,
                lamt::CompressionLevel::new(20),
            ))
            .set_encryption_mode(lamt::EncryptionMode::new(
                lamt::HashAlgorithm::Blake2b,
                lamt::AsymEncryptionAlgorithm::Ecdsa,
                lamt::SymEncryptionAlgorithm::Twofish,
            ))
            .set_client_id(lamt::ClientId::new())
            .set_numeric_topic(0xabcdef12);

        // Build payload
        let mut vec: Vec<u8> = Vec::from([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x51, 0x52,
            0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
            0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
        ]);
        let mut payload = lamt::Payload::new();
        payload.append(&mut vec);
        // Construct message
        let message = lamt::Message::new(header, Some(payload));
        let message_raw = message.raw();
        // Construct test message
        let testage = lamt::Message::from(&message_raw);
        assert_eq!(message, testage);
    }
}
