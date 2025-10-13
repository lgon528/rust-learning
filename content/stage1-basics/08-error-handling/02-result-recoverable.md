# Result 类型和可恢复错误

## 学习目标

通过本节学习，你将掌握：

- 理解 `Result<T, E>` 类型的设计和用途
- 掌握 Result 的各种处理方法
- 学会错误传播和 `?` 操作符的使用
- 了解自定义错误类型的设计
- 掌握错误处理的最佳实践和模式

## 基本概念

### Result 类型定义

`Result<T, E>` 是 Rust 中处理可恢复错误的核心类型：

```rust
enum Result<T, E> {
    Ok(T),   // 成功情况，包含类型 T 的值
    Err(E),  // 错误情况，包含类型 E 的错误信息
}
```

### 基本使用方式

```rust
use std::fs::File;
use std::io::ErrorKind;

fn basic_result_usage() {
    // 1. 基本的 Result 处理
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open file: {}", error),
    }
    
    // 2. 使用 if let 简化处理
    if let Ok(file) = File::open("hello.txt") {
        println!("File opened: {:?}", file);
    } else {
        println!("Failed to open file");
    }
    
    // 3. 使用 unwrap_or_else 提供默认处理
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

## Result 的常用方法

### 1. 基础方法

```rust
fn result_basic_methods() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("Something went wrong");
    
    // is_ok() 和 is_err()
    println!("Success is ok: {}", success.is_ok());     // true
    println!("Failure is err: {}", failure.is_err());   // true
    
    // unwrap() - 获取 Ok 值，Err 时 panic
    let value = success.unwrap();
    println!("Unwrapped value: {}", value);
    
    // expect() - 类似 unwrap，但可以自定义 panic 消息
    let value = Ok(100).expect("This should not fail");
    println!("Expected value: {}", value);
    
    // unwrap_or() - 提供默认值
    let value = failure.unwrap_or(0);
    println!("Value with default: {}", value);
    
    // unwrap_or_else() - 使用闭包计算默认值
    let value = failure.unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        -1
    });
    println!("Value from closure: {}", value);
}
```

### 2. 转换方法

```rust
fn result_transformation_methods() {
    let result: Result<i32, &str> = Ok(42);
    
    // map() - 转换 Ok 值
    let doubled = result.map(|x| x * 2);
    println!("Doubled: {:?}", doubled); // Ok(84)
    
    // map_err() - 转换 Err 值
    let error_result: Result<i32, &str> = Err("original error");
    let mapped_error = error_result.map_err(|e| format!("Mapped: {}", e));
    println!("Mapped error: {:?}", mapped_error);
    
    // and_then() - 链式操作（flatMap）
    let result = Ok(42)
        .and_then(|x| {
            if x > 0 {
                Ok(x * 2)
            } else {
                Err("Value must be positive")
            }
        })
        .and_then(|x| {
            if x < 100 {
                Ok(format!("Result: {}", x))
            } else {
                Err("Value too large")
            }
        });
    
    println!("Chained result: {:?}", result);
    
    // or_else() - 错误时的替代操作
    let backup_result = Err("primary failed")
        .or_else(|_| Ok("backup value"));
    println!("Backup result: {:?}", backup_result);
}
```

### 3. 组合方法

```rust
fn result_combination_methods() {
    // and() - 两个 Result 都成功时返回第二个
    let result1: Result<i32, &str> = Ok(1);
    let result2: Result<i32, &str> = Ok(2);
    let result3: Result<i32, &str> = Err("error");
    
    println!("result1.and(result2): {:?}", result1.and(result2)); // Ok(2)
    println!("result1.and(result3): {:?}", result1.and(result3)); // Err("error")
    
    // or() - 第一个成功时返回第一个，否则返回第二个
    println!("result3.or(result1): {:?}", result3.or(result1)); // Ok(1)
    
    // 处理多个 Result
    let results = vec![Ok(1), Ok(2), Err("error"), Ok(4)];
    
    // collect() - 收集所有成功的结果，遇到错误时返回第一个错误
    let collected: Result<Vec<i32>, &str> = results.into_iter().collect();
    println!("Collected: {:?}", collected); // Err("error")
    
    // 只收集成功的结果
    let results = vec![Ok(1), Ok(2), Err("error"), Ok(4)];
    let successful: Vec<i32> = results.into_iter().filter_map(Result::ok).collect();
    println!("Successful only: {:?}", successful); // [1, 2, 4]
}
```

## 错误传播和 ? 操作符

### 1. 传统的错误传播

```rust
use std::fs::File;
use std::io::{self, Read};

// 传统方式：手动传播错误
fn read_username_from_file_traditional() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 2. 使用 ? 操作符简化

```rust
// 使用 ? 操作符简化错误传播
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 进一步简化
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 最简化版本
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

### 3. ? 操作符的工作原理

```rust
// ? 操作符等价于这个宏
macro_rules! try_manual {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(From::from(err)),
        }
    };
}

fn demonstrate_question_mark() -> Result<i32, Box<dyn std::error::Error>> {
    // 这两行是等价的
    let value1 = "42".parse::<i32>()?;
    let value2 = try_manual!("42".parse::<i32>());
    
    Ok(value1 + value2)
}
```

## 自定义错误类型

### 1. 简单的自定义错误

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot calculate square root of negative number"),
            MathError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for MathError {}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn square_root(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn calculate() -> Result<f64, MathError> {
    let result = divide(10.0, 2.0)?;
    let sqrt_result = square_root(result)?;
    Ok(sqrt_result)
}
```

### 2. 使用 thiserror 简化错误定义

```rust
// 在 Cargo.toml 中添加: thiserror = "1.0"
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Data store disconnected")]
    Disconnect(#[from] io::Error),
    
    #[error("The data for key `{key}` is not available")]
    Redaction { key: String },
    
    #[error("Invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    
    #[error("Unknown data store error")]
    Unknown,
}

fn process_data(key: &str) -> Result<String, DataStoreError> {
    if key.is_empty() {
        return Err(DataStoreError::Redaction {
            key: key.to_string(),
        });
    }
    
    // 模拟数据处理
    Ok(format!("Data for key: {}", key))
}
```

### 3. 错误转换和 From trait

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Parse(err) => write!(f, "Parse error: {}", err),
            AppError::Custom(msg) => write!(f, "Application error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// 自动转换 io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

// 自动转换 ParseIntError
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn read_and_parse_number(filename: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(filename)?; // io::Error 自动转换
    let number = content.trim().parse()?; // ParseIntError 自动转换
    Ok(number)
}
```

## 实际应用示例

### 1. 文件处理系统

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
enum FileProcessError {
    FileNotFound(String),
    PermissionDenied(String),
    InvalidFormat(String),
    IoError(io::Error),
}

impl std::fmt::Display for FileProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileProcessError::FileNotFound(path) => 
                write!(f, "File not found: {}", path),
            FileProcessError::PermissionDenied(path) => 
                write!(f, "Permission denied: {}", path),
            FileProcessError::InvalidFormat(msg) => 
                write!(f, "Invalid format: {}", msg),
            FileProcessError::IoError(err) => 
                write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for FileProcessError {}

impl From<io::Error> for FileProcessError {
    fn from(error: io::Error) -> Self {
        FileProcessError::IoError(error)
    }
}

struct FileProcessor;

impl FileProcessor {
    fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, FileProcessError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        if !path.as_ref().exists() {
            return Err(FileProcessError::FileNotFound(path_str));
        }
        
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        
        for line in reader.lines() {
            lines.push(line?);
        }
        
        Ok(lines)
    }
    
    fn process_csv_line(line: &str) -> Result<Vec<String>, FileProcessError> {
        if line.trim().is_empty() {
            return Err(FileProcessError::InvalidFormat(
                "Empty line".to_string()
            ));
        }
        
        let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        
        if fields.len() < 2 {
            return Err(FileProcessError::InvalidFormat(
                format!("Line must have at least 2 fields, got {}", fields.len())
            ));
        }
        
        Ok(fields)
    }
    
    fn process_csv_file<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>, FileProcessError> {
        let lines = Self::read_lines(path)?;
        let mut records = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            match Self::process_csv_line(line) {
                Ok(fields) => records.push(fields),
                Err(e) => {
                    eprintln!("Warning: Error on line {}: {}", line_num + 1, e);
                    // 继续处理其他行，而不是立即返回错误
                }
            }
        }
        
        Ok(records)
    }
}

fn file_processing_example() {
    match FileProcessor::process_csv_file("data.csv") {
        Ok(records) => {
            println!("Successfully processed {} records", records.len());
            for (i, record) in records.iter().enumerate() {
                println!("Record {}: {:?}", i + 1, record);
            }
        }
        Err(e) => {
            eprintln!("Failed to process file: {}", e);
        }
    }
}
```

### 2. 网络请求处理

```rust
use std::collections::HashMap;

#[derive(Debug)]
enum NetworkError {
    ConnectionFailed(String),
    Timeout,
    InvalidResponse(String),
    AuthenticationFailed,
    RateLimited,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::ConnectionFailed(url) => 
                write!(f, "Failed to connect to {}", url),
            NetworkError::Timeout => 
                write!(f, "Request timed out"),
            NetworkError::InvalidResponse(msg) => 
                write!(f, "Invalid response: {}", msg),
            NetworkError::AuthenticationFailed => 
                write!(f, "Authentication failed"),
            NetworkError::RateLimited => 
                write!(f, "Rate limit exceeded"),
        }
    }
}

impl std::error::Error for NetworkError {}

struct ApiClient {
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        ApiClient {
            base_url,
            api_key: None,
        }
    }
    
    fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }
    
    fn get(&self, endpoint: &str) -> Result<String, NetworkError> {
        // 模拟网络请求
        if endpoint.is_empty() {
            return Err(NetworkError::InvalidResponse(
                "Empty endpoint".to_string()
            ));
        }
        
        if self.api_key.is_none() && endpoint.starts_with("/private") {
            return Err(NetworkError::AuthenticationFailed);
        }
        
        // 模拟成功响应
        Ok(format!("Response from {}{}", self.base_url, endpoint))
    }
    
    fn get_with_retry(&self, endpoint: &str, max_retries: u32) -> Result<String, NetworkError> {
        let mut last_error = NetworkError::ConnectionFailed("No attempts made".to_string());
        
        for attempt in 1..=max_retries {
            match self.get(endpoint) {
                Ok(response) => return Ok(response),
                Err(NetworkError::RateLimited) => {
                    if attempt < max_retries {
                        println!("Rate limited, waiting before retry {} of {}", attempt, max_retries);
                        // 在实际应用中，这里会有延迟
                        continue;
                    } else {
                        last_error = NetworkError::RateLimited;
                    }
                }
                Err(e) => {
                    last_error = e;
                    if attempt < max_retries {
                        println!("Request failed, retrying {} of {}", attempt, max_retries);
                    }
                }
            }
        }
        
        Err(last_error)
    }
}

fn network_example() {
    let client = ApiClient::new("https://api.example.com".to_string())
        .with_api_key("secret-key".to_string());
    
    // 简单请求
    match client.get("/users") {
        Ok(response) => println!("Success: {}", response),
        Err(e) => println!("Error: {}", e),
    }
    
    // 带重试的请求
    match client.get_with_retry("/data", 3) {
        Ok(response) => println!("Success after retries: {}", response),
        Err(e) => println!("Failed after retries: {}", e),
    }
}
```

### 3. 配置管理系统

```rust
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum ConfigError {
    MissingKey(String),
    InvalidValue { key: String, value: String, expected_type: String },
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::MissingKey(key) => 
                write!(f, "Missing required configuration key: {}", key),
            ConfigError::InvalidValue { key, value, expected_type } => 
                write!(f, "Invalid value '{}' for key '{}', expected {}", value, key, expected_type),
            ConfigError::ParseError(msg) => 
                write!(f, "Parse error: {}", msg),
            ConfigError::ValidationError(msg) => 
                write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

struct Config {
    values: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Config {
            values: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }
    
    fn get<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let value = self.values.get(key)
            .ok_or_else(|| ConfigError::MissingKey(key.to_string()))?;
        
        value.parse().map_err(|e| ConfigError::InvalidValue {
            key: key.to_string(),
            value: value.clone(),
            expected_type: std::any::type_name::<T>().to_string(),
        })
    }
    
    fn get_with_default<T>(&self, key: &str, default: T) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        self.get(key).unwrap_or(default)
    }
    
    fn validate(&self) -> Result<(), ConfigError> {
        // 验证必需的配置项
        let required_keys = ["database_url", "port", "log_level"];
        
        for key in &required_keys {
            if !self.values.contains_key(*key) {
                return Err(ConfigError::MissingKey(key.to_string()));
            }
        }
        
        // 验证端口范围
        let port: u16 = self.get("port")?;
        if port < 1024 {
            return Err(ConfigError::ValidationError(
                "Port must be >= 1024".to_string()
            ));
        }
        
        // 验证日志级别
        let log_level: String = self.get("log_level")?;
        let valid_levels = ["debug", "info", "warn", "error"];
        if !valid_levels.contains(&log_level.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid log level '{}', must be one of: {:?}", log_level, valid_levels)
            ));
        }
        
        Ok(())
    }
}

fn config_example() {
    let mut config = Config::new();
    config.set("database_url".to_string(), "postgresql://localhost/mydb".to_string());
    config.set("port".to_string(), "8080".to_string());
    config.set("log_level".to_string(), "info".to_string());
    config.set("max_connections".to_string(), "100".to_string());
    
    // 验证配置
    match config.validate() {
        Ok(()) => println!("Configuration is valid"),
        Err(e) => {
            println!("Configuration error: {}", e);
            return;
        }
    }
    
    // 使用配置
    let port: u16 = config.get("port").unwrap_or_else(|e| {
        println!("Warning: {}, using default port 3000", e);
        3000
    });
    
    let max_connections = config.get_with_default("max_connections", 50);
    let timeout = config.get_with_default("timeout", 30);
    
    println!("Server will run on port: {}", port);
    println!("Max connections: {}", max_connections);
    println!("Timeout: {} seconds", timeout);
}
```

## 最佳实践

### 1. 错误类型设计原则

```rust
// ✅ 好的错误设计
#[derive(Debug)]
enum UserServiceError {
    // 具体的错误类型
    UserNotFound(u32),
    InvalidEmail(String),
    DatabaseError(String),
    // 包含上下文信息
    ValidationFailed { field: String, reason: String },
    // 可以嵌套其他错误
    NetworkError(Box<dyn std::error::Error>),
}

// ❌ 避免过于泛化的错误
#[derive(Debug)]
enum BadError {
    Error(String), // 太泛化，没有提供足够信息
}
```

### 2. 错误处理策略

```rust
// 不同层级的错误处理策略

// 1. 库层：返回详细的 Result
fn library_function(input: &str) -> Result<String, LibraryError> {
    // 详细的错误处理
    if input.is_empty() {
        return Err(LibraryError::EmptyInput);
    }
    Ok(input.to_uppercase())
}

// 2. 应用层：转换和聚合错误
fn application_function(input: &str) -> Result<String, ApplicationError> {
    let result = library_function(input)
        .map_err(ApplicationError::LibraryError)?;
    
    // 添加应用层的逻辑
    Ok(format!("Processed: {}", result))
}

// 3. 用户界面层：转换为用户友好的消息
fn handle_user_request(input: &str) {
    match application_function(input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => {
            let user_message = match e {
                ApplicationError::LibraryError(LibraryError::EmptyInput) => 
                    "Please provide some input",
                _ => "An error occurred, please try again",
            };
            println!("Error: {}", user_message);
        }
    }
}

#[derive(Debug)]
enum LibraryError {
    EmptyInput,
}

#[derive(Debug)]
enum ApplicationError {
    LibraryError(LibraryError),
}
```

### 3. 性能优化

```rust
// 避免不必要的字符串分配
#[derive(Debug)]
enum OptimizedError {
    // 使用 &'static str 而不是 String
    InvalidFormat(&'static str),
    // 只在需要时分配字符串
    CustomMessage(String),
    // 使用数字代码减少内存使用
    ErrorCode(u32),
}

// 使用 Box<dyn Error> 减少枚举大小
#[derive(Debug)]
enum CompactError {
    Simple(&'static str),
    Complex(Box<ComplexErrorInfo>),
}

#[derive(Debug)]
struct ComplexErrorInfo {
    code: u32,
    message: String,
    context: HashMap<String, String>,
}
```

## 学习检查清单

完成本节学习后，你应该能够：

- [ ] 理解 Result 类型的设计和用途
- [ ] 掌握 Result 的各种处理方法
- [ ] 熟练使用 ? 操作符进行错误传播
- [ ] 能够设计合适的自定义错误类型
- [ ] 理解错误转换和 From trait 的使用
- [ ] 掌握不同层级的错误处理策略
- [ ] 了解错误处理的性能考虑
- [ ] 能够编写健壮的错误处理代码

## 扩展阅读

- [The Rust Programming Language - Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Rust by Example - Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [std::result 模块文档](https://doc.rust-lang.org/std/result/)
- [thiserror crate](https://docs.rs/thiserror/)
- [anyhow crate](https://docs.rs/anyhow/)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)

---

**下一节预告**：我们将学习错误处理的高级技巧，包括错误链、自定义错误类型的最佳实践，以及在实际项目中的错误处理架构设计。