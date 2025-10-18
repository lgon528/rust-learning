use criterion::{criterion_group, criterion_main, Criterion};
use progress_tracker::{
    ProgressTracker, LearningUnit, LearningUnitType, LearningStage,
    dashboard::generate_html_dashboard
};
use serde_json;
use std::collections::HashMap;

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

// HTML生成基准测试
fn bench_html_generation(c: &mut Criterion) {
    let tracker = create_test_tracker();
    
    c.bench_function("html_generation", |b| {
        b.iter(|| {
            generate_html_dashboard(&tracker)
        })
    });
}

// JSON序列化基准测试
fn bench_json_serialization(c: &mut Criterion) {
    let tracker = create_test_tracker();
    
    c.bench_function("json_serialization", |b| {
        b.iter(|| {
            serde_json::to_string(&tracker).unwrap()
        })
    });
}

// 进度统计基准测试
fn bench_progress_stats(c: &mut Criterion) {
    let tracker = create_test_tracker();
    
    c.bench_function("progress_stats", |b| {
        b.iter(|| {
            tracker.get_progress_stats()
        })
    });
}

// 学习路径推荐基准测试
fn bench_learning_path_recommendation(c: &mut Criterion) {
    let tracker = create_test_tracker();
    
    c.bench_function("learning_path_recommendation", |b| {
        b.iter(|| {
            tracker.get_learning_path_recommendation()
        })
    });
}

// 字符串操作基准测试 - 无预分配
fn bench_string_concat_no_capacity(c: &mut Criterion) {
    c.bench_function("string_concat_no_capacity", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..1000 {
                result.push_str(&format!("item_{} ", i));
            }
            result
        })
    });
}

// 字符串操作基准测试 - 预分配容量
fn bench_string_concat_with_capacity(c: &mut Criterion) {
    c.bench_function("string_concat_with_capacity", |b| {
        b.iter(|| {
            let mut result = String::with_capacity(10000);
            for i in 0..1000 {
                result.push_str(&format!("item_{} ", i));
            }
            result
        })
    });
}

// HashMap操作基准测试 - 无预分配
fn bench_hashmap_no_capacity(c: &mut Criterion) {
    c.bench_function("hashmap_no_capacity", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(format!("key_{}", i), i);
            }
            map
        })
    });
}

// HashMap操作基准测试 - 预分配容量
fn bench_hashmap_with_capacity(c: &mut Criterion) {
    c.bench_function("hashmap_with_capacity", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(1000);
            for i in 0..1000 {
                map.insert(format!("key_{}", i), i);
            }
            map
        })
    });
}

// 配置criterion以禁用所有图表和HTML生成
fn fast_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10)  // 减少样本数量
        .measurement_time(std::time::Duration::from_secs(5))  // 减少测量时间
        .warm_up_time(std::time::Duration::from_secs(1))     // 减少预热时间
        .without_plots()  // 禁用图表生成
        .with_output_color(false)  // 禁用颜色输出
}

criterion_group! {
    name = benches;
    config = fast_criterion();
    targets = 
        bench_html_generation,
        bench_json_serialization, 
        bench_progress_stats,
        bench_learning_path_recommendation,
        bench_string_concat_no_capacity,
        bench_string_concat_with_capacity,
        bench_hashmap_no_capacity,
        bench_hashmap_with_capacity
}

criterion_main!(benches);