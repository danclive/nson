# NSON 快速参考

## 数据类型速查表

| 类型 | Rust 类型 | 字节数 | 范围 | 用途示例 |
|------|----------|--------|------|----------|
| **I8** | `i8` | 1 | -128 到 127 | 温度偏移、信号强度 |
| **U8** | `u8` | 1 | 0 到 255 | 百分比、亮度级别、小计数 |
| **I16** | `i16` | 2 | -32,768 到 32,767 | 温度*100、坐标 |
| **U16** | `u16` | 2 | 0 到 65,535 | 端口号、产品ID、色温 |
| **I32** | `i32` | 4 | -2³¹ 到 2³¹-1 | 标准整数、计数器 |
| **U32** | `u32` | 4 | 0 到 2³²-1 | 无符号计数、ID |
| **I64** | `i64` | 8 | -2⁶³ 到 2⁶³-1 | 大整数、Unix 时间戳 |
| **U64** | `u64` | 8 | 0 到 2⁶⁴-1 | 大无符号数、累积值 |
| **F32** | `f32` | 4 | IEEE 754 | 一般浮点数 |
| **F64** | `f64` | 8 | IEEE 754 | 高精度浮点数 |
| **Bool** | `bool` | 1 | true/false | 布尔值 |
| **String** | `String` | 4+len | UTF-8 | 文本 |
| **Binary** | `Vec<u8>` | 4+len | 字节数组 | 二进制数据 |
| **Null** | - | 0 | null | 空值 |
| **Array** | `Array` | 4+data+1 | 有序集合 | 列表 |
| **Map** | `Map` | 4+data+1 | 键值对 | 对象/字典 |
| **TimeStamp** | `TimeStamp` | 8 | Unix 时间戳 | 时间 |
| **Id** | `Id` | 12 | 12字节 | 唯一标识符 |

## 快速示例

### 创建数据

```rust
use nson::m;

let data = m! {
    "name": "sensor-01",
    "value": 42u8,
    "enabled": true,
};
```

### 编码与解码

```rust
// 编码
let bytes = data.to_bytes().unwrap();

// 解码
let decoded = nson::Map::from_bytes(&bytes).unwrap();
```

### 访问值

```rust
// 类型安全的访问
let value = decoded.get_u8("value").unwrap();

// 通用访问
let name = decoded.get("name");
```

### 数组操作

```rust
use nson::{Array, Value};

let arr = Array::from_vec(vec![
    Value::U8(1),
    Value::U8(2),
    Value::U8(3),
]);

// 编码
let bytes = arr.to_bytes().unwrap();

// 解码
let decoded = Array::from_bytes(&bytes).unwrap();
```

## 类型选择建议

### 整数值

| 值范围 | 推荐类型 | 示例 |
|--------|----------|------|
| 0-100 | `u8` | 百分比、电池电量 |
| 0-255 | `u8` | RGB 颜色、亮度 |
| 0-1000 | `u16` | 小端口号、小ID |
| -100到+100 | `i8` 或 `i16` | 温度偏移、小偏差 |
| -1000到+1000 | `i16` | 定点数*100 |
| 0-65535 | `u16` | 端口号、产品ID |
| 其他小整数 | `i32`/`u32` | 标准整数 |
| 大整数 | `i64`/`u64` | 时间戳、大计数 |

### 浮点数

| 用途 | 推荐类型 | 原因 |
|------|----------|------|
| 温度、价格 | `i16` | 定点数节省空间、精确 |
| 一般浮点 | `f32` | 足够精度、省空间 |
| 科学计算 | `f64` | 高精度需求 |

## 常见模式

### IoT 传感器

```rust
m! {
    "temp": 2345i16,      // 23.45°C
    "humidity": 65u8,     // 65%
    "battery": 87u8,      // 87%
    "rssi": -45i8,        // -45 dBm
}
```

### 设备配置

```rust
m! {
    "vendor_id": 0x1234u16,
    "product_id": 0x5678u16,
    "brightness": 128u8,
    "enabled": true,
}
```

### 网络数据

```rust
m! {
    "host": "example.com",
    "port": 8080u16,
    "status": 200u16,
    "timeout": 30u8,
}
```

## Map 方法速查

```rust
// 创建
let mut map = Map::new();
let map = Map::with_capacity(10);

// 插入
map.insert("key", value);

// 访问
map.get("key");                    // Option<&Value>
map.get_u8("key").unwrap();       // u8
map.get_i16("key").unwrap();      // i16
map.get_str("key").unwrap();      // &str
map.get_bool("key").unwrap();     // bool

// 检查
map.contains_key("key");          // bool
map.is_null("key");               // bool
map.len();                        // usize
map.is_empty();                   // bool

// 迭代
for (key, value) in &map { }
for key in map.keys() { }
for value in map.values() { }

// 编码
let bytes = map.to_bytes().unwrap();
let map = Map::from_bytes(&bytes).unwrap();
```

## 错误处理

```rust
use nson::Map;

// 结果类型
let result: Result<Map, _> = Map::from_bytes(&bytes);

match result {
    Ok(map) => println!("Success: {:?}", map),
    Err(e) => println!("Error: {}", e),
}

// 或使用 unwrap
let map = Map::from_bytes(&bytes).unwrap();

// 或使用 ?
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::from_bytes(&bytes)?;
    Ok(())
}
```

## Serde 集成

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    value: u8,
    enabled: bool,
}

// 序列化
let config = Config { /*...*/ };
let bytes = nson::encode::to_bytes(&config)?;

// 反序列化
let config: Config = nson::decode::from_bytes(&bytes)?;
```

## 性能提示

1. **预分配**: `Map::with_capacity(n)`
2. **使用小类型**: u8 代替 u32 可节省 75%
3. **定点数**: i16 代替 f32 可节省 50%
4. **批量处理**: 批量编解码效率更高
5. **复用缓冲**: 重复使用 Vec<u8>

## 常见问题

### Q: 如何选择整数类型？
**A**: 选择能表示数据范围的最小类型。例如 0-100 用 u8，0-65535 用 u16。

### Q: 什么时候用定点数？
**A**: 当数据有固定精度时（如货币、温度），用整数类型乘以倍数存储。

### Q: 如何处理可选值？
**A**: 使用 `Value::Null` 或 `Option<T>` 配合 serde。

### Q: 如何嵌套结构？
**A**: Map 和 Array 可以任意嵌套：
```rust
m! {
    "level1": {
        "level2": {
            "value": 42u8,
        },
    },
}
```

## 运行示例

```bash
# 查看所有类型
cargo run --example basic_types

# IoT 应用
cargo run --example iot_device

# 性能对比
cargo run --example performance

# 运行测试
cargo test
```

## 更多资源

- [完整示例](EXAMPLES.md)
- [API 文档](https://docs.rs/nson)
- [变更日志](CHANGELOG_NEW_TYPES.md)
