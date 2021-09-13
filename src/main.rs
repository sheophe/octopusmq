mod protocol;
use protocol::lamt as lamt;

fn main() {
    println!("hello, octopusmq user or admin or whoever you are...");

    // Build header
    let mut header = lamt::Header::new();
    header
        .set_transport_mode(lamt::TransportMode::Unicast)
        .set_message_type(lamt::MessageType::Publish)
        .set_delivery_mode(lamt::DeliveryMode::ExactlyOnce)
        .set_message_flags(lamt::MessageFlags::with_payload())
        .set_compression_mode(lamt::CompressionMode::new_brotli())
        .set_client_id(lamt::ClientId::new())
        .set_numeric_topic(0xffffffff);

    // Build payload
    let mut vec: Vec<u8> = Vec::from([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    let mut payload = lamt::Payload::new();
    payload
        .append(&mut vec)
        .into_compressed(header.compression_mode());

    // Construct message
    let message = lamt::Message::new(header, Some(payload));
    let message_raw = message.raw();
    println!("message: {:02x?}", &message_raw);
    println!("message: {} bytes", &message_raw.len());

    // Construct test message
    let test_message = lamt::Message::from(&message_raw);
    let test_message_raw = test_message.raw();
    println!("testage: {:02x?}", &test_message_raw);
    println!("testage: {} bytes", &test_message_raw.len());

    println!("equal: {}", message == test_message);
}
