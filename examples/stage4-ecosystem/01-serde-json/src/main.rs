use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), serde_json::Error> {
    println!("Serde JSON 演示");

    // 创建一个 Point 实例
    let point = Point { x: 1, y: 2 };

    // --- 序列化 ---
    // 将 Point 实例序列化为 JSON 字符串
    let serialized = serde_json::to_string(&point)?;
    println!("\n--- 序列化 ---");
    println!("序列化后的 JSON: {}", serialized);

    // --- 反序列化 ---
    // 将 JSON 字符串反序列化为 Point 实例
    let deserialized: Point = serde_json::from_str(&serialized)?;
    println!("\n--- 反序列化 ---");
    println!("反序列化后的 Point: {:?}", deserialized);

    println!("\n演示完成。");

    Ok(())
}