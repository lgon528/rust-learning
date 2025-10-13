//! 模块系统演示程序
//! 
//! 这个二进制程序演示如何使用 module-system-demo 库的各种功能

use module_system_demo::{
    config::{Config, ConfigBuilder, Environment},
    network::{Client, Server, NetworkConfig},
    utils::{
        string_utils::{capitalize, to_snake_case, word_count},
        time_utils::{format_duration, current_timestamp},
        validation::{validate_email, validate_url, Validator},
    },
    init, NAME, VERSION,
};

#[cfg(feature = "serde_support")]
use module_system_demo::utils::time_utils::chrono_utils::{now_utc, format_iso_date};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== {} v{} 演示程序 ===", NAME, VERSION);
    println!();

    // 初始化库
    init()?;
    println!("✓ 库初始化完成");
    println!();

    // 演示配置管理
    demo_config_management()?;
    println!();

    // 演示网络模块
    demo_network_module()?;
    println!();

    // 演示字符串工具
    demo_string_utils();
    println!();

    // 演示时间工具
    demo_time_utils();
    println!();

    // 演示数据验证
    demo_validation()?;
    println!();

    // 演示条件编译功能
    demo_conditional_features();

    println!("演示程序执行完成！");
    Ok(())
}

/// 演示配置管理功能
fn demo_config_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 配置管理演示 ---");

    // 创建默认配置
    let config = Config::default();
    println!("默认配置: {}", config.app_name);
    println!("环境: {:?}", config.environment);
    println!("调试模式: {}", config.is_development());

    // 使用构建器创建自定义配置
    let custom_config = ConfigBuilder::new()
        .app_name("演示应用".to_string())
        .environment(Environment::Production)
        .build()
        .unwrap();

    println!("自定义配置: {}", custom_config.app_name);
    println!("网络地址: {}:{}", 
        custom_config.network.host,
        custom_config.network.port
    );

    // 验证配置
    match custom_config.validate() {
        Ok(_) => println!("✓ 配置验证通过"),
        Err(e) => println!("✗ 配置验证失败: {}", e),
    }

    Ok(())
}

/// 演示网络模块功能
fn demo_network_module() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 网络模块演示 ---");

    // 创建网络配置
    let _network_config = NetworkConfig {
        timeout: 30,
        retry_count: 3,
        buffer_size: 8192,
    };

    // 演示客户端
    let mut client = Client::new("127.0.0.1:8080".to_string());
    println!("客户端状态: {:?}", client.status());

    // 模拟连接
    match client.connect() {
        Ok(_) => {
            println!("✓ 客户端连接成功");
            println!("连接后状态: {:?}", client.status());
            
            // 模拟发送数据
            if let Err(e) = client.send(b"Hello, Server!") {
                println!("发送数据失败: {}", e);
            } else {
                println!("✓ 数据发送成功");
            }
            
            client.disconnect();
            println!("✓ 客户端断开连接");
        }
        Err(e) => println!("✗ 客户端连接失败: {}", e),
    }

    // 演示服务器
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    println!("服务器状态: {:?}", server.status());

    match server.start() {
        Ok(_) => {
            println!("✓ 服务器启动成功");
            println!("启动后状态: {:?}", server.status());
            
            server.stop();
            println!("✓ 服务器停止");
        }
        Err(e) => println!("✗ 服务器启动失败: {}", e),
    }

    Ok(())
}

/// 演示字符串工具功能
fn demo_string_utils() {
    println!("--- 字符串工具演示 ---");

    let test_string = "hello world rust programming";
    println!("原始字符串: '{}'", test_string);
    
    // 字符串操作
    println!("首字母大写: '{}'", capitalize(test_string));
    println!("蛇形命名: '{}'", to_snake_case(test_string));
    println!("单词数量: {}", word_count(test_string));
    
    // 更多示例
    let long_string = "这是一个很长的字符串，用来演示截断功能";
    println!("长字符串: '{}'", long_string);
    
    let examples = vec![
        "CamelCaseExample",
        "snake_case_example", 
        "kebab-case-example",
        "Mixed_Case-Example",
    ];
    
    for example in examples {
        println!("'{}' -> 蛇形: '{}'", example, to_snake_case(example));
    }
}

/// 演示时间工具功能
fn demo_time_utils() {
    println!("--- 时间工具演示 ---");

    // 当前时间戳
    let timestamp = current_timestamp();
    println!("当前时间戳: {}", timestamp);
    
    // 格式化持续时间
    let durations = vec![
        std::time::Duration::from_secs(30),
        std::time::Duration::from_secs(90),
        std::time::Duration::from_secs(3661),
        std::time::Duration::from_secs(86461),
    ];
    
    for duration in durations {
        println!("持续时间 {:?} -> '{}'", duration, format_duration(duration));
    }
    
    // 演示计时器
    use module_system_demo::utils::time_utils::Timer;
    let timer = Timer::new("demo_timer");
    
    // 模拟一些工作
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    let elapsed = timer.elapsed();
    println!("计时器测量: {}", format_duration(elapsed));
}

/// 演示数据验证功能
fn demo_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 数据验证演示 ---");

    // 邮箱验证
    let emails = vec![
        "user@example.com",
        "invalid-email",
        "test@domain.co.uk",
        "@invalid.com",
    ];
    
    for email in emails {
        match validate_email(email) {
            Ok(_) => println!("✓ 邮箱 '{}' 有效", email),
            Err(e) => println!("✗ 邮箱 '{}' 无效: {}", email, e),
        }
    }
    
    // URL验证
    let urls = vec![
        "https://www.example.com",
        "http://localhost:8080",
        "invalid-url",
        "ftp://files.example.com",
    ];
    
    for url in urls {
        match validate_url(url) {
            Ok(_) => println!("✓ URL '{}' 有效", url),
            Err(e) => println!("✗ URL '{}' 无效: {}", url, e),
        }
    }
    
    // 使用验证器进行批量验证
    let mut validator = Validator::new();
    
    // 添加多个验证
    validator.validate(|| validate_email("invalid@"));
    validator.validate(|| validate_url("not-a-url"));
    
    // 检查验证结果
    if validator.has_errors() {
        println!("验证器发现 {} 个错误:", validator.errors().len());
        for error in validator.errors() {
            println!("  - {}", error);
        }
    } else {
        println!("✓ 所有验证通过");
    }

    Ok(())
}

/// 演示条件编译功能
fn demo_conditional_features() {
    println!("--- 条件编译功能演示 ---");

    // 检查启用的功能
    let mut features = Vec::new();
    
    #[cfg(feature = "std")]
    features.push("std");
    
    #[cfg(feature = "serde_support")]
    features.push("serde_support");
    
    #[cfg(feature = "logging")]
    features.push("logging");
    
    if features.is_empty() {
        println!("当前没有启用可选功能");
    } else {
        println!("启用的功能: {}", features.join(", "));
    }

    // 演示 serde 功能（如果启用）
    #[cfg(feature = "serde_support")]
    {
        println!("\n--- Serde 功能演示 ---");
        let now = now_utc();
        let formatted = format_iso_date(now);
        println!("当前UTC时间: {}", formatted);
    }

    // 演示日志功能（如果启用）
    #[cfg(feature = "logging")]
    {
        println!("\n--- 日志功能演示 ---");
        log::info!("这是一条信息日志");
        log::warn!("这是一条警告日志");
        log::debug!("这是一条调试日志");
        println!("日志消息已发送（检查控制台输出）");
    }

    #[cfg(not(feature = "logging"))]
    println!("日志功能未启用");
}