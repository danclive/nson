#[cfg(any(feature = "std", feature = "serde"))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
fn main_std() {
    use nson::id::Id;
    use nson::{decode, encode};

    let id = Id::new();

    println!("{:?}", id);
    println!("{:?}", id.timestamp());
    println!("{:?}", id.bytes());

    let a = A { b: B(123) };

    let ret = encode::to_nson(&a);
    println!("{:?}", ret);

    let ret = decode::from_nson::<A>(ret.unwrap());
    println!("{:?}", ret);

    let m = nson::m! {"a": [123i32, 456f32], "b": "hello"};
    println!("{:?}", m);
    println!("{}", m);
    println!("{}", u8::MAX);
}

#[cfg(all(feature = "alloc", feature = "serde"))]
fn main_nostd() {
    use nson::Id;
    use nson::{decode, encode};

    let id = Id::new_raw(123, 45, 678);

    println!("{:?}", id);
    println!("{:?}", id.timestamp());
    println!("{:?}", id.bytes());

    let a = A { b: B(123) };

    let ret = encode::to_nson(&a);
    println!("{:?}", ret);

    let ret = decode::from_nson::<A>(ret.unwrap());
    println!("{:?}", ret);

    let m = nson::m! {"a": [123i32, 456f32], "b": "hello"};
    println!("{:?}", m);
    println!("{}", m);
    println!("{}", u8::MAX);
}

fn main() {
    #[cfg(feature = "std")]
    main_std();

    #[cfg(all(feature = "alloc", feature = "serde"))]
    main_nostd();
}

#[cfg(any(feature = "std", feature = "serde"))]
#[derive(Serialize, Deserialize, Debug)]
struct A {
    b: B,
}

#[cfg(any(feature = "std", feature = "serde"))]
#[derive(Serialize, Deserialize, Debug)]
struct B(u64);
