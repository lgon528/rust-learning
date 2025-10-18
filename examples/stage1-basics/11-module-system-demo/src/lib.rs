//! # 模块系统演示库
//!
//! 这个库演示了 Rust 模块系统的各种特性，包括：
//! - 模块定义和组织
//! - 可见性控制
//! - 路径和 use 语句
//! - 特性（features）的使用
//!
//! ## 使用示例
//!
//! ```rust
//! use module_system_demo::network::Client;
//! use module_system_demo::utils::string_utils::capitalize;
//!
//! let client = Client::new("example.com".to_string());
//! let message = capitalize("hello world");
//! println!("{}", message);
//! ```

// 公开的模块
pub mod network;
pub mod utils;
pub mod config;

// 私有的内部模块
mod internal;

// 重新导出常用类型
pub use network::client::Client;
pub use network::server::Server;
pub use config::Config;

// 条件编译的模块
#[cfg(feature = "serde_support")]
pub mod serialization;

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 初始化库
/// 
/// 这个函数应该在使用库之前调用
pub fn init() -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "logging")]
    {
        // 初始化日志记录器
        // 注意：这里只是示例，实际项目中应该使用 env_logger 或其他日志库
    }
    
    Ok(())
}

/// 库的主要错误类型
#[derive(Debug)]
pub enum LibError {
    /// 网络错误
    Network(String),
    /// 配置错误
    Config(String),
    /// 内部错误
    Internal(String),
    /// 序列化错误
    #[cfg(feature = "serde_support")]
    SerializationError(String),
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibError::Network(msg) => write!(f, "网络错误: {}", msg),
            LibError::Config(msg) => write!(f, "配置错误: {}", msg),
            LibError::Internal(msg) => write!(f, "内部错误: {}", msg),
            #[cfg(feature = "serde_support")]
            LibError::SerializationError(msg) => write!(f, "序列化错误: {}", msg),
        }
    }
}

impl std::error::Error for LibError {}

/// 库的结果类型
pub type Result<T> = std::result::Result<T, LibError>;

// 内部辅助函数（私有）
fn _internal_helper() -> &'static str {
    "这是一个内部辅助函数"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_constants() {

    }

    #[test]
    fn test_internal_helper() {
        assert_eq!(_internal_helper(), "这是一个内部辅助函数");
    }
}
