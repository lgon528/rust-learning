//! 配置管理模块
//! 
//! 提供应用程序配置的加载、验证和管理功能

use crate::{LibError, Result};
use std::collections::HashMap;
use std::env;
// use std::fs;
// use std::path::Path;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// 应用程序配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Config {
    /// 应用程序名称
    pub app_name: String,
    /// 应用程序版本
    pub version: String,
    /// 环境（development, production, test）
    pub environment: Environment,
    /// 网络配置
    pub network: NetworkConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 数据库配置
    pub database: Option<DatabaseConfig>,
    /// 自定义配置项
    pub custom: HashMap<String, String>,
}

/// 环境类型
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

impl std::str::FromStr for Environment {
    type Err = LibError;
    
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            "test" => Ok(Environment::Test),
            _ => Err(LibError::Config(format!("无效的环境类型: {}", s))),
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
            Environment::Test => write!(f, "test"),
        }
    }
}

/// 网络配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub max_connections: usize,
    pub enable_tls: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout_seconds: 30,
            max_connections: 100,
            enable_tls: false,
        }
    }
}

/// 日志配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub output: LogOutput,
    pub format: LogFormat,
    pub enable_colors: bool,
}

/// 日志级别
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// 日志输出目标
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LogOutput {
    Console,
    File(String),
    Both { file: String },
}

impl Default for LogOutput {
    fn default() -> Self {
        LogOutput::Console
    }
}

/// 日志格式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LogFormat {
    Simple,
    Json,
    Detailed,
}

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Simple
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::default(),
            output: LogOutput::default(),
            format: LogFormat::default(),
            enable_colors: true,
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub enable_logging: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "module-system-demo".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::default(),
            network: NetworkConfig::default(),
            logging: LoggingConfig::default(),
            database: None,
            custom: HashMap::new(),
        }
    }
}

impl Config {
    /// 创建新的配置实例
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();
        
        // 加载基本配置
        if let Ok(app_name) = env::var("APP_NAME") {
            config.app_name = app_name;
        }
        
        if let Ok(env_str) = env::var("ENVIRONMENT") {
            config.environment = env_str.parse()?;
        }
        
        // 加载网络配置
        if let Ok(host) = env::var("HOST") {
            config.network.host = host;
        }
        
        if let Ok(port_str) = env::var("PORT") {
            config.network.port = port_str.parse()
                .map_err(|e| LibError::Config(format!("无效的端口号: {}", e)))?;
        }
        
        if let Ok(timeout_str) = env::var("TIMEOUT_SECONDS") {
            config.network.timeout_seconds = timeout_str.parse()
                .map_err(|e| LibError::Config(format!("无效的超时时间: {}", e)))?;
        }
        
        if let Ok(max_conn_str) = env::var("MAX_CONNECTIONS") {
            config.network.max_connections = max_conn_str.parse()
                .map_err(|e| LibError::Config(format!("无效的最大连接数: {}", e)))?;
        }
        
        if let Ok(tls_str) = env::var("ENABLE_TLS") {
            config.network.enable_tls = tls_str.parse()
                .map_err(|e| LibError::Config(format!("无效的TLS设置: {}", e)))?;
        }
        
        // 加载日志配置
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            config.logging.level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => return Err(LibError::Config(format!("无效的日志级别: {}", log_level))),
            };
        }
        
        // 加载数据库配置
        if let Ok(db_url) = env::var("DATABASE_URL") {
            let mut db_config = DatabaseConfig {
                url: db_url,
                max_connections: 10,
                timeout_seconds: 30,
                enable_logging: false,
            };
            
            if let Ok(max_conn_str) = env::var("DB_MAX_CONNECTIONS") {
                db_config.max_connections = max_conn_str.parse()
                    .map_err(|e| LibError::Config(format!("无效的数据库最大连接数: {}", e)))?;
            }
            
            if let Ok(timeout_str) = env::var("DB_TIMEOUT_SECONDS") {
                db_config.timeout_seconds = timeout_str.parse()
                    .map_err(|e| LibError::Config(format!("无效的数据库超时时间: {}", e)))?;
            }
            
            if let Ok(logging_str) = env::var("DB_ENABLE_LOGGING") {
                db_config.enable_logging = logging_str.parse()
                    .map_err(|e| LibError::Config(format!("无效的数据库日志设置: {}", e)))?;
            }
            
            config.database = Some(db_config);
        }
        
        Ok(config)
    }
    
    /// 从文件加载配置（需要serde_support功能）
    #[cfg(feature = "serde_support")]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| LibError::Config(format!("无法读取配置文件: {}", e)))?;
        
        // 根据文件扩展名选择解析方式
        let config: Config = if content.trim_start().starts_with('{') {
            // JSON格式
            serde_json::from_str(&content)
                .map_err(|e| LibError::Config(format!("无法解析JSON配置: {}", e)))?
        } else {
            // TOML格式
            toml::from_str(&content)
                .map_err(|e| LibError::Config(format!("无法解析TOML配置: {}", e)))?
        };
        
        Ok(config)
    }
    
    /// 保存配置到文件（需要serde_support功能）
    #[cfg(feature = "serde_support")]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P, format: ConfigFormat) -> Result<()> {
        let content = match format {
            ConfigFormat::Json => serde_json::to_string_pretty(self)
                .map_err(|e| LibError::Config(format!("无法序列化为JSON: {}", e)))?,
            ConfigFormat::Toml => toml::to_string_pretty(self)
                .map_err(|e| LibError::Config(format!("无法序列化为TOML: {}", e)))?,
        };
        
        fs::write(path, content)
            .map_err(|e| LibError::Config(format!("无法写入配置文件: {}", e)))?;
        
        Ok(())
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 验证应用程序名称
        if self.app_name.is_empty() {
            return Err(LibError::Config("应用程序名称不能为空".to_string()));
        }
        
        // 验证网络配置
        if self.network.port == 0 {
            return Err(LibError::Config("端口号不能为0".to_string()));
        }
        
        if self.network.timeout_seconds == 0 {
            return Err(LibError::Config("超时时间不能为0".to_string()));
        }
        
        if self.network.max_connections == 0 {
            return Err(LibError::Config("最大连接数不能为0".to_string()));
        }
        
        // 验证数据库配置
        if let Some(ref db_config) = self.database {
            if db_config.url.is_empty() {
                return Err(LibError::Config("数据库URL不能为空".to_string()));
            }
            
            if db_config.max_connections == 0 {
                return Err(LibError::Config("数据库最大连接数不能为0".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// 获取自定义配置项
    pub fn get_custom(&self, key: &str) -> Option<&String> {
        self.custom.get(key)
    }
    
    /// 设置自定义配置项
    pub fn set_custom(&mut self, key: String, value: String) {
        self.custom.insert(key, value);
    }
    
    /// 是否为生产环境
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
    
    /// 是否为开发环境
    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }
    
    /// 是否为测试环境
    pub fn is_test(&self) -> bool {
        self.environment == Environment::Test
    }
    
    /// 获取服务器地址
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.network.host, self.network.port)
    }
}

/// 配置文件格式
#[cfg(feature = "serde_support")]
#[derive(Debug, Clone, Copy)]
pub enum ConfigFormat {
    Json,
    Toml,
}

/// 配置构建器
#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    /// 设置应用程序名称
    pub fn app_name(mut self, name: String) -> Self {
        self.config.app_name = name;
        self
    }
    
    /// 设置环境
    pub fn environment(mut self, env: Environment) -> Self {
        self.config.environment = env;
        self
    }
    
    /// 设置网络配置
    pub fn network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }
    
    /// 设置日志配置
    pub fn logging(mut self, logging: LoggingConfig) -> Self {
        self.config.logging = logging;
        self
    }
    
    /// 设置数据库配置
    pub fn database(mut self, database: Option<DatabaseConfig>) -> Self {
        self.config.database = database;
        self
    }
    
    /// 添加自定义配置项
    pub fn custom(mut self, key: String, value: String) -> Self {
        self.config.custom.insert(key, value);
        self
    }
    
    /// 构建配置
    pub fn build(self) -> Result<Config> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app_name, "module-system-demo");
        assert_eq!(config.environment, Environment::Development);
        assert_eq!(config.network.host, "127.0.0.1");
        assert_eq!(config.network.port, 8080);
    }

    #[test]
    fn test_environment_from_str() {
        assert_eq!("development".parse::<Environment>().unwrap(), Environment::Development);
        assert_eq!("production".parse::<Environment>().unwrap(), Environment::Production);
        assert_eq!("test".parse::<Environment>().unwrap(), Environment::Test);
        assert!("invalid".parse::<Environment>().is_err());
    }

    #[test]
    fn test_config_from_env() {
        env::set_var("APP_NAME", "test-app");
        env::set_var("ENVIRONMENT", "production");
        env::set_var("PORT", "9000");
        
        let config = Config::from_env().unwrap();
        assert_eq!(config.app_name, "test-app");
        assert_eq!(config.environment, Environment::Production);
        assert_eq!(config.network.port, 9000);
        
        // 清理环境变量
        env::remove_var("APP_NAME");
        env::remove_var("ENVIRONMENT");
        env::remove_var("PORT");
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());
        
        config.app_name = String::new();
        assert!(config.validate().is_err());
        
        config.app_name = "test".to_string();
        config.network.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .app_name("test-app".to_string())
            .environment(Environment::Production)
            .custom("key1".to_string(), "value1".to_string())
            .build()
            .unwrap();
        
        assert_eq!(config.app_name, "test-app");
        assert_eq!(config.environment, Environment::Production);
        assert_eq!(config.get_custom("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_config_helpers() {
        let mut config = Config::default();
        config.environment = Environment::Production;
        
        assert!(config.is_production());
        assert!(!config.is_development());
        assert!(!config.is_test());
        
        assert_eq!(config.server_address(), "127.0.0.1:8080");
    }
}