//! 性能测试示例
//!
//! 展示不同数据类型的编码效率

use nson::{Map, m};
use std::time::Instant;

fn main() {
    println!("=== NSON 性能与空间效率示例 ===\n");

    // 1. 小整数类型的空间优势
    println!("--- 空间效率对比 ---");
    space_efficiency_demo();

    println!("\n--- 编解码速度 ---");
    // 2. 编解码性能
    codec_performance_demo();

    println!("\n--- 批量数据处理 ---");
    // 3. 批量数据
    batch_data_demo();
}

/// 空间效率演示
fn space_efficiency_demo() {
    // 使用 u8 而不是 u32 存储小数值
    let efficient = m! {
        "status": 1u8,      // 1字节
        "level": 50u8,      // 1字节
        "mode": 2u8,        // 1字节
    };

    let inefficient = m! {
        "status": 1u32,     // 4字节
        "level": 50u32,     // 4字节
        "mode": 2u32,       // 4字节
    };

    let efficient_size = efficient.to_bytes().unwrap().len();
    let inefficient_size = inefficient.to_bytes().unwrap().len();

    println!("使用 u8 类型: {} 字节", efficient_size);
    println!("使用 u32 类型: {} 字节", inefficient_size);
    println!(
        "节省: {} 字节 ({:.1}%)",
        inefficient_size - efficient_size,
        (1.0 - efficient_size as f64 / inefficient_size as f64) * 100.0
    );

    // 温度数据用 i16 代替 f32
    let temp_i16 = m! {
        "temp1": 2345i16,   // 23.45°C * 100
        "temp2": 1890i16,   // 18.90°C * 100
        "temp3": -550i16,   // -5.50°C * 100
    };

    let temp_f32 = m! {
        "temp1": 23.45f32,
        "temp2": 18.90f32,
        "temp3": -5.50f32,
    };

    let i16_size = temp_i16.to_bytes().unwrap().len();
    let f32_size = temp_f32.to_bytes().unwrap().len();

    println!("\n温度存储 (i16 vs f32):");
    println!("i16 (定点): {} 字节", i16_size);
    println!("f32 (浮点): {} 字节", f32_size);
    println!("节省: {} 字节", f32_size - i16_size);
}

/// 编解码性能演示
fn codec_performance_demo() {
    let iterations = 10000;

    let data = m! {
        "id": 12345u32,
        "name": "TestDevice",
        "enabled": true,
        "level": 128u8,
        "temp": 2350i16,
        "power": 550u16,
        "voltage": 220u8,
        "values": vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    // 编码性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = data.to_bytes().unwrap();
    }
    let encode_duration = start.elapsed();

    let bytes = data.to_bytes().unwrap();
    println!("数据大小: {} 字节", bytes.len());

    // 解码性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Map::from_bytes(&bytes).unwrap();
    }
    let decode_duration = start.elapsed();

    println!(
        "编码 {} 次: {:?} ({:.2} μs/次)",
        iterations,
        encode_duration,
        encode_duration.as_micros() as f64 / iterations as f64
    );

    println!(
        "解码 {} 次: {:?} ({:.2} μs/次)",
        iterations,
        decode_duration,
        decode_duration.as_micros() as f64 / iterations as f64
    );
}

/// 批量数据处理演示
fn batch_data_demo() {
    // 模拟100个传感器的数据
    let sensor_count = 100;
    let mut sensors = Vec::new();

    for i in 0..sensor_count {
        let sensor = m! {
            "id": i as u16,
            "temp": (2000 + (i * 10) as i16),  // 20.00 - 29.90°C
            "humidity": (50 + (i % 30)) as u8, // 50-79%
            "battery": (100 - (i % 50)) as u8, // 电池电量
        };
        sensors.push(sensor);
    }

    let start = Instant::now();
    let mut total_bytes = 0;

    for sensor in &sensors {
        let bytes = sensor.to_bytes().unwrap();
        total_bytes += bytes.len();
    }

    let duration = start.elapsed();

    println!("{} 个传感器数据:", sensor_count);
    println!("总大小: {} 字节", total_bytes);
    println!(
        "平均: {:.1} 字节/传感器",
        total_bytes as f64 / sensor_count as f64
    );
    println!("处理时间: {:?}", duration);
    println!(
        "速率: {:.2} MB/s",
        total_bytes as f64 / duration.as_secs_f64() / 1_000_000.0
    );
}
