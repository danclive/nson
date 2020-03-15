use nson::{msg, Value, MessageId};
use nson::value::TimeStamp;

#[test]
fn size_bytes() {
    let msg = msg!{
        "a": 1.0f32,
        "b": 2.1f64,
        "c": 3i32,
        "d": 4i64,
        "e": 5u32,
        "f": 6u64,
        "g": "hello",
        "h": vec![1i32, 2i32],
        "i": {
                "j": 7i32
            },
        "k": true,
        "l": Value::Null,
        "m": vec![0u8, 1u8, 2u8, 4u8],
        "n": TimeStamp(1),
        "o": MessageId::new()

    };

    assert_eq!(msg.to_vec().unwrap().len(), msg.bytes_size());
}
