//! 结构体和方法演示
//!
//! 本示例代码旨在演示 Rust 的结构体（Structs）和方法（Methods），包括：
//! 1.  **定义结构体**：如何创建自定义数据类型。
//! 2.  **实例化结构体**：如何创建结构体的实例。
//! 3.  **方法**：如何在结构体上定义关联函数。
//! 4.  **关联函数**：如何定义与结构体相关但不作用于实例的函数。

// 1. 定义结构体
// 结构体是一种自定义数据类型，允许你将多个相关的值组合在一起。
#[derive(Debug)] // 派生 Debug trait，以便能够打印结构体
struct Rectangle {
    width: u32,
    height: u32,
}

// 2. 在 impl 块中定义方法
// 方法是与结构体（或枚举、trait）关联的函数。
impl Rectangle {
    // 方法的第一个参数通常是 `&self`、`&mut self` 或 `self`。
    // `&self` 表示该方法借用了结构体的不可变引用。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 该方法接受另一个 Rectangle 的不可变引用作为参数。
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（Associated Function）不以 `self` 作为第一个参数。
    // 它们通常用作构造函数。
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("Rust 结构体和方法演示\n");

    // 3. 实例化结构体
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // 使用 Debug trait 打印结构体
    println!("rect1 is {:?}", rect1);

    // 4. 调用方法
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // 5. 调用带参数的方法
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 6. 调用关联函数
    let sq = Rectangle::square(25);
    println!("sq is {:?}", sq);
    println!("The area of the square is {} square pixels.", sq.area());

    println!("\n演示完成。");
}