//! 集成测试
//!
//! 测试库的公共 API 和模块间的集成

use module_system_demo::{
    config::{Config, ConfigBuilder, Environment},
    network::{Client, Server, NetworkStatus},
    utils::{string_utils, time_utils, validation},
    NAME, VERSION,
    LibError,
};

#[cfg(feature = "serde_support")]
use module_system_demo::serialization::Serializable;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_library_initialization() {
    // 测试库常量
    assert_eq!(NAME, "module-system-demo");
    assert!(!VERSION.is_empty());
    
    // 测试库初始化
    assert!(module_system_demo::init().is_ok());
}

#[test]
fn test_config_management() {
    // 测试默认配置
    let config = Config::default();
    assert_eq!(config.app_name, "module-system-demo");
    assert_eq!(config.environment, Environment::Development);
    
    // 测试配置构建器
    let config = ConfigBuilder::new()
        .app_name("TestApp".to_string())
        .environment(Environment::Test)
        .build()
        .expect("配置构建失败");
    
    assert_eq!(config.app_name, "TestApp");
    assert_eq!(config.environment, Environment::Test);
    
    // 测试配置验证
    assert!(config.validate().is_ok());
    
    // 测试环境检查方法
    assert!(config.is_test());
    assert!(!config.is_production());
    assert!(!config.is_development());
}

#[test]
fn test_network_module() {
    // 测试客户端
    let mut client = Client::new("test-client".to_string());
    assert_eq!(client.status(), NetworkStatus::Disconnected);
    
    // 测试连接
    assert!(client.connect().is_ok());
    assert_eq!(client.status(), NetworkStatus::Connected);
    
    // 测试数据发送
    assert!(client.send(b"test message").is_ok());
    
    // 测试断开连接
    client.disconnect();
    assert_eq!(client.status(), NetworkStatus::Disconnected);
    
    // 测试服务器
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    assert_eq!(server.status(), NetworkStatus::Disconnected);
    
    // 测试启动服务器
    assert!(server.start().is_ok());
    assert_eq!(server.status(), NetworkStatus::Connected);
    
    // 测试停止服务器
    server.stop();
    assert_eq!(server.status(), NetworkStatus::Disconnected);
}

#[test]
fn test_string_utilities() {
    // 测试字符串工具
    assert_eq!(string_utils::capitalize("hello"), "Hello");
    assert_eq!(string_utils::to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(string_utils::to_camel_case("hello_world"), "helloWorld");
    assert_eq!(string_utils::word_count("hello world test"), 3);
    assert_eq!(string_utils::reverse_string("hello"), "olleh");
    
    // 测试字符串相似度
    let similarity = string_utils::string_similarity("hello", "hallo");
    assert!(similarity >= 0.0 && similarity <= 1.0);
}

#[test]
fn test_time_utilities() {
    // 测试时间工具
    let timestamp = time_utils::current_timestamp();
    assert!(timestamp > 0);
    
    // 测试持续时间格式化
    let duration = Duration::from_secs(3661); // 1小时1分1秒
    let formatted = time_utils::format_duration(duration);
    assert!(!formatted.is_empty());
    
    // 测试时间戳转换
    let system_time = time_utils::timestamp_to_system_time(timestamp);
    assert!(system_time > std::time::UNIX_EPOCH);
    
    // 测试时间差计算
    let start_time = time_utils::timestamp_to_system_time(timestamp);
    let end_time = time_utils::timestamp_to_system_time(timestamp + 1);
    let diff = time_utils::time_diff(start_time, end_time);
    assert_eq!(diff, Duration::from_secs(1));
    
    // 测试时间范围验证
    assert!(time_utils::is_timestamp_in_range(timestamp, timestamp - 100, timestamp + 100));
    assert!(!time_utils::is_timestamp_in_range(timestamp + 200, timestamp - 100, timestamp + 100));
}

#[test]
fn test_validation_utilities() {
    // 测试邮箱验证
    assert!(validation::validate_email("test@example.com").is_ok());
    assert!(validation::validate_email("invalid-email").is_err());
    
    // 测试 URL 验证
    assert!(validation::validate_url("https://example.com").is_ok());
    assert!(validation::validate_url("not-a-url").is_err());
    
    // 测试长度验证
    assert!(validation::validate_length("hello", "text", Some(1), Some(10)).is_ok());
    assert!(validation::validate_length("hello", "text", Some(10), Some(20)).is_err());
    
    // 测试范围验证
    assert!(validation::validate_range(5, "number", Some(1), Some(10)).is_ok());
    assert!(validation::validate_range(15, "number", Some(1), Some(10)).is_err());
    
    // 测试模式验证
    assert!(validation::validate_pattern("123", "text", r"^\d+$", "数字").is_ok());
    assert!(validation::validate_pattern("abc", "text", r"^\d+$", "数字").is_err());
}

#[cfg(feature = "serde_support")]
#[test]
fn test_serialization() {
    use module_system_demo::serialization::{utils, SerializationFormat, Serializable};
    
    let config = Config::default();
    
    // 测试 JSON 序列化
    let json_result = utils::to_json(&config);
    assert!(json_result.is_ok());
    
    let json_str = json_result.unwrap();
    assert!(json_str.contains("module-system-demo"));
    
    // 测试从 JSON 反序列化
    let deserialized: Result<Config, _> = utils::from_json(&json_str);
    assert!(deserialized.is_ok());
    
    let deserialized_config = deserialized.unwrap();
    assert_eq!(deserialized_config.app_name, config.app_name);
    
    // 测试 TOML 序列化
    let toml_result = utils::to_toml(&config);
    assert!(toml_result.is_ok());
    
    let toml_str = toml_result.unwrap();
    assert!(toml_str.contains("module-system-demo"));
    
    // 测试从 TOML 反序列化
    let deserialized: Result<Config, _> = utils::from_toml(&toml_str);
    assert!(deserialized.is_ok());
    
    // 测试 Serializable trait 方法
    let json_via_trait = config.serialize_to_string(SerializationFormat::Json).unwrap();
    let toml_via_trait = config.serialize_to_string(SerializationFormat::Toml).unwrap();
    assert!(json_via_trait.contains("module-system-demo"));
    assert!(toml_via_trait.contains("module-system-demo"));
}

#[test]
fn test_error_handling() {
    // 测试网络错误
    let mut client = Client::new("invalid-address".to_string());
    let connect_result = client.connect();
    assert!(connect_result.is_err());
    
    if let Err(LibError::Network(msg)) = connect_result {
        assert!(msg.contains("无效的地址"));
    } else {
        panic!("期望网络错误");
    }
    
    // 测试配置错误
    let invalid_config = ConfigBuilder::new()
        .app_name("".to_string()) // 空名称应该导致验证失败
        .build();
    
    assert!(invalid_config.is_err());
}

#[test]
fn test_concurrent_access() {
    let config = Arc::new(Config::default());
    let mut handles = vec![];
    
    // 创建多个线程同时访问配置
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            // 在每个线程中访问配置
            let app_name = config_clone.app_name.clone();
            let env = config_clone.environment;
            
            // 创建客户端（测试线程安全性）
            let client = Client::new(format!("client-{}", i));
            
            (app_name, env, client)
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        let (app_name, env, client) = handle.join().expect("线程执行失败");
        assert_eq!(app_name, "module-system-demo");
        assert_eq!(env, Environment::Development);
        assert_eq!(client.status(), NetworkStatus::Disconnected);
    }
}

#[test]
fn test_integrated_workflow() {
    // 创建配置
    let config = ConfigBuilder::new()
        .app_name("IntegrationTest".to_string())
        .environment(Environment::Test)
        .build()
        .expect("配置创建失败");
    
    // 验证配置
    assert!(config.validate().is_ok());
    
    // 创建网络组件
    let mut client = Client::new("test.example.com".to_string());
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    
    // 启动服务器
    assert!(server.start().is_ok());
    
    // 连接客户端
    assert!(client.connect().is_ok());
    
    // 处理消息
    let message = "Hello, Integration Test!";
    let processed_message = string_utils::capitalize(&format!("processed: {}", message));
    
    // 发送处理后的消息
    assert!(client.send(processed_message.as_bytes()).is_ok());
    
    // 使用验证工具验证配置
    assert!(validation::validate_length(&config.app_name, "app_name", Some(1), Some(100)).is_ok());
    
    // 清理
    client.disconnect();
    server.stop();
}