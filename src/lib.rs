pub use value::{Value, Binary};
pub use message::Message;
pub use array::Array;
pub use message_id::MessageId;

mod macros;
pub mod value;
pub mod message;
pub mod array;
pub mod encode;
pub mod decode;
pub mod serde_impl;
mod spec;
mod json;
pub mod util;
pub mod message_id;

pub const MAX_NSON_SIZE: u32 = 32 * 1024 * 1024; // 32 MB

#[cfg(test)]
mod test {
    use crate::message_id::MessageId;
    use serde::{Serialize, Deserialize};
    use serde_bytes;

    use crate::encode::to_nson;
    use crate::decode::from_nson;
    use crate::msg;
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
        i: MessageId,
        j: Binary
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
            i: MessageId::new(),
            j : vec![5, 6, 7, 8].into()
        };

        let nson = to_nson(&foo).unwrap();

        let foo2: Foo = from_nson(nson).unwrap();

        assert_eq!(foo, foo2);
    }

    #[test]
    fn binary() {
        let byte = vec![1u8, 2, 3, 4];
        let msg = msg!{"aa": "bb", "byte": byte.clone()};
        let byte2 = msg.get_binary("byte").unwrap();

        assert_eq!(byte, byte2.0);

        let mut msg2 = msg!{"aa": "bb"};
        msg2.insert("byte", byte);

        assert_eq!(msg, msg2);
    }
}
