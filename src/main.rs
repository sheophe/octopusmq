mod protocol;
use protocol::lamt as lamt;

fn main() {
    println!("hello, octopusmq user or admin or whoever you are...");
    let mut header = lamt::Header::new();
    header
        .set_transport_mode(lamt::TransportMode::Unicast)
        .set_message_type(lamt::MessageType::Publish)
        .set_delivery_mode(lamt::DeliveryMode::ExactlyOnce)
        .set_message_flags(lamt::MessageFlags::with_payload())
        .set_compression_mode(lamt::CompressionMode::new(lamt::CompressionAlgorithm::Deflate, 6))
        .set_encryption_algo(lamt::EncryptionAlgorithm::AesGCM)
        .set_numeric_topic(0xffffffff);
    let mut vec: Vec<u8> = Vec::from(
        [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
    );
    let mut payload = lamt::Payload::new();
    payload
        .append(&mut vec)
        .into_compressed(header.get_compression_mode());
    let message = lamt::Message::new(header, Some(payload));
    let message_raw = message.raw();
    println!("message: {:02x?}", &message_raw);
    println!("message: {} bytes", message_raw.len());
}
