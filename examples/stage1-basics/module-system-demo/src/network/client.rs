//! 网络客户端模块

use super::{NetworkConfig, NetworkStatus, validate_address};
use crate::{LibError, Result};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// 网络客户端
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Client {
    address: String,
    config: NetworkConfig,
    status: NetworkStatus,
    #[cfg_attr(feature = "serde_support", serde(skip))]
    connection_id: Option<u64>,
}

impl Client {
    /// 创建新的客户端
    pub fn new(address: String) -> Self {
        Self {
            address,
            config: NetworkConfig::default(),
            status: NetworkStatus::Disconnected,
            connection_id: None,
        }
    }

    /// 使用自定义配置创建客户端
    pub fn with_config(address: String, config: NetworkConfig) -> Self {
        Self {
            address,
            config,
            status: NetworkStatus::Disconnected,
            connection_id: None,
        }
    }

    /// 连接到服务器
    pub fn connect(&mut self) -> Result<()> {
        if !validate_address(&self.address) {
            return Err(LibError::Network("无效的地址".to_string()));
        }

        #[cfg(feature = "logging")]
        log::info!("正在连接到 {}", self.address);

        self.status = NetworkStatus::Connecting;
        
        // 模拟连接过程
        self.connection_id = Some(generate_connection_id());
        self.status = NetworkStatus::Connected;

        #[cfg(feature = "logging")]
        log::info!("已连接到 {}, 连接ID: {:?}", self.address, self.connection_id);

        Ok(())
    }

    /// 断开连接
    pub fn disconnect(&mut self) {
        #[cfg(feature = "logging")]
        log::info!("断开与 {} 的连接", self.address);

        self.status = NetworkStatus::Disconnected;
        self.connection_id = None;
    }

    /// 发送数据
    pub fn send(&self, data: &[u8]) -> Result<usize> {
        if self.status != NetworkStatus::Connected {
            return Err(LibError::Network("客户端未连接".to_string()));
        }

        if data.len() > self.config.buffer_size {
            return Err(LibError::Network("数据大小超过缓冲区限制".to_string()));
        }

        #[cfg(feature = "logging")]
        log::debug!("发送 {} 字节数据", data.len());

        // 模拟发送过程
        Ok(data.len())
    }

    /// 接收数据
    pub fn receive(&self) -> Result<Vec<u8>> {
        if self.status != NetworkStatus::Connected {
            return Err(LibError::Network("客户端未连接".to_string()));
        }

        #[cfg(feature = "logging")]
        log::debug!("接收数据");

        // 模拟接收过程
        Ok(b"Hello from server".to_vec())
    }

    /// 获取连接状态
    pub fn status(&self) -> NetworkStatus {
        self.status
    }

    /// 获取地址
    pub fn address(&self) -> &str {
        &self.address
    }

    /// 获取配置（包内可见）
    pub(crate) fn config(&self) -> &NetworkConfig {
        &self.config
    }

    /// 是否已连接
    pub fn is_connected(&self) -> bool {
        self.status == NetworkStatus::Connected
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if self.is_connected() {
            self.disconnect();
        }
    }
}

// 私有辅助函数
fn generate_connection_id() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::new("127.0.0.1".to_string());
        assert_eq!(client.address(), "127.0.0.1");
        assert_eq!(client.status(), NetworkStatus::Disconnected);
        assert!(!client.is_connected());
    }

    #[test]
    fn test_client_connect() {
        let mut client = Client::new("127.0.0.1".to_string());
        assert!(client.connect().is_ok());
        assert_eq!(client.status(), NetworkStatus::Connected);
        assert!(client.is_connected());
    }

    #[test]
    fn test_client_invalid_address() {
        let mut client = Client::new("invalid".to_string());
        assert!(client.connect().is_err());
    }

    #[test]
    fn test_client_send_without_connection() {
        let client = Client::new("127.0.0.1".to_string());
        assert!(client.send(b"test").is_err());
    }

    #[test]
    fn test_client_send_with_connection() {
        let mut client = Client::new("127.0.0.1".to_string());
        client.connect().unwrap();
        assert!(client.send(b"test").is_ok());
    }
}