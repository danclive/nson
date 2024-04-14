use alloc::vec::Vec;
use alloc::string::String;

use serde_json::{self, json, Map};

use base64::{Engine, engine::general_purpose};

use crate::core::{Value, Map as NsonMap, Array, Id};

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::F32(v) => json!(v),
            Value::F64(v) => json!({"$f64": v}),
            Value::I32(v) => json!(v),
            Value::I64(v) => json!({"$i64": v}),
            Value::U32(v) => json!({"$u32": v}),
            Value::U64(v) => json!({"$u64": v}),
            Value::String(v) => json!(v),
            Value::Array(v) => {
                let array: Vec<serde_json::Value> = v.into_iter().map(|v| v.into()).collect();
                json!(array)
            }
            Value::Map(v) => {
                let map: Map<String, serde_json::Value> = v.into_iter().map(|(k, v)| (k, v.into())).collect();
                json!(map)
            }
            Value::Bool(v) => json!(v),
            Value::Null => json!(null),
            Value::Binary(v) => json!({"$bin": general_purpose::STANDARD.encode(v.0)}),
            Value::TimeStamp(v) => json!({"$tim": v.0}),
            Value::Id(v) => json!({"$mid": v.to_hex()})
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(v) => {
                if let Some(i) = v.as_i64() {
                    Value::I32(i as i32)
                } else if let Some(u) = v.as_u64() {
                    Value::I32(u as i32)
                } else if let Some(f) = v.as_f64() {
                    Value::F32(f as f32)
                } else {
                   unreachable!()
                }
            }
            serde_json::Value::String(v) => v.into(),
            serde_json::Value::Bool(v) => v.into(),
            serde_json::Value::Array(v) => {
                let array: Vec<Value> = v.into_iter().map(|v| v.into()).collect();
                Value::Array(Array::from_vec(array))
            }
            serde_json::Value::Object(map) => {
                if map.len() == 1 {
                    let keys: Vec<_> = map.keys().map(|s| s.as_str()).collect();

                    match keys.as_slice() {
                        ["$tim"] => {
                            if let Some(v) = map.get("$tim") {
                                if let Some(u) = v.as_u64() {
                                    return Value::TimeStamp(u.into())
                                }
                            }
                        }
                        ["$bin"] => {
                            if let Some(v) = map.get("$bin") {
                                if let Some(hex) = v.as_str() {
                                    if let Ok(bin) = general_purpose::STANDARD.decode(hex) {
                                        return bin.into()
                                    }
                                }
                            }
                        }
                        ["$mid"] => {
                            if let Some(v) = map.get("$mid") {
                                if let Some(hex) = v.as_str() {
                                    if let Ok(message_id) = Id::with_string(hex) {
                                        return message_id.into()
                                    }
                                }
                            }
                        }
                        ["$f64"] => {
                            if let Some(v) = map.get("$f64") {
                                if let Some(f) = v.as_f64() {
                                    return Value::F64(f)
                                }
                            }
                        }
                        ["$i64"] => {
                            if let Some(v) = map.get("$i64") {
                                if let Some(i) = v.as_i64() {
                                    return Value::I64(i)
                                }
                            }
                        }
                        ["$u32"] => {
                            if let Some(v) = map.get("$u32") {
                                if let Some(u) = v.as_u64() {
                                    return Value::U32(u as u32)
                                }
                            }
                        }
                        ["$u64"] => {
                            if let Some(v) = map.get("$u64") {
                                if let Some(u) = v.as_u64() {
                                    return Value::U64(u)
                                }
                            }
                        }
                        _ => ()
                    }
                }

                let map: NsonMap = map.into_iter().map(|(k, v)| (k, v.into())).collect();

                Value::Map(map)
            }
            serde_json::Value::Null => Value::Null
        }
    }
}

impl From<NsonMap> for serde_json::Value {
    fn from(map: NsonMap) -> Self {
        Value::Map(map).into()
    }
}

impl From<serde_json::Value> for NsonMap {
    fn from(json: serde_json::Value) -> Self {
        let value: Value = json.into();

        match value {
            Value::Map(map) => map,
            _ => NsonMap::new()
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::{msg, Value, MessageId};
    use crate::value::TimeStamp;
    use serde_json::{self, json};

    #[test]
    fn convert_json() {
        let json = json!({
            "a": 1i32,
            "b": {"$i64": 2i64},
            "c": {"$u32": 3u32},
            "d": {"$u64": 4u64},
            "e": 5.6f32,
            "f": {"$f64": 7.8f64},
            "g": {
                "$tim": 456
            },
            "h": {
                "$mid": "0171253e54db9aef760d5fbd"
            },
            "i": {
                "$bin": "AQIDBAUG"
            }
        });

        let message = msg!{
            "a": 1i32,
            "b": 2i64,
            "c": 3u32,
            "d": 4u64,
            "e": 5.6f32,
            "f": 7.8f64,
            "g": TimeStamp(456),
            "h": MessageId::with_string("0171253e54db9aef760d5fbd").unwrap(),
            "i": vec![1u8, 2, 3, 4, 5, 6]
        };

        let nson_value: Value = message.clone().into();

        let value: serde_json::Value = message.into();

        assert!(json == value);

        let value2: Value = json.into();

        assert!(nson_value == value2);
    }
}
