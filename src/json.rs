use serde_json::{self, json, Map};
use base64;

use crate::{Value, Message, Array, MessageId};

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::F32(v) => json!(v),
            Value::F64(v) => json!(v),
            Value::I32(v) => json!(v),
            Value::I64(v) => json!(v),
            Value::U32(v) => json!(v),
            Value::U64(v) => json!(v),
            Value::String(v) => json!(v),
            Value::Array(v) => {
                let array: Vec<serde_json::Value> = v.into_iter().map(|v| v.into()).collect();
                json!(array)
            }
            Value::Message(v) => {
                let map: Map<String, serde_json::Value> = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
                json!(map)
            }
            Value::Bool(v) => json!(v),
            Value::Null => json!(null),
            Value::Binary(v) => json!({"$bin": base64::encode(v.0)}),
            Value::TimeStamp(v) => json!({"$tim": v}),
            Value::MessageId(v) => json!({"$mid": v.to_hex()})
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(v) => {
                if let Some(i) = v.as_i64() {
                    if i <= i32::max_value() as i64 {
                        Value::I32(i as i32)
                    } else {
                        Value::I64(i)
                    }
                } else if let Some(u) = v.as_u64() {
                    if u <= u32::max_value() as u64 {
                        Value::U32(u as u32)
                    } else {
                        Value::U64(u)
                    }
                } else if let Some(f) = v.as_f64() {
                    Value::F64(f)
                } else {
                    panic!("Invalid number value: {}", v);
                }
            }
            serde_json::Value::String(v) => v.into(),
            serde_json::Value::Bool(v) => v.into(),
            serde_json::Value::Array(v) => {
                let array: Vec<Value> = v.into_iter().map(|v| v.into()).collect();
                Value::Array(Array::from_vec(array))
            }
            serde_json::Value::Object(v) => {
                let message: Message = v.into_iter().map(|(k, v)| (k, v.into())).collect();

                if message.len() == 1 {
                    if let Ok(timestamp) = message.get_i32("$tim") {
                        return Value::TimeStamp((timestamp as u64).into())
                    } else if let Ok(timestamp) = message.get_u32("$tim") {
                        return Value::TimeStamp((timestamp as u64).into())
                    } else if let Ok(timestamp) = message.get_i64("$tim") {
                        return Value::TimeStamp((timestamp as u64).into())
                    } else if let Ok(timestamp) = message.get_u64("$tim") {
                        return Value::TimeStamp(timestamp.into())
                    } else if let Ok(hex) = message.get_str("$bin") {
                        if let Ok(bin) = base64::decode(hex) {
                            return bin.into()
                        }
                    } else if let Ok(hex) = message.get_str("$mid") {
                        if let Ok(message_id) = MessageId::with_string(hex) {
                            return message_id.into()
                        }
                    }
                }

                Value::Message(message)
            }
            serde_json::Value::Null => Value::Null
        }
    }
}

impl From<Message> for serde_json::Value {
    fn from(message: Message) -> Self {
        Value::Message(message).into()
    }
}

impl From<serde_json::Value> for Message {
    fn from(json: serde_json::Value) -> Self {
        let value: Value = json.into();

        match value {
            Value::Message(message) => message,
            _ => Message::new()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{msg, Value, MessageId};
    use crate::value::TimeStamp;
    use serde_json::{self, json};

    #[test]
    fn convert_json() {
        let json = json!({
            "a": 1i32,
            "b": i64::max_value(),
            "c": 1.1f64,
            "d": std::f64::MAX,
            "e": {
                "$tim": 456
            },
            "f": {
                "$mid": "0171253e54db9aef760d5fbdd048e368"
            },
            "g": {
                "$bin": "AQIDBAUG"
            }
        });

        let value: Value = json.into();

        let message = msg! {
            "a": 1i32,
            "b": i64::max_value(),
            "c": 1.1f64,
            "d": std::f64::MAX,
            "e": TimeStamp(456),
            "f": MessageId::with_string("0171253e54db9aef760d5fbdd048e368").unwrap(),
            "g": vec![1u8, 2, 3, 4, 5, 6]

        };

        let value2: Value = message.into();

        assert!(value == value2);
    }
}
