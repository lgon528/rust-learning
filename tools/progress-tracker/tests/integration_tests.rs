//! 进度跟踪工具集成测试

use progress_tracker::{
    ProgressTracker, LearningUnit, LearningUnitType, LearningStage, 
    LearningUnitStatus, dashboard::{DashboardRenderer, DashboardConfig}
};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_full_progress_tracking_workflow() {
    // 创建临时目录
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("test-progress.json");
    let html_file = temp_dir.path().join("dashboard.html");
    
    // 1. 创建进度跟踪器
    let mut tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
    
    // 验证初始状态
    assert_eq!(tracker.learner_name, "测试学习者");
    assert_eq!(tracker.learning_units.len(), 3); // 默认示例单元
    assert_eq!(tracker.achievements.len(), 4); // 默认示例成就
    
    // 2. 保存到文件
    tracker.to_file(&progress_file).unwrap();
    assert!(progress_file.exists());
    
    // 3. 从文件加载
    let loaded_tracker = ProgressTracker::from_file(&progress_file).unwrap();
    assert_eq!(loaded_tracker.learner_name, tracker.learner_name);
    assert_eq!(loaded_tracker.learning_units.len(), tracker.learning_units.len());
    
    // 4. 更新学习进度
    let mut updated_tracker = loaded_tracker;
    
    // 完成第一个单元
    if let Some(unit) = updated_tracker.get_unit_mut("stage1-environment") {
        unit.start();
        unit.complete(Some(90.0));
    }
    
    // 开始第二个单元
    if let Some(unit) = updated_tracker.get_unit_mut("stage1-syntax") {
        unit.start();
    }
    
    // 5. 检查成就解锁
    let newly_unlocked = updated_tracker.check_achievements();
    assert!(!newly_unlocked.is_empty()); // 应该解锁 "first_steps" 成就
    
    // 6. 获取进度统计
    let stats = updated_tracker.get_progress_stats();
    assert_eq!(stats.completed_units, 1);
    assert_eq!(stats.in_progress_units, 1);
    assert!(stats.overall_progress > 0.0);
    assert_eq!(stats.average_score, Some(90.0));
    
    // 7. 获取学习推荐
    let recommendation = updated_tracker.get_learning_path_recommendation();
    assert!(!recommendation.next_units.is_empty());
    assert!(recommendation.confidence_score > 0.0);
    
    // 8. 获取个性化建议
    let suggestions = updated_tracker.get_personalized_suggestions();
    assert!(!suggestions.is_empty());
    
    // 9. 生成文本仪表板
    let config = DashboardConfig::default();
    let renderer = DashboardRenderer::new(config);
    let text_dashboard = renderer.render(&updated_tracker);
    
    assert!(text_dashboard.contains("Rust 学习进度跟踪系统"));
    assert!(text_dashboard.contains("测试学习者"));
    assert!(text_dashboard.contains("总体学习进度"));
    assert!(text_dashboard.contains("90.0"));
    
    // 10. 生成 HTML 仪表板
    let html_dashboard = progress_tracker::dashboard::generate_html_dashboard(&updated_tracker);
    fs::write(&html_file, html_dashboard).unwrap();
    assert!(html_file.exists());
    
    let html_content = fs::read_to_string(&html_file).unwrap();
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("Rust 学习进度跟踪系统"));
    assert!(html_content.contains("测试学习者"));
    assert!(html_content.contains("progress-fill"));
    
    // 11. 保存最终状态
    updated_tracker.to_file(&progress_file).unwrap();
    
    println!("✅ 完整工作流测试通过！");
    println!("📊 进度文件: {:?}", progress_file);
    println!("🌐 HTML 仪表板: {:?}", html_file);
}

#[test]
fn test_learning_stages() {
    let stages = LearningStage::all_stages();
    assert_eq!(stages.len(), 5);
    
    for stage in &stages {
        assert!(!stage.name().is_empty());
        assert!(!stage.description().is_empty());
        assert!(stage.estimated_weeks() > 0);
    }
    
    // 测试阶段名称
    assert_eq!(LearningStage::Stage1Basics.name(), "阶段1: 基础入门");
    assert_eq!(LearningStage::Stage2Ownership.name(), "阶段2: 所有权系统");
    assert_eq!(LearningStage::Stage3AdvancedConcepts.name(), "阶段3: 高级概念");
    assert_eq!(LearningStage::Stage4Ecosystem.name(), "阶段4: 生态系统");
    assert_eq!(LearningStage::Stage5Projects.name(), "阶段5: 项目实战");
}

#[test]
fn test_learning_unit_types() {
    let unit_types = vec![
        LearningUnitType::ContentReading,
        LearningUnitType::CodeExample,
        LearningUnitType::Exercise,
        LearningUnitType::Project,
        LearningUnitType::Assessment,
    ];
    
    for unit_type in &unit_types {
        assert!(!unit_type.name().is_empty());
        assert!(unit_type.weight() > 0.0);
    }
    
    // 验证权重总和为1.0
    let total_weight: f32 = unit_types.iter().map(|t| t.weight()).sum();
    assert!((total_weight - 1.0).abs() < 0.001);
}

#[test]
fn test_achievement_system() {
    let mut tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
    
    // 初始状态：没有成就解锁
    let initial_unlocked = tracker.achievements.iter()
        .filter(|a| a.unlocked_at.is_some())
        .count();
    assert_eq!(initial_unlocked, 0);
    
    // 完成一个单元（解锁 "first_steps" 成就）
    if let Some(unit) = tracker.get_unit_mut("stage1-environment") {
        unit.complete(Some(85.0));
    }
    
    let newly_unlocked = tracker.check_achievements();
    assert_eq!(newly_unlocked.len(), 1);
    assert_eq!(newly_unlocked[0], "first_steps");
    
    // 验证成就已解锁
    let first_steps = tracker.achievements.iter()
        .find(|a| a.id == "first_steps")
        .unwrap();
    assert!(first_steps.unlocked_at.is_some());
}

#[test]
fn test_progress_calculation() {
    let mut tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
    
    // 初始进度为0
    let initial_stats = tracker.get_progress_stats();
    assert_eq!(initial_stats.overall_progress, 0.0);
    assert_eq!(initial_stats.completed_units, 0);
    
    // 完成不同类型的单元，验证加权计算
    let mut completed_weight = 0.0;
    let total_weight: f32 = tracker.learning_units.iter()
        .map(|u| u.unit_type.weight())
        .sum();
    
    // 完成第一个单元
    if let Some(unit) = tracker.get_unit_mut("stage1-environment") {
        let weight = unit.unit_type.weight();
        unit.complete(Some(90.0));
        completed_weight += weight;
    }
    
    let stats = tracker.get_progress_stats();
    let expected_progress = (completed_weight / total_weight) * 100.0;
    
    assert!((stats.overall_progress - expected_progress).abs() < 0.1);
    assert_eq!(stats.completed_units, 1);
}

#[test]
fn test_dashboard_config() {
    let config = DashboardConfig::default();
    assert!(config.show_progress_bars);
    assert!(config.show_stage_breakdown);
    assert!(config.show_achievements);
    assert!(config.show_recommendations);
    assert!(config.show_suggestions);
    assert_eq!(config.max_recommendations, 5);
    
    let theme = &config.theme;
    assert!(!theme.primary_color.is_empty());
    assert!(!theme.success_color.is_empty());
    assert!(!theme.warning_color.is_empty());
    assert!(!theme.danger_color.is_empty());
    assert!(!theme.info_color.is_empty());
    assert!(!theme.text_color.is_empty());
    assert!(!theme.background_color.is_empty());
}

#[test]
fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let non_existent_file = temp_dir.path().join("non-existent.json");
    
    // 测试加载不存在的文件
    let result = ProgressTracker::from_file(&non_existent_file);
    assert!(result.is_err());
    
    // 测试获取不存在的单元
    let tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
    let non_existent_unit = tracker.get_unit("non-existent-unit");
    assert!(non_existent_unit.is_none());
}

#[test]
fn test_file_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let progress_file = temp_dir.path().join("persistence-test.json");
    
    // 创建并修改跟踪器
    let mut tracker = ProgressTracker::new("persistence-test".to_string(), "持久化测试".to_string());
    
    // 完成一些单元
    for unit in &mut tracker.learning_units {
        unit.complete(Some(85.0));
    }
    
    // 保存到文件
    tracker.to_file(&progress_file).unwrap();
    
    // 从文件加载
    let loaded_tracker = ProgressTracker::from_file(&progress_file).unwrap();
    
    // 验证数据一致性
    assert_eq!(loaded_tracker.learner_id, tracker.learner_id);
    assert_eq!(loaded_tracker.learner_name, tracker.learner_name);
    assert_eq!(loaded_tracker.learning_units.len(), tracker.learning_units.len());
    assert_eq!(loaded_tracker.achievements.len(), tracker.achievements.len());
    
    // 验证单元状态
    for (original, loaded) in tracker.learning_units.iter().zip(loaded_tracker.learning_units.iter()) {
        assert_eq!(original.id, loaded.id);
        assert_eq!(original.status, loaded.status);
        assert_eq!(original.score, loaded.score);
    }
}