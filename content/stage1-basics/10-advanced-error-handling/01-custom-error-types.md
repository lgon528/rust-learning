# 自定义错误类型

## 学习目标

通过本节学习，你将掌握：

- 理解为什么需要自定义错误类型
- 掌握创建自定义错误类型的方法
- 学会实现标准错误 trait
- 了解错误类型的最佳实践
- 掌握错误转换和传播机制

## 基本概念

### 什么是自定义错误类型

自定义错误类型是为特定应用场景设计的错误表示方式，它们：

- **语义明确**：提供具体的错误信息和上下文
- **类型安全**：在编译时确保错误处理的正确性
- **可组合**：支持错误链和错误转换
- **可扩展**：便于添加新的错误变体

### 为什么需要自定义错误类型

```rust
// 使用字符串作为错误（不推荐）
fn parse_config(content: &str) -> Result<Config, String> {
    if content.is_empty() {
        return Err("配置文件为空".to_string());
    }
    // ...
}

// 使用自定义错误类型（推荐）
fn parse_config(content: &str) -> Result<Config, ConfigError> {
    if content.is_empty() {
        return Err(ConfigError::EmptyFile);
    }
    // ...
}
```

## 创建自定义错误类型

### 基础枚举错误类型

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

// 实现 Display trait
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "除零错误"),
            MathError::NegativeSquareRoot => write!(f, "负数开平方根"),
            MathError::Overflow => write!(f, "数值溢出"),
        }
    }
}

// 实现 Error trait
impl std::error::Error for MathError {}

// 使用示例
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
```

### 带数据的错误类型

```rust
#[derive(Debug)]
enum ParseError {
    InvalidFormat { line: usize, column: usize },
    UnknownField(String),
    MissingRequired(String),
    ValueError { field: String, value: String, expected: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat { line, column } => {
                write!(f, "格式错误：第 {} 行，第 {} 列", line, column)
            }
            ParseError::UnknownField(field) => {
                write!(f, "未知字段：{}", field)
            }
            ParseError::MissingRequired(field) => {
                write!(f, "缺少必需字段：{}", field)
            }
            ParseError::ValueError { field, value, expected } => {
                write!(f, "字段 '{}' 的值 '{}' 无效，期望：{}", field, value, expected)
            }
        }
    }
}

impl std::error::Error for ParseError {}
```

### 结构体错误类型

```rust
#[derive(Debug)]
struct NetworkError {
    pub code: u16,
    pub message: String,
    pub retry_after: Option<u64>,
}

impl NetworkError {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            retry_after: None,
        }
    }
    
    pub fn with_retry(mut self, seconds: u64) -> Self {
        self.retry_after = Some(seconds);
        self
    }
    
    pub fn is_retryable(&self) -> bool {
        matches!(self.code, 500..=599 | 429)
    }
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "网络错误 {}: {}", self.code, self.message)?;
        if let Some(retry_after) = self.retry_after {
            write!(f, " ({}秒后重试)", retry_after)?;
        }
        Ok(())
    }
}

impl std::error::Error for NetworkError {}
```

## 错误转换和传播

### 实现 From trait

```rust
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO 错误: {}", err),
            AppError::Parse(err) => write!(f, "解析错误: {}", err),
            AppError::Custom(msg) => write!(f, "应用错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            AppError::Custom(_) => None,
        }
    }
}

// 自动转换
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

// 使用 ? 操作符自动转换
fn read_number_from_file(path: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(path)?; // io::Error -> AppError
    let number = content.trim().parse()?; // ParseIntError -> AppError
    Ok(number)
}
```

### 错误链

```rust
#[derive(Debug)]
struct DatabaseError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl DatabaseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }
    
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "数据库错误: {}", self.message)
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

// 使用示例
fn connect_database() -> Result<(), DatabaseError> {
    std::fs::read_to_string("config.toml")
        .map_err(|e| DatabaseError::new("无法读取配置文件").with_source(e))?;
    
    // 模拟连接失败
    Err(DatabaseError::new("连接超时"))
}
```

## 实际应用示例

### 配置解析器错误

```rust
use std::collections::HashMap;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    InvalidFormat { line: usize, reason: String },
    MissingSection(String),
    InvalidValue { key: String, value: String, expected: String },
    DuplicateKey(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "配置文件未找到: {}", path)
            }
            ConfigError::InvalidFormat { line, reason } => {
                write!(f, "第 {} 行格式错误: {}", line, reason)
            }
            ConfigError::MissingSection(section) => {
                write!(f, "缺少配置节: {}", section)
            }
            ConfigError::InvalidValue { key, value, expected } => {
                write!(f, "配置项 '{}' 的值 '{}' 无效，期望: {}", key, value, expected)
            }
            ConfigError::DuplicateKey(key) => {
                write!(f, "重复的配置项: {}", key)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug: bool,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.to_string()))?;
        
        Self::parse(&content)
    }
    
    pub fn parse(content: &str) -> Result<Self, ConfigError> {
        let mut config = HashMap::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(ConfigError::InvalidFormat {
                    line: line_num + 1,
                    reason: "期望格式: key=value".to_string(),
                });
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            if config.contains_key(key) {
                return Err(ConfigError::DuplicateKey(key.to_string()));
            }
            
            config.insert(key, value);
        }
        
        // 验证必需的配置项
        let database_url = config.get("database_url")
            .ok_or_else(|| ConfigError::MissingSection("database_url".to_string()))?;
        
        let port_str = config.get("port")
            .ok_or_else(|| ConfigError::MissingSection("port".to_string()))?;
        
        let port = port_str.parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue {
                key: "port".to_string(),
                value: port_str.to_string(),
                expected: "有效的端口号 (0-65535)".to_string(),
            })?;
        
        let debug = config.get("debug")
            .map(|v| v == "true")
            .unwrap_or(false);
        
        Ok(Config {
            database_url: database_url.to_string(),
            port,
            debug,
        })
    }
}
```

### HTTP 客户端错误

```rust
#[derive(Debug)]
pub enum HttpError {
    Network(String),
    Timeout,
    InvalidUrl(String),
    BadRequest { status: u16, body: String },
    Unauthorized,
    Forbidden,
    NotFound,
    ServerError { status: u16, message: String },
    ParseError(String),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Network(msg) => write!(f, "网络错误: {}", msg),
            HttpError::Timeout => write!(f, "请求超时"),
            HttpError::InvalidUrl(url) => write!(f, "无效的 URL: {}", url),
            HttpError::BadRequest { status, body } => {
                write!(f, "请求错误 {}: {}", status, body)
            }
            HttpError::Unauthorized => write!(f, "未授权访问"),
            HttpError::Forbidden => write!(f, "访问被禁止"),
            HttpError::NotFound => write!(f, "资源未找到"),
            HttpError::ServerError { status, message } => {
                write!(f, "服务器错误 {}: {}", status, message)
            }
            HttpError::ParseError(msg) => write!(f, "解析错误: {}", msg),
        }
    }
}

impl std::error::Error for HttpError {}

impl HttpError {
    pub fn from_status(status: u16, body: String) -> Self {
        match status {
            400..=499 => match status {
                401 => HttpError::Unauthorized,
                403 => HttpError::Forbidden,
                404 => HttpError::NotFound,
                _ => HttpError::BadRequest { status, body },
            },
            500..=599 => HttpError::ServerError {
                status,
                message: body,
            },
            _ => HttpError::Network(format!("意外的状态码: {}", status)),
        }
    }
    
    pub fn is_retryable(&self) -> bool {
        matches!(self, 
            HttpError::Network(_) | 
            HttpError::Timeout | 
            HttpError::ServerError { status: 500..=599, .. }
        )
    }
}
```

## 最佳实践

### 1. 错误类型设计原则

```rust
// ✅ 好的设计：语义明确，易于处理
#[derive(Debug)]
enum ValidationError {
    TooShort { min_length: usize, actual: usize },
    TooLong { max_length: usize, actual: usize },
    InvalidCharacter { position: usize, character: char },
    EmptyInput,
}

// ❌ 不好的设计：信息不足，难以处理
#[derive(Debug)]
enum BadError {
    Error1,
    Error2,
    SomeError(String),
}
```

### 2. 实现必要的 trait

```rust
#[derive(Debug)] // 必须实现
enum MyError {
    // ...
}

// 必须实现 Display
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 提供用户友好的错误信息
    }
}

// 必须实现 Error
impl std::error::Error for MyError {
    // 可选：实现 source() 方法支持错误链
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // ...
    }
}
```

### 3. 错误转换策略

```rust
// 为常见错误类型实现 From
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

// 使用 map_err 进行特定转换
fn process_file(path: &str) -> Result<String, MyError> {
    std::fs::read_to_string(path)
        .map_err(|e| MyError::FileRead {
            path: path.to_string(),
            source: e,
        })
}
```

### 4. 错误上下文

```rust
#[derive(Debug)]
struct ContextError {
    message: String,
    context: Vec<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ContextError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
            source: None,
        }
    }
    
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }
}
```

## 常见错误

### 1. 忘记实现必要的 trait

```rust
// ❌ 错误：没有实现 Display 和 Error
#[derive(Debug)]
enum MyError {
    SomeError,
}

// ✅ 正确：实现所有必要的 trait
#[derive(Debug)]
enum MyError {
    SomeError,
}

impl std::fmt::Display for MyError { /* ... */ }
impl std::error::Error for MyError { /* ... */ }
```

### 2. 错误信息不够详细

```rust
// ❌ 错误：信息不足
Err("解析失败")

// ✅ 正确：提供详细信息
Err(ParseError::InvalidFormat {
    line: 42,
    column: 15,
    expected: "数字",
    found: "字母",
})
```

### 3. 过度使用字符串错误

```rust
// ❌ 错误：难以处理和匹配
fn process() -> Result<(), String> {
    Err("something went wrong".to_string())
}

// ✅ 正确：使用类型化错误
fn process() -> Result<(), ProcessError> {
    Err(ProcessError::InvalidInput)
}
```

## 学习检查清单

- [ ] 理解自定义错误类型的优势
- [ ] 能够创建枚举和结构体错误类型
- [ ] 掌握实现 Display 和 Error trait
- [ ] 了解错误转换和 From trait
- [ ] 能够设计错误链和上下文
- [ ] 掌握错误处理的最佳实践
- [ ] 能够为实际项目设计合适的错误类型

## 扩展阅读

- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [anyhow crate](https://docs.rs/anyhow/) - 简化错误处理
- [thiserror crate](https://docs.rs/thiserror/) - 简化自定义错误类型
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)