use progress_tracker::{
    ProgressTracker, LearningUnit, LearningUnitType, LearningStage,
    dashboard::generate_html_dashboard
};
use serde_json;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 简单的基准测试结构
#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    iterations: usize,
    total_time: Duration,
    avg_time: Duration,
    min_time: Duration,
    max_time: Duration,
}

impl BenchmarkResult {
    fn new(name: String, times: Vec<Duration>) -> Self {
        let iterations = times.len();
        let total_time = times.iter().sum();
        let avg_time = total_time / iterations as u32;
        let min_time = *times.iter().min().unwrap();
        let max_time = *times.iter().max().unwrap();
        
        Self {
            name,
            iterations,
            total_time,
            avg_time,
            min_time,
            max_time,
        }
    }
    
    fn print(&self) {
        println!("\n=== {} ===", self.name);
        println!("迭代次数: {}", self.iterations);
        println!("总时间: {:?}", self.total_time);
        println!("平均时间: {:?}", self.avg_time);
        println!("最小时间: {:?}", self.min_time);
        println!("最大时间: {:?}", self.max_time);
        println!("每秒操作数: {:.2}", 1_000_000_000.0 / self.avg_time.as_nanos() as f64);
    }
}

/// 运行基准测试
fn benchmark<F>(name: &str, iterations: usize, mut f: F) -> BenchmarkResult 
where
    F: FnMut() -> (),
{
    println!("运行基准测试: {} ({} 次迭代)", name, iterations);
    
    let mut times = Vec::with_capacity(iterations);
    
    // 预热
    for _ in 0..5 {
        f();
    }
    
    // 实际测量
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        times.push(elapsed);
    }
    
    BenchmarkResult::new(name.to_string(), times)
}

fn create_test_tracker() -> ProgressTracker {
    let mut tracker = ProgressTracker::new(
        "test_user".to_string(),
        "Test User".to_string()
    );
    
    // 创建测试学习单元
    for i in 0..100 {
        let unit = LearningUnit::new(
            format!("unit_{}", i),
            format!("Test Unit {}", i),
            LearningUnitType::Exercise,
            LearningStage::Stage1Basics,
            format!("path/to/unit_{}", i),
            30,
        );
        tracker.add_unit(unit);
    }
    
    tracker
}

fn main() {
    println!("\n🚀 Progress Tracker 性能基准测试");
    println!("=================================\n");
    
    let tracker = create_test_tracker();
    let mut results = Vec::new();
    
    // HTML生成基准测试
    let result = benchmark("HTML生成", 50, || {
        let _ = generate_html_dashboard(&tracker);
    });
    result.print();
    results.push(result);
    
    // JSON序列化基准测试
    let result = benchmark("JSON序列化", 100, || {
        let _ = serde_json::to_string(&tracker).unwrap();
    });
    result.print();
    results.push(result);
    
    // 进度统计基准测试
    let result = benchmark("进度统计计算", 1000, || {
        let _ = tracker.get_progress_stats();
    });
    result.print();
    results.push(result);
    
    // 学习路径推荐基准测试
    let result = benchmark("学习路径推荐", 100, || {
        let _ = tracker.get_learning_path_recommendation();
    });
    result.print();
    results.push(result);
    
    // 字符串操作基准测试 - 无预分配
    let result = benchmark("字符串拼接(无预分配)", 100, || {
        let mut result = String::new();
        for i in 0..1000 {
            result.push_str(&format!("item_{} ", i));
        }
    });
    result.print();
    results.push(result);
    
    // 字符串操作基准测试 - 预分配容量
    let result = benchmark("字符串拼接(预分配容量)", 100, || {
        let mut result = String::with_capacity(10000);
        for i in 0..1000 {
            result.push_str(&format!("item_{} ", i));
        }
    });
    result.print();
    results.push(result);
    
    // HashMap操作基准测试 - 无预分配
    let result = benchmark("HashMap插入(无预分配)", 100, || {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(format!("key_{}", i), i);
        }
    });
    result.print();
    results.push(result);
    
    // HashMap操作基准测试 - 预分配容量
    let result = benchmark("HashMap插入(预分配容量)", 100, || {
        let mut map = HashMap::with_capacity(1000);
        for i in 0..1000 {
            map.insert(format!("key_{}", i), i);
        }
    });
    result.print();
    results.push(result);
    
    // 性能对比分析
    println!("\n\n📊 性能对比分析");
    println!("===============\n");
    
    // 字符串操作对比
    let string_no_cap = &results[4];
    let string_with_cap = &results[5];
    let string_improvement = (string_no_cap.avg_time.as_nanos() as f64 / string_with_cap.avg_time.as_nanos() as f64 - 1.0) * 100.0;
    println!("字符串预分配容量性能提升: {:.1}%", string_improvement);
    
    // HashMap操作对比
    let hashmap_no_cap = &results[6];
    let hashmap_with_cap = &results[7];
    let hashmap_improvement = (hashmap_no_cap.avg_time.as_nanos() as f64 / hashmap_with_cap.avg_time.as_nanos() as f64 - 1.0) * 100.0;
    println!("HashMap预分配容量性能提升: {:.1}%", hashmap_improvement);
    
    println!("\n✅ 基准测试完成！");
}