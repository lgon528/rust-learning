# 枚举定义和使用

## 学习目标

通过本节学习，你将能够：

- [ ] 理解枚举类型的概念和用途
- [ ] 掌握枚举的定义语法
- [ ] 学会使用带数据的枚举变体
- [ ] 理解枚举与结构体的区别
- [ ] 掌握 `match` 表达式处理枚举
- [ ] 学会为枚举实现方法
- [ ] 理解 `Option` 和 `Result` 枚举
- [ ] 掌握枚举的最佳实践

## 枚举基础概念

### 什么是枚举

枚举（enum）允许你通过列举可能的成员来定义一个类型。枚举在表示一个值可能是多种可能类型之一的情况时非常有用。

### 基本枚举定义

```rust
// 基本枚举：表示 IP 地址类型
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    
    // 枚举值可以作为函数参数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由 IPv4 地址"),
        IpAddrKind::V6 => println!("路由 IPv6 地址"),
    }
}
```

### 带数据的枚举

```rust
// 枚举变体可以包含数据
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 匿名结构体
    Write(String),              // 单个字符串
    ChangeColor(i32, i32, i32), // 三个整数
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("本地地址: {:?}", home);
    println!("回环地址: {:?}", loopback);
    
    // 创建不同类型的消息
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);
    
    process_message(quit);
    process_message(move_msg);
    process_message(write_msg);
    process_message(color_msg);
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出程序"),
        Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
        Message::Write(text) => println!("写入文本: {}", text),
        Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
    }
}
```

## 枚举方法实现

### 为枚举定义方法

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // 关联函数：创建红灯
    fn new_red() -> TrafficLight {
        TrafficLight::Red
    }
    
    // 方法：获取持续时间（秒）
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
    
    // 方法：获取下一个状态
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    // 方法：检查是否可以通行
    fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }
    
    // 方法：获取颜色描述
    fn color_name(&self) -> &'static str {
        match self {
            TrafficLight::Red => "红色",
            TrafficLight::Yellow => "黄色",
            TrafficLight::Green => "绿色",
        }
    }
}

fn main() {
    let mut light = TrafficLight::new_red();
    
    for _ in 0..4 {
        println!(
            "当前灯: {} ({}秒), 可通行: {}",
            light.color_name(),
            light.duration(),
            light.can_go()
        );
        light = light.next();
    }
}
```

### 复杂枚举的方法实现

```rust
#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // 关联函数：创建圆形
    fn circle(radius: f64) -> Shape {
        Shape::Circle { radius }
    }
    
    // 关联函数：创建矩形
    fn rectangle(width: f64, height: f64) -> Shape {
        Shape::Rectangle { width, height }
    }
    
    // 关联函数：创建三角形
    fn triangle(base: f64, height: f64) -> Shape {
        Shape::Triangle { base, height }
    }
    
    // 方法：计算面积
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
    
    // 方法：计算周长
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                // 假设是等腰三角形
                let side = (height * height + (base / 2.0) * (base / 2.0)).sqrt();
                base + 2.0 * side
            }
        }
    }
    
    // 方法：获取形状名称
    fn shape_name(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "圆形",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
    
    // 方法：缩放形状
    fn scale(&mut self, factor: f64) {
        match self {
            Shape::Circle { radius } => *radius *= factor,
            Shape::Rectangle { width, height } => {
                *width *= factor;
                *height *= factor;
            }
            Shape::Triangle { base, height } => {
                *base *= factor;
                *height *= factor;
            }
        }
    }
}

fn main() {
    let mut shapes = vec![
        Shape::circle(5.0),
        Shape::rectangle(4.0, 6.0),
        Shape::triangle(8.0, 6.0),
    ];
    
    println!("原始形状:");
    for shape in &shapes {
        println!(
            "{}: 面积 = {:.2}, 周长 = {:.2}",
            shape.shape_name(),
            shape.area(),
            shape.perimeter()
        );
    }
    
    // 缩放所有形状
    for shape in &mut shapes {
        shape.scale(2.0);
    }
    
    println!("\n缩放后形状:");
    for shape in &shapes {
        println!(
            "{}: 面积 = {:.2}, 周长 = {:.2}",
            shape.shape_name(),
            shape.area(),
            shape.perimeter()
        );
    }
}
```

## match 表达式详解

### 基本 match 用法

```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Quarter;
    println!("硬币价值: {} 美分", value_in_cents(coin));
}
```

### 绑定值的 match

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // ... 其他州
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的25美分硬币!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("硬币价值: {} 美分", value_in_cents(coin));
}
```

### 复杂模式匹配

```rust
#[derive(Debug)]
enum Action {
    Move { x: i32, y: i32 },
    Speak(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn process_action(action: Action) {
    match action {
        // 匹配特定值
        Action::Move { x: 0, y: 0 } => println!("原地不动"),
        
        // 匹配范围
        Action::Move { x, y } if x > 0 && y > 0 => {
            println!("向右上方移动到 ({}, {})", x, y);
        }
        
        // 一般匹配
        Action::Move { x, y } => println!("移动到 ({}, {})", x, y),
        
        // 匹配字符串内容
        Action::Speak(ref message) if message.len() > 10 => {
            println!("长消息: {}", message);
        }
        
        Action::Speak(message) => println!("说: {}", message),
        
        // 匹配颜色
        Action::ChangeColor(255, 0, 0) => println!("变成红色"),
        Action::ChangeColor(0, 255, 0) => println!("变成绿色"),
        Action::ChangeColor(0, 0, 255) => println!("变成蓝色"),
        Action::ChangeColor(r, g, b) => println!("变成自定义颜色 RGB({}, {}, {})", r, g, b),
        
        Action::Quit => println!("退出"),
    }
}

fn main() {
    let actions = vec![
        Action::Move { x: 0, y: 0 },
        Action::Move { x: 10, y: 20 },
        Action::Move { x: -5, y: 3 },
        Action::Speak(String::from("Hello")),
        Action::Speak(String::from("This is a very long message")),
        Action::ChangeColor(255, 0, 0),
        Action::ChangeColor(128, 64, 192),
        Action::Quit,
    ];
    
    for action in actions {
        process_action(action);
    }
}
```

## Option 枚举

### Option 的基本使用

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    // 使用 match 处理 Option
    match result1 {
        Some(value) => println!("结果: {}", value),
        None => println!("无法计算"),
    }
    
    match result2 {
        Some(value) => println!("结果: {}", value),
        None => println!("除零错误"),
    }
    
    // 使用 if let 简化
    if let Some(value) = result1 {
        println!("简化处理结果: {}", value);
    }
    
    // 使用 unwrap_or 提供默认值
    let safe_result1 = result1.unwrap_or(0.0);
    let safe_result2 = result2.unwrap_or(0.0);
    
    println!("安全结果1: {}", safe_result1);
    println!("安全结果2: {}", safe_result2);
}
```

### Option 的常用方法

```rust
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

fn main() {
    let text = "Hello, world! This is a test.";
    
    // 查找单词位置
    let position = find_word(text, "world");
    
    // map: 转换 Option 内的值
    let position_plus_one = position.map(|pos| pos + 1);
    println!("位置+1: {:?}", position_plus_one);
    
    // and_then: 链式操作
    let result = position
        .and_then(|pos| {
            if pos < 10 {
                Some(format!("找到单词在位置 {}", pos))
            } else {
                None
            }
        });
    println!("链式结果: {:?}", result);
    
    // filter: 过滤值
    let filtered = position.filter(|&pos| pos < 10);
    println!("过滤结果: {:?}", filtered);
    
    // or_else: 提供备选值
    let backup = find_word(text, "missing")
        .or_else(|| find_word(text, "Hello"));
    println!("备选结果: {:?}", backup);
    
    // 组合多个 Option
    let word1_pos = find_word(text, "Hello");
    let word2_pos = find_word(text, "world");
    
    // 使用 zip 组合两个 Option
    if let (Some(pos1), Some(pos2)) = (word1_pos, word2_pos) {
        println!("两个单词的距离: {}", pos2 - pos1);
    }
}
```

## Result 枚举

### Result 的基本使用

```rust
use std::num::ParseIntError;

// 自定义错误类型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn safe_divide(x: f64, y: f64) -> Result<f64, MathError> {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let num: i32 = s.parse()?; // ? 操作符传播错误
    Ok(num * 2)
}

fn main() {
    // 处理除法结果
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("除法结果: {}", result),
        Err(e) => println!("除法错误: {:?}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("除法结果: {}", result),
        Err(e) => println!("除法错误: {:?}", e),
    }
    
    // 处理平方根结果
    match safe_sqrt(-4.0) {
        Ok(result) => println!("平方根: {}", result),
        Err(e) => println!("平方根错误: {:?}", e),
    }
    
    // 使用 ? 操作符
    match parse_and_double("21") {
        Ok(result) => println!("解析并加倍: {}", result),
        Err(e) => println!("解析错误: {:?}", e),
    }
    
    match parse_and_double("not_a_number") {
        Ok(result) => println!("解析并加倍: {}", result),
        Err(e) => println!("解析错误: {:?}", e),
    }
    
    // 链式操作
    let result = safe_divide(20.0, 4.0)
        .and_then(|x| safe_sqrt(x))
        .map(|x| x * 2.0);
    
    match result {
        Ok(value) => println!("链式计算结果: {}", value),
        Err(e) => println!("链式计算错误: {:?}", e),
    }
}
```

### 错误处理的最佳实践

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// 自定义错误类型
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    InvalidData(String),
}

// 实现 From trait 以便自动转换错误
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn read_number_from_file(filename: &str) -> Result<i32, AppError> {
    let mut file = File::open(filename)?; // io::Error 自动转换为 AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidData("文件为空".to_string()));
    }
    
    let number: i32 = trimmed.parse()?; // ParseIntError 自动转换为 AppError
    Ok(number)
}

fn process_numbers(filenames: &[&str]) -> Result<Vec<i32>, AppError> {
    let mut numbers = Vec::new();
    
    for filename in filenames {
        let number = read_number_from_file(filename)?;
        numbers.push(number);
    }
    
    Ok(numbers)
}

fn main() {
    // 创建测试文件（在实际应用中这些文件应该已存在）
    std::fs::write("number1.txt", "42").unwrap();
    std::fs::write("number2.txt", "100").unwrap();
    std::fs::write("empty.txt", "").unwrap();
    
    let files = ["number1.txt", "number2.txt"];
    
    match process_numbers(&files) {
        Ok(numbers) => {
            println!("成功读取数字: {:?}", numbers);
            let sum: i32 = numbers.iter().sum();
            println!("总和: {}", sum);
        }
        Err(e) => println!("处理错误: {:?}", e),
    }
    
    // 测试错误情况
    let files_with_error = ["number1.txt", "empty.txt"];
    match process_numbers(&files_with_error) {
        Ok(numbers) => println!("数字: {:?}", numbers),
        Err(e) => println!("预期的错误: {:?}", e),
    }
    
    // 清理测试文件
    let _ = std::fs::remove_file("number1.txt");
    let _ = std::fs::remove_file("number2.txt");
    let _ = std::fs::remove_file("empty.txt");
}
```

## 实际应用示例

### 状态机实现

```rust
#[derive(Debug, Clone)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { session_id: String },
    Error { message: String },
}

#[derive(Debug)]
struct Connection {
    state: ConnectionState,
    retry_count: u32,
    max_retries: u32,
}

impl Connection {
    fn new(max_retries: u32) -> Self {
        Connection {
            state: ConnectionState::Disconnected,
            retry_count: 0,
            max_retries,
        }
    }
    
    fn connect(&mut self) -> Result<(), String> {
        match &self.state {
            ConnectionState::Disconnected => {
                self.state = ConnectionState::Connecting;
                println!("开始连接...");
                
                // 模拟连接过程
                if self.retry_count < 2 {
                    self.retry_count += 1;
                    self.state = ConnectionState::Error {
                        message: format!("连接失败，重试次数: {}", self.retry_count),
                    };
                    Err("连接失败".to_string())
                } else {
                    self.state = ConnectionState::Connected {
                        session_id: format!("session_{}", rand::random::<u32>()),
                    };
                    self.retry_count = 0;
                    Ok(())
                }
            }
            ConnectionState::Connected { .. } => {
                Err("已经连接".to_string())
            }
            _ => Err("无法从当前状态连接".to_string()),
        }
    }
    
    fn disconnect(&mut self) {
        match &self.state {
            ConnectionState::Connected { session_id } => {
                println!("断开连接，会话ID: {}", session_id);
                self.state = ConnectionState::Disconnected;
            }
            _ => println!("未连接状态，无需断开"),
        }
    }
    
    fn retry(&mut self) -> Result<(), String> {
        match &self.state {
            ConnectionState::Error { .. } => {
                if self.retry_count < self.max_retries {
                    self.state = ConnectionState::Disconnected;
                    self.connect()
                } else {
                    Err("超过最大重试次数".to_string())
                }
            }
            _ => Err("当前状态不需要重试".to_string()),
        }
    }
    
    fn get_status(&self) -> String {
        match &self.state {
            ConnectionState::Disconnected => "未连接".to_string(),
            ConnectionState::Connecting => "连接中".to_string(),
            ConnectionState::Connected { session_id } => {
                format!("已连接 (会话: {})", session_id)
            }
            ConnectionState::Error { message } => {
                format!("错误: {}", message)
            }
        }
    }
}

// 简单的随机数生成（用于演示）
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn random<T: From<u64>>() -> T {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        T::from(hasher.finish())
    }
}

fn main() {
    let mut conn = Connection::new(3);
    
    println!("初始状态: {}", conn.get_status());
    
    // 尝试连接几次
    for i in 1..=4 {
        println!("\n=== 尝试 {} ===", i);
        match conn.connect() {
            Ok(()) => println!("连接成功!"),
            Err(e) => println!("连接失败: {}", e),
        }
        println!("当前状态: {}", conn.get_status());
        
        // 如果连接失败，尝试重试
        if matches!(conn.state, ConnectionState::Error { .. }) {
            match conn.retry() {
                Ok(()) => println!("重试成功!"),
                Err(e) => println!("重试失败: {}", e),
            }
            println!("重试后状态: {}", conn.get_status());
        }
    }
    
    // 断开连接
    conn.disconnect();
    println!("\n最终状态: {}", conn.get_status());
}
```

### 配置管理系统

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

#[derive(Debug)]
enum ConfigError {
    KeyNotFound(String),
    TypeMismatch { expected: String, found: String },
    InvalidFormat(String),
}

#[derive(Debug)]
struct Config {
    data: HashMap<String, ConfigValue>,
}

impl ConfigValue {
    fn type_name(&self) -> &'static str {
        match self {
            ConfigValue::String(_) => "String",
            ConfigValue::Integer(_) => "Integer",
            ConfigValue::Float(_) => "Float",
            ConfigValue::Boolean(_) => "Boolean",
            ConfigValue::Array(_) => "Array",
            ConfigValue::Object(_) => "Object",
        }
    }
    
    fn as_string(&self) -> Result<&String, ConfigError> {
        match self {
            ConfigValue::String(s) => Ok(s),
            _ => Err(ConfigError::TypeMismatch {
                expected: "String".to_string(),
                found: self.type_name().to_string(),
            }),
        }
    }
    
    fn as_integer(&self) -> Result<i64, ConfigError> {
        match self {
            ConfigValue::Integer(i) => Ok(*i),
            _ => Err(ConfigError::TypeMismatch {
                expected: "Integer".to_string(),
                found: self.type_name().to_string(),
            }),
        }
    }
    
    fn as_boolean(&self) -> Result<bool, ConfigError> {
        match self {
            ConfigValue::Boolean(b) => Ok(*b),
            _ => Err(ConfigError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: self.type_name().to_string(),
            }),
        }
    }
    
    fn as_array(&self) -> Result<&Vec<ConfigValue>, ConfigError> {
        match self {
            ConfigValue::Array(arr) => Ok(arr),
            _ => Err(ConfigError::TypeMismatch {
                expected: "Array".to_string(),
                found: self.type_name().to_string(),
            }),
        }
    }
}

impl Config {
    fn new() -> Self {
        Config {
            data: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: ConfigValue) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Result<&ConfigValue, ConfigError> {
        self.data
            .get(key)
            .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))
    }
    
    fn get_string(&self, key: &str) -> Result<&String, ConfigError> {
        self.get(key)?.as_string()
    }
    
    fn get_integer(&self, key: &str) -> Result<i64, ConfigError> {
        self.get(key)?.as_integer()
    }
    
    fn get_boolean(&self, key: &str) -> Result<bool, ConfigError> {
        self.get(key)?.as_boolean()
    }
    
    fn get_array(&self, key: &str) -> Result<&Vec<ConfigValue>, ConfigError> {
        self.get(key)?.as_array()
    }
    
    // 获取嵌套值
    fn get_nested(&self, path: &str) -> Result<&ConfigValue, ConfigError> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = self.get(parts[0])?;
        
        for part in &parts[1..] {
            match current {
                ConfigValue::Object(obj) => {
                    current = obj
                        .get(*part)
                        .ok_or_else(|| ConfigError::KeyNotFound(part.to_string()))?;
                }
                _ => {
                    return Err(ConfigError::TypeMismatch {
                        expected: "Object".to_string(),
                        found: current.type_name().to_string(),
                    });
                }
            }
        }
        
        Ok(current)
    }
}

fn main() {
    let mut config = Config::new();
    
    // 设置各种类型的配置
    config.set("app_name".to_string(), ConfigValue::String("MyApp".to_string()));
    config.set("port".to_string(), ConfigValue::Integer(8080));
    config.set("debug".to_string(), ConfigValue::Boolean(true));
    config.set(
        "features".to_string(),
        ConfigValue::Array(vec![
            ConfigValue::String("auth".to_string()),
            ConfigValue::String("logging".to_string()),
            ConfigValue::String("metrics".to_string()),
        ]),
    );
    
    // 设置嵌套对象
    let mut database_config = HashMap::new();
    database_config.insert("host".to_string(), ConfigValue::String("localhost".to_string()));
    database_config.insert("port".to_string(), ConfigValue::Integer(5432));
    database_config.insert("name".to_string(), ConfigValue::String("mydb".to_string()));
    
    config.set("database".to_string(), ConfigValue::Object(database_config));
    
    // 读取配置
    match config.get_string("app_name") {
        Ok(name) => println!("应用名称: {}", name),
        Err(e) => println!("获取应用名称失败: {:?}", e),
    }
    
    match config.get_integer("port") {
        Ok(port) => println!("端口: {}", port),
        Err(e) => println!("获取端口失败: {:?}", e),
    }
    
    match config.get_boolean("debug") {
        Ok(debug) => println!("调试模式: {}", debug),
        Err(e) => println!("获取调试模式失败: {:?}", e),
    }
    
    match config.get_array("features") {
        Ok(features) => {
            println!("功能列表:");
            for feature in features {
                if let Ok(name) = feature.as_string() {
                    println!("  - {}", name);
                }
            }
        }
        Err(e) => println!("获取功能列表失败: {:?}", e),
    }
    
    // 获取嵌套配置
    match config.get_nested("database.host") {
        Ok(ConfigValue::String(host)) => println!("数据库主机: {}", host),
        Ok(_) => println!("数据库主机类型错误"),
        Err(e) => println!("获取数据库主机失败: {:?}", e),
    }
    
    // 测试错误情况
    match config.get_string("nonexistent") {
        Ok(value) => println!("值: {}", value),
        Err(e) => println!("预期的错误: {:?}", e),
    }
    
    match config.get_string("port") {
        Ok(value) => println!("值: {}", value),
        Err(e) => println!("预期的类型错误: {:?}", e),
    }
}
```

## 最佳实践

### 1. 枚举设计原则

```rust
// 好的设计：清晰的变体名称
#[derive(Debug)]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
    BadRequest,
}

// 好的设计：使用结构体字段而不是元组
#[derive(Debug)]
enum Event {
    UserLogin { user_id: u64, timestamp: u64 },
    UserLogout { user_id: u64, timestamp: u64 },
    DataUpdate { table: String, record_id: u64 },
}

// 避免：过于复杂的嵌套
// enum ComplexEnum {
//     Variant1(Option<Result<Vec<String>, Error>>), // 太复杂
// }

// 更好的方式：使用类型别名
type ProcessResult = Result<Vec<String>, String>;

#[derive(Debug)]
enum SimpleEnum {
    Success(ProcessResult),
    Pending,
    Failed,
}
```

### 2. 错误处理模式

```rust
// 定义应用特定的错误类型
#[derive(Debug)]
enum AppError {
    InvalidInput(String),
    NetworkError(String),
    DatabaseError(String),
    NotFound,
}

// 实现 Display trait 以便更好的错误信息
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "输入无效: {}", msg),
            AppError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            AppError::NotFound => write!(f, "资源未找到"),
        }
    }
}

// 实现 Error trait
impl std::error::Error for AppError {}

// 使用 Result 类型别名
type AppResult<T> = Result<T, AppError>;

fn process_data(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::InvalidInput("输入不能为空".to_string()));
    }
    
    // 模拟处理
    Ok(format!("处理结果: {}", input))
}
```

### 3. 文档和测试

```rust
/// 表示文件操作的结果
#[derive(Debug, PartialEq)]
enum FileOperation {
    /// 文件读取成功
    Read { bytes: usize },
    /// 文件写入成功
    Write { bytes: usize },
    /// 文件删除成功
    Delete,
    /// 操作失败
    Error { message: String },
}

impl FileOperation {
    /// 检查操作是否成功
    /// 
    /// # 示例
    /// 
    /// ```
    /// let op = FileOperation::Read { bytes: 100 };
    /// assert!(op.is_success());
    /// 
    /// let op = FileOperation::Error { message: "失败".to_string() };
    /// assert!(!op.is_success());
    /// ```
    fn is_success(&self) -> bool {
        !matches!(self, FileOperation::Error { .. })
    }
    
    /// 获取操作涉及的字节数
    fn bytes(&self) -> Option<usize> {
        match self {
            FileOperation::Read { bytes } | FileOperation::Write { bytes } => Some(*bytes),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_operation_success() {
        let read_op = FileOperation::Read { bytes: 100 };
        assert!(read_op.is_success());
        assert_eq!(read_op.bytes(), Some(100));
        
        let delete_op = FileOperation::Delete;
        assert!(delete_op.is_success());
        assert_eq!(delete_op.bytes(), None);
    }
    
    #[test]
    fn test_file_operation_error() {
        let error_op = FileOperation::Error {
            message: "文件不存在".to_string(),
        };
        assert!(!error_op.is_success());
        assert_eq!(error_op.bytes(), None);
    }
}
```

## 常见错误

### 1. 忘记处理所有变体

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

fn process_status(status: Status) {
    match status {
        Status::Active => println!("激活"),
        Status::Inactive => println!("未激活"),
        // 错误：忘记处理 Pending
        // 编译器会报错
    }
}

// 正确的做法
fn process_status_correct(status: Status) {
    match status {
        Status::Active => println!("激活"),
        Status::Inactive => println!("未激活"),
        Status::Pending => println!("待处理"),
    }
}
```

### 2. 不必要的 clone

```rust
#[derive(Debug, Clone)]
enum Data {
    Text(String),
    Number(i32),
}

fn process_data(data: &Data) {
    match data {
        // 错误：不必要的 clone
        // Data::Text(text) => println!("{}", text.clone()),
        
        // 正确：使用引用
        Data::Text(text) => println!("{}", text),
        Data::Number(num) => println!("{}", num),
    }
}
```

## 学习检查清单

完成本节学习后，请确认你能够：

- [ ] 理解枚举的概念和用途
- [ ] 定义简单和复杂的枚举类型
- [ ] 使用 `match` 表达式处理枚举
- [ ] 为枚举实现方法和关联函数
- [ ] 理解和使用 `Option` 枚举
- [ ] 理解和使用 `Result` 枚举进行错误处理
- [ ] 使用 `?` 操作符简化错误传播
- [ ] 设计合理的枚举类型
- [ ] 避免常见的枚举使用错误
- [ ] 为枚举编写测试和文档

## 扩展阅读

- [Rust Book - 枚举和模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book - 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - 枚举](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Rust Reference - 枚举](https://doc.rust-lang.org/reference/items/enumerations.html)

---

**下一节预告**：在下一节中，我们将学习模式匹配的高级用法，包括复杂模式、守卫条件和解构赋值。