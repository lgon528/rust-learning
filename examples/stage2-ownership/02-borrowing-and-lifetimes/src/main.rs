//! 借用和生命周期演示
//!
//! 本示例代码旨在演示 Rust 的借用和生命周期，包括：
//! 1.  **不可变借用（Immutable Borrows）**：如何创建对数据的只读引用。
//! 2.  **可变借用（Mutable Borrows）**：如何创建对数据的可写引用。
//! 3.  **借用规则（Borrowing Rules）**：同一作用域内可变和不可变借用的限制。
//! 4.  **生命周期（Lifetimes）**：确保引用始终有效。
//! 5.  **NLL（Non-Lexical Lifetimes）**：非词法生命周期如何使借用检查更灵活。

fn main() {
    println!("Rust 借用和生命周期演示\n");

    // 1. 不可变借用
    immutable_borrow_demo();

    // 2. 可变借用
    mutable_borrow_demo();

    // 3. 借用规则
    borrowing_rules_demo();

    // 4. NLL（非词法生命周期）
    nll_demo();

    println!("\n演示完成。");
}

/// 演示不可变借用
fn immutable_borrow_demo() {
    println!("=== 1. 不可变借用 ===");

    let s1 = String::from("hello");

    // 创建多个不可变借用
    let r1 = &s1;
    let r2 = &s1;

    println!("s1 = {}, r1 = {}, r2 = {}", s1, r1, r2);
    println!("可以同时存在多个不可变借用。\n");
}

/// 演示可变借用
fn mutable_borrow_demo() {
    println!("\n=== 2. 可变借用 ===");

    let mut s = String::from("hello");
    println!("修改前的字符串: {}", s);

    let r1 = &mut s; // 可变借用
    r1.push_str(", world");
    println!("修改后的字符串: {}", r1);

    println!("通过可变借用修改了字符串。");
}

// 3. 借用规则：在特定作用域内，对一个变量，只能有以下两种情况之一：
//    - 一个或多个不可变引用（&T）
//    - 只有一个可变引用（&mut T）
fn borrowing_rules_demo() {
    println!("\n=== 3. 借用规则 ===");
    let s = String::from("hello");

    let r1 = &s; // 不可变借用
    let r2 = &s; // 另一个不可变借用
    println!("r1 = {}, r2 = {}", r1, r2);

    // 在不可变借用仍然有效时，不能创建可变借用
    // let r3 = &mut s; // 这将导致编译错误
    // println!("{}, {}, and {}", r1, r2, r3);

    println!("在存在不可变借用时，不能创建可变借用。\n");
}

/// 演示 NLL（非词法生命周期）
fn nll_demo() {
    println!("\n=== 4. NLL（非词法生命周期） ===");

    let mut s = String::from("hello");

    let r1 = &s; // 不可变借用开始
    println!("r1 = {}", r1);
    // r1 的生命周期在这里结束，因为它不再被使用

    let r2 = &mut s; // 现在可以创建可变借用
    r2.push_str(", world");
    println!("r2 = {}", r2);

    println!("NLL 允许在不再使用的借用之后创建新的借用。\n");
}