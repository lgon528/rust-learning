//! 高级模块系统用法示例
//! 
//! 这个示例演示了更复杂的模块系统使用场景，包括：
//! - 模块重新导出和别名
//! - 条件编译的高级用法
//! - 工作空间内的依赖管理
//! - 特征和泛型的模块组织

use module_system_demo::{
    // 使用别名简化长路径
    config::{Config as AppConfig, ConfigBuilder as Builder, Environment as Env},
    
    // 重新导出常用类型
    network::{
        Client, Server,
        NetworkConfig as NetConfig,
        NetworkStatus as NetStatus,
    },
    
    // 导入工具模块的特定功能
    utils::{
        string_utils,
        time_utils::{Timer, format_duration, current_timestamp},
        validation,
    },
    
    // 库级别的常量和函数
    NAME, VERSION, init,
    LibError,
};

// 条件导入：只在特定功能启用时导入
#[cfg(feature = "serde_support")]
#[cfg(feature = "logging")]
use log::{info};

/// 应用程序状态管理
#[derive(Debug)]
struct AppState {
    config: AppConfig,
    clients: Vec<Client>,
    servers: Vec<Server>,
    metrics: AppMetrics,
}

/// 应用程序指标
#[derive(Debug, Default)]
struct AppMetrics {
    connections_made: u64,
    messages_sent: u64,
    errors_encountered: u64,
    uptime_start: Option<std::time::SystemTime>,
}

/// 网络管理器
struct NetworkManager {
    _config: NetConfig,
    active_connections: std::collections::HashMap<String, NetStatus>,
}

/// 配置管理器
struct ConfigManager {
    current_config: AppConfig,
    config_history: Vec<AppConfig>,
    _validation_errors: Vec<LibError>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打印库信息
    println!("=== {} v{} 高级用法示例 ===", NAME, VERSION);
    
    // 初始化库
    init()?;
    
    #[cfg(feature = "logging")]
    info!("开始高级用法演示");
    
    // 创建应用程序状态
    let mut app_state = create_app_state()?;
    
    // 演示各种高级功能
    demo_advanced_config_management(&mut app_state)?;
    demo_network_management(&mut app_state)?;
    demo_conditional_compilation()?;
    demo_error_handling_and_validation()?;
    demo_performance_monitoring(&mut app_state)?;
    
    // 清理资源
    cleanup_app_state(&mut app_state)?;
    
    println!("\n高级用法演示完成！");
    
    #[cfg(feature = "logging")]
    info!("高级用法演示完成");
    
    Ok(())
}

/// 创建应用程序状态
fn create_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let config = Builder::new()
        .app_name("AdvancedDemo".to_string())
        .environment(Env::Development)
        .build()?;
    
    let metrics = AppMetrics {
        uptime_start: Some(std::time::SystemTime::now()),
        ..Default::default()
    };
    
    Ok(AppState {
        config,
        clients: Vec::new(),
        servers: Vec::new(),
        metrics,
    })
}

/// 演示高级配置管理
fn demo_advanced_config_management(app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 高级配置管理演示 ---");
    
    // 创建多个配置实例
    let dev_config = Builder::new()
        .app_name("DevApp".to_string())
        .environment(Env::Development)
        .build()?;
    
    let test_config = Builder::new()
        .app_name("TestApp".to_string())
        .environment(Env::Test)
        .build()?;
    
    let prod_config = Builder::new()
        .app_name("ProdApp".to_string())
        .environment(Env::Production)
        .build()?;
    
    // 演示配置验证
    println!("验证开发配置: {:?}", dev_config.validate());
    println!("验证测试配置: {:?}", test_config.validate());
    println!("验证生产配置: {:?}", prod_config.validate());
    
    // 演示环境检查
    println!("开发环境检查: {}", dev_config.is_development());
    println!("测试环境检查: {}", test_config.is_test());
    println!("生产环境检查: {}", prod_config.is_production());
    
    // 配置管理器演示
    let config_manager = ConfigManager {
        current_config: app_state.config.clone(),
        config_history: vec![dev_config, test_config, prod_config],
        _validation_errors: Vec::new(),
    };
    
    println!("配置历史记录数量: {}", config_manager.config_history.len());
    println!("当前配置应用名: {}", config_manager.current_config.app_name);
    
    #[cfg(feature = "serde_support")]
    {
        // 演示配置序列化
        use module_system_demo::serialization::utils;
        if let Ok(json) = utils::to_json(&config_manager.current_config) {
            println!("配置 JSON: {}", json);
        }
    }
    
    Ok(())
}

/// 演示网络管理
fn demo_network_management(app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 网络管理演示 ---");
    
    // 创建网络配置
    let net_config = NetConfig::default();
    
    // 创建网络管理器
    let mut network_manager = NetworkManager {
        _config: net_config,
        active_connections: std::collections::HashMap::new(),
    };
    
    // 创建客户端和服务器
    let mut client = Client::new("demo-client".to_string());
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    
    println!("客户端初始状态: {:?}", client.status());
    println!("服务器初始状态: {:?}", server.status());
    
    // 启动服务器
    if server.start().is_ok() {
        println!("服务器启动成功");
        network_manager.active_connections.insert("server".to_string(), server.status());
        app_state.metrics.connections_made += 1;
    }
    
    // 连接客户端
    if client.connect().is_ok() {
        println!("客户端连接成功");
        network_manager.active_connections.insert("client".to_string(), client.status());
        app_state.metrics.connections_made += 1;
    }
    
    // 发送数据
    let test_message = b"Hello from advanced demo!";
    if client.send(test_message).is_ok() {
        println!("消息发送成功: {:?}", std::str::from_utf8(test_message));
        app_state.metrics.messages_sent += 1;
    }
    
    // 断开连接
    client.disconnect();
    server.stop();
    
    println!("网络连接已清理");
    
    // 保存到应用状态
    app_state.clients.push(client);
    app_state.servers.push(server);
    
    Ok(())
}

/// 演示条件编译
fn demo_conditional_compilation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 条件编译演示 ---");
    
    #[cfg(feature = "serde_support")]
    {
        println!("serde_support 功能已启用");
        let config = AppConfig::default();
        use module_system_demo::serialization::utils;
        if let Ok(_json) = utils::to_json(&config) {
            println!("序列化功能正常工作");
        }
    }
    
    #[cfg(not(feature = "serde_support"))]
    {
        println!("serde_support 功能未启用");
    }
    
    #[cfg(feature = "logging")]
    {
        println!("logging 功能已启用");
        info!("这是一条日志消息");
    }
    
    #[cfg(not(feature = "logging"))]
    {
        println!("logging 功能未启用");
    }
    
    #[cfg(debug_assertions)]
    {
        println!("调试模式已启用");
    }
    
    #[cfg(not(debug_assertions))]
    {
        println!("发布模式已启用");
    }
    
    Ok(())
}

/// 演示错误处理和验证
fn demo_error_handling_and_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 错误处理和验证演示 ---");
    
    // 演示验证功能
    let test_email = "test@example.com";
    let invalid_email = "invalid-email";
    let test_url = "https://example.com";
    let invalid_url = "not-a-url";
    
    println!("邮箱验证 '{}': {}", test_email, validation::validate_email(test_email).is_ok());
    println!("邮箱验证 '{}': {}", invalid_email, validation::validate_email(invalid_email).is_ok());
    println!("URL验证 '{}': {}", test_url, validation::validate_url(test_url).is_ok());
    println!("URL验证 '{}': {}", invalid_url, validation::validate_url(invalid_url).is_ok());
    
    // 演示长度验证
    let test_string = "Hello World";
    println!("长度验证 '{}' (1-20): {}", test_string, validation::validate_length(test_string, "test_field", Some(1), Some(20)).is_ok());
    println!("长度验证 '{}' (20-30): {}", test_string, validation::validate_length(test_string, "test_field", Some(20), Some(30)).is_ok());
    
    // 演示范围验证
    println!("范围验证 5 (1-10): {}", validation::validate_range(5, "number_field", Some(1), Some(10)).is_ok());
    println!("范围验证 15 (1-10): {}", validation::validate_range(15, "number_field", Some(1), Some(10)).is_ok());
    
    // 演示模式验证
    println!("模式验证 '123' (数字): {}", validation::validate_pattern("123", "number_field", "^[0-9]+$", "数字").is_ok());
    println!("模式验证 'abc' (数字): {}", validation::validate_pattern("abc", "text_field", "^[0-9]+$", "数字").is_ok());
    
    // 演示错误处理
    let invalid_config_result = Builder::new()
        .app_name("".to_string()) // 空名称应该导致错误
        .build();
    
    match invalid_config_result {
        Ok(_) => println!("配置创建成功（意外）"),
        Err(e) => println!("配置创建失败（预期）: {}", e),
    }
    
    Ok(())
}

/// 演示性能监控
fn demo_performance_monitoring(app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 性能监控演示 ---");
    
    let timer = Timer::new("performance_test");
    
    // 模拟一些计算密集型工作
    println!("执行性能测试...");
    
    // 字符串处理性能测试
    let test_strings: Vec<String> = (0..1000)
        .map(|i| format!("TestString{}_WithSomeContent", i))
        .collect();
    
    let string_timer = Timer::new("string_test");
    let mut processed_count = 0;
    
    for s in &test_strings {
        let _snake_case = string_utils::to_snake_case(s);
        let _capitalized = string_utils::capitalize(s);
        let _word_count = string_utils::word_count(s);
        processed_count += 1;
    }
    
    let string_elapsed = string_timer.elapsed();
    println!("字符串处理: {} 个字符串，耗时 {}", 
        processed_count, 
        format_duration(string_elapsed)
    );
    
    // 时间戳生成性能测试
    let timestamp_timer = Timer::new("timestamp_test");
    let mut timestamps = Vec::new();
    
    for _ in 0..1000 {
        timestamps.push(current_timestamp());
    }
    
    let timestamp_elapsed = timestamp_timer.elapsed();
    println!("时间戳生成: {} 个时间戳，耗时 {}", 
        timestamps.len(), 
        format_duration(timestamp_elapsed)
    );
    
    // 计算总体性能指标
    let total_elapsed = timer.elapsed();
    
    if let Some(uptime_start) = app_state.metrics.uptime_start {
        let uptime = uptime_start.elapsed().unwrap_or_default();
        println!("\n性能摘要:");
        println!("  应用运行时间: {}", format_duration(uptime));
        println!("  性能测试耗时: {}", format_duration(total_elapsed));
        println!("  平均每次连接耗时: {:.2}ms", 
            total_elapsed.as_millis() as f64 / app_state.metrics.connections_made.max(1) as f64
        );
    }
    
    #[cfg(feature = "logging")]
    info!("性能监控完成，总耗时: {:?}", total_elapsed);
    
    Ok(())
}

/// 清理应用程序状态
fn cleanup_app_state(app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- 清理资源 ---");
    
    #[cfg(feature = "logging")]
    info!("开始清理应用程序资源");
    
    // 断开所有客户端连接
    for client in &mut app_state.clients {
        client.disconnect();
        #[cfg(feature = "logging")]
        info!("客户端已断开连接");
    }
    
    // 停止所有服务器
    for server in &mut app_state.servers {
        server.stop();
        #[cfg(feature = "logging")]
        info!("服务器已停止");
    }
    
    // 清空集合
    app_state.clients.clear();
    app_state.servers.clear();
    
    // 打印最终统计
    println!("最终统计:");
    println!("  连接数: {}", app_state.metrics.connections_made);
    println!("  消息数: {}", app_state.metrics.messages_sent);
    println!("  错误数: {}", app_state.metrics.errors_encountered);
    
    if let Some(uptime_start) = app_state.metrics.uptime_start {
        let total_uptime = uptime_start.elapsed().unwrap_or_default();
        println!("  总运行时间: {}", format_duration(total_uptime));
    }
    
    #[cfg(feature = "logging")]
    info!("资源清理完成");
    
    Ok(())
}