use nson::message_id::MessageId;

fn main() {
    let id = MessageId::new();

    println!("{:?}", id);
    println!("{:?}", id.timestamp());
    println!("{:?}", id.counter());
    println!("{:?}", id.identify());
    println!("{:?}", id.bytes());
}
