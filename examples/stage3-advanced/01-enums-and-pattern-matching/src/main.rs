//! 枚举和模式匹配演示
//!
//! 本示例代码旨在演示 Rust 的枚举（Enums）和模式匹配（Pattern Matching），包括：
//! 1.  **定义枚举**：如何创建包含不同变体的类型。
//! 2.  **`match` 控制流**：如何使用模式匹配来处理枚举的不同变体。
//! 3.  **`if let` 控制流**：作为 `match` 的一种简写，用于只关心一种匹配情况的场景。

// 1. 定义枚举
// 枚举允许你定义一个可以包含多种不同变体的类型。
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 可以在枚举上定义方法
impl Message {
    fn call(&self) {
        // 在方法内部使用 match
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
        }
    }
}

fn main() {
    println!("Rust 枚举和模式匹配演示\n");

    // 2. 实例化枚举
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    // 3. 使用 for 循环和方法调用来处理枚举
    for msg in &messages {
        msg.call();
    }

    // 4. 使用 `if let` 控制流
    let some_value = Some(5);
    let some_message = Message::Write(String::from("another message"));

    // 只关心 Some 变体
    if let Some(i) = some_value {
        println!("\nValue is {}", i);
    }

    // 只关心 Write 变体
    if let Message::Write(text) = some_message {
        println!("Message is: {}", text);
    } else {
        println!("Not a Write message");
    }

    println!("\n演示完成。");
}