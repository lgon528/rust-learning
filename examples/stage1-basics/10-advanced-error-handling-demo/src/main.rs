//! 高级错误处理示例
//! 
//! 本示例展示了 Rust 中的高级错误处理技术，包括：
//! - 自定义错误类型设计
//! - 错误处理库的使用（anyhow, thiserror）
//! - 错误传播和恢复策略
//! - 异步环境下的错误处理
//! - 错误监控和日志记录

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

// ============================================================================
// 1. 自定义错误类型设计
// ============================================================================

/// 数据访问层错误
#[derive(Error, Debug)]
pub enum DataAccessError {
    #[error("数据库连接失败: {message}")]
    ConnectionFailed { message: String },
    
    #[error("记录未找到: {table}.{id}")]
    RecordNotFound { table: String, id: String },
    
    #[error("数据验证失败: {field} - {reason}")]
    ValidationFailed { field: String, reason: String },
    
    #[error("查询超时: {timeout:?}")]
    QueryTimeout { timeout: Duration },
}

/// 业务逻辑层错误
#[derive(Error, Debug)]
pub enum BusinessLogicError {
    #[error("数据访问错误")]
    DataAccess(#[from] DataAccessError),
    
    #[error("业务规则违反: {rule}")]
    BusinessRuleViolation { rule: String },
    
    #[error("权限不足: 需要 {required}, 当前 {current}")]
    InsufficientPermissions { required: String, current: String },
    
    #[error("资源冲突: {resource}")]
    ResourceConflict { resource: String },
}

/// API 层错误
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("业务逻辑错误")]
    BusinessLogic(#[from] BusinessLogicError),
    
    #[error("请求格式错误: {message}")]
    InvalidRequest { message: String },
    
    #[error("认证失败")]
    AuthenticationFailed,
    
    #[error("服务不可用")]
    ServiceUnavailable,
    
    #[error("请求超时")]
    RequestTimeout,
}

/// 网络错误类型
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("连接超时")]
    Timeout,
    
    #[error("连接被拒绝")]
    ConnectionRefused,
    
    #[error("服务器错误: {status}")]
    ServerError { status: u16 },
    
    #[error("客户端错误: {status}")]
    ClientError { status: u16 },
    
    #[error("网络不可达")]
    NetworkUnreachable,
}

impl NetworkError {
    /// 判断错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            NetworkError::Timeout => true,
            NetworkError::ConnectionRefused => true,
            NetworkError::ServerError { status } => *status >= 500,
            NetworkError::ClientError { .. } => false,
            NetworkError::NetworkUnreachable => true,
        }
    }
}

// ============================================================================
// 2. 数据模型
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub api_timeout: u64,
    pub retry_attempts: usize,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// 3. 重试机制实现
// ============================================================================

/// 重试策略配置
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

/// 带指数退避的重试机制
pub async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    policy: RetryPolicy,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    let mut delay = policy.initial_delay;
    
    loop {
        attempts += 1;
        
        match operation().await {
            Ok(result) => {
                if attempts > 1 {
                    info!("操作在第 {} 次尝试后成功", attempts);
                }
                return Ok(result);
            }
            Err(err) => {
                if attempts >= policy.max_attempts {
                    error!("操作失败，已达到最大重试次数 {}: {:?}", policy.max_attempts, err);
                    return Err(err);
                }
                
                warn!("操作失败，第 {} 次重试，延迟 {:?}: {:?}", attempts, delay, err);
                sleep(delay).await;
                
                // 指数退避
                delay = std::cmp::min(
                    Duration::from_millis((delay.as_millis() as f64 * policy.backoff_multiplier) as u64),
                    policy.max_delay,
                );
            }
        }
    }
}

/// 条件重试 - 只对可重试的错误进行重试
pub async fn retry_on_condition<F, Fut, T, E>(
    operation: F,
    should_retry: impl Fn(&E) -> bool,
    max_attempts: usize,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                if !should_retry(&err) {
                    error!("不可重试的错误: {:?}", err);
                    return Err(err);
                }
                
                if attempts >= max_attempts {
                    error!("重试次数已达上限: {:?}", err);
                    return Err(err);
                }
                
                let delay = Duration::from_millis(100 * attempts as u64);
                warn!("第 {} 次重试，延迟 {:?}: {:?}", attempts, delay, err);
                sleep(delay).await;
            }
        }
    }
}

// ============================================================================
// 4. 断路器模式实现
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,   // 正常状态
    Open,     // 断路状态
    HalfOpen, // 半开状态
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_count: Arc<Mutex<usize>>,
    success_count: Arc<Mutex<usize>>,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
    failure_threshold: usize,
    recovery_timeout: Duration,
    success_threshold: usize,
}

impl CircuitBreaker {
    pub fn new(
        failure_threshold: usize,
        recovery_timeout: Duration,
        success_threshold: usize,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(Mutex::new(0)),
            success_count: Arc::new(Mutex::new(0)),
            last_failure_time: Arc::new(Mutex::new(None)),
            failure_threshold,
            recovery_timeout,
            success_threshold,
        }
    }
    
    pub async fn call<F, Fut, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError<E>>
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
                    info!("断路器从打开状态转为半开状态");
                } else {
                    return Err(CircuitBreakerError::CircuitOpen);
                }
            }
            CircuitState::HalfOpen => {
                info!("断路器处于半开状态，尝试执行操作");
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
                let mut success_count = self.success_count.lock().unwrap();
                *success_count += 1;
                
                if *success_count >= self.success_threshold {
                    self.set_state(CircuitState::Closed);
                    *self.failure_count.lock().unwrap() = 0;
                    *success_count = 0;
                    info!("断路器恢复到关闭状态");
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
            warn!("断路器打开，失败次数: {}", *failure_count);
        }
        
        if matches!(self.get_state(), CircuitState::HalfOpen) {
            self.set_state(CircuitState::Open);
            *self.success_count.lock().unwrap() = 0;
            warn!("半开状态下失败，断路器重新打开");
        }
    }
    
    pub fn get_stats(&self) -> CircuitBreakerStats {
        CircuitBreakerStats {
            state: self.get_state(),
            failure_count: *self.failure_count.lock().unwrap(),
            success_count: *self.success_count.lock().unwrap(),
        }
    }
}

#[derive(Error, Debug)]
pub enum CircuitBreakerError<E> {
    #[error("断路器处于打开状态")]
    CircuitOpen,
    
    #[error("操作执行失败")]
    OperationFailed(E),
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerStats {
    pub state: CircuitState,
    pub failure_count: usize,
    pub success_count: usize,
}

// ============================================================================
// 5. 服务降级和回退机制
// ============================================================================

/// 推荐服务 - 演示服务降级
pub struct RecommendationService {
    primary_client: HttpClient,
    fallback_client: HttpClient,
    cache: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl RecommendationService {
    pub fn new() -> Self {
        Self {
            primary_client: HttpClient::new("https://api.primary.com"),
            fallback_client: HttpClient::new("https://api.fallback.com"),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 获取推荐，支持多级降级
    pub async fn get_recommendations(&self, user_id: &str) -> Vec<String> {
        // 1. 尝试主服务
        match self.primary_client.get_recommendations(user_id).await {
            Ok(recommendations) => {
                // 缓存成功结果
                self.cache.lock().unwrap().insert(user_id.to_string(), recommendations.clone());
                info!("使用主推荐服务获取推荐");
                return recommendations;
            }
            Err(err) => {
                warn!("主推荐服务失败: {:?}", err);
            }
        }
        
        // 2. 尝试备用服务
        match self.fallback_client.get_recommendations(user_id).await {
            Ok(recommendations) => {
                info!("使用备用推荐服务");
                return recommendations;
            }
            Err(err) => {
                warn!("备用推荐服务失败: {:?}", err);
            }
        }
        
        // 3. 尝试缓存
        if let Some(cached) = self.cache.lock().unwrap().get(user_id) {
            info!("使用缓存的推荐结果");
            return cached.clone();
        }
        
        // 4. 返回默认推荐
        info!("使用默认推荐");
        self.get_default_recommendations()
    }
    
    fn get_default_recommendations(&self) -> Vec<String> {
        vec![
            "热门商品1".to_string(),
            "热门商品2".to_string(),
            "热门商品3".to_string(),
        ]
    }
}

// ============================================================================
// 6. HTTP 客户端实现
// ============================================================================

#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    circuit_breaker: Arc<CircuitBreaker>,
    _retry_policy: RetryPolicy,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            circuit_breaker: Arc::new(CircuitBreaker::new(
                3,                              // 失败阈值
                Duration::from_secs(30),        // 恢复超时
                2,                              // 成功阈值
            )),
            _retry_policy: RetryPolicy::default(),
        }
    }
    
    pub async fn get_recommendations(&self, user_id: &str) -> Result<Vec<String>, NetworkError> {
        let url = format!("{}/recommendations/{}", self.base_url, user_id);
        
        self.circuit_breaker.call(|| async {
            self.make_request(&url).await
        }).await.map_err(|e| match e {
            CircuitBreakerError::CircuitOpen => NetworkError::ServerError { status: 503 },
            CircuitBreakerError::OperationFailed(err) => err,
        })
    }
    
    async fn make_request(&self, url: &str) -> Result<Vec<String>, NetworkError> {
        // 模拟网络请求
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // 模拟随机失败
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hash = hasher.finish();
        
        if hash % 4 == 0 {
            return Err(NetworkError::Timeout);
        } else if hash % 4 == 1 {
            return Err(NetworkError::ServerError { status: 500 });
        }
        
        Ok(vec![
            format!("推荐商品A for {}", url),
            format!("推荐商品B for {}", url),
        ])
    }
}

// ============================================================================
// 7. 错误监控和日志
// ============================================================================

#[derive(Debug, Serialize)]
pub struct ErrorContext {
    pub error_id: String,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub error_type: String,
    pub error_message: String,
    pub additional_data: serde_json::Value,
}

pub struct ErrorLogger {
    service_name: String,
}

impl ErrorLogger {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }
    
    pub fn log_error<E: std::error::Error + 'static>(
        &self,
        error: &E,
        context: Option<ErrorContext>,
    ) {
        let error_context = context.unwrap_or_else(|| ErrorContext {
            error_id: Uuid::new_v4().to_string(),
            user_id: None,
            request_id: None,
            timestamp: chrono::Utc::now(),
            error_type: std::any::type_name::<E>().to_string(),
            error_message: error.to_string(),
            additional_data: serde_json::json!({}),
        });
        
        error!(
            service = %self.service_name,
            error_id = %error_context.error_id,
            error_type = %error_context.error_type,
            user_id = ?error_context.user_id,
            request_id = ?error_context.request_id,
            "Error occurred: {}",
            error_context.error_message
        );
        
        // 在实际应用中，这里会发送到监控系统
        self.send_to_monitoring_system(&error_context);
    }
    
    fn send_to_monitoring_system(&self, _context: &ErrorContext) {
        // 模拟发送到监控系统（如 Sentry, DataDog 等）
        info!("错误已发送到监控系统");
    }
}

// ============================================================================
// 8. 用户服务实现 - 展示分层错误处理
// ============================================================================

pub struct UserService {
    error_logger: ErrorLogger,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            error_logger: ErrorLogger::new("user-service"),
        }
    }
    
    /// 数据访问层 - 模拟数据库操作
    async fn find_user_by_id(&self, id: u64) -> Result<User, DataAccessError> {
        // 模拟数据库查询延迟
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // 模拟查询失败的情况
        if id == 0 {
            return Err(DataAccessError::ValidationFailed {
                field: "id".to_string(),
                reason: "ID cannot be zero".to_string(),
            });
        }
        
        if id == 999 {
            return Err(DataAccessError::RecordNotFound {
                table: "users".to_string(),
                id: id.to_string(),
            });
        }
        
        if id > 1000 {
            return Err(DataAccessError::ConnectionFailed {
                message: "Database connection timeout".to_string(),
            });
        }
        
        Ok(User {
            id,
            name: format!("User {}", id),
            email: format!("user{}@example.com", id),
            role: if id == 1 { "admin".to_string() } else { "user".to_string() },
            created_at: chrono::Utc::now(),
        })
    }
    
    /// 业务逻辑层 - 更新用户邮箱
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
        if !self.is_valid_email(new_email) {
            return Err(BusinessLogicError::BusinessRuleViolation {
                rule: "valid email format required".to_string(),
            });
        }
        
        // 数据访问（错误自动传播）
        let mut user = self.find_user_by_id(user_id).await?;
        
        // 检查邮箱是否已被使用
        if self.email_exists(new_email).await? {
            return Err(BusinessLogicError::ResourceConflict {
                resource: format!("email: {}", new_email),
            });
        }
        
        user.email = new_email.to_string();
        self.save_user(&user).await?;
        
        info!("用户 {} 的邮箱已更新为 {}", user_id, new_email);
        Ok(())
    }
    
    async fn email_exists(&self, email: &str) -> Result<bool, DataAccessError> {
        // 模拟邮箱唯一性检查
        tokio::time::sleep(Duration::from_millis(30)).await;
        Ok(email == "existing@example.com")
    }
    
    async fn save_user(&self, user: &User) -> Result<(), DataAccessError> {
        // 模拟保存用户
        tokio::time::sleep(Duration::from_millis(40)).await;
        info!("用户 {} 已保存", user.id);
        Ok(())
    }
    
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
    
    /// API 层错误处理
    pub async fn handle_update_email_request(
        &self,
        user_id: u64,
        new_email: String,
        current_user: User,
    ) -> Result<ApiResponse<()>, ApiError> {
        // 请求验证
        if new_email.is_empty() {
            return Err(ApiError::InvalidRequest {
                message: "email field is required".to_string(),
            });
        }
        
        // 调用业务逻辑（错误自动传播）
        match self.update_user_email(user_id, &new_email, &current_user).await {
            Ok(()) => Ok(ApiResponse {
                success: true,
                data: Some(()),
                error: None,
                timestamp: chrono::Utc::now(),
            }),
            Err(err) => {
                // 记录错误
                self.error_logger.log_error(&err, Some(ErrorContext {
                    error_id: Uuid::new_v4().to_string(),
                    user_id: Some(current_user.id.to_string()),
                    request_id: Some(Uuid::new_v4().to_string()),
                    timestamp: chrono::Utc::now(),
                    error_type: "BusinessLogicError".to_string(),
                    error_message: err.to_string(),
                    additional_data: serde_json::json!({
                        "target_user_id": user_id,
                        "new_email": new_email,
                        "current_user_role": current_user.role
                    }),
                }));
                
                Err(ApiError::BusinessLogic(err))
            }
        }
    }
}

// ============================================================================
// 9. 异步并发错误处理
// ============================================================================

/// 并发任务管理器
pub struct ConcurrentTaskManager {
    _max_concurrent: usize,
}

impl ConcurrentTaskManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self { _max_concurrent: max_concurrent }
    }
    
    /// 并行处理多个任务，收集所有结果（包括错误）
    pub async fn process_all<T, R, E, F, Fut>(
        &self,
        items: Vec<T>,
        processor: F,
    ) -> (Vec<R>, Vec<E>)
    where
        F: Fn(T) -> Fut + Clone + Send + 'static,
        Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
        E: Send + 'static,
    {
        use futures::stream::{FuturesUnordered, StreamExt};
        
        let mut futures = FuturesUnordered::new();
        
        for item in items {
            let processor = processor.clone();
            futures.push(tokio::spawn(async move {
                processor(item).await
            }));
        }
        
        let mut successes = Vec::new();
        let mut errors = Vec::new();
        
        while let Some(result) = futures.next().await {
            match result {
                Ok(Ok(success)) => successes.push(success),
                Ok(Err(error)) => errors.push(error),
                Err(join_error) => {
                    error!("任务执行失败: {:?}", join_error);
                }
            }
        }
        
        (successes, errors)
    }
    
    /// 并行处理，遇到错误立即停止
    pub async fn process_fail_fast<T, R, E, F, Fut>(
        &self,
        items: Vec<T>,
        processor: F,
    ) -> Result<Vec<R>, E>
    where
        F: Fn(T) -> Fut + Clone + Send + 'static,
        Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
        E: Send + 'static,
    {
        use futures::future::try_join_all;
        
        let futures: Vec<_> = items.into_iter()
            .map(|item| {
                let processor = processor.clone();
                tokio::spawn(async move {
                    processor(item).await
                })
            })
            .collect();
        
        let results = try_join_all(futures).await
            .map_err(|e| {
                error!("任务执行失败: {:?}", e);
                // 这里需要将 JoinError 转换为 E，实际应用中需要更好的错误处理
                panic!("Task join error: {:?}", e);
            })?;
        
        results.into_iter().collect()
    }
}

// ============================================================================
// 10. 主函数和示例
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    info!("启动高级错误处理示例");
    
    // 1. 演示自定义错误类型和分层错误处理
    demo_layered_error_handling().await?;
    
    // 2. 演示重试机制
    demo_retry_mechanisms().await?;
    
    // 3. 演示断路器模式
    demo_circuit_breaker().await?;
    
    // 4. 演示服务降级
    demo_service_fallback().await?;
    
    // 5. 演示并发错误处理
    demo_concurrent_error_handling().await?;
    
    info!("所有示例执行完成");
    Ok(())
}

async fn demo_layered_error_handling() -> Result<()> {
    info!("=== 演示分层错误处理 ===");
    
    let user_service = UserService::new();
    let current_user = User {
        id: 1,
        name: "Admin".to_string(),
        email: "admin@example.com".to_string(),
        role: "admin".to_string(),
        created_at: chrono::Utc::now(),
    };
    
    // 成功案例
    match user_service.handle_update_email_request(
        2,
        "newemail@example.com".to_string(),
        current_user.clone(),
    ).await {
        Ok(response) => info!("邮箱更新成功: {:?}", response),
        Err(err) => error!("邮箱更新失败: {:?}", err),
    }
    
    // 权限不足案例
    let regular_user = User {
        id: 2,
        name: "Regular User".to_string(),
        email: "user@example.com".to_string(),
        role: "user".to_string(),
        created_at: chrono::Utc::now(),
    };
    
    match user_service.handle_update_email_request(
        3,
        "anotheremail@example.com".to_string(),
        regular_user,
    ).await {
        Ok(response) => info!("邮箱更新成功: {:?}", response),
        Err(err) => error!("邮箱更新失败（预期的权限错误）: {:?}", err),
    }
    
    // 用户不存在案例
    match user_service.handle_update_email_request(
        999,
        "notfound@example.com".to_string(),
        current_user.clone(),
    ).await {
        Ok(response) => info!("邮箱更新成功: {:?}", response),
        Err(err) => error!("邮箱更新失败（预期的用户不存在错误）: {:?}", err),
    }
    
    Ok(())
}

async fn demo_retry_mechanisms() -> Result<()> {
    info!("=== 演示重试机制 ===");
    
    // 基本重试
    let result = retry_with_backoff(
        || async {
            // 模拟不稳定的操作
            if rand::random::<f64>() < 0.7 {
                Err(NetworkError::Timeout)
            } else {
                Ok("操作成功")
            }
        },
        RetryPolicy::default(),
    ).await;
    
    match result {
        Ok(value) => info!("重试操作成功: {}", value),
        Err(err) => error!("重试操作最终失败: {:?}", err),
    }
    
    // 条件重试
    let result = retry_on_condition(
        || async {
            let error_type = rand::random::<u8>() % 4;
            match error_type {
                0 => Ok("成功"),
                1 => Err(NetworkError::Timeout),           // 可重试
                2 => Err(NetworkError::ServerError { status: 500 }), // 可重试
                _ => Err(NetworkError::ClientError { status: 400 }), // 不可重试
            }
        },
        |err| err.is_retryable(),
        3,
    ).await;
    
    match result {
        Ok(value) => info!("条件重试成功: {}", value),
        Err(err) => error!("条件重试失败: {:?}", err),
    }
    
    Ok(())
}

async fn demo_circuit_breaker() -> Result<()> {
    info!("=== 演示断路器模式 ===");
    
    let circuit_breaker = CircuitBreaker::new(
        2,                              // 失败阈值
        Duration::from_secs(2),         // 恢复超时
        1,                              // 成功阈值
    );
    
    // 模拟多次调用，观察断路器状态变化
    for i in 1..=10 {
        let result = circuit_breaker.call(|| async {
            // 模拟不稳定的服务
            if i <= 3 {
                Err(NetworkError::ServerError { status: 500 })
            } else if i == 8 {
                Ok(format!("成功调用 {}", i))
            } else {
                Err(NetworkError::Timeout)
            }
        }).await;
        
        let stats = circuit_breaker.get_stats();
        match result {
            Ok(value) => info!("调用 {} 成功: {}, 断路器状态: {:?}", i, value, stats.state),
            Err(err) => warn!("调用 {} 失败: {:?}, 断路器状态: {:?}", i, err, stats.state),
        }
        
        // 在断路器打开后等待一段时间
        if matches!(stats.state, CircuitState::Open) && i == 4 {
            info!("等待断路器恢复...");
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
        
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    Ok(())
}

async fn demo_service_fallback() -> Result<()> {
    info!("=== 演示服务降级 ===");
    
    let recommendation_service = RecommendationService::new();
    
    // 测试多个用户的推荐获取
    let user_ids = vec!["user1", "user2", "user3", "user4"];
    
    for user_id in user_ids {
        let recommendations = recommendation_service.get_recommendations(user_id).await;
        info!("用户 {} 的推荐: {:?}", user_id, recommendations);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}

async fn demo_concurrent_error_handling() -> Result<()> {
    info!("=== 演示并发错误处理 ===");
    
    let task_manager = ConcurrentTaskManager::new(3);
    
    // 准备测试数据
    let items: Vec<u32> = (1..=10).collect();
    
    // 并发处理，收集所有结果
    let (successes, errors) = task_manager.process_all(
        items.clone(),
        |item| async move {
            // 模拟处理时间
            tokio::time::sleep(Duration::from_millis(100 * item as u64)).await;
            
            // 模拟随机失败
            if item % 3 == 0 {
                Err(format!("处理项目 {} 失败", item))
            } else {
                Ok(format!("处理项目 {} 成功", item))
            }
        },
    ).await;
    
    info!("并发处理结果:");
    info!("成功: {:?}", successes);
    info!("失败: {:?}", errors);
    
    Ok(())
}

// ============================================================================
// 11. 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_retry_mechanism() {
        let attempt_count = Arc::new(Mutex::new(0));
        let count_clone = attempt_count.clone();
        
        let result = retry_with_backoff(
            move || {
                let count = count_clone.clone();
                async move {
                    let mut current_count = count.lock().unwrap();
                    *current_count += 1;
                    let attempts = *current_count;
                    drop(current_count);
                    
                    if attempts < 3 {
                        Err(NetworkError::Timeout)
                    } else {
                        Ok("success")
                    }
                }
            },
            RetryPolicy::default(),
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(*attempt_count.lock().unwrap(), 3);
    }
    
    #[tokio::test]
    async fn test_circuit_breaker() {
        let circuit_breaker = CircuitBreaker::new(2, Duration::from_millis(100), 1);
        
        // 触发失败，打开断路器
        for _ in 0..3 {
            let _ = circuit_breaker.call(|| async {
                Result::<(), NetworkError>::Err(NetworkError::ServerError { status: 500 })
            }).await;
        }
        
        // 断路器应该处于打开状态
        let stats = circuit_breaker.get_stats();
        assert_eq!(stats.state, CircuitState::Open);
        
        // 立即调用应该失败
        let result = circuit_breaker.call(|| async {
            Ok::<(), NetworkError>(())
        }).await;
        
        assert!(matches!(result, Err(CircuitBreakerError::CircuitOpen)));
    }
    
    #[tokio::test]
    async fn test_user_service_error_handling() {
        let user_service = UserService::new();
        let admin_user = User {
            id: 1,
            name: "Admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            created_at: chrono::Utc::now(),
        };
        
        // 测试成功案例
        let result = user_service.handle_update_email_request(
            2,
            "test@example.com".to_string(),
            admin_user.clone(),
        ).await;
        assert!(result.is_ok());
        
        // 测试用户不存在
        let result = user_service.handle_update_email_request(
            999,
            "test@example.com".to_string(),
            admin_user.clone(),
        ).await;
        assert!(result.is_err());
        
        // 测试权限不足
        let regular_user = User {
            id: 2,
            name: "User".to_string(),
            email: "user@example.com".to_string(),
            role: "user".to_string(),
            created_at: chrono::Utc::now(),
        };
        
        let result = user_service.handle_update_email_request(
            3,
            "test@example.com".to_string(),
            regular_user,
        ).await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_network_error_retryable() {
        assert!(NetworkError::Timeout.is_retryable());
        assert!(NetworkError::ConnectionRefused.is_retryable());
        assert!(NetworkError::ServerError { status: 500 }.is_retryable());
        assert!(!NetworkError::ClientError { status: 400 }.is_retryable());
    }
    
    #[tokio::test]
    async fn test_concurrent_task_processing() {
        let task_manager = ConcurrentTaskManager::new(2);
        let items = vec![1, 2, 3, 4, 5];
        
        let (successes, errors) = task_manager.process_all(
            items,
            |item| async move {
                if item % 2 == 0 {
                    Err(format!("Error for {}", item))
                } else {
                    Ok(item * 2)
                }
            },
        ).await;
        
        assert_eq!(successes.len(), 3); // 1, 3, 5
        assert_eq!(errors.len(), 2);    // 2, 4
    }
}

// 添加 rand crate 的简单实现，避免额外依赖
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn random<T>() -> T
    where
        T: From<u8>,
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        let hash = hasher.finish();
        let random_u8 = (hash % 256) as u8;
        T::from(random_u8)
    }
}
