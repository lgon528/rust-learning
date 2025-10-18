//! 性能基准测试
//! 
//! 这个文件包含了库各个模块的性能基准测试
//! 运行命令: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use module_system_demo::{
    config::{Config, ConfigBuilder, Environment},
    network::{Client, Server, NetworkConfig},
    utils::{
        string_utils::*,
        time_utils::*,
        validation::*,
    },
};
use std::time::Duration;

/// 字符串工具性能基准测试
fn bench_string_utils(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_utils");
    
    // 测试不同长度的字符串
    let long_string = "very_long_string_with_many_words_and_characters".repeat(10);
    let test_strings = vec![
        ("short", "hello"),
        ("medium", "hello_world_rust_programming"),
        ("long", &long_string),
    ];
    
    for (name, input) in test_strings {
        // 首字母大写基准测试
        group.bench_with_input(
            BenchmarkId::new("capitalize", name),
            input,
            |b, s| b.iter(|| capitalize(black_box(s)))
        );
        
        // 蛇形命名转换基准测试
        group.bench_with_input(
            BenchmarkId::new("to_snake_case", name),
            input,
            |b, s| b.iter(|| to_snake_case(black_box(s)))
        );
        
        // 驼峰命名转换基准测试
        group.bench_with_input(
            BenchmarkId::new("to_camel_case", name),
            input,
            |b, s| b.iter(|| to_camel_case(black_box(s)))
        );
        
        // 单词计数基准测试
        group.bench_with_input(
            BenchmarkId::new("word_count", name),
            input,
            |b, s| b.iter(|| word_count(black_box(s)))
        );
        
        // 字符串反转基准测试
        group.bench_with_input(
            BenchmarkId::new("reverse_string", name),
            input,
            |b, s| b.iter(|| reverse_string(black_box(s)))
        );
    }
    
    // 字符串相似度基准测试
    let string_pairs = [("hello", "hallo"),
        ("programming", "programing"),
        ("rust", "trust"),
        ("benchmark", "benchmrk")];
    
    for (i, (s1, s2)) in string_pairs.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("string_similarity", i),
            &(s1, s2),
            |b, (s1, s2)| b.iter(|| string_similarity(black_box(s1), black_box(s2)))
        );
    }
    
    group.finish();
}

/// 时间工具性能基准测试
fn bench_time_utils(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_utils");
    
    // 时间戳生成基准测试
    group.bench_function("current_timestamp", |b| {
        b.iter(current_timestamp)
    });
    
    group.bench_function("current_timestamp_millis", |b| {
        b.iter(current_timestamp_millis)
    });
    
    // 持续时间格式化基准测试
    let durations = [Duration::from_secs(30),
        Duration::from_secs(90),
        Duration::from_secs(3661),
        Duration::from_secs(86461)];
    
    for (i, duration) in durations.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("format_duration", i),
            duration,
            |b, d| b.iter(|| format_duration(black_box(*d)))
        );
    }
    
    // 时间戳转换基准测试
    let timestamp = current_timestamp();
    group.bench_function("timestamp_to_system_time", |b| {
        b.iter(|| timestamp_to_system_time(black_box(timestamp)))
    });
    
    // 时间差计算基准测试
    let timestamp1 = current_timestamp();
    let timestamp2 = timestamp1 + 1000;
    let systime1 = timestamp_to_system_time(timestamp1);
    let systime2 = timestamp_to_system_time(timestamp2);
    group.bench_function("time_diff", |b| {
        b.iter(|| time_diff(black_box(systime1), black_box(systime2)))
    });
    
    // 时间范围检查基准测试
    let now = current_timestamp();
    group.bench_function("is_timestamp_in_range", |b| {
        b.iter(|| is_timestamp_in_range(
            black_box(now),
            black_box(now - 100),
            black_box(now + 100)
        ))
    });
    
    // 计时器基准测试
    group.bench_function("timer_creation", |b| {
        b.iter(|| Timer::new("benchmark"))
    });
    
    group.bench_function("timer_elapsed", |b| {
        let timer = Timer::new("benchmark");
        b.iter(|| timer.elapsed())
    });
    
    group.finish();
}

/// 验证工具性能基准测试
fn bench_validation_utils(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_utils");
    
    // 邮箱验证基准测试
    let emails = ["user@example.com",
        "test.email+tag@domain.co.uk",
        "invalid-email",
        "very.long.email.address.with.many.dots@very.long.domain.name.com"];
    
    for (i, email) in emails.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("validate_email", i),
            email,
            |b, e| b.iter(|| validate_email(black_box(e)))
        );
    }
    
    // URL验证基准测试
    let urls = ["https://www.example.com",
        "http://localhost:8080/path/to/resource?param=value",
        "ftp://files.example.com",
        "invalid-url"];
    
    for (i, url) in urls.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("validate_url", i),
            url,
            |b, u| b.iter(|| validate_url(black_box(u)))
        );
    }
    
    // 长度验证基准测试
    let long_string = "very_long_string".repeat(100);
    let test_strings = ["short",
        "medium_length_string",
        &long_string];
    
    for (i, s) in test_strings.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("validate_length", i),
            s,
            |b, s| b.iter(|| validate_length(black_box(s), black_box("test_field"), black_box(Some(1)), black_box(Some(1000))))
        );
    }
    
    // 数值范围验证基准测试
    let test_values = [1, 50, 100, 999];
    
    for (i, value) in test_values.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("validate_range", i),
            value,
            |b, v| b.iter(|| validate_range(black_box(*v), black_box("test_field"), black_box(Some(1)), black_box(Some(1000))))
        );
    }
    
    // 模式验证基准测试
    let email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    let test_emails = [
        "user@example.com",
        "invalid-email",
        "test@domain.co.uk",
    ];
    
    for (i, email) in test_emails.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("validate_pattern", i),
            email,
            |b, e| b.iter(|| validate_pattern(black_box(e), black_box("email"), black_box(email_pattern), black_box("email format")))
        );
    }
    
    group.finish();
}

/// 配置管理性能基准测试
fn bench_config_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_management");
    
    // 默认配置创建基准测试
    group.bench_function("config_default", |b| {
        b.iter(Config::default)
    });
    
    // 配置构建器基准测试
    group.bench_function("config_builder", |b| {
        b.iter(|| {
            ConfigBuilder::new()
                .app_name(black_box("test_app".to_string()))
                .environment(black_box(Environment::Production))
                .build()
        })
    });
    
    // 配置验证基准测试
    let config = Config::default();
    group.bench_function("config_validate", |b| {
        b.iter(|| config.validate())
    });
    
    // 配置克隆基准测试
    group.bench_function("config_clone", |b| {
        b.iter(|| config.clone())
    });
    
    // 环境变量加载基准测试（注意：这可能会失败，但我们测试性能）
    group.bench_function("config_from_env", |b| {
        b.iter(Config::from_env)
    });
    
    group.finish();
}

/// 网络模块性能基准测试
fn bench_network_module(c: &mut Criterion) {
    let mut group = c.benchmark_group("network_module");
    
    let _network_config = NetworkConfig {
        timeout: 30,
        retry_count: 3,
        buffer_size: 8192,
    };
    
    // 客户端创建基准测试
    group.bench_function("client_creation", |b| {
        b.iter(|| Client::new(
            black_box("127.0.0.1".to_string())
        ))
    });
    
    // 服务器创建基准测试
    group.bench_function("server_creation", |b| {
        b.iter(|| Server::new(
            black_box("127.0.0.1".to_string()),
            black_box(8080)
        ))
    });
    
    // 客户端状态检查基准测试
    let client = Client::new("127.0.0.1".to_string());
    group.bench_function("client_status", |b| {
        b.iter(|| client.status())
    });
    
    // 服务器状态检查基准测试
    let server = Server::new("127.0.0.1".to_string(), 8080);
    group.bench_function("server_status", |b| {
        b.iter(|| server.status())
    });
    
    group.finish();
}

/// 综合性能基准测试
fn bench_integrated_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("integrated_workflow");
    
    // 完整的配置-网络-验证工作流
    group.bench_function("complete_workflow", |b| {
        b.iter(|| {
            // 1. 创建配置
            let config = ConfigBuilder::new()
                .app_name(black_box("benchmark_app".to_string()))
                .build();
            
            // 2. 验证配置
            let config = config.unwrap();
            let _ = config.validate();
            
            // 3. 创建网络组件
            let client = Client::new(black_box("127.0.0.1".to_string()));
            let server = Server::new(black_box("127.0.0.1".to_string()), black_box(8080));
            
            // 4. 处理字符串数据
            let message = black_box("Hello Benchmark Test");
            let processed = to_snake_case(&capitalize(message));
            
            // 5. 验证处理后的数据
            let _ = validate_length(&processed, "processed", Some(1), Some(100));
            
            // 6. 获取时间戳
            let _ = current_timestamp();
            
            (client, server, processed)
        })
    });
    
    // 批量字符串处理基准测试
    let test_strings: Vec<String> = (0..100)
        .map(|i| format!("TestString{}_WithContent", i))
        .collect();
    
    group.bench_function("batch_string_processing", |b| {
        b.iter(|| {
            for s in &test_strings {
                let _ = capitalize(black_box(s));
                let _ = to_snake_case(black_box(s));
                let _ = word_count(black_box(s));
            }
        })
    });
    
    // 批量验证基准测试
    let test_emails = [
        "user1@example.com",
        "user2@test.org",
        "invalid-email-1",
        "user3@domain.co.uk",
        "invalid-email-2",
    ];
    
    group.bench_function("batch_validation", |b| {
        b.iter(|| {
            let mut validator = Validator::new();
            for email in &test_emails {
                validator.validate(|| validate_email(black_box(email)));
            }
            validator.has_errors()
        })
    });
    
    group.finish();
}

/// 内存使用基准测试
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    // 大量配置对象创建
    group.bench_function("mass_config_creation", |b| {
        b.iter(|| {
            let configs: Vec<Config> = (0..1000)
                .map(|i| ConfigBuilder::new()
                    .app_name(format!("app_{}", i))
                    .build().unwrap())
                .collect();
            configs.len()
        })
    });
    
    // 大量字符串操作
    group.bench_function("mass_string_operations", |b| {
        b.iter(|| {
            let results: Vec<String> = (0..1000)
                .map(|i| {
                    let input = format!("TestString{}", i);
                    let snake = to_snake_case(&input);
                    let camel = to_camel_case(&snake);
                    capitalize(&camel)
                })
                .collect();
            results.len()
        })
    });
    
    group.finish();
}

// 将所有基准测试组合到一起
criterion_group!(
    benches,
    bench_string_utils,
    bench_time_utils,
    bench_validation_utils,
    bench_config_management,
    bench_network_module,
    bench_integrated_workflow,
    bench_memory_usage
);

criterion_main!(benches);