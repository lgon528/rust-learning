# 错误处理的高级技巧

## 学习目标

通过本节学习，你将掌握：

- 理解错误链和错误上下文的概念
- 掌握 anyhow 和 thiserror 等错误处理库的使用
- 学会设计可组合的错误类型
- 了解异步环境下的错误处理
- 掌握错误处理的架构设计模式
- 学会错误监控和日志记录的最佳实践

## 错误链和上下文

### 1. 错误链的概念

错误链允许我们保留原始错误信息的同时添加上下文：

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct DatabaseError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl DatabaseError {
    fn new(message: String) -> Self {
        DatabaseError {
            message,
            source: None,
        }
    }
    
    fn with_source<E>(message: String, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        DatabaseError {
            message,
            source: Some(Box::new(source)),
        }
    }
}

// 使用错误链
fn connect_to_database() -> Result<(), DatabaseError> {
    std::fs::read_to_string("config.toml")
        .map_err(|e| DatabaseError::with_source(
            "Failed to read database config".to_string(),
            e
        ))?;
    
    // 模拟数据库连接失败
    Err(DatabaseError::new("Connection timeout".to_string()))
}

fn print_error_chain(mut err: &dyn Error) {
    eprintln!("Error: {}", err);
    while let Some(source) = err.source() {
        eprintln!("  Caused by: {}", source);
        err = source;
    }
}

fn error_chain_example() {
    if let Err(e) = connect_to_database() {
        print_error_chain(&e);
    }
}
```

### 2. 使用 anyhow 简化错误处理

```rust
// 在 Cargo.toml 中添加: anyhow = "1.0"
use anyhow::{Context, Result, anyhow};
use std::fs;

// anyhow::Result 是 Result<T, anyhow::Error> 的别名
fn read_config_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))
}

fn parse_config(content: &str) -> Result<Config> {
    if content.trim().is_empty() {
        return Err(anyhow!("Config file is empty"));
    }
    
    // 模拟解析过程
    Ok(Config {
        database_url: "postgresql://localhost/mydb".to_string(),
        port: 8080,
    })
}

#[derive(Debug)]
struct Config {
    database_url: String,
    port: u16,
}

fn load_application_config() -> Result<Config> {
    let content = read_config_file("app.toml")
        .context("Failed to load application configuration")?;
    
    let config = parse_config(&content)
        .context("Failed to parse configuration")?;
    
    // 添加验证逻辑
    if config.port < 1024 {
        return Err(anyhow!("Port {} is reserved, use port >= 1024", config.port));
    }
    
    Ok(config)
}

fn anyhow_example() {
    match load_application_config() {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            
            // 打印完整的错误链
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }
        }
    }
}
```

### 3. 使用 thiserror 定义结构化错误

```rust
// 在 Cargo.toml 中添加: thiserror = "1.0"
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("Validation failed for field '{field}': {reason}")]
    Validation { field: String, reason: String },
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid value for {field}: expected {expected}, got {actual}")]
    InvalidValue {
        field: String,
        expected: String,
        actual: String,
    },
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Configuration error")]
    Config(#[from] ConfigError),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Authentication failed: {reason}")]
    Auth { reason: String },
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection timeout")]
    Timeout,
    
    #[error("DNS resolution failed for {host}")]
    DnsFailure { host: String },
    
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
}

// 使用结构化错误
fn validate_config_field(field: &str, value: &str) -> Result<(), ConfigError> {
    match field {
        "port" => {
            let port: u16 = value.parse().map_err(|_| ConfigError::InvalidValue {
                field: field.to_string(),
                expected: "valid port number (1-65535)".to_string(),
                actual: value.to_string(),
            })?;
            
            if port < 1024 {
                return Err(ConfigError::Validation {
                    field: field.to_string(),
                    reason: "Port must be >= 1024".to_string(),
                });
            }
        }
        "database_url" => {
            if !value.starts_with("postgresql://") && !value.starts_with("mysql://") {
                return Err(ConfigError::Validation {
                    field: field.to_string(),
                    reason: "Must be a valid database URL".to_string(),
                });
            }
        }
        _ => {
            return Err(ConfigError::MissingField(field.to_string()));
        }
    }
    
    Ok(())
}

fn thiserror_example() {
    // 测试不同类型的错误
    let test_cases = [
        ("port", "80"),
        ("port", "invalid"),
        ("database_url", "invalid-url"),
        ("unknown_field", "value"),
    ];
    
    for (field, value) in &test_cases {
        match validate_config_field(field, value) {
            Ok(()) => println!("✓ {}: {} is valid", field, value),
            Err(e) => println!("✗ {}: {} - {}", field, value, e),
        }
    }
}
```

## 可组合的错误类型

### 1. 错误类型的组合模式

```rust
use std::collections::HashMap;

// 基础错误类型
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' is required")]
    Required { field: String },
    
    #[error("Field '{field}' has invalid format: {reason}")]
    InvalidFormat { field: String, reason: String },
    
    #[error("Field '{field}' is out of range: {value}")]
    OutOfRange { field: String, value: String },
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Item not found: {id}")]
    NotFound { id: String },
    
    #[error("Storage is full")]
    Full,
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum BusinessLogicError {
    #[error("Insufficient permissions for operation: {operation}")]
    InsufficientPermissions { operation: String },
    
    #[error("Business rule violation: {rule}")]
    RuleViolation { rule: String },
    
    #[error("Resource conflict: {resource}")]
    Conflict { resource: String },
}

// 组合错误类型
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Validation failed")]
    Validation(#[from] ValidationError),
    
    #[error("Storage operation failed")]
    Storage(#[from] StorageError),
    
    #[error("Business logic error")]
    BusinessLogic(#[from] BusinessLogicError),
    
    #[error("External service error: {service}")]
    ExternalService { service: String, #[source] source: Box<dyn Error + Send + Sync> },
}

// 服务层实现
struct UserService {
    storage: HashMap<String, User>,
}

#[derive(Debug, Clone)]
struct User {
    id: String,
    email: String,
    age: u32,
}

impl UserService {
    fn new() -> Self {
        UserService {
            storage: HashMap::new(),
        }
    }
    
    fn validate_user(&self, user: &User) -> Result<(), ValidationError> {
        if user.id.is_empty() {
            return Err(ValidationError::Required {
                field: "id".to_string(),
            });
        }
        
        if !user.email.contains('@') {
            return Err(ValidationError::InvalidFormat {
                field: "email".to_string(),
                reason: "Must contain @ symbol".to_string(),
            });
        }
        
        if user.age > 150 {
            return Err(ValidationError::OutOfRange {
                field: "age".to_string(),
                value: user.age.to_string(),
            });
        }
        
        Ok(())
    }
    
    fn check_business_rules(&self, user: &User) -> Result<(), BusinessLogicError> {
        if user.age < 13 {
            return Err(BusinessLogicError::RuleViolation {
                rule: "Users must be at least 13 years old".to_string(),
            });
        }
        
        if self.storage.contains_key(&user.id) {
            return Err(BusinessLogicError::Conflict {
                resource: format!("User with ID {}", user.id),
            });
        }
        
        Ok(())
    }
    
    fn store_user(&mut self, user: User) -> Result<(), StorageError> {
        if self.storage.len() >= 1000 {
            return Err(StorageError::Full);
        }
        
        self.storage.insert(user.id.clone(), user);
        Ok(())
    }
    
    fn create_user(&mut self, user: User) -> Result<(), ServiceError> {
        // 验证输入
        self.validate_user(&user)?;
        
        // 检查业务规则
        self.check_business_rules(&user)?;
        
        // 存储用户
        self.store_user(user)?;
        
        Ok(())
    }
    
    fn get_user(&self, id: &str) -> Result<&User, ServiceError> {
        self.storage.get(id)
            .ok_or_else(|| ServiceError::Storage(StorageError::NotFound {
                id: id.to_string(),
            }))
    }
}

fn composable_errors_example() {
    let mut service = UserService::new();
    
    let test_users = vec![
        User { id: "".to_string(), email: "test@example.com".to_string(), age: 25 },
        User { id: "1".to_string(), email: "invalid-email".to_string(), age: 25 },
        User { id: "2".to_string(), email: "child@example.com".to_string(), age: 10 },
        User { id: "3".to_string(), email: "valid@example.com".to_string(), age: 25 },
        User { id: "3".to_string(), email: "duplicate@example.com".to_string(), age: 30 },
    ];
    
    for user in test_users {
        match service.create_user(user.clone()) {
            Ok(()) => println!("✓ User {} created successfully", user.id),
            Err(e) => {
                println!("✗ Failed to create user {}: {}", user.id, e);
                
                // 根据错误类型提供不同的处理建议
                match &e {
                    ServiceError::Validation(v) => {
                        println!("  Suggestion: Fix the validation error and try again");
                    }
                    ServiceError::BusinessLogic(b) => {
                        println!("  Suggestion: Check business requirements");
                    }
                    ServiceError::Storage(s) => {
                        println!("  Suggestion: Check storage capacity or connectivity");
                    }
                    ServiceError::ExternalService { service, .. } => {
                        println!("  Suggestion: Check {} service status", service);
                    }
                }
            }
        }
    }
}
```

### 2. 错误转换和适配器模式

```rust
// 外部库错误的适配
#[derive(Error, Debug)]
pub enum ExternalApiError {
    #[error("HTTP client error")]
    Http(#[from] HttpError),
    
    #[error("JSON parsing error")]
    Json(#[from] JsonError),
    
    #[error("Rate limit exceeded")]
    RateLimit,
}

// 模拟外部库的错误类型
#[derive(Error, Debug)]
pub struct HttpError {
    pub status: u16,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP {}: {}", self.status, self.message)
    }
}

#[derive(Error, Debug)]
pub struct JsonError {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSON error at line {}, column {}", self.line, self.column)
    }
}

// 错误适配器
struct ErrorAdapter;

impl ErrorAdapter {
    fn adapt_http_error(status: u16, message: &str) -> ServiceError {
        match status {
            400..=499 => ServiceError::Validation(ValidationError::InvalidFormat {
                field: "request".to_string(),
                reason: message.to_string(),
            }),
            500..=599 => ServiceError::ExternalService {
                service: "HTTP API".to_string(),
                source: Box::new(HttpError {
                    status,
                    message: message.to_string(),
                }),
            },
            _ => ServiceError::ExternalService {
                service: "HTTP API".to_string(),
                source: Box::new(HttpError {
                    status,
                    message: message.to_string(),
                }),
            },
        }
    }
    
    fn adapt_json_error(line: usize, column: usize) -> ServiceError {
        ServiceError::Validation(ValidationError::InvalidFormat {
            field: "json_data".to_string(),
            reason: format!("Invalid JSON at line {}, column {}", line, column),
        })
    }
}

// 使用适配器
fn call_external_api() -> Result<String, ServiceError> {
    // 模拟外部 API 调用
    let http_result: Result<String, HttpError> = Err(HttpError {
        status: 400,
        message: "Bad Request".to_string(),
    });
    
    http_result.map_err(|e| ErrorAdapter::adapt_http_error(e.status, &e.message))
}
```

## 异步环境下的错误处理

### 1. 异步函数中的错误处理

```rust
use std::time::Duration;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum AsyncError {
    #[error("Timeout after {seconds} seconds")]
    Timeout { seconds: u64 },
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Concurrent operation failed")]
    Concurrency,
}

// 异步错误处理示例
async fn fetch_data_with_timeout(url: &str, timeout_secs: u64) -> Result<String, AsyncError> {
    let fetch_future = async {
        // 模拟网络请求
        sleep(Duration::from_secs(2)).await;
        
        if url.is_empty() {
            return Err(AsyncError::Network("Empty URL".to_string()));
        }
        
        Ok(format!("Data from {}", url))
    };
    
    let timeout_future = async {
        sleep(Duration::from_secs(timeout_secs)).await;
        Err(AsyncError::Timeout { seconds: timeout_secs })
    };
    
    // 使用 tokio::select! 处理超时
    tokio::select! {
        result = fetch_future => result,
        timeout = timeout_future => timeout,
    }
}

// 并发错误处理
async fn fetch_multiple_sources(urls: Vec<&str>) -> Result<Vec<String>, AsyncError> {
    let mut handles = Vec::new();
    
    for url in urls {
        let handle = tokio::spawn(fetch_data_with_timeout(url, 5));
        handles.push(handle);
    }
    
    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Ok(data)) => results.push(data),
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(AsyncError::Concurrency),
        }
    }
    
    Ok(results)
}

// 错误恢复和重试
async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, AsyncError> {
    let mut last_error = AsyncError::Network("No attempts made".to_string());
    
    for attempt in 1..=max_retries {
        match fetch_data_with_timeout(url, 3).await {
            Ok(data) => return Ok(data),
            Err(AsyncError::Timeout { .. }) => {
                last_error = AsyncError::Timeout { seconds: 3 };
                if attempt < max_retries {
                    let delay = Duration::from_millis(100 * attempt as u64);
                    sleep(delay).await;
                }
            }
            Err(e) => {
                last_error = e;
                break; // 对于非超时错误，不重试
            }
        }
    }
    
    Err(last_error)
}

#[tokio::main]
async fn async_error_example() {
    // 单个请求
    match fetch_data_with_timeout("https://api.example.com", 1).await {
        Ok(data) => println!("Success: {}", data),
        Err(e) => println!("Error: {}", e),
    }
    
    // 多个并发请求
    let urls = vec!["https://api1.example.com", "https://api2.example.com", ""];
    match fetch_multiple_sources(urls).await {
        Ok(results) => println!("All results: {:?}", results),
        Err(e) => println!("Failed to fetch all: {}", e),
    }
    
    // 带重试的请求
    match fetch_with_retry("https://unreliable-api.example.com", 3).await {
        Ok(data) => println!("Success after retries: {}", data),
        Err(e) => println!("Failed after retries: {}", e),
    }
}
```

### 2. 流式处理中的错误处理

```rust
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

// 自定义流，演示错误处理
struct DataStream {
    current: usize,
    max: usize,
    error_at: Option<usize>,
}

impl DataStream {
    fn new(max: usize, error_at: Option<usize>) -> Self {
        DataStream {
            current: 0,
            max,
            error_at,
        }
    }
}

impl Stream for DataStream {
    type Item = Result<String, AsyncError>;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current >= self.max {
            return Poll::Ready(None);
        }
        
        let current = self.current;
        self.current += 1;
        
        if let Some(error_at) = self.error_at {
            if current == error_at {
                return Poll::Ready(Some(Err(AsyncError::Network(
                    format!("Error at item {}", current)
                ))));
            }
        }
        
        Poll::Ready(Some(Ok(format!("Item {}", current))))
    }
}

// 流错误处理示例
async fn process_stream_with_error_handling() {
    let stream = DataStream::new(5, Some(2));
    
    tokio::pin!(stream);
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(item) => println!("Processed: {}", item),
            Err(e) => {
                println!("Stream error: {}", e);
                // 决定是否继续处理
                match e {
                    AsyncError::Network(_) => {
                        println!("Network error, continuing with next item...");
                        continue;
                    }
                    _ => {
                        println!("Fatal error, stopping stream processing");
                        break;
                    }
                }
            }
        }
    }
}

// 流的错误恢复
async fn process_stream_with_recovery() {
    let stream = DataStream::new(10, Some(3));
    
    let recovered_stream = stream.map(|result| {
        match result {
            Ok(item) => Ok(item),
            Err(AsyncError::Network(msg)) => {
                // 网络错误时返回默认值
                Ok(format!("Recovered from: {}", msg))
            }
            Err(e) => Err(e), // 其他错误继续传播
        }
    });
    
    tokio::pin!(recovered_stream);
    
    while let Some(result) = recovered_stream.next().await {
        match result {
            Ok(item) => println!("Item: {}", item),
            Err(e) => println!("Unrecoverable error: {}", e),
        }
    }
}
```

## 错误监控和日志记录

### 1. 结构化日志记录

```rust
// 在 Cargo.toml 中添加: tracing = "0.1", tracing-subscriber = "0.3"
use tracing::{error, warn, info, debug, instrument, Span};
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum MonitoredError {
    #[error("Database connection failed: {reason}")]
    DatabaseConnection { reason: String },
    
    #[error("Authentication failed for user {user_id}")]
    Authentication { user_id: String },
    
    #[error("Rate limit exceeded for IP {ip}")]
    RateLimit { ip: String },
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
}

impl MonitoredError {
    // 为错误添加监控标签
    fn severity(&self) -> &'static str {
        match self {
            MonitoredError::DatabaseConnection { .. } => "critical",
            MonitoredError::Authentication { .. } => "warning",
            MonitoredError::RateLimit { .. } => "info",
            MonitoredError::Internal { .. } => "error",
        }
    }
    
    fn error_code(&self) -> &'static str {
        match self {
            MonitoredError::DatabaseConnection { .. } => "DB_CONN_FAIL",
            MonitoredError::Authentication { .. } => "AUTH_FAIL",
            MonitoredError::RateLimit { .. } => "RATE_LIMIT",
            MonitoredError::Internal { .. } => "INTERNAL_ERROR",
        }
    }
    
    fn should_alert(&self) -> bool {
        matches!(self, 
            MonitoredError::DatabaseConnection { .. } | 
            MonitoredError::Internal { .. }
        )
    }
}

// 错误监控器
struct ErrorMonitor {
    error_counts: HashMap<String, u64>,
}

impl ErrorMonitor {
    fn new() -> Self {
        ErrorMonitor {
            error_counts: HashMap::new(),
        }
    }
    
    fn record_error(&mut self, error: &MonitoredError, context: &str) {
        let error_code = error.error_code();
        *self.error_counts.entry(error_code.to_string()).or_insert(0) += 1;
        
        // 结构化日志记录
        let span = tracing::error_span!(
            "error_occurred",
            error_code = error_code,
            severity = error.severity(),
            context = context,
            should_alert = error.should_alert()
        );
        
        let _enter = span.enter();
        
        match error.severity() {
            "critical" => error!("Critical error: {}", error),
            "error" => error!("Error: {}", error),
            "warning" => warn!("Warning: {}", error),
            "info" => info!("Info: {}", error),
            _ => debug!("Debug: {}", error),
        }
        
        // 发送告警
        if error.should_alert() {
            self.send_alert(error, context);
        }
    }
    
    fn send_alert(&self, error: &MonitoredError, context: &str) {
        // 模拟发送告警
        println!("🚨 ALERT: {} in context: {}", error, context);
    }
    
    fn get_error_stats(&self) -> &HashMap<String, u64> {
        &self.error_counts
    }
}

// 带监控的服务
struct MonitoredService {
    monitor: ErrorMonitor,
}

impl MonitoredService {
    fn new() -> Self {
        MonitoredService {
            monitor: ErrorMonitor::new(),
        }
    }
    
    #[instrument(skip(self), fields(user_id = %user_id))]
    fn authenticate_user(&mut self, user_id: &str, password: &str) -> Result<(), MonitoredError> {
        info!("Attempting to authenticate user");
        
        if password.len() < 8 {
            let error = MonitoredError::Authentication {
                user_id: user_id.to_string(),
            };
            self.monitor.record_error(&error, "user_authentication");
            return Err(error);
        }
        
        info!("User authenticated successfully");
        Ok(())
    }
    
    #[instrument(skip(self))]
    fn connect_database(&mut self) -> Result<(), MonitoredError> {
        info!("Attempting database connection");
        
        // 模拟连接失败
        let error = MonitoredError::DatabaseConnection {
            reason: "Connection timeout".to_string(),
        };
        self.monitor.record_error(&error, "database_connection");
        Err(error)
    }
    
    fn get_error_statistics(&self) -> &HashMap<String, u64> {
        self.monitor.get_error_stats()
    }
}

fn monitoring_example() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    
    let mut service = MonitoredService::new();
    
    // 模拟一些操作
    let _ = service.authenticate_user("user1", "short");
    let _ = service.authenticate_user("user2", "weak");
    let _ = service.connect_database();
    let _ = service.connect_database();
    
    // 查看错误统计
    println!("Error statistics: {:?}", service.get_error_statistics());
}
```

### 2. 错误聚合和分析

```rust
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct ErrorEvent {
    timestamp: u64,
    error_code: String,
    severity: String,
    context: String,
    message: String,
    metadata: HashMap<String, String>,
}

struct ErrorAnalyzer {
    events: Vec<ErrorEvent>,
    window_size: u64, // 时间窗口大小（秒）
}

impl ErrorAnalyzer {
    fn new(window_size: u64) -> Self {
        ErrorAnalyzer {
            events: Vec::new(),
            window_size,
        }
    }
    
    fn record_event(&mut self, error: &MonitoredError, context: &str, metadata: HashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let event = ErrorEvent {
            timestamp,
            error_code: error.error_code().to_string(),
            severity: error.severity().to_string(),
            context: context.to_string(),
            message: error.to_string(),
            metadata,
        };
        
        self.events.push(event);
        self.cleanup_old_events();
    }
    
    fn cleanup_old_events(&mut self) {
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.window_size;
        
        self.events.retain(|event| event.timestamp > cutoff);
    }
    
    fn get_error_rate(&self, error_code: &str) -> f64 {
        let count = self.events.iter()
            .filter(|event| event.error_code == error_code)
            .count();
        
        count as f64 / self.window_size as f64
    }
    
    fn detect_anomalies(&self) -> Vec<String> {
        let mut anomalies = Vec::new();
        
        // 检测错误率异常
        let mut error_counts: HashMap<String, usize> = HashMap::new();
        for event in &self.events {
            *error_counts.entry(event.error_code.clone()).or_insert(0) += 1;
        }
        
        for (error_code, count) in error_counts {
            let rate = count as f64 / self.window_size as f64;
            if rate > 0.1 { // 每秒超过 0.1 个错误
                anomalies.push(format!(
                    "High error rate for {}: {:.2} errors/second", 
                    error_code, rate
                ));
            }
        }
        
        // 检测错误突增
        let recent_events = self.events.iter()
            .filter(|event| {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                event.timestamp > now - 60 // 最近一分钟
            })
            .count();
        
        if recent_events > 10 {
            anomalies.push(format!(
                "Error spike detected: {} errors in the last minute", 
                recent_events
            ));
        }
        
        anomalies
    }
    
    fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Error Analysis Report ===\n");
        
        // 错误统计
        let mut error_counts: HashMap<String, usize> = HashMap::new();
        let mut severity_counts: HashMap<String, usize> = HashMap::new();
        
        for event in &self.events {
            *error_counts.entry(event.error_code.clone()).or_insert(0) += 1;
            *severity_counts.entry(event.severity.clone()).or_insert(0) += 1;
        }
        
        report.push_str(&format!("Total events in window: {}\n", self.events.len()));
        report.push_str("\nError counts by type:\n");
        for (error_code, count) in &error_counts {
            report.push_str(&format!("  {}: {}\n", error_code, count));
        }
        
        report.push_str("\nError counts by severity:\n");
        for (severity, count) in &severity_counts {
            report.push_str(&format!("  {}: {}\n", severity, count));
        }
        
        // 异常检测
        let anomalies = self.detect_anomalies();
        if !anomalies.is_empty() {
            report.push_str("\n🚨 Anomalies detected:\n");
            for anomaly in anomalies {
                report.push_str(&format!("  - {}\n", anomaly));
            }
        }
        
        report
    }
}

fn error_analysis_example() {
    let mut analyzer = ErrorAnalyzer::new(3600); // 1小时窗口
    
    // 模拟一些错误事件
    let errors = vec![
        MonitoredError::Authentication { user_id: "user1".to_string() },
        MonitoredError::DatabaseConnection { reason: "timeout".to_string() },
        MonitoredError::RateLimit { ip: "192.168.1.1".to_string() },
        MonitoredError::Authentication { user_id: "user2".to_string() },
        MonitoredError::Internal { message: "null pointer".to_string() },
    ];
    
    for error in errors {
        let mut metadata = HashMap::new();
        metadata.insert("service".to_string(), "web-api".to_string());
        metadata.insert("version".to_string(), "1.0.0".to_string());
        
        analyzer.record_event(&error, "api_request", metadata);
    }
    
    // 生成分析报告
    println!("{}", analyzer.generate_report());
}
```

## 学习检查清单

完成本节学习后，你应该能够：

- [ ] 理解错误链和上下文的重要性
- [ ] 熟练使用 anyhow 和 thiserror 库
- [ ] 设计可组合和可扩展的错误类型
- [ ] 处理异步环境下的错误
- [ ] 实现错误监控和日志记录
- [ ] 进行错误分析和异常检测
- [ ] 掌握错误处理的架构设计模式
- [ ] 了解错误处理的性能优化技巧

## 扩展阅读

- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [anyhow crate documentation](https://docs.rs/anyhow/)
- [thiserror crate documentation](https://docs.rs/thiserror/)
- [Rust Error Handling - A Deep Dive](https://www.lpalmieri.com/posts/error-handling-rust/)
- [Structured Logging in Rust](https://docs.rs/tracing/)
- [Async Error Handling Patterns](https://tokio.rs/tokio/topics/async)

---

**下一节预告**：我们将学习 Rust 中的泛型、trait 和生命周期，这些是 Rust 类型系统的核心特性，能够帮助我们编写更加灵活和安全的代码。