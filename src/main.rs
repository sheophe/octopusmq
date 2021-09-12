mod protocol;
use protocol::lamt as lamt;

fn main() {
    let mut header = lamt::Header::new();
    let modified_header = header
        .set_transport_mode(lamt::TransportMode::Unicast)
        .set_numeric_topic(4)
        .set_message_type(lamt::MessageType::Publish);
    println!("hello, octopusmq user or admin or whoever you are...");
    println!("topic: {:?}", modified_header.raw())
}
