#[cfg(any(feature = "std", feature = "serde"))]
use serde::{Serialize, Deserialize};

#[cfg(feature = "std")]
fn main1() {
    use nson::{decode, encode};
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

#[cfg(all(feature = "alloc", feature = "serde", feature = "embedded"))]
fn main2() {
    use nson::embedded::{decode, encode};
    use nson::core::message_id::MessageId;

    let id = MessageId::new_raw(123, 45, 678);

    println!("{:?}", id);
    println!("{:?}", id.timestamp());
    println!("{:?}", id.bytes());

    let a = A { b: B(123) };

    let ret = encode::to_nson::<_, core::convert::Infallible>(&a);
    println!("{:?}", ret);

    let ret = decode::from_nson::<A, core::convert::Infallible>(ret.unwrap());
    println!("{:?}", ret);

    let m = nson::msg! {"a": [123i32, 456f32], "b": "hello"};
    println!("{:?}", m);
    println!("{}", m);
    println!("{}", u8::MAX);
}

fn main() {
    #[cfg(feature = "std")]
    main1();

    #[cfg(all(feature = "alloc", feature = "serde", feature = "embedded"))]
    main2();
}

#[cfg(any(feature = "std", feature = "serde"))]
#[derive(Serialize, Deserialize, Debug)]
struct A {
    b: B
}

#[cfg(any(feature = "std", feature = "serde"))]
#[derive(Serialize, Deserialize, Debug)]
struct B(u64);
