//! 生命周期基础概念演示
//! 
//! 本示例展示了 Rust 中生命周期的核心概念，包括：
//! - 基本生命周期概念
//! - 生命周期注解
//! - 函数中的生命周期
//! - 结构体中的生命周期
//! - 静态生命周期

use std::collections::HashMap;

/// 演示基本的生命周期概念
fn basic_lifetime_demo() {
    println!("=== 基本生命周期概念 ===");
    
    let x = 5;                    // x 的生命周期开始
    let r = &x;                   // r 借用 x
    println!("x = {}, r = {}", x, r);
    // x 和 r 的生命周期在这里结束
}

/// 演示悬垂引用的预防
fn dangling_reference_demo() {
    println!("\n=== 悬垂引用预防 ===");
    
    // 演示悬垂引用问题（注释掉的代码会导致编译错误）
    /*
    let r;
    {
        let x = 5;
        r = &x;  // 错误：x 的生命周期太短
    }
    println!("r = {}", r);  // 这里会编译失败
    */
    
    // 正确的做法：确保被引用的值有足够长的生命周期
    let y = 10;
    let valid_ref = &y;
    println!("有效的引用: {}", valid_ref);
    
    // 演示作用域和生命周期
    {
        let x = 5;
        println!("x 在内部作用域: {}", x);
        // x 在这里被销毁
    }
    // x 在这里已经不可访问
}

/// 演示函数中的生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 演示多个生命周期参数
fn first_word<'a, 'b>(s: &'a str, _separator: &'b str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

/// 演示生命周期省略规则
fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

/// 包含引用的结构体
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// 方法中的生命周期
    fn level(&self) -> i32 {
        3
    }
    
    /// 返回引用的方法
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
}

/// 演示静态生命周期
static GLOBAL_STR: &str = "我是全局字符串";

fn static_lifetime_demo() {
    println!("\n=== 静态生命周期 ===");
    
    // 字符串字面量具有 'static 生命周期
    let s: &'static str = "Hello, world!";
    println!("静态字符串: {}", s);
    
    // 全局变量也具有 'static 生命周期
    println!("全局字符串: {}", GLOBAL_STR);
    
    // 静态生命周期的强制转换
    let static_ref: &'static str = "静态引用";
    let shorter_ref: &str = static_ref;  // 'static 可以强制转换为任何生命周期
    println!("转换后的引用: {}", shorter_ref);
}

/// 演示生命周期子类型
fn lifetime_subtyping_demo() {
    println!("\n=== 生命周期子类型 ===");
    
    let string1 = String::from("长字符串");
    let result;
    {
        let string2 = String::from("短");
        result = longest(string1.as_str(), string2.as_str());
        println!("在内部作用域中，最长的字符串是: {}", result);
    }
    // result 在这里仍然有效，因为它引用的是 string1
    // 但如果 result 引用的是 string2，这里就会出错
}

/// 演示复杂的生命周期场景
fn complex_lifetime_demo() {
    println!("\n=== 复杂生命周期场景 ===");
    
    let novel = String::from("很久很久以前，在一个遥远的星系中...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {:?}", excerpt);
    println!("等级: {}", excerpt.level());
    
    let announcement = "这是一个重要的公告";
    let returned_part = excerpt.announce_and_return_part(announcement);
    println!("返回的部分: {}", returned_part);
}

/// 演示生命周期与迭代器
fn iterator_lifetime_demo() {
    println!("\n=== 生命周期与迭代器 ===");
    
    let data = vec!["apple", "banana", "cherry"];
    
    // 迭代器保持对原始数据的引用
    let mut iter = data.iter();
    
    while let Some(item) = iter.next() {
        println!("水果: {}", item);
    }
    
    // 使用 HashMap 演示更复杂的情况
    let mut config = HashMap::new();
    config.insert("debug", "true");
    config.insert("port", "8080");
    
    for (key, value) in &config {
        println!("配置 {}: {}", key, value);
    }
}

/// 演示生命周期错误的常见情况
fn common_lifetime_errors_demo() {
    println!("\n=== 常见生命周期错误演示 ===");
    
    // 1. 返回局部变量的引用（编译错误）
    // fn invalid_function() -> &str {
    //     let s = String::from("hello");
    //     &s  // 错误：返回对局部变量的引用
    // }
    
    // 2. 正确的做法：返回拥有的值
    fn valid_function() -> String {
        let s = String::from("hello");
        s  // 正确：返回拥有的值
    }
    
    let owned_string = valid_function();
    println!("拥有的字符串: {}", owned_string);
    
    // 3. 使用生命周期参数解决引用问题
    fn choose_str<'a>(first: &'a str, second: &'a str, use_first: bool) -> &'a str {
        if use_first {
            first
        } else {
            second
        }
    }
    
    let str1 = "第一个字符串";
    let str2 = "第二个字符串";
    let chosen = choose_str(str1, str2, true);
    println!("选择的字符串: {}", chosen);
}

fn main() {
    println!("Rust 生命周期基础概念演示\n");
    
    basic_lifetime_demo();
    dangling_reference_demo();
    
    // 演示函数生命周期
    println!("\n=== 函数生命周期 ===");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);
    
    let sentence = "Hello world from Rust";
    let separator = " ";
    let first = first_word(sentence, separator);
    println!("第一个单词: {}", first);
    
    // 演示生命周期省略
    if let Some(ch) = get_first_char("Hello") {
        println!("第一个字符: {}", ch);
    }
    
    static_lifetime_demo();
    lifetime_subtyping_demo();
    complex_lifetime_demo();
    iterator_lifetime_demo();
    common_lifetime_errors_demo();
    
    println!("\n=== 演示完成 ===");
    println!("这些示例展示了 Rust 生命周期系统如何确保内存安全。");
    println!("通过编译时检查，Rust 防止了悬垂指针和内存安全问题。");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "world"), "world");  // "world" 比 "hello" 长
        assert_eq!(longest("hi", "goodbye"), "goodbye");  // "goodbye" 比 "hi" 长
        assert_eq!(longest("rust", "go"), "rust");        // "rust" 比 "go" 长
        assert_eq!(longest("abc", "def"), "def");         // 长度相等时返回第二个（else分支）
    }
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world", " "), "hello");
        assert_eq!(first_word("rust", " "), "rust");
    }
    
    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("hello"), Some('h'));
        assert_eq!(get_first_char(""), None);
    }
    
    #[test]
    fn test_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };
        
        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.part, "Call me Ishmael");
    }
}
