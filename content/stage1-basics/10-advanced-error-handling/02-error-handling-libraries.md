# 错误处理库和工具

## 学习目标

通过本节学习，你将掌握：

- 了解 Rust 生态系统中的错误处理库
- 掌握 `anyhow` 库的使用方法
- 学会使用 `thiserror` 简化自定义错误类型
- 了解其他实用的错误处理工具
- 掌握选择合适错误处理方案的策略

## 错误处理库概览

### 主要库对比

| 库名 | 用途 | 适用场景 | 特点 |
|------|------|----------|------|
| `anyhow` | 简化错误处理 | 应用程序 | 动态错误，易用 |
| `thiserror` | 自定义错误类型 | 库开发 | 编译时类型安全 |
| `eyre` | 错误报告 | 应用程序 | 丰富的错误上下文 |
| `miette` | 诊断报告 | 编译器/工具 | 美观的错误显示 |
| `color-eyre` | 彩色错误报告 | 命令行工具 | 可视化错误信息 |

## anyhow 库

### 基本概念

`anyhow` 提供了 `anyhow::Error` 类型，可以包装任何实现了 `std::error::Error` 的错误：

```rust
use anyhow::{Result, Error, Context};

// 使用 anyhow::Result 替代 std::result::Result
fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")?;
    Ok(content)
}

// 错误转换是自动的
fn parse_number(s: &str) -> Result<i32> {
    let num = s.parse::<i32>()?; // ParseIntError 自动转换为 anyhow::Error
    Ok(num)
}
```

### 添加错误上下文

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取文件: {}", path))?;
    
    let processed = content.to_uppercase()
        .parse::<String>()
        .context("处理文件内容失败")?;
    
    Ok(processed)
}

fn main() -> Result<()> {
    match process_file("nonexistent.txt") {
        Ok(content) => println!("内容: {}", content),
        Err(e) => {
            eprintln!("错误: {}", e);
            
            // 打印错误链
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  原因: {}", err);
                source = err.source();
            }
        }
    }
    Ok(())
}
```

### 创建自定义错误

```rust
use anyhow::{anyhow, bail, ensure, Result};

fn validate_age(age: i32) -> Result<()> {
    // 使用 ensure! 宏进行条件检查
    ensure!(age >= 0, "年龄不能为负数: {}", age);
    ensure!(age <= 150, "年龄不能超过 150: {}", age);
    Ok(())
}

fn divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
        // 使用 bail! 宏提前返回错误
        bail!("除数不能为零");
    }
    Ok(a / b)
}

fn complex_operation(x: i32) -> Result<String> {
    if x < 0 {
        // 使用 anyhow! 宏创建错误
        return Err(anyhow!("输入值必须为正数，得到: {}", x));
    }
    
    let result = format!("处理结果: {}", x * 2);
    Ok(result)
}
```

### 错误降级

```rust
use anyhow::{Result, Error};
use std::io;

// 将 anyhow::Error 转换为具体错误类型
fn downcast_error(err: Error) -> io::Result<()> {
    match err.downcast::<io::Error>() {
        Ok(io_err) => Err(io_err),
        Err(other_err) => {
            // 转换为 io::Error
            Err(io::Error::new(
                io::ErrorKind::Other,
                other_err.to_string(),
            ))
        }
    }
}

// 检查错误类型
fn handle_specific_error(err: &Error) {
    if let Some(io_err) = err.downcast_ref::<io::Error>() {
        println!("这是一个 IO 错误: {}", io_err);
    } else if let Some(parse_err) = err.downcast_ref::<std::num::ParseIntError>() {
        println!("这是一个解析错误: {}", parse_err);
    } else {
        println!("其他错误: {}", err);
    }
}
```

## thiserror 库

### 基本用法

`thiserror` 通过派生宏简化自定义错误类型的创建：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("数据序列化失败")]
    Serialization,
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("无效的用户 ID: {id}")]
    InvalidUserId { id: u64 },
    
    #[error("网络超时，重试 {retry_count} 次后失败")]
    NetworkTimeout { retry_count: u32 },
    
    #[error("数据库连接失败")]
    DatabaseConnection(#[source] DatabaseError),
}

#[derive(Error, Debug)]
#[error("数据库错误: {message}")]
pub struct DatabaseError {
    message: String,
}

// 使用示例
fn save_user_data(id: u64, data: &str) -> Result<(), DataStoreError> {
    if id == 0 {
        return Err(DataStoreError::InvalidUserId { id });
    }
    
    // IO 错误会自动转换
    std::fs::write(format!("user_{}.dat", id), data)?;
    
    // 解析错误会自动转换
    let _parsed: i32 = data.parse()?;
    
    Ok(())
}
```

### 高级特性

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("配置文件 '{file}' 未找到")]
    FileNotFound { file: String },
    
    #[error("第 {line} 行格式错误: {reason}")]
    InvalidFormat { line: usize, reason: String },
    
    #[error("缺少必需的配置项 '{key}'")]
    MissingKey { key: String },
    
    #[error("配置项 '{key}' 的值无效")]
    InvalidValue {
        key: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

// 实现自定义构造函数
impl ConfigError {
    pub fn invalid_value<E>(key: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::InvalidValue {
            key: key.into(),
            source: Box::new(source),
        }
    }
}
```

### 与 anyhow 结合使用

```rust
// 库代码使用 thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("无效的输入参数")]
    InvalidInput,
    
    #[error("操作失败: {reason}")]
    OperationFailed { reason: String },
}

pub fn library_function(input: &str) -> Result<String, LibraryError> {
    if input.is_empty() {
        return Err(LibraryError::InvalidInput);
    }
    Ok(input.to_uppercase())
}

// 应用代码使用 anyhow
use anyhow::{Context, Result};

fn application_function() -> Result<()> {
    let result = library_function("")
        .context("调用库函数失败")?;
    
    println!("结果: {}", result);
    Ok(())
}
```

## 其他实用库

### eyre 库

`eyre` 是 `anyhow` 的替代品，提供更好的错误报告：

```rust
use eyre::{Result, WrapErr, eyre};

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let config = load_config()
        .wrap_err("加载配置失败")
        .wrap_err("应用程序初始化失败")?;
    
    process_data(&config)
        .wrap_err("数据处理失败")?;
    
    Ok(())
}

fn load_config() -> Result<Config> {
    let path = "config.toml";
    let content = std::fs::read_to_string(path)
        .wrap_err_with(|| format!("无法读取配置文件: {}", path))?;
    
    toml::from_str(&content)
        .wrap_err("配置文件格式错误")
}

fn process_data(config: &Config) -> Result<()> {
    if config.database_url.is_empty() {
        return Err(eyre!("数据库 URL 不能为空"));
    }
    Ok(())
}
```

### miette 库

`miette` 专门用于创建美观的诊断报告：

```rust
use miette::{Diagnostic, Result, miette};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("解析错误")]
#[diagnostic(
    code(parser::invalid_syntax),
    help("检查语法是否正确")
)]
struct ParseError {
    #[source_code]
    src: String,
    
    #[label("这里有语法错误")]
    span: (usize, usize),
}

fn parse_input(input: &str) -> Result<(), ParseError> {
    // 模拟解析错误
    if input.contains("error") {
        let error_pos = input.find("error").unwrap();
        return Err(ParseError {
            src: input.to_string(),
            span: (error_pos, error_pos + 5),
        });
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = "这是一个 error 示例";
    parse_input(input)?;
    Ok(())
}
```

## 选择策略

### 库开发

```rust
// 推荐：使用 thiserror 创建具体的错误类型
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyLibError {
    #[error("无效的参数: {param}")]
    InvalidParameter { param: String },
    
    #[error("操作超时")]
    Timeout,
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn library_function() -> Result<(), MyLibError> {
    // 库的实现
    Ok(())
}
```

### 应用开发

```rust
// 推荐：使用 anyhow 简化错误处理
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_config()
        .context("加载配置失败")?;
    
    run_application(config)
        .context("应用程序运行失败")?;
    
    Ok(())
}

fn load_config() -> Result<Config> {
    // 使用第三方库，错误自动转换
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### 混合使用

```rust
// 库层：使用 thiserror
mod my_lib {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum LibError {
        #[error("库错误: {message}")]
        Custom { message: String },
    }
    
    pub fn lib_function() -> Result<(), LibError> {
        Err(LibError::Custom {
            message: "示例错误".to_string(),
        })
    }
}

// 应用层：使用 anyhow
use anyhow::{Context, Result};

fn main() -> Result<()> {
    my_lib::lib_function()
        .context("调用库函数失败")?;
    Ok(())
}
```

## 实际项目示例

### Web 应用错误处理

```rust
use thiserror::Error;
use serde_json;
use reqwest;

#[derive(Error, Debug)]
pub enum WebAppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("HTTP 请求错误: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("验证失败: {field}")]
    Validation { field: String },
    
    #[error("用户未找到: {id}")]
    UserNotFound { id: u64 },
    
    #[error("权限不足")]
    Unauthorized,
}

// HTTP 响应转换
impl From<WebAppError> for HttpResponse {
    fn from(err: WebAppError) -> Self {
        match err {
            WebAppError::UserNotFound { .. } => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: err.to_string(),
                })
            }
            WebAppError::Validation { .. } => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: err.to_string(),
                })
            }
            WebAppError::Unauthorized => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "权限不足".to_string(),
                })
            }
            _ => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "内部服务器错误".to_string(),
                })
            }
        }
    }
}
```

### CLI 工具错误处理

```rust
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    // 安装更好的错误报告
    color_eyre::install()?;
    
    let args = Args::parse();
    
    let content = std::fs::read_to_string(&args.input)
        .with_context(|| format!("无法读取输入文件: {}", args.input))?;
    
    let processed = process_content(&content)
        .context("处理内容失败")?;
    
    let output_path = args.output.unwrap_or_else(|| {
        format!("{}.processed", args.input)
    });
    
    std::fs::write(&output_path, processed)
        .with_context(|| format!("无法写入输出文件: {}", output_path))?;
    
    println!("处理完成: {} -> {}", args.input, output_path);
    Ok(())
}

fn process_content(content: &str) -> Result<String> {
    // 模拟处理逻辑
    if content.is_empty() {
        anyhow::bail!("输入内容不能为空");
    }
    
    Ok(content.to_uppercase())
}
```

## 性能考虑

### 错误类型大小

```rust
use std::mem;

// 检查错误类型大小
#[derive(Debug)]
enum SmallError {
    A,
    B,
    C,
}

#[derive(Debug)]
enum LargeError {
    A(String),
    B(Vec<u8>),
    C { data: [u8; 1024] },
}

fn main() {
    println!("SmallError 大小: {} 字节", mem::size_of::<SmallError>());
    println!("LargeError 大小: {} 字节", mem::size_of::<LargeError>());
    println!("anyhow::Error 大小: {} 字节", mem::size_of::<anyhow::Error>());
}
```

### 优化建议

```rust
// ✅ 好的设计：使用 Box 减少枚举大小
#[derive(Debug)]
enum OptimizedError {
    Small,
    Large(Box<LargeErrorData>),
}

#[derive(Debug)]
struct LargeErrorData {
    message: String,
    context: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
}

// ✅ 好的设计：延迟格式化
#[derive(Debug)]
enum LazyError {
    Format {
        template: &'static str,
        args: Vec<String>,
    },
}

impl std::fmt::Display for LazyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyError::Format { template, args } => {
                // 只在需要时格式化
                write!(f, "{}", format_args(template, args))
            }
        }
    }
}
```

## 最佳实践总结

### 1. 选择合适的库

- **库开发**：使用 `thiserror` 创建具体的错误类型
- **应用开发**：使用 `anyhow` 简化错误处理
- **诊断工具**：使用 `miette` 或 `eyre` 提供更好的错误报告

### 2. 错误设计原则

- 提供足够的上下文信息
- 保持错误类型的大小合理
- 实现必要的 trait（Debug, Display, Error）
- 支持错误转换和链式调用

### 3. 错误处理模式

- 在库的边界使用具体的错误类型
- 在应用内部使用动态错误类型
- 提供有意义的错误消息
- 支持错误恢复和重试机制

## 学习检查清单

- [ ] 了解主要错误处理库的特点和用途
- [ ] 掌握 anyhow 库的基本用法
- [ ] 学会使用 thiserror 简化错误类型定义
- [ ] 了解 eyre 和 miette 的特殊用途
- [ ] 能够为不同场景选择合适的错误处理方案
- [ ] 掌握库和应用的错误处理最佳实践
- [ ] 了解错误处理的性能考虑

## 扩展阅读

- [anyhow 文档](https://docs.rs/anyhow/)
- [thiserror 文档](https://docs.rs/thiserror/)
- [eyre 文档](https://docs.rs/eyre/)
- [miette 文档](https://docs.rs/miette/)
- [Error Handling Survey](https://blog.yoshuawuyts.com/error-handling-survey/)
- [Rust Error Handling](https://nick.groenen.me/posts/rust-error-handling/)