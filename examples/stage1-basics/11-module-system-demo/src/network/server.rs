//! 网络服务器模块

use super::{NetworkConfig, NetworkStatus};
use crate::{LibError, Result};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// 网络服务器
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Server {
    bind_address: String,
    port: u16,
    config: NetworkConfig,
    status: NetworkStatus,
    #[cfg_attr(feature = "serde_support", serde(skip))]
    active_connections: Vec<u64>,
}

impl Server {
    /// 创建新的服务器
    pub fn new(bind_address: String, port: u16) -> Self {
        Self {
            bind_address,
            port,
            config: NetworkConfig::default(),
            status: NetworkStatus::Disconnected,
            active_connections: Vec::new(),
        }
    }

    /// 使用自定义配置创建服务器
    pub fn with_config(bind_address: String, port: u16, config: NetworkConfig) -> Self {
        Self {
            bind_address,
            port,
            config,
            status: NetworkStatus::Disconnected,
            active_connections: Vec::new(),
        }
    }

    /// 启动服务器
    pub fn start(&mut self) -> Result<()> {
        if self.port == 0 {
            return Err(LibError::Network("无效的端口号".to_string()));
        }

        #[cfg(feature = "logging")]
        log::info!("启动服务器 {}:{}", self.bind_address, self.port);

        self.status = NetworkStatus::Connecting;
        
        // 模拟启动过程
        self.status = NetworkStatus::Connected;

        #[cfg(feature = "logging")]
        log::info!("服务器已启动，监听 {}:{}", self.bind_address, self.port);

        Ok(())
    }

    /// 停止服务器
    pub fn stop(&mut self) {
        #[cfg(feature = "logging")]
        log::info!("停止服务器 {}:{}", self.bind_address, self.port);

        // 断开所有连接
        self.active_connections.clear();
        self.status = NetworkStatus::Disconnected;
    }

    /// 接受新连接
    pub fn accept_connection(&mut self) -> Result<u64> {
        if self.status != NetworkStatus::Connected {
            return Err(LibError::Network("服务器未启动".to_string()));
        }

        let connection_id = generate_connection_id();
        self.active_connections.push(connection_id);

        #[cfg(feature = "logging")]
        log::info!("接受新连接，ID: {}", connection_id);

        Ok(connection_id)
    }

    /// 关闭连接
    pub fn close_connection(&mut self, connection_id: u64) -> Result<()> {
        if let Some(pos) = self.active_connections.iter().position(|&id| id == connection_id) {
            self.active_connections.remove(pos);
            
            #[cfg(feature = "logging")]
            log::info!("关闭连接，ID: {}", connection_id);
            
            Ok(())
        } else {
            Err(LibError::Network("连接不存在".to_string()))
        }
    }

    /// 向指定连接发送数据
    pub fn send_to(&self, connection_id: u64, data: &[u8]) -> Result<usize> {
        if !self.active_connections.contains(&connection_id) {
            return Err(LibError::Network("连接不存在".to_string()));
        }

        if data.len() > self.config.buffer_size {
            return Err(LibError::Network("数据大小超过缓冲区限制".to_string()));
        }

        #[cfg(feature = "logging")]
        log::debug!("向连接 {} 发送 {} 字节数据", connection_id, data.len());

        // 模拟发送过程
        Ok(data.len())
    }

    /// 从指定连接接收数据
    pub fn receive_from(&self, connection_id: u64) -> Result<Vec<u8>> {
        if !self.active_connections.contains(&connection_id) {
            return Err(LibError::Network("连接不存在".to_string()));
        }

        #[cfg(feature = "logging")]
        log::debug!("从连接 {} 接收数据", connection_id);

        // 模拟接收过程
        Ok(b"Hello from client".to_vec())
    }

    /// 广播数据到所有连接
    pub fn broadcast(&self, data: &[u8]) -> Result<Vec<usize>> {
        let mut results = Vec::new();
        
        for &connection_id in &self.active_connections {
            match self.send_to(connection_id, data) {
                Ok(size) => results.push(size),
                Err(_) => results.push(0),
            }
        }
        
        Ok(results)
    }

    /// 获取服务器状态
    pub fn status(&self) -> NetworkStatus {
        self.status
    }

    /// 获取绑定地址
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }

    /// 获取端口
    pub fn port(&self) -> u16 {
        self.port
    }

    /// 获取活跃连接数
    pub fn active_connections_count(&self) -> usize {
        self.active_connections.len()
    }

    /// 获取所有活跃连接ID
    pub fn active_connections(&self) -> &[u64] {
        &self.active_connections
    }

    /// 是否正在运行
    pub fn is_running(&self) -> bool {
        self.status == NetworkStatus::Connected
    }

    /// 获取配置（包内可见）
    pub(crate) fn config(&self) -> &NetworkConfig {
        &self.config
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if self.is_running() {
            self.stop();
        }
    }
}

// 私有辅助函数
fn generate_connection_id() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = Server::new("127.0.0.1".to_string(), 8080);
        assert_eq!(server.bind_address(), "127.0.0.1");
        assert_eq!(server.port(), 8080);
        assert_eq!(server.status(), NetworkStatus::Disconnected);
        assert!(!server.is_running());
    }

    #[test]
    fn test_server_start() {
        let mut server = Server::new("127.0.0.1".to_string(), 8080);
        assert!(server.start().is_ok());
        assert_eq!(server.status(), NetworkStatus::Connected);
        assert!(server.is_running());
    }

    #[test]
    fn test_server_invalid_port() {
        let mut server = Server::new("127.0.0.1".to_string(), 0);
        assert!(server.start().is_err());
    }

    #[test]
    fn test_server_accept_connection() {
        let mut server = Server::new("127.0.0.1".to_string(), 8080);
        server.start().unwrap();
        
        let connection_id = server.accept_connection().unwrap();
        assert_eq!(server.active_connections_count(), 1);
        assert!(server.active_connections().contains(&connection_id));
    }

    #[test]
    fn test_server_close_connection() {
        let mut server = Server::new("127.0.0.1".to_string(), 8080);
        server.start().unwrap();
        
        let connection_id = server.accept_connection().unwrap();
        assert!(server.close_connection(connection_id).is_ok());
        assert_eq!(server.active_connections_count(), 0);
    }

    #[test]
    fn test_server_send_to_nonexistent_connection() {
        let server = Server::new("127.0.0.1".to_string(), 8080);
        assert!(server.send_to(999, b"test").is_err());
    }
}