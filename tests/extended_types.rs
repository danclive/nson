//! 扩展类型测试 (I8, U8, I16, U16)

use nson::{Array, Map, Value, m};

#[test]
fn test_u8_encode_decode() {
    let original = Value::U8(42);
    let bytes = original.to_bytes().unwrap();
    let decoded = Value::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);
    if let Value::U8(v) = decoded {
        assert_eq!(v, 42);
    } else {
        panic!("Decoded value is not U8");
    }
}

#[test]
fn test_u16_encode_decode() {
    let original = Value::U16(1234);
    let bytes = original.to_bytes().unwrap();
    let decoded = Value::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);
    if let Value::U16(v) = decoded {
        assert_eq!(v, 1234);
    } else {
        panic!("Decoded value is not U16");
    }
}

#[test]
fn test_i8_encode_decode() {
    let original = Value::I8(-42);
    let bytes = original.to_bytes().unwrap();
    let decoded = Value::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);
    if let Value::I8(v) = decoded {
        assert_eq!(v, -42);
    } else {
        panic!("Decoded value is not I8");
    }
}

#[test]
fn test_i16_encode_decode() {
    let original = Value::I16(-1234);
    let bytes = original.to_bytes().unwrap();
    let decoded = Value::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);
    if let Value::I16(v) = decoded {
        assert_eq!(v, -1234);
    } else {
        panic!("Decoded value is not I16");
    }
}

#[test]
fn test_map_with_extended_types() {
    let original = m! {
        "vendor_id": 0x1234u16,
        "product_id": 0x5678u16,
        "level": 128u8,
        "offset": -10i8,
        "temperature": 2350i16,
        "humidity": 65u8,
    };

    let bytes = original.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);

    // 验证各个字段
    assert_eq!(decoded.get_u16("vendor_id").unwrap(), 0x1234);
    assert_eq!(decoded.get_u16("product_id").unwrap(), 0x5678);
    assert_eq!(decoded.get_u8("level").unwrap(), 128);
    assert_eq!(decoded.get_i8("offset").unwrap(), -10);
    assert_eq!(decoded.get_i16("temperature").unwrap(), 2350);
    assert_eq!(decoded.get_u8("humidity").unwrap(), 65);
}

#[test]
fn test_array_with_extended_types() {
    let original = Array::from_vec(vec![
        Value::U8(10),
        Value::U16(1000),
        Value::I8(-5),
        Value::I16(-500),
    ]);

    let bytes = original.to_bytes().unwrap();
    let decoded = Array::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 4);

    assert_eq!(decoded[0].as_u8(), Some(10));
    assert_eq!(decoded[1].as_u16(), Some(1000));
    assert_eq!(decoded[2].as_i8(), Some(-5));
    assert_eq!(decoded[3].as_i16(), Some(-500));
}

#[test]
fn test_boundary_values() {
    let test_cases = vec![
        ("u8_min", Value::U8(u8::MIN)),
        ("u8_max", Value::U8(u8::MAX)),
        ("u16_min", Value::U16(u16::MIN)),
        ("u16_max", Value::U16(u16::MAX)),
        ("i8_min", Value::I8(i8::MIN)),
        ("i8_max", Value::I8(i8::MAX)),
        ("i16_min", Value::I16(i16::MIN)),
        ("i16_max", Value::I16(i16::MAX)),
    ];

    for (name, value) in test_cases {
        let bytes = value.to_bytes().unwrap();
        let decoded = Value::from_bytes(&bytes).unwrap();
        assert_eq!(value, decoded, "Failed for {}", name);
    }
}

#[test]
fn test_mixed_types_in_map() {
    let map = m! {
        "i8": -128i8,
        "u8": 255u8,
        "i16": -32768i16,
        "u16": 65535u16,
        "i32": -2147483648i32,
        "u32": 4294967295u32,
        "i64": -9223372036854775808i64,
        "u64": 18446744073709551615u64,
        "f32": 3.14159f32,
        "f64": 2.718281828f64,
        "bool": true,
        "string": "test",
        "null": Value::Null,
    };

    let bytes = map.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(map, decoded);

    // 验证每种类型
    assert_eq!(decoded.get_i8("i8").unwrap(), -128);
    assert_eq!(decoded.get_u8("u8").unwrap(), 255);
    assert_eq!(decoded.get_i16("i16").unwrap(), -32768);
    assert_eq!(decoded.get_u16("u16").unwrap(), 65535);
    assert_eq!(decoded.get_i32("i32").unwrap(), -2147483648);
    assert_eq!(decoded.get_u32("u32").unwrap(), 4294967295);
    assert_eq!(decoded.get_i64("i64").unwrap(), -9223372036854775808);
    assert_eq!(decoded.get_u64("u64").unwrap(), 18446744073709551615);
    assert_eq!(decoded.get_f32("f32").unwrap(), 3.14159f32);
    assert_eq!(decoded.get_f64("f64").unwrap(), 2.718281828f64);
    assert_eq!(decoded.get_bool("bool").unwrap(), true);
    assert_eq!(decoded.get_str("string").unwrap(), "test");
    assert!(decoded.is_null("null"));
}

#[test]
fn test_nested_structure_with_new_types() {
    let map = m! {
        "device": {
            "id": 42u16,
            "status": 1u8,
            "readings": [
                {
                    "sensor": 0u8,
                    "value": 2350i16,
                },
                {
                    "sensor": 1u8,
                    "value": 1890i16,
                },
            ],
        },
    };

    let bytes = map.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(map, decoded);

    let device = decoded.get_map("device").unwrap();
    assert_eq!(device.get_u16("id").unwrap(), 42);
    assert_eq!(device.get_u8("status").unwrap(), 1);

    let readings = device.get_array("readings").unwrap();
    assert_eq!(readings.len(), 2);
}

#[test]
fn test_type_size() {
    // 验证类型大小
    assert_eq!(Value::I8(0).bytes_size(), 1);
    assert_eq!(Value::U8(0).bytes_size(), 1);
    assert_eq!(Value::I16(0).bytes_size(), 2);
    assert_eq!(Value::U16(0).bytes_size(), 2);
    assert_eq!(Value::I32(0).bytes_size(), 4);
    assert_eq!(Value::U32(0).bytes_size(), 4);
    assert_eq!(Value::I64(0).bytes_size(), 8);
    assert_eq!(Value::U64(0).bytes_size(), 8);
}

#[test]
fn test_value_accessors() {
    let value_i8 = Value::I8(-42);
    let value_u8 = Value::U8(200);
    let value_i16 = Value::I16(-1000);
    let value_u16 = Value::U16(50000);

    assert_eq!(value_i8.as_i8(), Some(-42));
    assert_eq!(value_u8.as_u8(), Some(200));
    assert_eq!(value_i16.as_i16(), Some(-1000));
    assert_eq!(value_u16.as_u16(), Some(50000));

    // 错误类型应该返回 None
    assert_eq!(value_i8.as_u8(), None);
    assert_eq!(value_u16.as_i16(), None);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_with_extended_types() {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct DeviceConfig {
        vendor_id: u16,
        product_id: u16,
        brightness: u8,
        offset: i8,
        temperature: i16,
    }

    let config = DeviceConfig {
        vendor_id: 0x1234,
        product_id: 0x5678,
        brightness: 128,
        offset: -10,
        temperature: 2350,
    };

    let bytes = nson::encode::to_bytes(&config).unwrap();
    let decoded: DeviceConfig = nson::decode::from_bytes(&bytes).unwrap();

    assert_eq!(config, decoded);
}
