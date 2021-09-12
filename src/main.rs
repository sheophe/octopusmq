mod protocol;

use std::str;
use std::ptr;

use protocol::lamt::Header as Header;
use protocol::lamt::PublishSettings as PublishSettings;
use protocol::lamt::DeliveryMode as DeliveryMode;
use protocol::lamt::TransportMode as TransportMode;
use protocol::lamt::MessageType as MessageType;

fn main() {
    let header = PublishSettings{
        DeliveryMode::PublishAndForget,
        TransportMode::Multicast,
        MessageType::Publish,
        ptr::null(),
        ptr::null()
    };
    println!("hello, octopusmq user or admin or whoever you are...");
    println!("topic: {:#?}", str::from_utf8(header.raw().as_slice()))
}
