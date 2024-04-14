//! NSON is short for NEW JSON, a binary encoded serialization of JSON-like documents. Similar to JSON, NSON supports embedding maps and arrays within other maps and arrays. Unlike JSON, NSON also includes int32/uint32, int64/uint64, f32/f64, binary, timestamp, id types.
//!
//! NSON borrows from BSON and can be thought of as a streamlined version of BSON, removing some of the less common or mongodb-proprietary types. NSON also categorizes Double into f32 and f64, considering that f64 is not needed in most cases for high-precision floating-point numbers. Also added uint32 and uint64 to make it clear that values cannot be complex.
//!
//! In the rust language, NSON can be easily written without necessarily serializing/unserializing to structures, thanks to the macro.
//!
//! In addition, NSON is convenient to parse from binary, and the library implements "no_std", which can be used on microcontrollers.
//!
//! ## Example
//!
//! ```rust
//! use nson::m;
//!
//! fn main() {
//!     let mut value = m!{
//!         "code": 200,
//!         "success": true,
//!         "payload": {
//!             "some": [
//!                 "pay",
//!                 "loads",
//!             ]
//!         }
//!     };
//!
//!     println!("{:?}", value);
//!     // print: Map{"code": I32(200), "success": Bool(true), "payload":
//!     // Map{"some": Array([String("pay"), String("loads")])}}
//!
//!     println!("{:?}", value.get("code"));
//!     // print: Some(I32(200))
//!
//!     // insert new key, value
//!     value.insert("hello", "world");
//!
//!     println!("{:?}", value.get("hello"));
//!     // print: Some(String("world"))
//! }
//! ```
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("nson requires that either `std` (default) or `alloc` feature is enabled");

#[cfg(all(feature = "std", feature = "embedded"))]
compile_error!("nson requires that either `std` (default) or `embedded` feature don't enabled same time");

extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

pub mod core;
mod macros;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "std")]
pub use value::{Value, Binary, TimeStamp};
#[cfg(feature = "std")]
pub use map::Map;
#[cfg(feature = "std")]
pub use array::Array;
#[cfg(feature = "std")]
pub use id::Id;

#[cfg(feature = "std")]
pub mod value;
#[cfg(feature = "std")]
pub mod map;
#[cfg(feature = "std")]
pub mod array;
#[cfg(feature = "std")]
pub mod id;
#[cfg(feature = "std")]
pub mod encode;
#[cfg(feature = "std")]
pub mod decode;
#[cfg(feature = "std")]
pub mod spec;
#[cfg(feature = "json")]
mod json;

#[cfg(feature = "embedded")]
pub mod embedded;

pub const MAX_NSON_SIZE: u32 = 64 * 1024 * 1024; // 64 MB
pub const MIN_NSON_SIZE: u32 = 4 + 1;

#[cfg(all(test, feature = "std", feature = "serde"))]
mod tests {
    use crate::id::Id;
    use serde::{Serialize, Deserialize};

    use crate::encode::{to_nson, to_bytes};
    use crate::decode::{from_nson, from_bytes};
    use crate::m;
    use crate::value::{TimeStamp, Binary};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct Foo {
        a: i32,
        b: i64,
        c: f64,
        d: String,
        #[serde(with = "serde_bytes")]
        e: Vec<u8>,
        t: TimeStamp,
        i: Id,
        j: Binary,
        k: NewType,
        l: NewType2,
        m: NewType3,
        n: NewType4,
        o: E,
        p: Vec<i32>
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct NewType(u64);

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct NewType2(u32, u64);

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct NewType3 { a: i32, b: i64 }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct NewType4;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum E {
        M(String),
        N(u8),
    }

    #[test]
    fn serialize_and_deserialize() {
        let foo = Foo {
            a: 1,
            b: 2,
            c: 3.0,
            d: "4".to_string(),
            e: vec![1, 2, 3, 4],
            t: TimeStamp(123),
            i: Id::new(),
            j : vec![5, 6, 7, 8].into(),
            k: NewType(123),
            l: NewType2(456, 789),
            m: NewType3 { a: 111, b: 222 },
            n: NewType4,
            o: E::N(123),
            p: vec![111, 222]
        };

        let nson = to_nson(&foo).unwrap();

        let foo2: Foo = from_nson(nson).unwrap();

        assert_eq!(foo, foo2);

        let bytes = to_bytes(&foo).unwrap();

        let foo3: Foo = from_bytes(&bytes).unwrap();

        assert_eq!(foo, foo3);
    }

    #[test]
    fn binary() {
        let byte = vec![1u8, 2, 3, 4];
        let msg = m!{"aa": "bb", "byte": byte.clone()};
        let byte2 = msg.get_binary("byte").unwrap();

        assert_eq!(byte, byte2.0);

        let mut msg2 = m!{"aa": "bb"};
        msg2.insert("byte", byte);

        assert_eq!(msg, msg2);
    }
}
