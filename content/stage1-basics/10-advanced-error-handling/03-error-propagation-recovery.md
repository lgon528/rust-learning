# 错误传播和恢复策略

## 学习目标

通过本节学习，你将掌握：

- 理解错误传播的机制和最佳实践
- 掌握多种错误恢复策略
- 学会设计健壮的错误处理架构
- 了解异步环境下的错误处理
- 掌握错误监控和日志记录

## 错误传播机制

### 基本传播模式

```rust
use std::fs;
use std::io;

// 1. 直接传播 - 使用 ? 操作符
fn read_config_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?; // 直接传播 IO 错误
    Ok(content)
}

// 2. 转换传播 - 实现 From trait
#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?; // io::Error 自动转换为 ConfigError
    
    let config = parse_config(&content)
        .map_err(|e| ConfigError::Parse(e.to_string()))?; // 手动转换
    
    Ok(config)
}

// 3. 上下文传播 - 添加错误上下文
use anyhow::{Context, Result};

fn load_user_config(user_id: u64) -> Result<Config> {
    let config_path = format!("/etc/app/users/{}.toml", user_id);
    
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("无法读取用户 {} 的配置文件: {}", user_id, config_path))?;
    
    let config = parse_config(&content)
        .with_context(|| format!("解析用户 {} 的配置失败", user_id))?;
    
    Ok(config)
}
```

### 错误边界设计

```rust
use thiserror::Error;

// 定义不同层次的错误类型

// 1. 数据访问层错误
#[derive(Error, Debug)]
pub enum DataAccessError {
    #[error("数据库连接失败: {0}")]
    ConnectionFailed(#[from] sqlx::Error),
    
    #[error("记录未找到: {table}.{id}")]
    RecordNotFound { table: String, id: String },
    
    #[error("数据验证失败: {field}")]
    ValidationFailed { field: String },
}

// 2. 业务逻辑层错误
#[derive(Error, Debug)]
pub enum BusinessLogicError {
    #[error("数据访问错误")]
    DataAccess(#[from] DataAccessError),
    
    #[error("业务规则违反: {rule}")]
    BusinessRuleViolation { rule: String },
    
    #[error("权限不足: 需要 {required}, 当前 {current}")]
    InsufficientPermissions { required: String, current: String },
}

// 3. API 层错误
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("业务逻辑错误")]
    BusinessLogic(#[from] BusinessLogicError),
    
    #[error("请求格式错误: {0}")]
    InvalidRequest(String),
    
    #[error("认证失败")]
    AuthenticationFailed,
    
    #[error("服务不可用")]
    ServiceUnavailable,
}

// 错误边界处理
mod user_service {
    use super::*;
    
    pub struct UserService {
        db: Database,
    }
    
    impl UserService {
        // 数据访问层
        async fn find_user_by_id(&self, id: u64) -> Result<User, DataAccessError> {
            let user = self.db.query_user(id).await
                .map_err(DataAccessError::ConnectionFailed)?;
            
            user.ok_or_else(|| DataAccessError::RecordNotFound {
                table: "users".to_string(),
                id: id.to_string(),
            })
        }
        
        // 业务逻辑层
        pub async fn update_user_email(
            &self, 
            user_id: u64, 
            new_email: &str,
            current_user: &User,
        ) -> Result<(), BusinessLogicError> {
            // 权限检查
            if current_user.id != user_id && !current_user.is_admin() {
                return Err(BusinessLogicError::InsufficientPermissions {
                    required: "admin or self".to_string(),
                    current: current_user.role.clone(),
                });
            }
            
            // 业务规则检查
            if !is_valid_email(new_email) {
                return Err(BusinessLogicError::BusinessRuleViolation {
                    rule: "valid email format required".to_string(),
                });
            }
            
            // 数据访问（错误自动传播）
            let mut user = self.find_user_by_id(user_id).await?;
            user.email = new_email.to_string();
            self.db.save_user(&user).await
                .map_err(DataAccessError::ConnectionFailed)?;
            
            Ok(())
        }
    }
}

// API 层错误处理
mod api_handlers {
    use super::*;
    use serde_json::json;
    
    pub async fn update_user_email_handler(
        user_id: u64,
        request: UpdateEmailRequest,
        current_user: User,
        service: UserService,
    ) -> Result<JsonResponse, ApiError> {
        // 请求验证
        if request.email.is_empty() {
            return Err(ApiError::InvalidRequest(
                "email field is required".to_string()
            ));
        }
        
        // 调用业务逻辑（错误自动传播）
        service.update_user_email(user_id, &request.email, &current_user).await?;
        
        Ok(JsonResponse::new(json!({
            "message": "Email updated successfully"
        })))
    }
    
    // 统一错误处理
    impl From<ApiError> for HttpResponse {
        fn from(err: ApiError) -> Self {
            match err {
                ApiError::InvalidRequest(msg) => {
                    HttpResponse::BadRequest().json(json!({
                        "error": "invalid_request",
                        "message": msg
                    }))
                }
                ApiError::AuthenticationFailed => {
                    HttpResponse::Unauthorized().json(json!({
                        "error": "authentication_failed",
                        "message": "Authentication required"
                    }))
                }
                ApiError::BusinessLogic(BusinessLogicError::InsufficientPermissions { .. }) => {
                    HttpResponse::Forbidden().json(json!({
                        "error": "insufficient_permissions",
                        "message": err.to_string()
                    }))
                }
                ApiError::BusinessLogic(BusinessLogicError::DataAccess(
                    DataAccessError::RecordNotFound { .. }
                )) => {
                    HttpResponse::NotFound().json(json!({
                        "error": "not_found",
                        "message": err.to_string()
                    }))
                }
                _ => {
                    // 记录内部错误
                    log::error!("Internal server error: {:?}", err);
                    HttpResponse::InternalServerError().json(json!({
                        "error": "internal_server_error",
                        "message": "An internal error occurred"
                    }))
                }
            }
        }
    }
}
```

## 错误恢复策略

### 1. 重试机制

```rust
use std::time::Duration;
use tokio::time::sleep;

// 基本重试
async fn retry_operation<F, T, E>(
    mut operation: F,
    max_retries: usize,
    delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempts += 1;
                if attempts > max_retries {
                    log::error!("操作失败，已重试 {} 次: {:?}", max_retries, err);
                    return Err(err);
                }
                
                log::warn!("操作失败，第 {} 次重试: {:?}", attempts, err);
                sleep(delay).await;
            }
        }
    }
}

// 指数退避重试
async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    max_retries: usize,
    initial_delay: Duration,
    max_delay: Duration,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    let mut delay = initial_delay;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempts += 1;
                if attempts > max_retries {
                    return Err(err);
                }
                
                log::warn!("重试第 {} 次，延迟 {:?}: {:?}", attempts, delay, err);
                sleep(delay).await;
                
                // 指数退避
                delay = std::cmp::min(delay * 2, max_delay);
            }
        }
    }
}

// 条件重试
#[derive(Debug)]
enum NetworkError {
    Timeout,
    ConnectionRefused,
    ServerError(u16),
    ClientError(u16),
}

impl NetworkError {
    fn is_retryable(&self) -> bool {
        match self {
            NetworkError::Timeout => true,
            NetworkError::ConnectionRefused => true,
            NetworkError::ServerError(code) => *code >= 500,
            NetworkError::ClientError(_) => false,
        }
    }
}

async fn retry_network_request<F, Fut, T>(
    operation: F,
    max_retries: usize,
) -> Result<T, NetworkError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, NetworkError>>,
{
    let mut attempts = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                if !err.is_retryable() {
                    log::error!("不可重试的错误: {:?}", err);
                    return Err(err);
                }
                
                attempts += 1;
                if attempts > max_retries {
                    log::error!("重试次数已达上限: {:?}", err);
                    return Err(err);
                }
                
                let delay = Duration::from_millis(100 * attempts as u64);
                sleep(delay).await;
            }
        }
    }
}
```

### 2. 降级和回退

```rust
use serde_json::Value;

// 服务降级
struct RecommendationService {
    primary_service: PrimaryRecommendationService,
    fallback_service: FallbackRecommendationService,
    cache: Cache<String, Vec<Recommendation>>,
}

impl RecommendationService {
    async fn get_recommendations(&self, user_id: &str) -> Vec<Recommendation> {
        // 1. 尝试主服务
        match self.primary_service.get_recommendations(user_id).await {
            Ok(recommendations) => {
                // 缓存成功结果
                self.cache.set(user_id.to_string(), recommendations.clone()).await;
                return recommendations;
            }
            Err(err) => {
                log::warn!("主推荐服务失败: {:?}", err);
            }
        }
        
        // 2. 尝试备用服务
        match self.fallback_service.get_recommendations(user_id).await {
            Ok(recommendations) => {
                log::info!("使用备用推荐服务");
                return recommendations;
            }
            Err(err) => {
                log::warn!("备用推荐服务失败: {:?}", err);
            }
        }
        
        // 3. 尝试缓存
        if let Some(cached) = self.cache.get(user_id).await {
            log::info!("使用缓存的推荐结果");
            return cached;
        }
        
        // 4. 返回默认推荐
        log::info!("使用默认推荐");
        self.get_default_recommendations()
    }
    
    fn get_default_recommendations(&self) -> Vec<Recommendation> {
        // 返回热门或默认推荐
        vec![
            Recommendation {
                id: "default-1".to_string(),
                title: "热门推荐".to_string(),
                score: 0.8,
            }
        ]
    }
}

// 配置降级
struct ConfigService {
    remote_config: RemoteConfigService,
    local_config: LocalConfigService,
    default_config: Config,
}

impl ConfigService {
    async fn get_config(&self, key: &str) -> Value {
        // 1. 远程配置
        if let Ok(value) = self.remote_config.get(key).await {
            return value;
        }
        
        // 2. 本地配置
        if let Ok(value) = self.local_config.get(key) {
            log::info!("使用本地配置: {}", key);
            return value;
        }
        
        // 3. 默认配置
        log::info!("使用默认配置: {}", key);
        self.default_config.get(key).unwrap_or(Value::Null)
    }
}
```

### 3. 断路器模式

```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum CircuitState {
    Closed,    // 正常状态
    Open,      // 断路状态
    HalfOpen,  // 半开状态
}

struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_count: Arc<Mutex<usize>>,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
    failure_threshold: usize,
    recovery_timeout: Duration,
    success_threshold: usize,
    half_open_success_count: Arc<Mutex<usize>>,
}

impl CircuitBreaker {
    fn new(
        failure_threshold: usize,
        recovery_timeout: Duration,
        success_threshold: usize,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(Mutex::new(0)),
            last_failure_time: Arc::new(Mutex::new(None)),
            failure_threshold,
            recovery_timeout,
            success_threshold,
            half_open_success_count: Arc::new(Mutex::new(0)),
        }
    }
    
    async fn call<F, Fut, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        // 检查断路器状态
        match self.get_state() {
            CircuitState::Open => {
                if self.should_attempt_reset() {
                    self.set_state(CircuitState::HalfOpen);
                } else {
                    return Err(CircuitBreakerError::CircuitOpen);
                }
            }
            CircuitState::HalfOpen => {
                // 半开状态下限制并发
            }
            CircuitState::Closed => {
                // 正常执行
            }
        }
        
        // 执行操作
        match operation().await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(err) => {
                self.on_failure();
                Err(CircuitBreakerError::OperationFailed(err))
            }
        }
    }
    
    fn get_state(&self) -> CircuitState {
        self.state.lock().unwrap().clone()
    }
    
    fn set_state(&self, new_state: CircuitState) {
        *self.state.lock().unwrap() = new_state;
    }
    
    fn should_attempt_reset(&self) -> bool {
        if let Some(last_failure) = *self.last_failure_time.lock().unwrap() {
            Instant::now().duration_since(last_failure) > self.recovery_timeout
        } else {
            false
        }
    }
    
    fn on_success(&self) {
        match self.get_state() {
            CircuitState::HalfOpen => {
                let mut success_count = self.half_open_success_count.lock().unwrap();
                *success_count += 1;
                
                if *success_count >= self.success_threshold {
                    self.set_state(CircuitState::Closed);
                    *self.failure_count.lock().unwrap() = 0;
                    *success_count = 0;
                    log::info!("断路器恢复到关闭状态");
                }
            }
            CircuitState::Closed => {
                *self.failure_count.lock().unwrap() = 0;
            }
            _ => {}
        }
    }
    
    fn on_failure(&self) {
        let mut failure_count = self.failure_count.lock().unwrap();
        *failure_count += 1;
        *self.last_failure_time.lock().unwrap() = Some(Instant::now());
        
        if *failure_count >= self.failure_threshold {
            self.set_state(CircuitState::Open);
            log::warn!("断路器打开，失败次数: {}", *failure_count);
        }
        
        if matches!(self.get_state(), CircuitState::HalfOpen) {
            self.set_state(CircuitState::Open);
            *self.half_open_success_count.lock().unwrap() = 0;
            log::warn!("半开状态下失败，断路器重新打开");
        }
    }
}

#[derive(Debug)]
enum CircuitBreakerError<E> {
    CircuitOpen,
    OperationFailed(E),
}

// 使用示例
struct ExternalService {
    circuit_breaker: CircuitBreaker,
    client: HttpClient,
}

impl ExternalService {
    async fn call_api(&self, request: ApiRequest) -> Result<ApiResponse, CircuitBreakerError<ApiError>> {
        self.circuit_breaker.call(|| async {
            self.client.post("/api/endpoint")
                .json(&request)
                .send()
                .await
                .map_err(ApiError::Network)?
                .json::<ApiResponse>()
                .await
                .map_err(ApiError::Deserialization)
        }).await
    }
}
```

### 4. 超时和取消

```rust
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

// 超时处理
async fn with_timeout<F, T>(
    operation: F,
    timeout_duration: Duration,
) -> Result<T, TimeoutError>
where
    F: std::future::Future<Output = T>,
{
    match timeout(timeout_duration, operation).await {
        Ok(result) => Ok(result),
        Err(_) => Err(TimeoutError::Timeout),
    }
}

// 可取消的操作
async fn cancellable_operation(
    token: CancellationToken,
    work: impl std::future::Future<Output = Result<String, WorkError>>,
) -> Result<String, OperationError> {
    tokio::select! {
        result = work => {
            result.map_err(OperationError::Work)
        }
        _ = token.cancelled() => {
            log::info!("操作被取消");
            Err(OperationError::Cancelled)
        }
    }
}

// 组合超时和取消
struct TaskManager {
    cancellation_token: CancellationToken,
}

impl TaskManager {
    async fn execute_with_timeout<F, T>(
        &self,
        operation: F,
        timeout_duration: Duration,
    ) -> Result<T, TaskError>
    where
        F: std::future::Future<Output = Result<T, TaskError>>,
    {
        let timeout_future = timeout(timeout_duration, operation);
        let cancellation_future = self.cancellation_token.cancelled();
        
        tokio::select! {
            result = timeout_future => {
                match result {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(err)) => Err(err),
                    Err(_) => Err(TaskError::Timeout),
                }
            }
            _ = cancellation_future => {
                Err(TaskError::Cancelled)
            }
        }
    }
    
    fn cancel(&self) {
        self.cancellation_token.cancel();
    }
}
```

## 异步错误处理

### 并发错误处理

```rust
use futures::future::{join_all, try_join_all};
use tokio::task::JoinHandle;

// 并行执行，收集所有错误
async fn process_items_collect_errors<T>(
    items: Vec<T>,
    processor: impl Fn(T) -> BoxFuture<'static, Result<ProcessResult, ProcessError>> + Clone,
) -> (Vec<ProcessResult>, Vec<ProcessError>) {
    let futures: Vec<_> = items.into_iter()
        .map(|item| processor(item))
        .collect();
    
    let results = join_all(futures).await;
    
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    
    for result in results {
        match result {
            Ok(success) => successes.push(success),
            Err(error) => errors.push(error),
        }
    }
    
    (successes, errors)
}

// 并行执行，遇到错误立即停止
async fn process_items_fail_fast<T>(
    items: Vec<T>,
    processor: impl Fn(T) -> BoxFuture<'static, Result<ProcessResult, ProcessError>> + Clone,
) -> Result<Vec<ProcessResult>, ProcessError> {
    let futures: Vec<_> = items.into_iter()
        .map(|item| processor(item))
        .collect();
    
    try_join_all(futures).await
}

// 限制并发数量
use futures::stream::{StreamExt, FuturesUnordered};

async fn process_items_with_concurrency_limit<T>(
    items: Vec<T>,
    processor: impl Fn(T) -> BoxFuture<'static, Result<ProcessResult, ProcessError>> + Clone,
    max_concurrent: usize,
) -> Vec<Result<ProcessResult, ProcessError>> {
    let stream = futures::stream::iter(items)
        .map(|item| processor(item))
        .buffer_unordered(max_concurrent);
    
    stream.collect().await
}

// 任务管理器
struct ConcurrentTaskManager {
    max_concurrent: usize,
    active_tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl ConcurrentTaskManager {
    async fn execute_batch<T, F, Fut>(
        &self,
        items: Vec<T>,
        task_fn: F,
    ) -> Vec<Result<(), TaskError>>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), TaskError>> + Send + 'static,
        T: Send + 'static,
    {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
        let results = Arc::new(Mutex::new(Vec::new()));
        
        let mut handles = Vec::new();
        
        for (index, item) in items.into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let results = results.clone();
            let task_fn = &task_fn;
            
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                let result = task_fn(item).await;
                results.lock().unwrap().push((index, result));
            });
            
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            if let Err(e) = handle.await {
                log::error!("任务执行失败: {:?}", e);
            }
        }
        
        // 按原始顺序返回结果
        let mut indexed_results = results.lock().unwrap().clone();
        indexed_results.sort_by_key(|(index, _)| *index);
        
        indexed_results.into_iter().map(|(_, result)| result).collect()
    }
}
```

### 流式错误处理

```rust
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

// 错误恢复流
struct RecoverableStream<S, F> {
    stream: S,
    recovery_fn: F,
}

impl<S, F, T, E> Stream for RecoverableStream<S, F>
where
    S: Stream<Item = Result<T, E>> + Unpin,
    F: Fn(E) -> Option<T>,
{
    type Item = T;
    
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.stream.poll_next_unpin(cx) {
            std::task::Poll::Ready(Some(Ok(item))) => {
                std::task::Poll::Ready(Some(item))
            }
            std::task::Poll::Ready(Some(Err(err))) => {
                if let Some(recovered) = (self.recovery_fn)(err) {
                    std::task::Poll::Ready(Some(recovered))
                } else {
                    // 跳过错误项，继续处理下一个
                    self.poll_next(cx)
                }
            }
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

// 批量处理流
async fn process_stream_in_batches<S, T, E>(
    mut stream: S,
    batch_size: usize,
    processor: impl Fn(Vec<T>) -> BoxFuture<'static, Result<Vec<ProcessResult>, BatchError>>,
) -> Vec<Result<ProcessResult, BatchError>>
where
    S: Stream<Item = Result<T, E>> + Unpin,
    E: std::fmt::Debug,
{
    let mut results = Vec::new();
    let mut batch = Vec::new();
    
    while let Some(item) = stream.next().await {
        match item {
            Ok(value) => {
                batch.push(value);
                
                if batch.len() >= batch_size {
                    match processor(std::mem::take(&mut batch)).await {
                        Ok(batch_results) => {
                            results.extend(batch_results.into_iter().map(Ok));
                        }
                        Err(err) => {
                            log::error!("批量处理失败: {:?}", err);
                            results.push(Err(err));
                        }
                    }
                }
            }
            Err(err) => {
                log::warn!("跳过错误项: {:?}", err);
            }
        }
    }
    
    // 处理剩余项
    if !batch.is_empty() {
        match processor(batch).await {
            Ok(batch_results) => {
                results.extend(batch_results.into_iter().map(Ok));
            }
            Err(err) => {
                results.push(Err(err));
            }
        }
    }
    
    results
}
```

## 错误监控和日志

### 结构化日志

```rust
use serde_json::json;
use tracing::{error, warn, info, instrument};

#[derive(Debug, serde::Serialize)]
struct ErrorContext {
    error_id: String,
    user_id: Option<String>,
    request_id: Option<String>,
    timestamp: chrono::DateTime<chrono::Utc>,
    error_type: String,
    error_message: String,
    stack_trace: Option<String>,
    additional_data: serde_json::Value,
}

struct ErrorLogger {
    service_name: String,
}

impl ErrorLogger {
    #[instrument(skip(self, error))]
    fn log_error<E: std::error::Error + 'static>(
        &self,
        error: &E,
        context: Option<ErrorContext>,
    ) {
        let error_context = context.unwrap_or_else(|| ErrorContext {
            error_id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            request_id: None,
            timestamp: chrono::Utc::now(),
            error_type: std::any::type_name::<E>().to_string(),
            error_message: error.to_string(),
            stack_trace: None,
            additional_data: json!({}),
        });
        
        error!(
            error_id = %error_context.error_id,
            error_type = %error_context.error_type,
            user_id = ?error_context.user_id,
            request_id = ?error_context.request_id,
            "Error occurred: {}",
            error_context.error_message
        );
        
        // 发送到监控系统
        self.send_to_monitoring_system(&error_context);
    }
    
    fn send_to_monitoring_system(&self, context: &ErrorContext) {
        // 发送到 Sentry, DataDog, 等监控系统
        // 这里是示例实现
        tokio::spawn(async move {
            // 异步发送错误报告
        });
    }
}

// 错误指标收集
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

struct ErrorMetrics {
    error_counts: Arc<Mutex<HashMap<String, AtomicU64>>>,
    total_errors: AtomicU64,
}

impl ErrorMetrics {
    fn record_error(&self, error_type: &str) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);
        
        let mut counts = self.error_counts.lock().unwrap();
        counts.entry(error_type.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }
    
    fn get_error_count(&self, error_type: &str) -> u64 {
        self.error_counts.lock().unwrap()
            .get(error_type)
            .map(|counter| counter.load(Ordering::Relaxed))
            .unwrap_or(0)
    }
    
    fn get_total_errors(&self) -> u64 {
        self.total_errors.load(Ordering::Relaxed)
    }
}
```

### 健康检查

```rust
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct HealthStatus {
    is_healthy: bool,
    last_check: Instant,
    error_rate: f64,
    response_time: Duration,
    details: HashMap<String, String>,
}

struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck + Send + Sync>>,
    error_threshold: f64,
    response_time_threshold: Duration,
}

#[async_trait::async_trait]
trait HealthCheck {
    async fn check(&self) -> Result<HealthCheckResult, HealthCheckError>;
    fn name(&self) -> &str;
}

struct HealthCheckResult {
    is_healthy: bool,
    response_time: Duration,
    details: HashMap<String, String>,
}

impl HealthChecker {
    async fn check_health(&self) -> HealthStatus {
        let start_time = Instant::now();
        let mut all_healthy = true;
        let mut details = HashMap::new();
        let mut failed_checks = 0;
        
        for check in &self.checks {
            match check.check().await {
                Ok(result) => {
                    if !result.is_healthy {
                        all_healthy = false;
                        failed_checks += 1;
                    }
                    
                    details.insert(
                        check.name().to_string(),
                        if result.is_healthy { "healthy".to_string() } else { "unhealthy".to_string() }
                    );
                    
                    for (key, value) in result.details {
                        details.insert(format!("{}_{}", check.name(), key), value);
                    }
                }
                Err(err) => {
                    all_healthy = false;
                    failed_checks += 1;
                    details.insert(
                        check.name().to_string(),
                        format!("error: {}", err)
                    );
                }
            }
        }
        
        let response_time = start_time.elapsed();
        let error_rate = failed_checks as f64 / self.checks.len() as f64;
        
        HealthStatus {
            is_healthy: all_healthy && 
                       error_rate <= self.error_threshold && 
                       response_time <= self.response_time_threshold,
            last_check: Instant::now(),
            error_rate,
            response_time,
            details,
        }
    }
}

// 数据库健康检查
struct DatabaseHealthCheck {
    db_pool: DatabasePool,
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> Result<HealthCheckResult, HealthCheckError> {
        let start_time = Instant::now();
        
        match self.db_pool.execute("SELECT 1").await {
            Ok(_) => {
                let response_time = start_time.elapsed();
                Ok(HealthCheckResult {
                    is_healthy: true,
                    response_time,
                    details: HashMap::from([
                        ("status".to_string(), "connected".to_string()),
                        ("response_time_ms".to_string(), response_time.as_millis().to_string()),
                    ]),
                })
            }
            Err(err) => {
                Ok(HealthCheckResult {
                    is_healthy: false,
                    response_time: start_time.elapsed(),
                    details: HashMap::from([
                        ("status".to_string(), "disconnected".to_string()),
                        ("error".to_string(), err.to_string()),
                    ]),
                })
            }
        }
    }
    
    fn name(&self) -> &str {
        "database"
    }
}
```

## 最佳实践总结

### 1. 错误传播设计

- 在适当的层次定义错误边界
- 使用类型系统确保错误处理的完整性
- 提供有意义的错误上下文
- 避免过度嵌套的错误类型

### 2. 恢复策略选择

- **重试**：适用于临时性错误
- **降级**：适用于非关键功能
- **断路器**：适用于外部服务调用
- **超时**：适用于可能长时间阻塞的操作

### 3. 异步错误处理

- 合理控制并发度
- 实现优雅的取消机制
- 处理部分失败的情况
- 使用流式处理大量数据

### 4. 监控和观察

- 记录结构化的错误日志
- 收集错误指标和趋势
- 实现健康检查机制
- 建立告警和通知系统

## 学习检查清单

- [ ] 理解错误传播的机制和模式
- [ ] 掌握错误边界的设计原则
- [ ] 学会实现重试和退避策略
- [ ] 了解服务降级和回退机制
- [ ] 掌握断路器模式的实现
- [ ] 学会处理超时和取消
- [ ] 了解异步环境下的错误处理
- [ ] 掌握错误监控和日志记录
- [ ] 能够设计健壮的错误处理架构

## 扩展阅读

- [Error Handling Patterns](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Async Error Handling](https://rust-lang.github.io/async-book/)
- [Circuit Breaker Pattern](https://martinfowler.com/bliki/CircuitBreaker.html)
- [Resilience Patterns](https://docs.microsoft.com/en-us/azure/architecture/patterns/)
- [Observability in Rust](https://tokio.rs/tokio/topics/tracing)
- [Error Handling Survey](https://blog.yoshuawuyts.com/error-handling-survey/)