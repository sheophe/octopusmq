mod protocol;
use protocol::lamt as lamt;

fn main() {
    println!("hello, octopusmq user or admin or whoever you are...");
    let mut header = lamt::Header::new();
    header
        .set_transport_mode(lamt::TransportMode::Unicast)
        .set_message_type(lamt::MessageType::Publish)
        .set_delivery_mode(lamt::DeliveryMode::ExactlyOnce)
        .set_compression_mode(lamt::CompressionMode::new(lamt::CompressionAlgorithm::Zstd, 15))
        .set_encryption_algo(lamt::EncryptionAlgorithm::AesGCM)
        .set_numeric_topic(1234567890);
    let mut vec: Vec<u8> = Vec::from([0x43, 0x52, 0xde, 0x04]);
    let mut payload = lamt::Payload::new();
    payload
        .append(&mut vec);
    println!("header:  {:02x?}", header.raw());
    println!("payload: {:02x?}", payload.raw());
    let message = lamt::Message::new(header, payload);
    let message_raw = message.raw();
    println!("message: {:02x?}", &message_raw);
    println!("message: {} bytes", message_raw.len());
}
