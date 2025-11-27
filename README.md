# nson

[![crates.io](https://img.shields.io/crates/v/nson.svg)](https://crates.io/crates/nson)
[![docs.rs](https://docs.rs/mongodb/badge.svg)](https://docs.rs/nson)
[![crates.io](https://img.shields.io/crates/l/nson.svg)](https://crates.io/crates/nson)

NSON is short for NEW JSON, a binary encoded serialization of JSON-like documents. Similar to JSON, NSON supports embedding maps and arrays within other maps and arrays. Unlike JSON, NSON also includes comprehensive integer types (i8/u8, i16/u16, i32/u32, i64/u64), floating-point types (f32/f64), binary, timestamp, and id types.

NSON borrows from BSON and can be thought of as a streamlined version of BSON, removing some of the less common or mongodb-proprietary types. NSON provides fine-grained integer types (8-bit, 16-bit, 32-bit, and 64-bit, both signed and unsigned) to optimize storage space and bandwidth usage - especially useful for IoT devices and embedded systems.

Key features:
- 🚀 **Type-rich**: Comprehensive integer types from 8-bit to 64-bit
- 📦 **Space-efficient**: Choose the smallest type for your data range
- 🎯 **Type-safe**: Type-safe getter methods for all types
- 🔧 **Convenient**: Easy-to-use macros for JSON-like syntax
- 🪶 **Lightweight**: `no_std` support for embedded systems
- ⚡ **Fast**: Zero-copy parsing and efficient encoding

## Table of Contents

- [Quick Start](#quick-start)
- [Macros](#macros)
- [Data Types](#data-types)
- [Usage Examples](#usage-examples)
- [Type Selection Guide](#type-selection-guide)
- [IoT Applications](#iot-applications)
- [API Reference](#api-reference)
- [Performance Tips](#performance-tips)
- [Testing](#testing)

## Quick Start

```rust
use nson::{m, a};

fn main() {
    // Create a map with nested structures
    let mut document = m!{
        "code": 200,
        "success": true,
        "payload": {
            "items": ["apple", "banana", "orange"],
            "count": 3u8
        }
    };

    println!("{:?}", document);
    // Map{"code": I32(200), "success": Bool(true), "payload": Map{...}}

    // Type-safe access
    let code = document.get_i32("code").unwrap();
    println!("Status code: {}", code);

    // Add new data
    document.insert("tags", ["rust", "nson", "binary"]);

    // Encode to bytes
    let bytes = document.to_bytes().unwrap();

    // Decode from bytes
    let decoded = nson::Map::from_bytes(&bytes).unwrap();
    assert_eq!(document, decoded);
}
```

## Macros

NSON provides convenient macros for creating data structures with JSON-like syntax:

### Basic Macro Usage

```rust
use nson::{m, a};

// Create a Map
let config = m! {
    "name": "device1",
    "enabled": true,
    "count": 42
};

// Create an Array
let numbers = a![1, 2, 3, 4, 5];
let mixed = a!["hello", 42, true, null];
```

### Auto-Detection Feature

**Key feature**: Macros automatically detect nested `[...]` and `{...}` syntax, so you don't need to explicitly nest macros!

```rust
use nson::m;

// ✅ Recommended: Simple and clean (auto-detection)
let document = m! {
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ],
    "settings": {
        "theme": "dark",
        "notifications": true
    }
};

// Also works: Explicit macro usage
use nson::a;
let document2 = m! {
    "users": a![
        m!{"id": 1, "name": "Alice"},
        m!{"id": 2, "name": "Bob"}
    ],
    "settings": m!{
        "theme": "dark",
        "notifications": true
    }
};

// Both are equivalent!
assert_eq!(document, document2);
```

The auto-detection works at all nesting levels:

```rust
// Complex nested structure - all automatic!
let gateway = m! {
    "name": "IoT Gateway",
    "devices": [
        {
            "id": 1,
            "readings": [23.5, 24.1, 23.8],
            "metadata": {
                "location": "room1",
                "active": true
            }
        }
    ]
};
```

## Data Types

NSON supports a comprehensive set of data types optimized for different use cases:

| Type | Rust Type | Bytes | Range | Use Cases |
|------|-----------|-------|-------|-----------|
| **I8** | `i8` | 1 | -128 to 127 | Temperature offset, signal strength |
| **U8** | `u8` | 1 | 0 to 255 | Percentages, brightness, small counts |
| **I16** | `i16` | 2 | -32,768 to 32,767 | Temperature×100, coordinates |
| **U16** | `u16` | 2 | 0 to 65,535 | Port numbers, product IDs, color temp |
| **I32** | `i32` | 4 | -2³¹ to 2³¹-1 | Standard integers, counters |
| **U32** | `u32` | 4 | 0 to 2³²-1 | Unsigned counts, IDs |
| **I64** | `i64` | 8 | -2⁶³ to 2⁶³-1 | Large integers, Unix timestamps |
| **U64** | `u64` | 8 | 0 to 2⁶⁴-1 | Large unsigned numbers |
| **F32** | `f32` | 4 | IEEE 754 | General floating-point |
| **F64** | `f64` | 8 | IEEE 754 | High-precision floating-point |
| **Bool** | `bool` | 1 | true/false | Boolean values |
| **String** | `String` | 4+len | UTF-8 | Text data |
| **Binary** | `Vec<u8>` | 4+len | Byte array | Binary data |
| **Array** | `Array` | 4+data+1 | Ordered list | Collections |
| **Map** | `Map` | 4+data+1 | Key-value pairs | Objects/dictionaries |
| **TimeStamp** | `TimeStamp` | 8 | Unix timestamp | Timestamps |
| **Id** | `Id` | 12 | 12-byte ID | Unique identifiers |
| **Null** | - | 0 | null | Null value |

## Usage Examples

### Working with Different Integer Types

```rust
use nson::m;

// Use appropriate integer types to save space
let device_config = m! {
    "vendor_id": 0x1234u16,      // 16-bit is enough for vendor ID
    "product_id": 0x5678u16,     // 16-bit for product ID
    "brightness": 128u8,         // 8-bit for 0-255 range
    "temperature": 2350i16,      // 16-bit for temperature × 100 (23.50°C)
    "humidity": 65u8,            // 8-bit for percentage
    "offset": -10i8,             // 8-bit for small offsets
};

// Type-safe access with getter methods
let brightness = device_config.get_u8("brightness").unwrap();
let temp = device_config.get_i16("temperature").unwrap();
println!("Brightness: {}, Temperature: {:.2}°C", brightness, temp as f32 / 100.0);
```

### Encoding and Decoding

```rust
use nson::Map;

// Create data
let data = m! {
    "name": "sensor-01",
    "value": 42u8,
    "enabled": true,
};

// Encode to bytes
let bytes = data.to_bytes().unwrap();
println!("Encoded size: {} bytes", bytes.len());

// Decode from bytes
let decoded = Map::from_bytes(&bytes).unwrap();
assert_eq!(data, decoded);

// Access values
let value = decoded.get_u8("value").unwrap();
let name = decoded.get_str("name").unwrap();
```

### Using Serde

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DeviceConfig {
    vendor_id: u16,
    product_id: u16,
    brightness: u8,
    temperature: i16,
    enabled: bool,
}

let config = DeviceConfig {
    vendor_id: 0x1234,
    product_id: 0x5678,
    brightness: 128,
    temperature: 2350,
    enabled: true,
};

// Serialize
let bytes = nson::encode::to_bytes(&config).unwrap();

// Deserialize
let decoded: DeviceConfig = nson::decode::from_bytes(&bytes).unwrap();
assert_eq!(config.vendor_id, decoded.vendor_id);
```

## Type Selection Guide

### Choose the Smallest Appropriate Type

Choosing the right data type significantly reduces storage space and bandwidth:

```rust
use nson::m;

// ✅ Good: Using appropriate types
let efficient = m! {
    "age": 25u8,              // 0-255 range
    "percentage": 75u8,       // 0-100 range
    "port": 8080u16,          // 0-65535 range
    "temperature": 2350i16,   // Temperature × 100
};

// ❌ Bad: Using oversized types
let inefficient = m! {
    "age": 25u32,             // Wastes 3 bytes
    "percentage": 75u32,      // Wastes 3 bytes
    "port": 8080u32,          // Wastes 2 bytes
    "temperature": 23.5f32,   // Wastes space
};

println!("Efficient: {} bytes", efficient.to_bytes().unwrap().len());
println!("Inefficient: {} bytes", inefficient.to_bytes().unwrap().len());
// Output: Efficient: ~20 bytes, Inefficient: ~35 bytes (57% larger!)
```

### Type Selection Rules

| Data Range | Recommended Type | Examples |
|------------|------------------|----------|
| 0-100 | `u8` | Percentages, battery level |
| 0-255 | `u8` | RGB colors, brightness |
| -100 to +100 | `i8` | Small offsets, signal strength (dBm) |
| 0-1,000 | `u16` | Small port numbers, small IDs |
| -1,000 to +1,000 | `i16` | Fixed-point × 100 |
| 0-65,535 | `u16` | Port numbers, product IDs |
| Other small integers | `i32`/`u32` | Standard integers |
| Large numbers | `i64`/`u64` | Timestamps, large counts |

### Fixed-Point vs Floating-Point

For fixed-precision values (temperature, prices), use fixed-point arithmetic:

```rust
// ✅ Fixed-point: 2 bytes, exact
let temp_fixed = m! {
    "temperature": 2345i16,  // 23.45°C (value × 100)
};

// ❌ Floating-point: 4 bytes, precision issues
let temp_float = m! {
    "temperature": 23.45f32,
};

// Helper functions for fixed-point
fn celsius_to_i16(celsius: f64) -> i16 {
    (celsius * 100.0) as i16
}

fn i16_to_celsius(value: i16) -> f64 {
    value as f64 / 100.0
}
```

## IoT Applications

### Temperature and Humidity Sensor

```rust
use nson::{m, Id, TimeStamp};

let sensor_reading = m! {
    "device_id": Id::new(),
    "timestamp": TimeStamp::from(1732694400u64),
    "temperature": 2345i16,      // 23.45°C (× 100)
    "humidity": 65u8,            // 65%
    "battery": 87u8,             // 87%
    "signal_strength": -45i8,    // -45 dBm
};

// Encode for transmission
let bytes = sensor_reading.to_bytes().unwrap();
println!("Packet size: {} bytes", bytes.len());  // Very compact!

// Decode on receiver
let decoded = nson::Map::from_bytes(&bytes).unwrap();
let temp = decoded.get_i16("temperature").unwrap() as f32 / 100.0;
let humidity = decoded.get_u8("humidity").unwrap();
println!("Temp: {:.2}°C, Humidity: {}%", temp, humidity);
```

### Smart Light Control

```rust
use nson::m;

let light_command = m! {
    "device_id": 42u16,
    "command": "set_state",
    "on": true,
    "brightness": 192u8,       // 0-255
    "color_temp": 4000u16,     // 2700-6500K
    "transition": 500u16,      // 500ms transition
};

let bytes = light_command.to_bytes().unwrap();
println!("Command size: {} bytes", bytes.len());
```

### Matter Protocol Device Attributes

```rust
use nson::m;

let device_attrs = m! {
    "VendorID": 0x1234u16,
    "ProductID": 0x5678u16,
    "HardwareVersion": 2u8,
    "SoftwareVersion": 0x00020100u32,  // v2.1.0
    "OnOff": true,
    "CurrentLevel": 128u8,              // 0-254
    "ColorTemperatureMireds": 250u16,
};
```

### Batch Processing

```rust
use nson::{a, m};

// Collect multiple sensor readings
let batch = a![
    {"id": 1, "temp": 2345i16, "humidity": 65u8},
    {"id": 2, "temp": 2367i16, "humidity": 68u8},
    {"id": 3, "temp": 2312i16, "humidity": 62u8},
];

let bytes = batch.to_bytes().unwrap();
println!("Batch of 3 readings: {} bytes", bytes.len());
```

## API Reference

### Map Methods

```rust
use nson::Map;

// Create
let mut map = Map::new();
let map = Map::with_capacity(10);  // Pre-allocate

// Insert
map.insert("key", value);
map.insert("number", 42u8);

// Get (generic)
let value = map.get("key");         // Option<&Value>

// Type-safe getters
let num = map.get_u8("num").unwrap();       // u8
let num = map.get_i8("num").unwrap();       // i8
let num = map.get_u16("num").unwrap();      // u16
let num = map.get_i16("num").unwrap();      // i16
let num = map.get_u32("num").unwrap();      // u32
let num = map.get_i32("num").unwrap();      // i32
let num = map.get_u64("num").unwrap();      // u64
let num = map.get_i64("num").unwrap();      // i64
let num = map.get_f32("num").unwrap();      // f32
let num = map.get_f64("num").unwrap();      // f64
let text = map.get_str("text").unwrap();    // &str
let flag = map.get_bool("flag").unwrap();   // bool
let bin = map.get_binary("data").unwrap();  // &Binary

// Check
map.contains_key("key");            // bool
map.is_null("key");                 // bool
map.len();                          // usize
map.is_empty();                     // bool

// Iterate
for (key, value) in &map { }
for key in map.keys() { }
for value in map.values() { }

// Encode/Decode
let bytes = map.to_bytes().unwrap();
let map = Map::from_bytes(&bytes).unwrap();
```

### Array Methods

```rust
use nson::Array;

// Create
let arr = Array::new();
let arr = Array::from_vec(vec![value1, value2]);

// Access
let value = arr.get(0);             // Option<&Value>
let len = arr.len();

// Iterate
for value in &arr { }

// Encode/Decode
let bytes = arr.to_bytes().unwrap();
let arr = Array::from_bytes(&bytes).unwrap();
```

## Performance Tips

1. **Pre-allocate capacity**: Use `Map::with_capacity(n)` when you know the size

```rust
let mut map = Map::with_capacity(100);  // Avoid reallocations
```

2. **Use smaller types**: `u8` instead of `u32` saves 75% space

```rust
m! { "value": 100u8 }  // 1 byte vs 4 bytes
```

3. **Fixed-point arithmetic**: `i16` instead of `f32` saves 50% space

```rust
m! { "temp": 2345i16 }  // 2 bytes vs 4 bytes
```

4. **Batch operations**: Process multiple items together

```rust
let batch: Vec<Map> = sensors.iter()
    .map(|s| create_reading(s))
    .collect();
```

5. **Reuse buffers**: Reuse Vec<u8> for encoding

```rust
buffer.clear();
buffer.extend_from_slice(&data.to_bytes()?);
```

## Testing

Run the test suite:

```bash
cargo test                      # All tests
cargo test extended_types       # Test new integer types
cargo test integration_test     # Integration tests
cargo test --doc                # Documentation tests
```

Run examples:

```bash
cargo run --example basic_types     # All data types demo
cargo run --example iot_device      # IoT device modeling
cargo run --example performance     # Performance comparison
cargo run --example array_macro     # Array macro examples
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## More Resources

- [API Documentation](https://docs.rs/nson)
- [Crates.io](https://crates.io/crates/nson)
- [Examples](examples/) - More complete examples in the repository
