#[cfg(feature = "std")]
use nson::{decode, encode};
use serde::{Serialize, Deserialize};

#[cfg(feature = "std")]
fn main() {
    use nson::message_id::MessageId;

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
    println!("{}", u8::MAX);
}

#[cfg(not(feature = "std"))]
fn main() {

}

#[derive(Serialize, Deserialize, Debug)]
struct A {
    b: B
}

#[derive(Serialize, Deserialize, Debug)]
struct B(u64);
