# panic! 和不可恢复错误

## 学习目标

通过本节学习，你将掌握：

- 理解 `panic!` 宏的作用和使用场景
- 掌握不可恢复错误的处理机制
- 学会设置 panic 行为和调试技巧
- 了解何时使用 panic 以及最佳实践
- 掌握 panic 的性能影响和优化策略

## 基本概念

### 什么是 panic

`panic!` 是 Rust 中处理不可恢复错误的机制：

- **不可恢复性**：程序无法继续正常执行
- **立即终止**：当前线程会立即停止执行
- **栈展开**：默认情况下会进行栈展开和清理
- **错误信息**：提供详细的错误信息和调用栈

### panic 的触发方式

```rust
// 1. 显式调用 panic! 宏
panic!("Something went wrong!");

// 2. 带格式化的 panic
let x = 42;
panic!("Value is {}, expected less than 10", x);

// 3. 数组越界访问
let arr = [1, 2, 3];
let _value = arr[10]; // 这会触发 panic

// 4. 除零操作
let result = 10 / 0; // 这会触发 panic

// 5. unwrap() 调用失败
let option: Option<i32> = None;
let _value = option.unwrap(); // 这会触发 panic
```

## panic 行为配置

### Cargo.toml 配置

```toml
[profile.dev]
# 开发模式下的 panic 行为
panic = "unwind"  # 默认：栈展开
# panic = "abort"   # 立即终止，不进行栈展开

[profile.release]
# 发布模式下的 panic 行为
panic = "abort"   # 通常设置为 abort 以减小二进制大小
```

### 环境变量控制

```bash
# 显示详细的 panic 信息
RUST_BACKTRACE=1 cargo run

# 显示完整的 panic 信息（包括依赖库）
RUST_BACKTRACE=full cargo run
```

## 实际应用示例

### 1. 基本 panic 使用

```rust
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
}

fn main() {
    let result = divide(10.0, 2.0);
    println!("Result: {}", result);
    
    // 这会触发 panic
    let _bad_result = divide(10.0, 0.0);
    println!("This line will never be reached");
}
```

### 2. 条件性 panic

```rust
fn validate_age(age: i32) {
    if age < 0 {
        panic!("Age cannot be negative: {}", age);
    }
    if age > 150 {
        panic!("Age seems unrealistic: {}", age);
    }
}

fn process_user_data(name: &str, age: i32) {
    if name.is_empty() {
        panic!("Name cannot be empty");
    }
    
    validate_age(age);
    
    println!("Processing user: {} (age: {})", name, age);
}

fn main() {
    process_user_data("Alice", 25); // 正常执行
    process_user_data("", 30);      // 触发 panic
}
```

### 3. 断言宏

```rust
fn calculate_square_root(x: f64) -> f64 {
    // 使用 assert! 进行前置条件检查
    assert!(x >= 0.0, "Cannot calculate square root of negative number: {}", x);
    
    x.sqrt()
}

fn test_equality() {
    let a = 5;
    let b = 3 + 2;
    
    // 断言相等
    assert_eq!(a, b);
    
    // 断言不相等
    let c = 10;
    assert_ne!(a, c);
    
    println!("All assertions passed!");
}

fn main() {
    println!("Square root of 16: {}", calculate_square_root(16.0));
    test_equality();
    
    // 这会触发 panic
    calculate_square_root(-4.0);
}
```

### 4. 自定义 panic hook

```rust
use std::panic;

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("🚨 Application panicked!");
        
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Panic message: {}", message);
        }
        
        if let Some(location) = panic_info.location() {
            eprintln!("Panic occurred at {}:{}", 
                     location.file(), 
                     location.line());
        }
        
        eprintln!("Please report this issue to the development team.");
    }));
}

fn risky_operation(value: i32) {
    if value < 0 {
        panic!("Negative values are not supported: {}", value);
    }
    println!("Processing value: {}", value);
}

fn main() {
    setup_panic_handler();
    
    risky_operation(42);  // 正常执行
    risky_operation(-1);  // 触发自定义 panic 处理
}
```

## 调试和诊断

### 1. 获取调用栈信息

```rust
use std::backtrace::Backtrace;

fn deep_function() {
    panic!("Something went wrong in deep function!");
}

fn middle_function() {
    deep_function();
}

fn top_function() {
    middle_function();
}

fn main() {
    // 设置环境变量以显示调用栈
    std::env::set_var("RUST_BACKTRACE", "1");
    
    top_function();
}
```

### 2. 条件性调试 panic

```rust
#[cfg(debug_assertions)]
macro_rules! debug_panic {
    ($($arg:tt)*) => {
        panic!($($arg)*)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_panic {
    ($($arg:tt)*) => {
        eprintln!("Debug panic (ignored in release): {}", format!($($arg)*));
    };
}

fn validate_input(value: i32) {
    if value < 0 {
        debug_panic!("Negative value detected: {}", value);
    }
}

fn main() {
    validate_input(10);  // 正常
    validate_input(-5);  // 在 debug 模式下 panic，release 模式下只打印警告
}
```

## 性能考虑

### 1. panic 的性能影响

```rust
use std::time::Instant;

// 使用 panic 的版本
fn divide_with_panic(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

// 使用 Result 的版本
fn divide_with_result(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn benchmark_error_handling() {
    let iterations = 1_000_000;
    
    // 测试正常情况下的性能
    let start = Instant::now();
    for i in 1..=iterations {
        let _ = divide_with_result(100, i % 10 + 1);
    }
    let result_time = start.elapsed();
    
    println!("Result-based error handling: {:?}", result_time);
    
    // 注意：我们不能轻易测试 panic 的性能，因为它会终止程序
    // 但通常 panic 比 Result 慢得多，特别是在栈展开时
}

fn main() {
    benchmark_error_handling();
}
```

### 2. 避免不必要的 panic

```rust
// ❌ 不好的做法：过度使用 panic
fn bad_parse_number(s: &str) -> i32 {
    s.parse().unwrap() // 解析失败时会 panic
}

// ✅ 好的做法：返回 Result
fn good_parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// ✅ 或者提供默认值
fn parse_number_with_default(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

fn main() {
    // 使用 Result 版本
    match good_parse_number("42") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // 使用默认值版本
    let num = parse_number_with_default("invalid", 0);
    println!("Number with default: {}", num);
}
```

## 最佳实践

### 1. 何时使用 panic

```rust
// ✅ 适合使用 panic 的场景

// 1. 程序逻辑错误（不应该发生的情况）
fn get_element(vec: &Vec<i32>, index: usize) -> i32 {
    if index >= vec.len() {
        panic!("Index {} is out of bounds for vector of length {}", 
               index, vec.len());
    }
    vec[index]
}

// 2. 不可恢复的系统错误
fn initialize_critical_resource() {
    let _resource = std::fs::File::open("/critical/config.toml")
        .expect("Critical configuration file is missing");
    // 如果配置文件不存在，程序无法继续运行
}

// 3. 开发阶段的占位符
fn unimplemented_feature() {
    todo!("This feature will be implemented in the next sprint");
    // 或者使用 unimplemented!()
}

// ❌ 不适合使用 panic 的场景

// 1. 用户输入错误
fn bad_validate_email(email: &str) -> bool {
    if !email.contains('@') {
        panic!("Invalid email format"); // 不好：用户输入错误不应该 panic
    }
    true
}

// ✅ 应该返回 Result
fn good_validate_email(email: &str) -> Result<bool, &'static str> {
    if !email.contains('@') {
        Err("Invalid email format")
    } else {
        Ok(true)
    }
}
```

### 2. 错误信息最佳实践

```rust
// ✅ 提供有用的错误信息
fn process_config(config: &str) {
    if config.is_empty() {
        panic!("Configuration string cannot be empty. \
                Please provide a valid configuration.");
    }
    
    let parts: Vec<&str> = config.split('=').collect();
    if parts.len() != 2 {
        panic!("Invalid configuration format: '{}'. \
                Expected format: 'key=value'", config);
    }
    
    println!("Config: {} = {}", parts[0], parts[1]);
}

// ✅ 包含上下文信息
fn read_user_data(user_id: u32) {
    let filename = format!("user_{}.json", user_id);
    let _content = std::fs::read_to_string(&filename)
        .unwrap_or_else(|e| {
            panic!("Failed to read user data for user {}: {}. \
                    File: {}", user_id, e, filename)
        });
}

fn main() {
    process_config("debug=true");
    // process_config(""); // 会提供清晰的错误信息
}
```

### 3. 测试中的 panic

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        divide_with_panic(10, 0);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero_with_message() {
        divide_with_panic(10, 0);
    }
    
    #[test]
    fn test_panic_catch() {
        let result = std::panic::catch_unwind(|| {
            panic!("Test panic");
        });
        
        assert!(result.is_err());
    }
}

fn divide_with_panic(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}
```

## 常见错误和解决方案

### 1. 过度使用 unwrap()

```rust
// ❌ 问题代码
fn bad_example() {
    let numbers = vec!["1", "2", "three", "4"];
    let parsed: Vec<i32> = numbers
        .iter()
        .map(|s| s.parse().unwrap()) // 遇到 "three" 时会 panic
        .collect();
    println!("{:?}", parsed);
}

// ✅ 改进版本
fn good_example() {
    let numbers = vec!["1", "2", "three", "4"];
    let parsed: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok()) // 忽略解析失败的项
        .collect();
    println!("{:?}", parsed); // [1, 2, 4]
}

// ✅ 或者处理错误
fn better_example() {
    let numbers = vec!["1", "2", "three", "4"];
    for (i, s) in numbers.iter().enumerate() {
        match s.parse::<i32>() {
            Ok(num) => println!("Parsed {}: {}", i, num),
            Err(e) => println!("Failed to parse '{}' at index {}: {}", s, i, e),
        }
    }
}
```

### 2. 在库代码中使用 panic

```rust
// ❌ 库代码中不应该轻易 panic
pub fn library_function(input: &str) -> String {
    if input.is_empty() {
        panic!("Input cannot be empty"); // 这会让库的用户难以处理
    }
    input.to_uppercase()
}

// ✅ 库代码应该返回 Result
pub fn better_library_function(input: &str) -> Result<String, &'static str> {
    if input.is_empty() {
        Err("Input cannot be empty")
    } else {
        Ok(input.to_uppercase())
    }
}

// ✅ 或者提供两个版本
pub fn safe_library_function(input: &str) -> Result<String, &'static str> {
    better_library_function(input)
}

pub fn unsafe_library_function(input: &str) -> String {
    better_library_function(input)
        .expect("library_function: input cannot be empty")
}
```

## 学习检查清单

完成本节学习后，你应该能够：

- [ ] 理解 panic 的基本概念和触发条件
- [ ] 掌握 panic 行为的配置方法
- [ ] 能够编写合适的 panic 错误信息
- [ ] 了解何时使用 panic，何时使用 Result
- [ ] 掌握调试 panic 的技巧和工具
- [ ] 理解 panic 的性能影响
- [ ] 能够在测试中正确处理 panic
- [ ] 避免常见的 panic 使用错误

## 扩展阅读

- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Panic](https://doc.rust-lang.org/rust-by-example/std/panic.html)
- [std::panic 模块文档](https://doc.rust-lang.org/std/panic/)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [Rust Error Handling Best Practices](https://www.lpalmieri.com/posts/error-handling-rust/)

---

**下一节预告**：我们将学习 `Result` 类型和可恢复错误的处理，这是 Rust 错误处理的核心机制。