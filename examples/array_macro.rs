//! Example: Using the a! macro to create arrays
//!
//! This example demonstrates how to use the `a!` macro to create
//! arrays in a convenient and expressive way.

use nson::{Array, Value, a, m};

fn main() {
    println!("=== NSON Array Macro Examples ===\n");

    // 1. Simple arrays
    println!("1. Simple Arrays:");
    let numbers = a![1, 2, 3, 4, 5];
    println!("   Numbers: {:?}", numbers);

    let strings = a!["hello", "world", "nson"];
    println!("   Strings: {:?}", strings);

    // 2. Mixed type arrays
    println!("\n2. Mixed Type Arrays:");
    let mixed = a!["text", 42, true, null, 1.23];
    println!("   Mixed: {:?}", mixed);

    // 3. Empty array
    println!("\n3. Empty Array:");
    let empty = a![];
    println!("   Empty: {:?}", empty);

    // 4. Nested arrays - Auto-detection syntax (recommended)
    println!("\n4. Nested Arrays:");
    let nested = a!["item1", ["nested", "array"], [1, 2, 3], [[1, 2], [3, 4]]];
    println!("   Nested (auto): {:?}", nested);

    // You can also use explicit a! macros, but it's not necessary
    let nested2 = a![
        "item1",
        a!["nested", "array"],
        a![1, 2, 3],
        a![a![1, 2], a![3, 4]]
    ];
    println!("   Nested (explicit): {:?}", nested2);
    println!("   Both equal: {}", nested == nested2);

    // 5. Arrays with maps - Auto-detection syntax
    println!("\n5. Arrays with Maps:");
    let users = a![
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
        {"id": 3, "name": "Charlie"}
    ];
    println!("   Users (auto): {:?}", users);

    // Explicit macro syntax also works
    let users2 = a![
        m! {"id": 1, "name": "Alice"},
        m! {"id": 2, "name": "Bob"},
        m! {"id": 3, "name": "Charlie"}
    ];
    println!("   Users (explicit): {:?}", users2);
    println!("   Both equal: {}", users == users2);

    // 6. Using a! macro in a map
    println!("\n6. Using a! in Maps:");
    let document = m! {
        "title": "Shopping List",
        "items": ["Milk", "Bread", "Eggs"],
        "quantities": [2u8, 1u8, 12u8],
        "prices": [3.5f32, 2.99f32, 4.89f32],
        "tags": ["grocery", "food", "weekly"]
    };
    println!("   Document: {:?}", document);

    // 7. Integer types in arrays
    println!("\n7. Integer Type Arrays:");
    let int8_array = a![1i8, 2i8, 3i8, -1i8, -2i8];
    let uint8_array = a![10u8, 20u8, 30u8, 255u8];
    let int16_array = a![1000i16, 2000i16, -1000i16];
    let uint16_array = a![10000u16, 20000u16, 65535u16];

    println!("   i8:  {:?}", int8_array);
    println!("   u8:  {:?}", uint8_array);
    println!("   i16: {:?}", int16_array);
    println!("   u16: {:?}", uint16_array);

    // 8. Encoding and decoding
    println!("\n8. Encoding and Decoding:");
    let original = a![1, 2, 3, "four", true];
    let bytes = original.to_bytes().unwrap();
    println!("   Original: {:?}", original);
    println!("   Encoded size: {} bytes", bytes.len());

    let decoded = Array::from_bytes(&bytes).unwrap();
    println!("   Decoded: {:?}", decoded);
    println!("   Match: {}", original == decoded);

    // 9. IoT sensor readings example
    println!("\n9. IoT Sensor Readings:");
    let sensor_readings = m! {
        "device_id": "sensor-001",
        "timestamp": 1732694400u64,
        "temperature": [23.5f32, 23.7f32, 23.6f32, 23.8f32],
        "humidity": [65u8, 66u8, 64u8, 67u8],
        "light_level": [450u16, 460u16, 455u16, 465u16],
        "motion_detected": [false, false, true, false]
    };
    println!("   Sensor readings: {:?}", sensor_readings);

    // 10. Comparison with manual construction
    println!("\n10. Macro vs Manual Construction:");

    // Using macro - clean and concise
    let with_macro = a![1, 2, 3];

    // Manual construction - verbose
    let manual = Array::from_vec(vec![Value::I32(1), Value::I32(2), Value::I32(3)]);

    println!("    Macro:  {:?}", with_macro);
    println!("    Manual: {:?}", manual);
    println!("    Equal: {}", with_macro == manual);

    println!("\n=== All examples completed successfully! ===");
}
