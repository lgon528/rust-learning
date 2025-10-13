//! 网络模块
//!
//! 提供客户端和服务器功能

pub mod client;
pub mod server;

// 重新导出常用类型
pub use client::Client;
pub use server::Server;

/// 网络配置
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub struct NetworkConfig {
    pub timeout: u64,
    pub retry_count: u32,
    pub buffer_size: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            retry_count: 3,
            buffer_size: 8192,
        }
    }
}

/// 网络状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub enum NetworkStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// 内部网络工具函数（包内可见）
pub(crate) fn validate_address(address: &str) -> bool {
    if address.is_empty() {
        return false;
    }
    
    // 支持IP地址、域名、localhost和测试地址
    address.contains('.') || address == "localhost" || address.starts_with("test-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.timeout, 30);
        assert_eq!(config.retry_count, 3);
        assert_eq!(config.buffer_size, 8192);
    }

    #[test]
    fn test_validate_address() {
        assert!(validate_address("127.0.0.1"));
        assert!(validate_address("example.com"));
        assert!(validate_address("localhost"));
        assert!(validate_address("test-client"));
        assert!(!validate_address(""));
        assert!(!validate_address("invalid"));
    }
}