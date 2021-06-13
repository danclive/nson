use nson::message_id::MessageId;
use nson::{decode, encode};
use serde::{Serialize, Deserialize};

fn main() {
    let id = MessageId::new();

    println!("{:?}", id);
    println!("{:?}", id.timestamp());
    println!("{:?}", id.bytes());

    let a = A { b: B(123) };

    let ret = encode::to_nson(&a);
    println!("{:?}", ret);

    let ret = decode::from_nson::<A>(ret.unwrap());
    println!("{:?}", ret);

    let m = nson::msg! {"a": [123i32, 456f32], "b": "hello"};
    println!("{:?}", m);
    println!("{}", m);
}

#[derive(Serialize, Deserialize, Debug)]
struct A {
    b: B
}

#[derive(Serialize, Deserialize, Debug)]
struct B(u64);
