//! 高级功能示例
//!
//! 演示模块系统的高级功能，包括条件编译和特性门控

use module_system_demo::{
    Config,
    config::Environment,
    utils::time_utils::{current_timestamp, format_duration, Timer},
};

#[cfg(feature = "serde_support")]
use module_system_demo::serialization::utils;

#[cfg(feature = "logging")]
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 模块系统高级功能示例 ===");
    
    // 初始化日志（如果启用了 logging 功能）
    #[cfg(feature = "logging")]
    {
        env_logger::init();
        info!("日志系统已初始化");
    }
    
    // 1. 时间工具演示
    println!("\n--- 时间工具 ---");
    let timestamp = current_timestamp();
    println!("当前时间戳: {}", timestamp);
    
    let duration = std::time::Duration::from_secs(3661); // 1小时1分1秒
    println!("格式化持续时间: {}", format_duration(duration));
    
    // 2. 计时器演示
    println!("\n--- 计时器 ---");
    let timer = Timer::new("advanced_demo");
    
    // 模拟一些工作
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let elapsed = timer.elapsed();
    println!("操作耗时: {:?}", elapsed);
    
    // 3. 序列化功能（如果启用了 serde_support 功能）
    #[cfg(feature = "serde_support")]
    {
        println!("\n--- 序列化功能 ---");
        let config = Config::default();
        // 可以通过 from_env() 或其他方法创建配置
        
        match utils::to_json(&config) {
            Ok(json) => println!("配置 JSON: {}", json),
            Err(e) => println!("序列化失败: {}", e),
        }
        
        match utils::to_toml(&config) {
            Ok(toml) => println!("配置 TOML: {}", toml),
            Err(e) => println!("TOML 序列化失败: {}", e),
        }
    }
    
    #[cfg(not(feature = "serde_support"))]
    {
        println!("\n--- 序列化功能未启用 ---");
        println!("使用 --features serde_support 启用序列化功能");
    }
    
    // 4. 环境特定功能
    println!("\n--- 环境配置 ---");
    let mut config = Config::default();
    
    for env in [Environment::Development, Environment::Production, Environment::Test] {
        config.environment = env;
        println!("环境: {:?}", env);
        println!("  - 是否为生产环境: {}", config.is_production());
        println!("  - 是否为开发环境: {}", config.is_development());
        println!("  - 是否为测试环境: {}", config.is_test());
    }
    
    println!("\n=== 高级功能示例完成 ===");
    Ok(())
}