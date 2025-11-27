//! 综合集成测试

use nson::{Array, Id, Map, TimeStamp, Value, m};

#[test]
fn test_complete_encode_decode() {
    let mid = Id::new();

    let original = m! {
        "a": 123.123f32,
        "b": 456.456f64,
        "c": {
            "d": 789.789f64,
        },
        "e": 1i32,
        "f": 2i64,
        "g": 3u32,
        "h": 4u64,
        "i": "aaa",
        "j": Array::from_vec(vec![Value::F32(666.777), Value::from("hello")]),
        "k": false,
        "l": Value::Null,
        "m": vec![1u8, 2, 3, 4, 5, 6],
        "n": TimeStamp(12345),
        "p": mid.clone(),
        // 新类型
        "q": 10i8,
        "r": 200u8,
        "s": -1000i16,
        "t": 50000u16,
    };

    let bytes = original.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(original, decoded);

    // 验证特定字段
    assert_eq!(decoded.get_f32("a").unwrap(), 123.123f32);
    assert_eq!(decoded.get_f64("b").unwrap(), 456.456f64);
    assert_eq!(decoded.get_i32("e").unwrap(), 1);
    assert_eq!(decoded.get_i64("f").unwrap(), 2);
    assert_eq!(decoded.get_u32("g").unwrap(), 3);
    assert_eq!(decoded.get_u64("h").unwrap(), 4);
    assert_eq!(decoded.get_str("i").unwrap(), "aaa");
    assert_eq!(decoded.get_bool("k").unwrap(), false);
    assert!(decoded.is_null("l"));
    assert_eq!(decoded.get_timestamp("n").unwrap(), &TimeStamp(12345));
    assert_eq!(decoded.get_id("p").unwrap(), &mid);
    // 新类型
    assert_eq!(decoded.get_i8("q").unwrap(), 10);
    assert_eq!(decoded.get_u8("r").unwrap(), 200);
    assert_eq!(decoded.get_i16("s").unwrap(), -1000);
    assert_eq!(decoded.get_u16("t").unwrap(), 50000);
}

#[test]
fn test_nested_structures() {
    let data = m! {
        "level1": {
            "level2": {
                "level3": {
                    "value": 42u16,
                    "name": "deep",
                },
            },
        },
        "array": [
            {"id": 1u8, "name": "first"},
            {"id": 2u8, "name": "second"},
            {"id": 3u8, "name": "third"},
        ],
    };

    let bytes = data.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(data, decoded);

    let level1 = decoded.get_map("level1").unwrap();
    let level2 = level1.get_map("level2").unwrap();
    let level3 = level2.get_map("level3").unwrap();

    assert_eq!(level3.get_u16("value").unwrap(), 42);
    assert_eq!(level3.get_str("name").unwrap(), "deep");

    let array = decoded.get_array("array").unwrap();
    assert_eq!(array.len(), 3);
}

#[test]
fn test_large_dataset() {
    let mut map = Map::new();

    // 创建1000个条目
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = m! {
            "index": i as u16,
            "value": (i * 2) as i16,
            "flag": i % 2 == 0,
        };
        map.insert(key, value);
    }

    let bytes = map.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(map.len(), decoded.len());
    assert_eq!(map, decoded);

    // 验证部分数据
    let key_500 = decoded.get_map("key_500").unwrap();
    assert_eq!(key_500.get_u16("index").unwrap(), 500);
    assert_eq!(key_500.get_i16("value").unwrap(), 1000);
    assert_eq!(key_500.get_bool("flag").unwrap(), true);
}

#[test]
fn test_empty_structures() {
    let empty_map = Map::new();
    let empty_array = Array::new();

    let map_bytes = empty_map.to_bytes().unwrap();
    let array_bytes = empty_array.to_bytes().unwrap();

    let decoded_map = Map::from_bytes(&map_bytes).unwrap();
    let decoded_array = Array::from_bytes(&array_bytes).unwrap();

    assert_eq!(empty_map, decoded_map);
    assert_eq!(empty_array, decoded_array);
    assert!(decoded_map.is_empty());
    assert!(decoded_array.is_empty());
}

#[test]
fn test_all_types_in_array() {
    let array = Array::from_vec(vec![
        Value::I8(-128),
        Value::U8(255),
        Value::I16(-32768),
        Value::U16(65535),
        Value::I32(-2147483648),
        Value::U32(4294967295),
        Value::I64(-9223372036854775808),
        Value::U64(18446744073709551615),
        Value::F32(3.14),
        Value::F64(2.718),
        Value::from("string"),
        Value::Bool(true),
        Value::Null,
        Value::from(vec![1u8, 2, 3]),
        Value::TimeStamp(TimeStamp(999)),
        Value::Id(Id::new()),
    ]);

    let bytes = array.to_bytes().unwrap();
    let decoded = Array::from_bytes(&bytes).unwrap();

    assert_eq!(array, decoded);
    assert_eq!(decoded.len(), 16);
}

#[test]
fn test_unicode_strings() {
    let map = m! {
        "chinese": "你好世界",
        "emoji": "😀🎉🚀",
        "mixed": "Hello 世界 🌍",
        "arabic": "مرحبا",
        "russian": "Привет",
    };

    let bytes = map.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    assert_eq!(map, decoded);
    assert_eq!(decoded.get_str("chinese").unwrap(), "你好世界");
    assert_eq!(decoded.get_str("emoji").unwrap(), "😀🎉🚀");
    assert_eq!(decoded.get_str("mixed").unwrap(), "Hello 世界 🌍");
}

#[test]
fn test_binary_data() {
    let data: Vec<u8> = (0..=255).collect();

    let map = m! {
        "data": data.clone(),
        "length": data.len() as u16,
    };

    let bytes = map.to_bytes().unwrap();
    let decoded = Map::from_bytes(&bytes).unwrap();

    let decoded_data = decoded.get_binary("data").unwrap();
    assert_eq!(decoded_data.0, data);
    assert_eq!(decoded.get_u16("length").unwrap(), 256);
}
