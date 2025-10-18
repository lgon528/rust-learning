//! 所有权核心概念演示
//!
//! 本示例代码旨在演示 Rust 的所有权系统，包括：
//! 1.  **所有权（Ownership）**：变量如何拥有值。
//! 2.  **移动（Move）**：值如何在变量之间转移所有权。
//! 3.  **克隆（Clone）**：如何显式复制数据。
//! 4.  **借用（Borrowing）**：如何通过引用访问数据而不获取所有权。
//! 5.  **可变借用（Mutable Borrows）**：如何修改被借用的数据。
//! 6.  **切片（Slices）**：如何引用集合的一部分。

fn main() {
    println!("Rust 所有权核心概念演示\n");

    // 1. 所有权和移动
    ownership_and_move_demo();

    // 2. 克隆
    clone_demo();

    // 3. 借用
    borrowing_demo();

    // 4. 可变借用
    mutable_borrow_demo();

    // 5. 切片
    slice_demo();

    println!("\n演示完成。");
}

/// 演示所有权和移动
fn ownership_and_move_demo() {
    println!("=== 1. 所有权和移动 ===");

    // s1 拥有一个 String
    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // s2 接收 s1 的所有权，s1 失效
    let s2 = s1;
    println!("s2 = {}", s2);

    // 下一行代码将导致编译错误，因为 s1 的所有权已经移动
    // println!("s1 = {}", s1);

    println!("s1 的所有权已移动到 s2。\n");
}

/// 演示克隆
fn clone_demo() {
    println!("=== 2. 克隆 ===");

    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // s2 是 s1 的克隆，s1 仍然有效
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("s2 是 s1 的克隆，两者都有效。\n");
}

/// 演示借用
fn borrowing_demo() {
    println!("=== 3. 借用 ===");

    let s1 = String::from("hello");

    // calculate_length 函数借用了 s1
    let len = calculate_length(&s1);

    println!("字符串 '{}' 的长度是 {}。", s1, len);
    println!("s1 在借用后仍然有效。\n");
}

// 该函数通过引用借用一个 String
fn calculate_length(s: &str) -> usize {
    s.len()
}

/// 演示可变借用
fn mutable_borrow_demo() {
    println!("=== 4. 可变借用 ===");

    let mut s = String::from("hello");
    println!("修改前的字符串: {}", s);

    // change 函数可变地借用了 s
    change(&mut s);

    println!("修改后的字符串: {}", s);
    println!("字符串被可变地借用和修改。\n");
}

// 该函数通过可变引用借用一个 String 并修改它
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// 演示切片
fn slice_demo() {
    println!("=== 5. 切片 ===");

    let s = String::from("hello world");

    // 创建字符串切片
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("从 '{}' 中切片: '{}' 和 '{}'", s, hello, world);

    // 使用函数查找第一个单词
    let word = first_word(&s);
    println!("第一个单词是: '{}'", word);

    // 清空字符串后，切片将失效（如果再次使用）
    // s.clear(); // 这会导致编译错误，因为存在不可变借用
    println!("字符串切片提供了对部分数据的安全访问。\n");
}

// 该函数返回字符串的第一个单词的切片
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}