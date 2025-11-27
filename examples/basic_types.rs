//! 基本类型使用示例
//!
//! 演示 NSON 支持的所有数字类型

use nson::{Array, Map, Value, m};

fn main() {
    println!("=== NSON 基本类型示例 ===\n");

    // 1. 所有数字类型
    let numbers = m! {
        "i8": -128i8,           // 8位有符号整数
        "u8": 255u8,            // 8位无符号整数
        "i16": -32768i16,       // 16位有符号整数
        "u16": 65535u16,        // 16位无符号整数
        "i32": -2147483648i32,  // 32位有符号整数
        "u32": 4294967295u32,   // 32位无符号整数
        "i64": -9223372036854775808i64,  // 64位有符号整数
        "u64": 18446744073709551615u64,  // 64位无符号整数
        "f32": 3.14159f32,      // 32位浮点数
        "f64": 2.718281828459045f64,  // 64位浮点数
    };

    println!("数字类型 Map:");
    println!("{}\n", numbers);

    // 2. 编码和解码
    let bytes = numbers.to_bytes().unwrap();
    println!("编码后大小: {} 字节", bytes.len());

    let decoded = Map::from_bytes(&bytes).unwrap();
    println!("解码成功: {:?}\n", decoded == numbers);

    // 3. 类型访问
    println!("访问各种类型:");
    println!("  i8 value: {:?}", decoded.get_i8("i8"));
    println!("  u8 value: {:?}", decoded.get_u8("u8"));
    println!("  i16 value: {:?}", decoded.get_i16("i16"));
    println!("  u16 value: {:?}", decoded.get_u16("u16"));
    println!("  i32 value: {:?}", decoded.get_i32("i32"));
    println!("  u32 value: {:?}", decoded.get_u32("u32"));
    println!("  i64 value: {:?}", decoded.get_i64("i64"));
    println!("  u64 value: {:?}", decoded.get_u64("u64"));
    println!("  f32 value: {:?}", decoded.get_f32("f32"));
    println!("  f64 value: {:?}", decoded.get_f64("f64"));

    println!("\n=== 数组中的类型 ===\n");

    // 4. 数组中使用不同类型
    let mixed_array = Array::from_vec(vec![
        Value::I8(-10),
        Value::U8(200),
        Value::I16(-1000),
        Value::U16(50000),
        Value::F32(1.5),
        Value::from("text"),
    ]);

    println!("混合类型数组: {:?}", mixed_array);

    let array_bytes = mixed_array.to_bytes().unwrap();
    let decoded_array = Array::from_bytes(&array_bytes).unwrap();
    println!("数组解码成功: {:?}\n", decoded_array == mixed_array);

    // 5. 类型大小比较
    println!("=== 类型存储大小 ===");
    let size_demo = m! {
        "i8": -1i8,
        "i16": -1i16,
        "i32": -1i32,
        "i64": -1i64,
    };

    for (key, value) in size_demo.iter() {
        println!("{}: {} 字节 + 1字节类型标记", key, value.bytes_size());
    }
}
