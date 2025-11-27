//! IoT 设备数据示例
//!
//! 模拟智能家居设备使用 NSON 传输数据

use nson::{m, Id, Map, TimeStamp};

fn main() {
    println!("=== IoT 智能设备数据示例 ===\n");

    // 模拟温湿度传感器数据
    let sensor_data = create_sensor_reading();
    println!("传感器数据:");
    println!("{}\n", sensor_data);

    let bytes = sensor_data.to_bytes().unwrap();
    println!("传输大小: {} 字节\n", bytes.len());

    // 模拟智能灯泡状态
    let light_state = create_light_state();
    println!("智能灯泡状态:");
    println!("{}\n", light_state);

    // 模拟电器能耗数据
    let power_usage = create_power_usage();
    println!("电器能耗数据:");
    println!("{}\n", power_usage);

    // 模拟设备配置
    let device_config = create_device_config();
    println!("设备配置:");
    println!("{}\n", device_config);

    // 验证往返编解码
    let decoded = Map::from_bytes(&bytes).unwrap();
    assert_eq!(sensor_data, decoded);
    println!("✓ 数据完整性验证通过");
}

/// 创建温湿度传感器读数
fn create_sensor_reading() -> Map {
    m! {
        "device_id": Id::new(),
        "device_type": "TempHumiditySensor",
        "timestamp": TimeStamp::from(1732694400u64),
        "temperature": 2345i16,        // 23.45°C (温度 * 100)
        "humidity": 65u8,              // 65% 湿度
        "battery_level": 87u8,         // 87% 电量
        "signal_strength": -45i8,      // -45 dBm
        "firmware_version": "1.2.5",
    }
}

/// 创建智能灯泡状态
fn create_light_state() -> Map {
    m! {
        "device_id": Id::new(),
        "device_type": "SmartBulb",
        "on_off": true,
        "brightness": 192u8,           // 0-255 亮度级别
        "color_temp": 4000u16,         // 4000K 色温
        "power_watts": 9u8,            // 9W 功率
        "mode": "normal",
        "transition_time": 500u16,     // 500ms 过渡时间
    }
}

/// 创建电器能耗数据
fn create_power_usage() -> Map {
    m! {
        "device_id": Id::new(),
        "device_type": "SmartPlug",
        "timestamp": TimeStamp::from(1732694400u64),
        "voltage": 220u8,              // 220V 电压
        "current": 2500u16,            // 2.5A 电流 (mA)
        "power": 550u16,               // 550W 功率
        "energy_today": 12345u32,      // 今日用电 Wh
        "energy_total": 9876543u32,    // 总用电量 Wh
        "status": "on",
    }
}

/// 创建设备配置
fn create_device_config() -> Map {
    m! {
        "device_id": Id::new(),
        "vendor_id": 0x1234u16,        // 厂商ID
        "product_id": 0x5678u16,       // 产品ID
        "hardware_version": 2u8,       // 硬件版本
        "software_version": "2.1.0",
        "max_brightness": 255u8,
        "min_brightness": 10u8,
        "default_brightness": 128u8,
        "auto_off_delay": 3600u16,     // 1小时自动关闭(秒)
        "enabled_features": vec![
            "dimming",
            "color_temp",
            "schedule",
        ],
    }
}
