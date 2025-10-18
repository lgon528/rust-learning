//! Rust 学习进度跟踪工具 - 主程序

use progress_tracker::{
    ProgressTracker, dashboard::{DashboardRenderer, DashboardConfig, generate_html_dashboard}
};
use std::path::Path;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rust 学习进度跟踪工具");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("init") => init_progress_tracker(&args),
        Some("show") => show_progress(&args),
        Some("update") => update_progress(&args),
        Some("recommend") => show_recommendations(&args),
        Some("export") => export_dashboard(&args),
        Some("help") | None => {
            show_help();
            Ok(())
        },
        Some(command) => {
            eprintln!("❌ 未知命令: {}", command);
            show_help();
            std::process::exit(1);
        }
    }
}

fn show_help() {
    println!("\n📖 使用方法:");
    println!("  progress-tracker init <learner_name>     - 初始化新的进度跟踪器");
    println!("  progress-tracker show [progress.json]   - 显示学习进度");
    println!("  progress-tracker update <unit_id>       - 更新学习单元状态");
    println!("  progress-tracker recommend [progress.json] - 显示学习推荐");
    println!("  progress-tracker export [progress.json] - 导出 HTML 仪表板");
    println!("  progress-tracker help                   - 显示此帮助信息");
    println!("\n💡 示例:");
    println!("  progress-tracker init \"张三\"");
    println!("  progress-tracker show");
    println!("  progress-tracker update stage1-environment");
    println!("  progress-tracker export");
}

fn init_progress_tracker(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("❌ 请提供学习者名称");
        return Ok(());
    }

    let learner_name = &args[2];
    let learner_id = learner_name.to_lowercase().replace(" ", "-");
    
    println!("🎯 初始化进度跟踪器...");
    println!("学习者名称: {}", learner_name);
    println!("学习者ID: {}", learner_id);

    let tracker = ProgressTracker::new(learner_id.clone(), learner_name.to_string());
    
    let filename = format!("{}-progress.json", learner_id);
    tracker.to_file(&filename)?;
    
    println!("✅ 进度跟踪器已创建: {}", filename);
    println!("📊 已创建 {} 个学习单元和 {} 个成就", 
             tracker.learning_units.len(), 
             tracker.achievements.len());
    
    Ok(())
}

fn show_progress(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let filename = args.get(2).map(|s| s.as_str()).unwrap_or("progress.json");
    
    if !Path::new(filename).exists() {
        eprintln!("❌ 找不到进度文件: {}", filename);
        eprintln!("💡 请先运行: progress-tracker init <learner_name>");
        return Ok(());
    }

    println!("📊 加载进度文件: {}", filename);
    let tracker = ProgressTracker::from_file(filename)?;
    
    let config = DashboardConfig::default();
    let renderer = DashboardRenderer::new(config);
    
    let dashboard = renderer.render(&tracker);
    println!("{}", dashboard);
    
    Ok(())
}

fn update_progress(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("❌ 请提供学习单元ID");
        return Ok(());
    }

    let unit_id = &args[2];
    let filename = args.get(3).map(|s| s.as_str()).unwrap_or("progress.json");
    
    if !Path::new(filename).exists() {
        eprintln!("❌ 找不到进度文件: {}", filename);
        return Ok(());
    }

    let mut tracker = ProgressTracker::from_file(filename)?;
    
    let unit = match tracker.get_unit_mut(unit_id) {
        Some(unit) => unit,
        None => {
            eprintln!("❌ 找不到学习单元: {}", unit_id);
            return Ok(());
        }
    };

    println!("📝 更新学习单元: {}", unit.name);
    println!("当前状态: {}", unit.status.name());
    
    println!("\n📋 可用操作:");
    println!("1. 开始 (start)");
    println!("2. 完成 (complete)");
    println!("3. 跳过 (skip)");
    println!("4. 取消 (cancel)");
    
    print!("请选择操作 (1-4): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    match input.trim() {
        "1" | "start" => {
            unit.start();
            println!("✅ 学习单元已开始");
        },
        "2" | "complete" => {
            print!("请输入分数 (0-100，可选): ");
            io::stdout().flush()?;
            
            let mut score_input = String::new();
            io::stdin().read_line(&mut score_input)?;
            
            let score = score_input.trim().parse::<f32>().ok()
                .filter(|&s| (0.0..=100.0).contains(&s));
            
            unit.complete(score);
            
            if let Some(s) = score {
                println!("✅ 学习单元已完成，分数: {:.1}", s);
            } else {
                println!("✅ 学习单元已完成");
            }
        },
        "3" | "skip" => {
            unit.skip();
            println!("✅ 学习单元已跳过");
        },
        "4" | "cancel" => {
            println!("❌ 操作已取消");
            return Ok(());
        },
        _ => {
            println!("❌ 无效选择");
            return Ok(());
        }
    }

    // 检查成就解锁
    let newly_unlocked = tracker.check_achievements();
    if !newly_unlocked.is_empty() {
        println!("\n🎉 恭喜！解锁了新成就:");
        for achievement_id in newly_unlocked {
            if let Some(achievement) = tracker.achievements.iter().find(|a| a.id == achievement_id) {
                println!("  🏆 {} - {}", achievement.name, achievement.description);
            }
        }
    }

    // 保存更新
    tracker.to_file(filename)?;
    println!("\n💾 进度已保存到: {}", filename);
    
    Ok(())
}

fn show_recommendations(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let filename = args.get(2).map(|s| s.as_str()).unwrap_or("progress.json");
    
    if !Path::new(filename).exists() {
        eprintln!("❌ 找不到进度文件: {}", filename);
        return Ok(());
    }

    let tracker = ProgressTracker::from_file(filename)?;
    let recommendation = tracker.get_learning_path_recommendation();
    
    println!("🎯 学习路径推荐");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("推荐阶段: {}", recommendation.recommended_stage.name());
    println!("置信度: {:.1}%", recommendation.confidence_score * 100.0);
    println!("预计时间: {} 分钟", recommendation.estimated_time_minutes);
    println!("推荐理由: {}", recommendation.reasoning);
    
    if !recommendation.next_units.is_empty() {
        println!("\n📚 推荐学习单元:");
        for (i, unit) in recommendation.next_units.iter().enumerate() {
            let priority_icon = match i {
                0 => "🥇",
                1 => "🥈", 
                2 => "🥉",
                _ => "📖",
            };
            
            println!("  {} {} ({}, {}分钟)", 
                     priority_icon, unit.name, unit.unit_type.name(), unit.estimated_time_minutes);
        }
    }
    
    // 显示个性化建议
    let suggestions = tracker.get_personalized_suggestions();
    if !suggestions.is_empty() {
        println!("\n💡 个性化学习建议:");
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!("  {}. {}", i + 1, suggestion);
        }
    }
    
    Ok(())
}

fn export_dashboard(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let filename = args.get(2).map(|s| s.as_str()).unwrap_or("progress.json");
    let output_file = args.get(3).map(|s| s.as_str()).unwrap_or("dashboard.html");
    
    if !Path::new(filename).exists() {
        eprintln!("❌ 找不到进度文件: {}", filename);
        return Ok(());
    }

    println!("📊 加载进度文件: {}", filename);
    let tracker = ProgressTracker::from_file(filename)?;
    
    println!("🎨 生成 HTML 仪表板...");
    let html_content = generate_html_dashboard(&tracker);
    
    fs::write(output_file, html_content)?;
    
    println!("✅ HTML 仪表板已导出: {}", output_file);
    println!("💡 请在浏览器中打开文件查看可视化仪表板");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use progress_tracker::LearningUnitStatus;
    use tempfile::TempDir;

    #[test]
    fn test_init_and_show() {
        let temp_dir = TempDir::new().unwrap();
        let progress_file = temp_dir.path().join("test-progress.json");
        
        // 初始化进度跟踪器
        let tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
        tracker.to_file(&progress_file).unwrap();
        
        // 加载并显示
        let loaded_tracker = ProgressTracker::from_file(&progress_file).unwrap();
        assert_eq!(loaded_tracker.learner_name, "测试学习者");
        assert_eq!(loaded_tracker.learning_units.len(), 3); // 示例单元
    }

    #[test]
    fn test_update_progress() {
        let mut tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
        
        // 开始一个单元
        if let Some(unit) = tracker.get_unit_mut("stage1-environment") {
            unit.start();
            assert_eq!(unit.status, LearningUnitStatus::InProgress);
            
            unit.complete(Some(85.0));
            assert_eq!(unit.status, LearningUnitStatus::Completed);
            assert_eq!(unit.score, Some(85.0));
        }
        
        // 检查进度统计
        let stats = tracker.get_progress_stats();
        assert_eq!(stats.completed_units, 1);
        assert!(stats.overall_progress > 0.0);
    }
}