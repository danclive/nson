# nson

[![crates.io](https://img.shields.io/crates/v/nson.svg)](https://crates.io/crates/nson)
[![docs.rs](https://docs.rs/mongodb/badge.svg)](https://docs.rs/nson)
[![crates.io](https://img.shields.io/crates/l/nson.svg)](https://crates.io/crates/nson)

NSON is short for NEW JSON, a binary encoded serialization of JSON-like documents. Similar to JSON, NSON supports embedding maps and arrays within other maps and arrays. Unlike JSON, NSON also includes comprehensive integer types (i8/u8, i16/u16, i32/u32, i64/u64), floating-point types (f32/f64), binary, timestamp, and id types.

NSON borrows from BSON and can be thought of as a streamlined version of BSON, removing some of the less common or mongodb-proprietary types. NSON also categorizes Double into f32 and f64, considering that f64 is not needed in most cases for high-precision floating-point numbers. NSON provides fine-grained integer types (8-bit, 16-bit, 32-bit, and 64-bit, both signed and unsigned) to optimize storage space and bandwidth usage.

In the rust language, NSON can be easily written without necessarily serializing/unserializing to structures, thanks to the macro.

In addition, NSON is convenient to parse from binary, and the library implements "no_std", which can be used on microcontrollers.

## Example

```rust
use nson::m;

fn main() {
    let mut value = m!{
        "code": 200,
        "success": true,
        "payload": {
            "some": [
                "pay",
                "loads",
            ]
        }
    };

    println!("{:?}", value);
    // print: Map{"code": I32(200), "success": Bool(true), "payload":
    // Map{"some": Array([String("pay"), String("loads")])}}

    println!("{:?}", value.get("code"));
    // print: Some(I32(200))

    // insert new key, value
    value.insert("hello", "world");

    println!("{:?}", value.get("hello"));
    // print: Some(String("world"))
}
```

## Supported Types

NSON supports a rich set of data types optimized for different use cases:

### Integer Types
- **i8/u8**: 8-bit signed/unsigned integers (range: -128 to 127 / 0 to 255)
- **i16/u16**: 16-bit signed/unsigned integers (range: -32,768 to 32,767 / 0 to 65,535)
- **i32/u32**: 32-bit signed/unsigned integers
- **i64/u64**: 64-bit signed/unsigned integers

### Floating-Point Types
- **f32**: 32-bit floating-point number
- **f64**: 64-bit floating-point number (double precision)

### Other Types
- **String**: UTF-8 encoded strings
- **Binary**: Raw binary data
- **Bool**: Boolean value (true/false)
- **Null**: Null value
- **Array**: Ordered collection of values
- **Map**: Key-value pairs (ordered)
- **TimeStamp**: 64-bit timestamp
- **Id**: 12-byte unique identifier

## Usage Examples

### Working with Different Integer Types

```rust
use nson::m;

// Use appropriate integer types to save space
let device_config = m! {
    "vendor_id": 0x1234u16,      // 16-bit is enough for vendor ID
    "product_id": 0x5678u16,     // 16-bit for product ID
    "brightness": 128u8,         // 8-bit for 0-255 range
    "temperature": 2350i16,      // 16-bit for temperature * 100 (23.50°C)
    "humidity": 65u8,            // 8-bit for percentage
    "offset": -10i8,             // 8-bit for small offsets
};

// Access values with type-safe getters
let brightness = device_config.get_u8("brightness").unwrap();
let temp = device_config.get_i16("temperature").unwrap();
println!("Brightness: {}, Temperature: {:.2}°C", brightness, temp as f32 / 100.0);
```

### IoT Sensor Data

```rust
use nson::{m, Id, TimeStamp};

let sensor_data = m! {
    "device_id": Id::new(),
    "timestamp": TimeStamp::from(1732694400u64),
    "temperature": 2345i16,      // 23.45°C (stored as integer * 100)
    "humidity": 65u8,            // 65%
    "battery": 87u8,             // 87%
    "signal_strength": -45i8,    // -45 dBm
};

// Encode to bytes for transmission
let bytes = sensor_data.to_bytes().unwrap();
println!("Encoded size: {} bytes", bytes.len());

// Decode on the receiving end
let decoded = nson::Map::from_bytes(&bytes).unwrap();
assert_eq!(sensor_data, decoded);
```

### Performance Benefits

Using smaller integer types can significantly reduce data size:

```rust
use nson::m;

// Efficient: uses u8 (1 byte each)
let efficient = m! {
    "status": 1u8,
    "level": 50u8,
    "mode": 2u8,
};

// Less efficient: uses u32 (4 bytes each)
let inefficient = m! {
    "status": 1u32,
    "level": 50u32,
    "mode": 2u32,
};

println!("Efficient size: {} bytes", efficient.to_bytes().unwrap().len());
println!("Inefficient size: {} bytes", inefficient.to_bytes().unwrap().len());
// Saves 9 bytes (50% smaller)!
```

## Examples

Check out the [examples](examples/) directory for more usage examples:
- `basic_types.rs` - Demonstrates all supported data types
- `iot_device.rs` - IoT device data modeling
- `performance.rs` - Performance and space efficiency comparison

## Testing

Run the test suite:

```bash
cargo test
```

Run specific test modules:

```bash
cargo test extended_types    # Test new integer types
cargo test integration_test  # Integration tests
```

Run examples:

```bash
cargo run --example basic_types
cargo run --example iot_device
cargo run --example performance
```
