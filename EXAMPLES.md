# NSON Examples and Usage Guide

本文档提供 NSON 库的详细使用示例和最佳实践。

## 目录

- [基本类型](#基本类型)
- [数据类型选择](#数据类型选择)
- [IoT 应用场景](#iot-应用场景)
- [性能优化](#性能优化)
- [序列化与反序列化](#序列化与反序列化)
- [测试](#测试)

## 基本类型

### 整数类型

NSON 提供了完整的整数类型支持，从 8 位到 64 位：

```rust
use nson::m;

let numbers = m! {
    "tiny": 42i8,           // -128 到 127
    "small": 1000i16,       // -32,768 到 32,767
    "medium": 1000000i32,   // -2^31 到 2^31-1
    "large": 1000000000i64, // -2^63 到 2^63-1

    "utiny": 200u8,         // 0 到 255
    "usmall": 50000u16,     // 0 到 65,535
    "umedium": 3000000u32,  // 0 到 2^32-1
    "ularge": 5000000000u64,// 0 到 2^64-1
};
```

### 类型访问

使用类型安全的 getter 方法：

```rust
let value = m! {
    "temperature": 2350i16,
    "humidity": 65u8,
};

// 成功获取
assert_eq!(value.get_i16("temperature").unwrap(), 2350);
assert_eq!(value.get_u8("humidity").unwrap(), 65);

// 类型不匹配返回错误
assert!(value.get_u16("temperature").is_err());
```

## 数据类型选择

### 原则：选择最小的合适类型

选择正确的数据类型可以显著减少存储空间和传输带宽：

```rust
use nson::m;

// ✅ 好的实践：使用合适的类型
let efficient = m! {
    "age": 25u8,              // 年龄 0-255
    "percentage": 75u8,       // 百分比 0-100
    "port": 8080u16,          // 端口号 0-65535
    "temperature": 2350i16,   // 温度 * 100 (23.50°C)
    "enabled": true,          // 布尔值
};

// ❌ 不好的实践：使用过大的类型
let inefficient = m! {
    "age": 25u32,             // 浪费 3 字节
    "percentage": 75u32,      // 浪费 3 字节
    "port": 8080u32,          // 浪费 2 字节
    "temperature": 23.5f32,   // 浪费空间且精度不必要
    "enabled": 1u32,          // 浪费 3 字节
};

// 空间对比
println!("高效版: {} 字节", efficient.to_bytes().unwrap().len());
println!("低效版: {} 字节", inefficient.to_bytes().unwrap().len());
// 输出类似：高效版: 27 字节, 低效版: 45 字节
```

### 定点数 vs 浮点数

对于固定精度的数值（如温度、价格），使用定点数更高效：

```rust
// ✅ 定点数：2 字节，精确
let temp_fixed = m! {
    "temperature": 2345i16,  // 23.45°C (值 * 100)
};

// ❌ 浮点数：4 字节，可能有精度问题
let temp_float = m! {
    "temperature": 23.45f32,
};

// 使用定点数
fn celsius_to_i16(celsius: f64) -> i16 {
    (celsius * 100.0) as i16
}

fn i16_to_celsius(value: i16) -> f64 {
    value as f64 / 100.0
}
```

## IoT 应用场景

### 温湿度传感器

```rust
use nson::{m, Id, TimeStamp};

fn create_sensor_reading(temp: f64, humidity: u8) -> nson::Map {
    m! {
        "device_id": Id::new(),
        "timestamp": TimeStamp::from(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()),
        "temperature": (temp * 100.0) as i16,  // 精度到 0.01°C
        "humidity": humidity,                   // 0-100%
        "battery": 87u8,                        // 87%
        "signal": -45i8,                        // -45 dBm
    }
}

// 使用示例
let reading = create_sensor_reading(23.45, 65);
let bytes = reading.to_bytes().unwrap();

// 传输或存储 bytes...

// 接收端解码
let decoded = nson::Map::from_bytes(&bytes).unwrap();
let temp = decoded.get_i16("temperature").unwrap() as f64 / 100.0;
let humidity = decoded.get_u8("humidity").unwrap();
println!("温度: {:.2}°C, 湿度: {}%", temp, humidity);
```

### 智能灯泡控制

```rust
use nson::m;

fn create_light_command(brightness: u8, color_temp: u16) -> nson::Map {
    m! {
        "device_id": 42u16,
        "command": "set_state",
        "on": true,
        "brightness": brightness,      // 0-255
        "color_temp": color_temp,      // 2700-6500K
        "transition": 500u16,          // 500ms
    }
}

// 创建命令
let cmd = create_light_command(192, 4000);
let bytes = cmd.to_bytes().unwrap();
println!("命令大小: {} 字节", bytes.len());
```

### Matter 设备属性

```rust
use nson::m;

// Matter 协议中的设备属性
let device_attributes = m! {
    "VendorID": 0x1234u16,
    "ProductID": 0x5678u16,
    "HardwareVersion": 2u8,
    "SoftwareVersion": 0x00020100u32,  // v2.1.0
    "OnOff": true,
    "CurrentLevel": 128u8,              // 0-254
    "ColorTemperatureMireds": 250u16,   // 色温倒数
    "CurrentX": 24939u16,               // CIE x 坐标
    "CurrentY": 24701u16,               // CIE y 坐标
};

let bytes = device_attributes.to_bytes().unwrap();
println!("属性数据: {} 字节", bytes.len());
```

## 性能优化

### 1. 批量处理

```rust
use nson::{m, Array, Value};

// 批量处理多个传感器
fn process_sensor_batch(count: usize) -> Vec<nson::Map> {
    (0..count)
        .map(|i| m! {
            "id": i as u16,
            "value": (2000 + i * 10) as i16,
        })
        .collect()
}

// 批量编码
let sensors = process_sensor_batch(100);
let total_bytes: usize = sensors.iter()
    .map(|s| s.to_bytes().unwrap().len())
    .sum();

println!("100 个传感器总大小: {} 字节", total_bytes);
```

### 2. 预分配容量

```rust
use nson::Map;

// 如果知道大概的条目数，预分配容量
let mut map = Map::with_capacity(10);
for i in 0..10 {
    map.insert(format!("key_{}", i), i as u16);
}
```

### 3. 复用缓冲区

```rust
use nson::Map;

fn encode_reuse_buffer(data: &Map, buffer: &mut Vec<u8>) -> Result<(), nson::encode::EncodeError> {
    buffer.clear();
    buffer.extend_from_slice(&data.to_bytes()?);
    Ok(())
}
```

## 序列化与反序列化

### 使用 Serde

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

// 序列化
let bytes = nson::encode::to_bytes(&config).unwrap();

// 反序列化
let decoded: DeviceConfig = nson::decode::from_bytes(&bytes).unwrap();
assert_eq!(config, decoded);
```

### 直接使用 Map

```rust
use nson::{m, Map};

// 创建
let config = m! {
    "vendor_id": 0x1234u16,
    "product_id": 0x5678u16,
    "brightness": 128u8,
};

// 编码
let bytes = config.to_bytes().unwrap();

// 解码
let decoded = Map::from_bytes(&bytes).unwrap();

// 访问
let vendor_id = decoded.get_u16("vendor_id").unwrap();
```

## 测试

### 单元测试示例

```rust
#[cfg(test)]
mod tests {
    use nson::{m, Map};

    #[test]
    fn test_encode_decode() {
        let original = m! {
            "temperature": 2350i16,
            "humidity": 65u8,
        };

        let bytes = original.to_bytes().unwrap();
        let decoded = Map::from_bytes(&bytes).unwrap();

        assert_eq!(original, decoded);
        assert_eq!(decoded.get_i16("temperature").unwrap(), 2350);
        assert_eq!(decoded.get_u8("humidity").unwrap(), 65);
    }

    #[test]
    fn test_boundary_values() {
        let map = m! {
            "u8_max": u8::MAX,
            "i8_min": i8::MIN,
            "u16_max": u16::MAX,
            "i16_min": i16::MIN,
        };

        let bytes = map.to_bytes().unwrap();
        let decoded = Map::from_bytes(&bytes).unwrap();

        assert_eq!(decoded.get_u8("u8_max").unwrap(), 255);
        assert_eq!(decoded.get_i8("i8_min").unwrap(), -128);
        assert_eq!(decoded.get_u16("u16_max").unwrap(), 65535);
        assert_eq!(decoded.get_i16("i16_min").unwrap(), -32768);
    }
}
```

## 最佳实践总结

1. **选择合适的类型**：使用能表示数据范围的最小类型
2. **使用定点数**：对于固定精度的数值（温度、价格等）
3. **预分配容量**：当知道数据大小时使用 `with_capacity`
4. **批量处理**：批量编码/解码可以提高效率
5. **错误处理**：始终处理编解码可能的错误
6. **类型安全**：使用类型安全的 getter 方法而不是模式匹配
7. **测试边界值**：测试类型的最大和最小值
8. **文档化约定**：如果使用定点数，在代码中注释转换规则

## 更多示例

查看 [examples](examples/) 目录获取更多完整示例：

- `basic_types.rs` - 所有支持的数据类型演示
- `iot_device.rs` - IoT 设备数据建模
- `performance.rs` - 性能和空间效率对比

运行示例：

```bash
cargo run --example basic_types
cargo run --example iot_device
cargo run --example performance
```

运行测试：

```bash
cargo test                      # 所有测试
cargo test extended_types       # 扩展类型测试
cargo test integration_test     # 集成测试
```
