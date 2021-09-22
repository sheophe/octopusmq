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
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn message_serde() {
        // Build header
        let mut header = Header::new();
        header
            .set_transport_mode(TransportMode::Unicast)
            .set_message_type(MessageType::Publish)
            .set_delivery_mode(DeliveryMode::ExactlyOnce)
            .set_message_flags(MessageFlags::with_payload())
            .set_compression_mode(CompressionMode::new(
                CompressionAlgorithm::Brotli,
                CompressionLevel::new(20),
            ))
            .set_encryption_mode(EncryptionMode::new(
                HashAlgorithm::Blake2b,
                AsymEncryptionAlgorithm::Ecdsa,
                SymEncryptionAlgorithm::Twofish,
            ))
            .set_client_id(ClientId::new())
            .set_numeric_topic(0xabcdef12);

        // Build payload
        let range = Uniform::new(0x0, 0xff);
        let mut rng = rand::thread_rng();
        let mut vec: Vec<u8> = (0..255).map(|_| rng.sample(&range)).collect();
        let mut payload = Payload::new();
        payload.append(&mut vec);
        // Construct message
        let message = Message::new(header, Some(payload));
        let message_raw = message.raw();
        // Construct test message
        let testage = Message::from(message_raw.as_ref());
        assert_eq!(message, testage);
    }
}
