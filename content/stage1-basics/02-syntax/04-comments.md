# 2.4 注释和文档

良好的注释和文档是高质量代码的重要组成部分。Rust提供了多种注释方式和强大的文档生成工具。本节将详细介绍Rust中的注释类型、文档注释的编写以及文档生成。

## 🎯 学习目标

- 掌握Rust中不同类型的注释
- 学会编写高质量的文档注释
- 了解文档测试的概念和使用
- 掌握cargo doc工具的使用
- 学习文档的最佳实践

## 💬 注释类型

### 行注释

```rust
fn main() {
    // 这是一个行注释
    let x = 5; // 行尾注释
    
    // 多行注释可以这样写
    // 每行都需要 // 开头
    // 这样可以逐行控制注释内容
    
    let y = 10;
    
    // TODO: 实现更复杂的逻辑
    // FIXME: 这里有一个已知的bug
    // NOTE: 这个算法的时间复杂度是O(n)
    // HACK: 临时解决方案，需要重构
    
    println!("x = {}, y = {}", x, y);
}
```

### 块注释

```rust
fn main() {
    /*
     * 这是一个块注释
     * 可以跨越多行
     * 通常用于较长的说明
     */
    
    let result = calculate(10, 20);
    
    /* 块注释也可以在行内使用 */ println!("结果: {}", result);
    
    /*
    块注释可以嵌套
    /* 这是嵌套的注释 */
    外层注释继续
    */
}

fn calculate(a: i32, b: i32) -> i32 {
    /*
    这个函数执行简单的加法运算
    参数:
    - a: 第一个加数
    - b: 第二个加数
    返回: 两数之和
    */
    a + b
}
```

## 📚 文档注释

### 外部文档注释（///）

```rust
/// 计算两个数的最大公约数
/// 
/// 使用欧几里得算法实现，这是一个高效的算法
/// 时间复杂度为 O(log(min(a, b)))
/// 
/// # 参数
/// 
/// * `a` - 第一个正整数
/// * `b` - 第二个正整数
/// 
/// # 返回值
/// 
/// 返回 `a` 和 `b` 的最大公约数
/// 
/// # 示例
/// 
/// ```
/// let result = gcd(48, 18);
/// assert_eq!(result, 6);
/// ```
/// 
/// # Panics
/// 
/// 当任一参数为0时会panic
/// 
/// ```should_panic
/// gcd(0, 5); // 这会panic
/// ```
fn gcd(a: u32, b: u32) -> u32 {
    assert!(a > 0 && b > 0, "参数必须大于0");
    
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 表示一个二维点的结构体
/// 
/// 这个结构体用于表示笛卡尔坐标系中的一个点，
/// 包含x和y两个坐标值。
/// 
/// # 示例
/// 
/// ```
/// let point = Point { x: 3.0, y: 4.0 };
/// let distance = point.distance_from_origin();
/// assert!((distance - 5.0).abs() < f64::EPSILON);
/// ```
#[derive(Debug, Clone, Copy)]
struct Point {
    /// 点的x坐标
    x: f64,
    /// 点的y坐标
    y: f64,
}

impl Point {
    /// 创建一个新的点
    /// 
    /// # 参数
    /// 
    /// * `x` - x坐标
    /// * `y` - y坐标
    /// 
    /// # 示例
    /// 
    /// ```
    /// let point = Point::new(1.0, 2.0);
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, 2.0);
    /// ```
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// 计算点到原点的距离
    /// 
    /// 使用欧几里得距离公式: √(x² + y²)
    /// 
    /// # 返回值
    /// 
    /// 返回到原点的距离
    /// 
    /// # 示例
    /// 
    /// ```
    /// let point = Point::new(3.0, 4.0);
    /// let distance = point.distance_from_origin();
    /// assert!((distance - 5.0).abs() < f64::EPSILON);
    /// ```
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// 计算两点之间的距离
    /// 
    /// # 参数
    /// 
    /// * `other` - 另一个点
    /// 
    /// # 返回值
    /// 
    /// 返回两点之间的欧几里得距离
    /// 
    /// # 示例
    /// 
    /// ```
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// let distance = p1.distance_to(&p2);
    /// assert!((distance - 5.0).abs() < f64::EPSILON);
    /// ```
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let result = gcd(48, 18);
    println!("最大公约数: {}", result);
    
    let point = Point::new(3.0, 4.0);
    println!("点: {:?}", point);
    println!("到原点距离: {:.2}", point.distance_from_origin());
}
```

### 内部文档注释（//!）

```rust
//! # 数学工具库
//! 
//! 这个模块提供了各种数学计算功能，包括：
//! 
//! - 基础算术运算
//! - 几何计算
//! - 统计函数
//! 
//! ## 使用示例
//! 
//! ```
//! use math_utils::*;
//! 
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! 
//! let point = Point::new(0.0, 0.0);
//! let distance = point.distance_from_origin();
//! ```
//! 
//! ## 特性
//! 
//! - 高性能实现
//! - 内存安全
//! - 全面的测试覆盖

/// 执行加法运算
/// 
/// # 示例
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 执行减法运算
/// 
/// # 示例
/// 
/// ```
/// let result = subtract(5, 3);
/// assert_eq!(result, 2);
/// ```
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 计算数组的平均值
/// 
/// # 参数
/// 
/// * `numbers` - 数字数组的切片
/// 
/// # 返回值
/// 
/// 如果数组不为空，返回 `Some(平均值)`，否则返回 `None`
/// 
/// # 示例
/// 
/// ```
/// let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let avg = average(&numbers).unwrap();
/// assert!((avg - 3.0).abs() < f64::EPSILON);
/// 
/// let empty: [f64; 0] = [];
/// assert_eq!(average(&empty), None);
/// ```
fn average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        let sum: f64 = numbers.iter().sum();
        Some(sum / numbers.len() as f64)
    }
}

fn main() {
    println!("加法: {}", add(2, 3));
    println!("减法: {}", subtract(5, 3));
    
    let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
    if let Some(avg) = average(&numbers) {
        println!("平均值: {:.2}", avg);
    }
}
```

## 🧪 文档测试

### 基础文档测试

```rust
/// 计算阶乘
/// 
/// # 参数
/// 
/// * `n` - 要计算阶乘的非负整数
/// 
/// # 返回值
/// 
/// 返回 n 的阶乘
/// 
/// # 示例
/// 
/// ```
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// 
/// let result = factorial(0);
/// assert_eq!(result, 1);
/// ```
/// 
/// # 边界情况
/// 
/// ```
/// // 0的阶乘是1
/// assert_eq!(factorial(0), 1);
/// 
/// // 1的阶乘是1
/// assert_eq!(factorial(1), 1);
/// ```
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// 安全除法运算
/// 
/// # 参数
/// 
/// * `a` - 被除数
/// * `b` - 除数
/// 
/// # 返回值
/// 
/// 如果除数不为0，返回 `Ok(商)`，否则返回错误
/// 
/// # 示例
/// 
/// ```
/// let result = safe_divide(10.0, 2.0).unwrap();
/// assert_eq!(result, 5.0);
/// ```
/// 
/// # 错误处理
/// 
/// ```
/// let result = safe_divide(10.0, 0.0);
/// assert!(result.is_err());
/// ```
fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("除数不能为零")
    } else {
        Ok(a / b)
    }
}

/// 字符串处理工具
/// 
/// # 示例
/// 
/// ```
/// let result = reverse_string("hello");
/// assert_eq!(result, "olleh");
/// 
/// let result = reverse_string("");
/// assert_eq!(result, "");
/// 
/// // 测试Unicode字符
/// let result = reverse_string("你好");
/// assert_eq!(result, "好你");
/// ```
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("5! = {}", factorial(5));
    
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    println!("反转 'hello': {}", reverse_string("hello"));
}
```

### 高级文档测试

```rust
/// 复杂的数据结构示例
/// 
/// # 示例
/// 
/// ```
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
/// 
/// assert_eq!(stack.pop(), Some(3));
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.peek(), Some(&1));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
/// 
/// # 性能测试
/// 
/// ```
/// let mut stack = Stack::new();
/// 
/// // 测试大量数据
/// for i in 0..1000 {
///     stack.push(i);
/// }
/// 
/// assert_eq!(stack.len(), 1000);
/// 
/// for i in (0..1000).rev() {
///     assert_eq!(stack.pop(), Some(i));
/// }
/// 
/// assert!(stack.is_empty());
/// ```
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建一个新的空栈
    /// 
    /// # 示例
    /// 
    /// ```
    /// let stack: Stack<i32> = Stack::new();
    /// assert!(stack.is_empty());
    /// ```
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    /// 将元素压入栈顶
    /// 
    /// # 参数
    /// 
    /// * `item` - 要压入的元素
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut stack = Stack::new();
    /// stack.push(42);
    /// assert_eq!(stack.len(), 1);
    /// ```
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    /// 从栈顶弹出元素
    /// 
    /// # 返回值
    /// 
    /// 如果栈不为空，返回 `Some(元素)`，否则返回 `None`
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut stack = Stack::new();
    /// assert_eq!(stack.pop(), None);
    /// 
    /// stack.push(42);
    /// assert_eq!(stack.pop(), Some(42));
    /// assert_eq!(stack.pop(), None);
    /// ```
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    /// 查看栈顶元素但不弹出
    /// 
    /// # 返回值
    /// 
    /// 如果栈不为空，返回栈顶元素的引用，否则返回 `None`
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut stack = Stack::new();
    /// assert_eq!(stack.peek(), None);
    /// 
    /// stack.push(42);
    /// assert_eq!(stack.peek(), Some(&42));
    /// assert_eq!(stack.len(), 1); // 元素仍在栈中
    /// ```
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    /// 检查栈是否为空
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut stack = Stack::new();
    /// assert!(stack.is_empty());
    /// 
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// 获取栈中元素的数量
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut stack = Stack::new();
    /// assert_eq!(stack.len(), 0);
    /// 
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.len(), 2);
    /// ```
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈: {:?}", stack);
    println!("弹出: {:?}", stack.pop());
    println!("栈顶: {:?}", stack.peek());
}
```

### 特殊的文档测试

```rust
/// 演示各种文档测试特性
/// 
/// # 基本测试
/// 
/// ```
/// let result = demo_function(5);
/// assert_eq!(result, 10);
/// ```
/// 
/// # 应该panic的测试
/// 
/// ```should_panic
/// demo_function(0); // 这会panic
/// ```
/// 
/// # 忽略的测试（不会运行）
/// 
/// ```ignore
/// // 这个测试需要网络连接，所以忽略
/// let response = make_network_request();
/// assert!(response.is_ok());
/// ```
/// 
/// # 不是Rust代码的示例
/// 
/// ```text
/// 这不是Rust代码，只是文本示例
/// 用于展示配置文件格式等
/// ```
/// 
/// # 编译但不运行的测试
/// 
/// ```no_run
/// // 这段代码会编译但不会运行
/// // 适用于需要特殊环境的代码
/// std::process::exit(0);
/// ```
/// 
/// # 隐藏部分代码
/// 
/// ```
/// # // 这行代码在文档中不显示，但会执行
/// # fn setup() -> i32 { 42 }
/// let value = setup();
/// assert_eq!(value, 42);
/// ```
fn demo_function(x: i32) -> i32 {
    if x == 0 {
        panic!("输入不能为0");
    }
    x * 2
}

/// 错误处理示例
/// 
/// # 示例
/// 
/// ```
/// match parse_number("42") {
///     Ok(n) => println!("解析成功: {}", n),
///     Err(e) => println!("解析失败: {}", e),
/// }
/// 
/// // 测试错误情况
/// assert!(parse_number("not_a_number").is_err());
/// ```
/// 
/// # 使用?操作符的示例
/// 
/// ```
/// fn example() -> Result<i32, Box<dyn std::error::Error>> {
///     let num = parse_number("42")?;
///     Ok(num * 2)
/// }
/// 
/// assert_eq!(example().unwrap(), 84);
/// ```
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn main() {
    println!("演示函数: {}", demo_function(5));
    
    match parse_number("42") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
}
```

## 🛠️ 文档生成和工具

### 使用cargo doc

```bash
# 生成当前项目的文档
cargo doc

# 生成文档并打开浏览器
cargo doc --open

# 包含私有项的文档
cargo doc --document-private-items

# 不包含依赖项的文档
cargo doc --no-deps

# 生成文档并运行文档测试
cargo test --doc
```

### Cargo.toml中的文档配置

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
description = "一个示例Rust项目"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
readme = "README.md"
homepage = "https://github.com/username/my_project"
repository = "https://github.com/username/my_project"
documentation = "https://docs.rs/my_project"
keywords = ["example", "tutorial", "rust"]
categories = ["development-tools"]

[package.metadata.docs.rs]
# docs.rs特定配置
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### 文档属性和配置

```rust
#![warn(missing_docs)]  // 警告缺少文档的公共项
#![deny(rustdoc::broken_intra_doc_links)]  // 禁止损坏的文档链接

//! # 项目文档配置示例
//! 
//! 这个模块展示了如何配置文档生成的各种选项。

/// 这个函数有完整的文档
/// 
/// # 示例
/// 
/// ```
/// let result = documented_function(42);
/// assert_eq!(result, 84);
/// ```
pub fn documented_function(x: i32) -> i32 {
    x * 2
}

/// 使用内部链接的函数
/// 
/// 这个函数调用了 [`documented_function`] 来处理输入。
/// 你也可以链接到结构体 [`MyStruct`] 或者模块 [`sub_module`]。
/// 
/// 外部链接示例：[Rust官网](https://www.rust-lang.org/)
pub fn function_with_links(x: i32) -> i32 {
    documented_function(x) + 1
}

/// 示例结构体
/// 
/// 这个结构体展示了如何为字段编写文档。
#[derive(Debug)]
pub struct MyStruct {
    /// 结构体的主要值
    pub value: i32,
    /// 可选的描述信息
    pub description: Option<String>,
}

impl MyStruct {
    /// 创建新的实例
    /// 
    /// # 参数
    /// 
    /// * `value` - 初始值
    /// 
    /// # 示例
    /// 
    /// ```
    /// let instance = MyStruct::new(42);
    /// assert_eq!(instance.value, 42);
    /// assert_eq!(instance.description, None);
    /// ```
    pub fn new(value: i32) -> Self {
        MyStruct {
            value,
            description: None,
        }
    }
    
    /// 设置描述信息
    /// 
    /// # 参数
    /// 
    /// * `desc` - 描述字符串
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut instance = MyStruct::new(42);
    /// instance.set_description("测试实例".to_string());
    /// assert_eq!(instance.description, Some("测试实例".to_string()));
    /// ```
    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }
}

/// 子模块示例
pub mod sub_module {
    //! 这是一个子模块
    //! 
    //! 它包含了一些辅助功能。
    
    /// 子模块中的函数
    /// 
    /// # 示例
    /// 
    /// ```
    /// use my_project::sub_module::helper_function;
    /// 
    /// let result = helper_function("test");
    /// assert_eq!(result.len(), 4);
    /// ```
    pub fn helper_function(input: &str) -> String {
        input.to_uppercase()
    }
}

fn main() {
    let result = documented_function(21);
    println!("结果: {}", result);
    
    let mut my_struct = MyStruct::new(100);
    my_struct.set_description("示例结构体".to_string());
    println!("结构体: {:?}", my_struct);
    
    let helper_result = sub_module::helper_function("hello");
    println!("辅助函数结果: {}", helper_result);
}
```

## 🧪 实践练习

### 练习1：基础文档编写

```rust
// TODO: 为以下函数添加完整的文档注释
// 包括：描述、参数、返回值、示例、可能的错误

fn calculate_bmi(weight_kg: f64, height_m: f64) -> Result<f64, String> {
    if weight_kg <= 0.0 {
        return Err("体重必须大于0".to_string());
    }
    if height_m <= 0.0 {
        return Err("身高必须大于0".to_string());
    }
    
    Ok(weight_kg / (height_m * height_m))
}

fn bmi_category(bmi: f64) -> &'static str {
    match bmi {
        bmi if bmi < 18.5 => "体重不足",
        bmi if bmi < 25.0 => "正常体重",
        bmi if bmi < 30.0 => "超重",
        _ => "肥胖",
    }
}

struct Person {
    name: String,
    age: u32,
    weight_kg: f64,
    height_m: f64,
}

impl Person {
    fn new(name: String, age: u32, weight_kg: f64, height_m: f64) -> Result<Self, String> {
        if age > 150 {
            return Err("年龄不能超过150".to_string());
        }
        if weight_kg <= 0.0 || weight_kg > 1000.0 {
            return Err("体重必须在0-1000kg之间".to_string());
        }
        if height_m <= 0.0 || height_m > 3.0 {
            return Err("身高必须在0-3m之间".to_string());
        }
        
        Ok(Person { name, age, weight_kg, height_m })
    }
    
    fn calculate_bmi(&self) -> Result<f64, String> {
        calculate_bmi(self.weight_kg, self.height_m)
    }
    
    fn health_report(&self) -> String {
        match self.calculate_bmi() {
            Ok(bmi) => {
                let category = bmi_category(bmi);
                format!("{} ({}岁): BMI = {:.1}, 分类: {}", 
                        self.name, self.age, bmi, category)
            },
            Err(e) => format!("计算BMI时出错: {}", e),
        }
    }
}

fn main() {
    // 测试你的文档
    match Person::new("张三".to_string(), 25, 70.0, 1.75) {
        Ok(person) => println!("{}", person.health_report()),
        Err(e) => println!("创建人员时出错: {}", e),
    }
}
```

### 练习2：文档测试编写

```rust
// TODO: 为以下数据结构添加完整的文档和测试

struct Calculator {
    memory: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { memory: 0.0 }
    }
    
    fn add(&mut self, value: f64) -> f64 {
        self.memory += value;
        self.memory
    }
    
    fn subtract(&mut self, value: f64) -> f64 {
        self.memory -= value;
        self.memory
    }
    
    fn multiply(&mut self, value: f64) -> f64 {
        self.memory *= value;
        self.memory
    }
    
    fn divide(&mut self, value: f64) -> Result<f64, String> {
        if value == 0.0 {
            Err("不能除以零".to_string())
        } else {
            self.memory /= value;
            Ok(self.memory)
        }
    }
    
    fn clear(&mut self) {
        self.memory = 0.0;
    }
    
    fn get_memory(&self) -> f64 {
        self.memory
    }
    
    fn set_memory(&mut self, value: f64) {
        self.memory = value;
    }
}

fn main() {
    let mut calc = Calculator::new();
    calc.add(10.0);
    calc.multiply(2.0);
    println!("计算结果: {}", calc.get_memory());
}
```

### 练习3：模块文档

```rust
// TODO: 为这个模块添加完整的模块级文档
// 包括：模块用途、使用示例、子模块说明

pub mod string_utils {
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
    
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }
    
    pub fn reverse_words(s: &str) -> String {
        s.split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
    
    pub mod validation {
        pub fn is_email(s: &str) -> bool {
            s.contains('@') && s.contains('.')
        }
        
        pub fn is_phone_number(s: &str) -> bool {
            s.chars().all(|c| c.is_ascii_digit() || c == '-' || c == ' ' || c == '+')
        }
    }
}

pub mod math_utils {
    pub fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as u64 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}

fn main() {
    println!("首字母大写: {}", string_utils::capitalize("hello world"));
    println!("单词数量: {}", string_utils::word_count("hello world rust"));
    println!("是否为邮箱: {}", string_utils::validation::is_email("test@example.com"));
    
    println!("斐波那契数列第10项: {}", math_utils::fibonacci(10));
    println!("17是质数: {}", math_utils::is_prime(17));
    println!("48和18的最大公约数: {}", math_utils::gcd(48, 18));
}
```

## 🔧 最佳实践

### 1. 文档结构

```rust
/// 简短的一行描述
/// 
/// 更详细的描述，可以包含多个段落。
/// 解释函数的用途、算法、复杂度等。
/// 
/// # 参数
/// 
/// * `param1` - 参数1的描述
/// * `param2` - 参数2的描述
/// 
/// # 返回值
/// 
/// 描述返回值的含义和类型
/// 
/// # 错误
/// 
/// 描述可能出现的错误情况
/// 
/// # 示例
/// 
/// ```
/// // 基本用法示例
/// let result = function_name(arg1, arg2);
/// assert_eq!(result, expected);
/// ```
/// 
/// # Panics
/// 
/// 描述什么情况下会panic
/// 
/// # Safety
/// 
/// 如果是unsafe函数，描述安全使用的条件
/// 
/// # 性能
/// 
/// 描述时间复杂度、空间复杂度等性能特征
fn well_documented_function(param1: i32, param2: &str) -> Result<String, &'static str> {
    if param1 < 0 {
        return Err("param1 must be non-negative");
    }
    Ok(format!("{}: {}", param2, param1))
}
```

### 2. 示例代码质量

```rust
/// 高质量的示例代码
/// 
/// # 示例
/// 
/// ## 基本用法
/// 
/// ```
/// let mut config = Config::new();
/// config.set_timeout(30);
/// config.set_retries(3);
/// 
/// assert_eq!(config.timeout(), 30);
/// assert_eq!(config.retries(), 3);
/// ```
/// 
/// ## 错误处理
/// 
/// ```
/// let mut config = Config::new();
/// 
/// // 无效的超时值
/// assert!(config.set_timeout(0).is_err());
/// 
/// // 无效的重试次数
/// assert!(config.set_retries(100).is_err());
/// ```
/// 
/// ## 链式调用
/// 
/// ```
/// let config = Config::new()
///     .with_timeout(60)
///     .with_retries(5)
///     .with_debug(true);
/// 
/// assert_eq!(config.timeout(), 60);
/// assert_eq!(config.retries(), 5);
/// assert!(config.debug());
/// ```
#[derive(Debug, Clone)]
struct Config {
    timeout: u32,
    retries: u32,
    debug: bool,
}

impl Config {
    /// 创建默认配置
    fn new() -> Self {
        Config {
            timeout: 10,
            retries: 1,
            debug: false,
        }
    }
    
    /// 设置超时时间（秒）
    fn set_timeout(&mut self, timeout: u32) -> Result<(), &'static str> {
        if timeout == 0 {
            Err("超时时间必须大于0")
        } else {
            self.timeout = timeout;
            Ok(())
        }
    }
    
    /// 设置重试次数
    fn set_retries(&mut self, retries: u32) -> Result<(), &'static str> {
        if retries > 10 {
            Err("重试次数不能超过10")
        } else {
            self.retries = retries;
            Ok(())
        }
    }
    
    /// 链式设置超时时间
    fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// 链式设置重试次数
    fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
    
    /// 链式设置调试模式
    fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    
    /// 获取超时时间
    fn timeout(&self) -> u32 {
        self.timeout
    }
    
    /// 获取重试次数
    fn retries(&self) -> u32 {
        self.retries
    }
    
    /// 获取调试模式
    fn debug(&self) -> bool {
        self.debug
    }
}

fn main() {
    let config = Config::new()
        .with_timeout(60)
        .with_retries(5)
        .with_debug(true);
    
    println!("配置: {:?}", config);
}
```

### 3. 避免的问题

```rust
// 不好的文档示例

/// 这个函数做一些事情  // 太模糊
fn bad_function1(x: i32) -> i32 {
    x + 1
}

/// 加1  // 太简单，没有提供有用信息
fn bad_function2(x: i32) -> i32 {
    x + 1
}

/// 这个函数接受一个整数参数x，然后对x执行加法操作，
/// 具体来说是将x与常数1相加，最后返回相加的结果。
/// 这个函数的时间复杂度是O(1)，空间复杂度也是O(1)。  // 太冗长
fn bad_function3(x: i32) -> i32 {
    x + 1
}

// 好的文档示例

/// 将输入值增加1
/// 
/// 这是一个简单的递增函数，常用于计数器或索引操作。
/// 
/// # 参数
/// 
/// * `x` - 要递增的整数
/// 
/// # 返回值
/// 
/// 返回 `x + 1`
/// 
/// # 示例
/// 
/// ```
/// let result = increment(5);
/// assert_eq!(result, 6);
/// 
/// let counter = increment(0);
/// assert_eq!(counter, 1);
/// ```
/// 
/// # 溢出行为
/// 
/// 在debug模式下，如果发生整数溢出会panic。
/// 在release模式下，会发生环绕（wrapping）。
/// 
/// ```should_panic
/// let result = increment(i32::MAX); // debug模式下会panic
/// ```
fn good_function(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("递增5: {}", good_function(5));
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 理解不同类型注释的用途和语法
- [ ] 编写清晰、有用的文档注释
- [ ] 使用正确的文档注释格式和结构
- [ ] 编写可执行的文档测试
- [ ] 使用cargo doc生成和查看文档
- [ ] 理解文档测试的特殊语法（should_panic、ignore等）
- [ ] 为模块、结构体、函数编写完整文档
- [ ] 避免常见的文档编写错误
- [ ] 遵循文档编写的最佳实践

## 📚 延伸阅读

- [Rust Book - Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
- [Rust Book - Documentation Tests](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
- [The rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [RFC: Documentation Comments](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html)

---

**注释和文档掌握完成！** 🎯 你现在能够编写高质量的Rust文档。

[← 上一节：函数定义和调用](./03-functions.md) | [下一节：控制流程 →](../03-control-flow/01-conditionals.md)