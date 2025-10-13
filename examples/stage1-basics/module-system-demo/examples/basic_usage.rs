//! 基本使用示例
//!
//! 演示模块系统的基本功能

use module_system_demo::{
    Config,
    config::Environment,
    network::{Client, Server},
    utils::validation::Validator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 模块系统基本使用示例 ===");
    
    // 1. 配置管理
    println!("\n--- 配置管理 ---");
    let mut config = Config::default();
    config.environment = Environment::Development;
    println!("应用名称: {}", config.app_name);
    println!("环境: {:?}", config.environment);
    println!("是否为开发环境: {}", config.is_development());
    
    // 2. 网络客户端
    println!("\n--- 网络客户端 ---");
    let client = Client::new("127.0.0.1:8080".to_string());
    println!("客户端状态: {:?}", client.status());
    println!("客户端地址: {}", client.address());
    
    // 3. 网络服务器
    println!("\n--- 网络服务器 ---");
    let server = Server::new("127.0.0.1".to_string(), 8080);
    println!("服务器状态: {:?}", server.status());
    println!("服务器绑定地址: {}", server.bind_address());
    println!("服务器端口: {}", server.port());
    
    // 4. 数据验证
    println!("\n--- 数据验证 ---");
    let mut validator = Validator::new();
    
    // 验证一些数据
    validator.validate(|| {
        if "test@example.com".contains('@') {
            Ok(())
        } else {
            Err(module_system_demo::utils::validation::ValidationError::Custom("无效的邮箱格式".to_string()))
        }
    });
    
    validator.validate(|| {
        if "password123".len() >= 8 {
            Ok(())
        } else {
            Err(module_system_demo::utils::validation::ValidationError::Custom("密码长度不足".to_string()))
        }
    });
    
    if validator.has_errors() {
        println!("验证失败，错误数量: {}", validator.errors().len());
        for error in validator.errors() {
            println!("  - {}", error);
        }
    } else {
        println!("✓ 所有验证通过");
    }
    
    println!("\n=== 基本使用示例完成 ===");
    Ok(())
}