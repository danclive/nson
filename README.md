# nson

[![crates.io](https://img.shields.io/crates/v/nson.svg)](https://crates.io/crates/nson)
[![docs.rs](https://docs.rs/mongodb/badge.svg)](https://docs.rs/nson)
[![crates.io](https://img.shields.io/crates/l/nson.svg)](https://crates.io/crates/nson)

NSON is short for NEW JSON, a binary encoded serialization of JSON-like documents. Similar to JSON, NSON supports embedding maps and arrays within other maps and arrays. Unlike JSON, NSON also includes int32/uint32, int64/uint64, f32/f64, binary, timestamp, id types.

NSON borrows from BSON and can be thought of as a streamlined version of BSON, removing some of the less common or mongodb-proprietary types. NSON also categorizes Double into f32 and f64, considering that f64 is not needed in most cases for high-precision floating-point numbers. Also added uint32 and uint64 to make it clear that values cannot be complex.

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
