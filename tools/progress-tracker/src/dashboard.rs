//! 学习进度可视化仪表板模块

use crate::{ProgressTracker, LearningUnitStatus, LearningStage};
use serde::{Deserialize, Serialize};

/// 仪表板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub show_progress_bars: bool,
    pub show_stage_breakdown: bool,
    pub show_achievements: bool,
    pub show_recommendations: bool,
    pub show_suggestions: bool,
    pub max_recommendations: usize,
    pub theme: DashboardTheme,
}

/// 仪表板主题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTheme {
    pub primary_color: String,
    pub success_color: String,
    pub warning_color: String,
    pub danger_color: String,
    pub info_color: String,
    pub text_color: String,
    pub background_color: String,
}

/// 仪表板渲染器
pub struct DashboardRenderer {
    config: DashboardConfig,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            show_progress_bars: true,
            show_stage_breakdown: true,
            show_achievements: true,
            show_recommendations: true,
            show_suggestions: true,
            max_recommendations: 5,
            theme: DashboardTheme::default(),
        }
    }
}

impl Default for DashboardTheme {
    fn default() -> Self {
        Self {
            primary_color: "#007bff".to_string(),
            success_color: "#28a745".to_string(),
            warning_color: "#ffc107".to_string(),
            danger_color: "#dc3545".to_string(),
            info_color: "#17a2b8".to_string(),
            text_color: "#333333".to_string(),
            background_color: "#ffffff".to_string(),
        }
    }
}

impl DashboardRenderer {
    pub fn new(config: DashboardConfig) -> Self {
        Self { config }
    }

    /// 渲染整个仪表板
    pub fn render(&self, tracker: &ProgressTracker) -> String {
        let mut output = String::new();
        
        // 头部
        output.push_str(&self.render_header(tracker));
        
        // 总体进度
        if self.config.show_progress_bars {
            output.push_str(&self.render_overall_progress(tracker));
        }
        
        // 阶段详细进度
        if self.config.show_stage_breakdown {
            output.push_str(&self.render_stage_breakdown(tracker));
        }
        
        // 成就展示
        if self.config.show_achievements {
            output.push_str(&self.render_achievements(tracker));
        }
        
        // 学习推荐
        if self.config.show_recommendations {
            output.push_str(&self.render_recommendations(tracker));
        }
        
        // 个性化建议
        if self.config.show_suggestions {
            output.push_str(&self.render_suggestions(tracker));
        }
        
        // 页脚
        output.push_str(&self.render_footer(tracker));
        
        output
    }

    /// 渲染头部
    fn render_header(&self, tracker: &ProgressTracker) -> String {
        format!(
            r#"
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                           🦀 Rust 学习进度跟踪系统                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

👋 学习者: {}
📅 最后更新: {}
"#,
            tracker.learner_name,
            self.format_datetime(&tracker.last_updated)
        )
    }

    /// 渲染总体进度
    fn render_overall_progress(&self, tracker: &ProgressTracker) -> String {
        let stats = tracker.get_progress_stats();
        let progress_bar = self.create_progress_bar(stats.overall_progress, 40);
        
        format!(
            r#"
📊 总体学习进度
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
{}

✅ 已完成: {} 个单元    🔄 进行中: {} 个单元    📋 总计: {} 个单元
🎯 平均分数: {}    ⏱️  总学习时间: {} 分钟
"#,
            progress_bar,
            stats.completed_units,
            stats.in_progress_units,
            stats.total_units,
            stats.average_score.map(|s| format!("{:.1}", s)).unwrap_or_else(|| "无".to_string()),
            stats.completed_time_minutes
        )
    }

    /// 渲染阶段详细进度
    fn render_stage_breakdown(&self, tracker: &ProgressTracker) -> String {
        let stats = tracker.get_progress_stats();
        let mut output = String::new();
        
        output.push_str("\n📋 各阶段学习进度\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        for stage in LearningStage::all_stages() {
            let stage_name = stage.name();
            let stage_key = format!("{:?}", stage);
            let progress = stats.stage_progress.get(&stage_key).copied().unwrap_or(0.0);
            
            let progress_bar = self.create_progress_bar(progress, 30);
            
            output.push_str(&format!(
                "{}\n{}",
                stage_name,
                progress_bar
            ));
            
            // 显示该阶段的单元状态
            let stage_units: Vec<_> = tracker.learning_units.iter()
                .filter(|u| u.stage == stage)
                .collect();
            
            for unit in stage_units {
                let status_icon = match unit.status {
                    LearningUnitStatus::NotStarted => "📋",
                    LearningUnitStatus::InProgress => "🔄",
                    LearningUnitStatus::Completed => "✅",
                    LearningUnitStatus::Skipped => "⏭️",
                };
                
                let score_text = unit.score.map(|s| format!(" [{:.0}]", s)).unwrap_or_else(|| "".to_string());
                
                output.push_str(&format!("  {} {}{}\n", status_icon, unit.name, score_text));
            }
            
            output.push('\n');
        }
        
        output
    }

    /// 渲染成就展示
    fn render_achievements(&self, tracker: &ProgressTracker) -> String {
        let unlocked_achievements: Vec<_> = tracker.achievements.iter()
            .filter(|a| a.unlocked_at.is_some())
            .collect();
        
        let locked_achievements: Vec<_> = tracker.achievements.iter()
            .filter(|a| a.unlocked_at.is_none())
            .collect();
        
        let mut output = String::new();
        
        output.push_str("\n🏆 成就系统\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        if !unlocked_achievements.is_empty() {
            output.push_str("\n✨ 已解锁成就:\n");
            for achievement in unlocked_achievements {
                let rarity_icon = match achievement.rarity {
                    crate::AchievementRarity::Common => "🌟",
                    crate::AchievementRarity::Rare => "⭐",
                    crate::AchievementRarity::Epic => "🌟",
                    crate::AchievementRarity::Legendary => "💫",
                };
                
                output.push_str(&format!(
                    "  {} {} - {}\n",
                    rarity_icon,
                    achievement.name,
                    achievement.description
                ));
            }
        }
        
        if !locked_achievements.is_empty() {
            output.push_str(&format!("\n🔒 未解锁成就: {} 个\n", locked_achievements.len()));
        }
        
        output
    }

    /// 渲染学习推荐
    fn render_recommendations(&self, tracker: &ProgressTracker) -> String {
        let recommendation = tracker.get_learning_path_recommendation();
        let mut output = String::new();
        
        output.push_str("\n🎯 学习路径推荐\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        if recommendation.next_units.is_empty() {
            output.push_str("\n🎉 恭喜！您已完成所有学习单元。\n");
            output.push_str("💡 建议开始实际项目练习或复习之前的内容。\n");
        } else {
            output.push_str(&format!("\n💡 {}\n", recommendation.reasoning));
            output.push_str(&format!("📅 预计学习时间: {} 分钟\n", recommendation.estimated_time_minutes));
            output.push_str(&format!("🎯 推荐置信度: {:.0}%\n\n", recommendation.confidence_score * 100.0));
            
            output.push_str("📚 推荐学习单元:\n");
            for (i, unit) in recommendation.next_units.iter().enumerate() {
                output.push_str(&format!(
                    "  {}. {} ({} - {} 分钟)\n",
                    i + 1,
                    unit.name,
                    unit.unit_type.name(),
                    unit.estimated_time_minutes
                ));
            }
        }
        
        output
    }

    /// 渲染个性化建议
    fn render_suggestions(&self, tracker: &ProgressTracker) -> String {
        let suggestions = tracker.get_personalized_suggestions();
        let mut output = String::new();
        
        output.push_str("\n💡 个性化学习建议\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        for (i, suggestion) in suggestions.iter().enumerate() {
            output.push_str(&format!("{} {}\n", i + 1, suggestion));
        }
        
        output
    }

    /// 渲染页脚
    fn render_footer(&self, _tracker: &ProgressTracker) -> String {
        r#"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🦀 Rust 学习进度跟踪系统 - 让学习更高效，让进步看得见！
"#.to_string()
    }

    /// 创建进度条
    fn create_progress_bar(&self, percentage: f32, width: usize) -> String {
        let filled_width = ((percentage / 100.0) * width as f32) as usize;
        let empty_width = width - filled_width;
        
        let filled = "█".repeat(filled_width);
        let empty = "░".repeat(empty_width);
        
        format!("[{}{}] {:.1}%", filled, empty, percentage)
    }

    /// 格式化日期时间
    fn format_datetime(&self, dt: &chrono::DateTime<chrono::Utc>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

/// 生成 HTML 格式的仪表板 - 优化版本，使用预分配容量
pub fn generate_html_dashboard(tracker: &ProgressTracker) -> String {
    let stats = tracker.get_progress_stats();
    let recommendation = tracker.get_learning_path_recommendation();
    let suggestions = tracker.get_personalized_suggestions();
    
    let unlocked_achievements: Vec<_> = tracker.achievements.iter()
        .filter(|a| a.unlocked_at.is_some())
        .collect();
    
    // 预分配HTML字符串容量，避免多次重新分配
    // 基于典型HTML大小估算：约15KB基础 + 每个成就500字节 + 每个建议200字节
    let estimated_capacity = 15_000 + 
        unlocked_achievements.len() * 500 + 
        suggestions.len() * 200 + 
        recommendation.next_units.len() * 300;
    
    let mut html = String::with_capacity(estimated_capacity);
    
    // 使用push_str和write!宏替代format!，减少运行时分配
    html.push_str("<!DOCTYPE html>\n<html lang=\"zh-CN\">\n<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <title>Rust 学习进度跟踪系统</title>\n");
    html.push_str("    <style>\n");
    
    // CSS样式 - 使用常量字符串避免重复分配
    const CSS_STYLES: &str = r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        
        .dashboard {
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        
        .learner-info {
            font-size: 1.2em;
            opacity: 0.9;
        }
        
        .content {
            padding: 30px;
        }
        
        .section {
            margin-bottom: 40px;
            padding: 25px;
            background: #f8f9fa;
            border-radius: 10px;
            border-left: 5px solid #667eea;
        }
        
        .section h2 {
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.8em;
        }
        
        .progress-container {
            margin: 20px 0;
        }
        
        .progress-bar {
            background: #e9ecef;
            border-radius: 10px;
            overflow: hidden;
            height: 30px;
            position: relative;
        }
        
        .progress-fill {
            background: linear-gradient(90deg, #28a745, #20c997);
            height: 100%;
            border-radius: 10px;
            transition: width 0.3s ease;
            position: relative;
        }
        
        .progress-text {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: white;
            font-weight: bold;
            font-size: 1.1em;
            text-shadow: 1px 1px 2px rgba(0,0,0,0.5);
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }
        
        .stat-card {
            background: white;
            padding: 20px;
            border-radius: 10px;
            text-align: center;
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
            border-top: 3px solid #667eea;
        }
        
        .stat-number {
            font-size: 2em;
            font-weight: bold;
            color: #667eea;
            margin-bottom: 5px;
        }
        
        .stat-label {
            color: #666;
            font-size: 0.9em;
        }
        
        .achievement-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
            margin: 20px 0;
        }
        
        .achievement-card {
            background: white;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
            border-left: 4px solid #ffc107;
            transition: transform 0.2s ease;
        }
        
        .achievement-card:hover {
            transform: translateY(-2px);
        }
        
        .achievement-title {
            font-weight: bold;
            color: #333;
            margin-bottom: 5px;
        }
        
        .achievement-desc {
            color: #666;
            font-size: 0.9em;
        }
        
        .recommendation-list {
            list-style: none;
            margin: 20px 0;
        }
        
        .recommendation-item {
            background: white;
            margin: 10px 0;
            padding: 15px;
            border-radius: 8px;
            border-left: 4px solid #28a745;
            box-shadow: 0 3px 10px rgba(0,0,0,0.1);
        }
        
        .suggestion-list {
            list-style: none;
            margin: 20px 0;
        }
        
        .suggestion-item {
            background: white;
            margin: 10px 0;
            padding: 15px;
            border-radius: 8px;
            border-left: 4px solid #17a2b8;
            box-shadow: 0 3px 10px rgba(0,0,0,0.1);
        }
        
        .footer {
            background: #343a40;
            color: white;
            text-align: center;
            padding: 20px;
            font-size: 0.9em;
        }
        
        @media (max-width: 768px) {
            .container {
                padding: 10px;
            }
            
            .header h1 {
                font-size: 2em;
            }
            
            .stats-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
    "#;
    
    html.push_str(CSS_STYLES);
    html.push_str("    </style>\n</head>\n<body>\n");
    
    // 添加主要HTML结构 - 使用format!宏进行字符串插值
    html.push_str(&format!(r#"    <div class="container">
        <div class="dashboard">
            <div class="header">
                <h1>🦀 Rust 学习进度跟踪系统</h1>
                <div class="learner-info">
                    👋 学习者: {} | 📅 最后更新: {}
                </div>
            </div>
            
            <div class="content">
                <div class="section">
                    <h2>📊 总体学习进度</h2>
                    <div class="progress-container">
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: {:.1}%">
                                <div class="progress-text">{:.1}%</div>
                            </div>
                        </div>
                    </div>
                    <div class="stats-grid">
                        <div class="stat-card">
                            <div class="stat-number">{}</div>
                            <div class="stat-label">已完成单元</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-number">{}</div>
                            <div class="stat-label">进行中单元</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-number">{}</div>
                            <div class="stat-label">总单元数</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-number">{}</div>
                            <div class="stat-label">总学习时间 (分钟)</div>
                        </div>
                    </div>
                </div>
                
                <div class="section">
                    <h2>🏆 已解锁成就</h2>
                    <div class="achievement-grid">
"#,
        tracker.learner_name,
        tracker.last_updated.format("%Y-%m-%d %H:%M:%S UTC"),
        stats.overall_progress,
        stats.overall_progress,
        stats.completed_units,
        stats.in_progress_units,
        stats.total_units,
        stats.completed_time_minutes
    ));
    
    // 添加成就卡片 - 使用预分配的字符串构建
    let achievement_capacity = unlocked_achievements.len() * 200; // 每个成就约200字符
    let mut achievement_html = String::with_capacity(achievement_capacity);
    
    for achievement in &unlocked_achievements {
        achievement_html.push_str("                        <div class=\"achievement-card\">\n");
        achievement_html.push_str("                            <div class=\"achievement-title\">");
        achievement_html.push_str(&achievement.name);
        achievement_html.push_str("</div>\n");
        achievement_html.push_str("                            <div class=\"achievement-desc\">");
        achievement_html.push_str(&achievement.description);
        achievement_html.push_str("</div>\n");
        achievement_html.push_str("                        </div>\n");
    }
    
    if unlocked_achievements.is_empty() {
        achievement_html.push_str("                        <p style='text-align: center; color: #666;'>暂无已解锁成就</p>\n");
    }
    
    html.push_str(&achievement_html);
    html.push_str("                    </div>\n                </div>\n");
    
    // 添加学习推荐 - 使用预分配的字符串构建
    let recommendation_capacity = 500 + recommendation.next_units.len() * 150; // 基础500 + 每个推荐150字符
    let mut recommendation_html = String::with_capacity(recommendation_capacity);
    
    recommendation_html.push_str("                <div class=\"section\">\n");
    recommendation_html.push_str("                    <h2>🎯 学习路径推荐</h2>\n");
    recommendation_html.push_str("                    <p><strong>推荐阶段:</strong> ");
    recommendation_html.push_str(recommendation.recommended_stage.name());
    recommendation_html.push_str("</p>\n");
    recommendation_html.push_str("                    <p><strong>预计学习时间:</strong> ");
    recommendation_html.push_str(&recommendation.estimated_time_minutes.to_string());
    recommendation_html.push_str(" 分钟</p>\n");
    recommendation_html.push_str("                    <p><strong>推荐置信度:</strong> ");
    recommendation_html.push_str(&(recommendation.confidence_score * 100.0).to_string());
    recommendation_html.push_str("%</p>\n");
    recommendation_html.push_str("                    <p><strong>推荐理由:</strong> ");
    recommendation_html.push_str(&recommendation.reasoning);
    recommendation_html.push_str("</p>\n");
    recommendation_html.push_str("                    <ul class=\"recommendation-list\">\n");
    
    for (i, unit) in recommendation.next_units.iter().enumerate() {
        recommendation_html.push_str("                        <li class=\"recommendation-item\">\n");
        recommendation_html.push_str("                            <strong>");
        recommendation_html.push_str(&(i + 1).to_string());
        recommendation_html.push_str(".</strong> ");
        recommendation_html.push_str(&unit.name);
        recommendation_html.push_str(" (");
        recommendation_html.push_str(unit.unit_type.name());
        recommendation_html.push_str(" - ");
        recommendation_html.push_str(&unit.estimated_time_minutes.to_string());
        recommendation_html.push_str(" 分钟)\n");
        recommendation_html.push_str("                        </li>\n");
    }
    
    if recommendation.next_units.is_empty() {
        recommendation_html.push_str("                        <p style='text-align: center; color: #666;'>暂无推荐学习单元</p>\n");
    }
    
    recommendation_html.push_str("                    </ul>\n");
    recommendation_html.push_str("                </div>\n");
    
    html.push_str(&recommendation_html);
    
    // 添加个性化建议
    html.push_str("                <div class=\"section\">\n                    <h2>💡 个性化学习建议</h2>\n                    <ul class=\"suggestion-list\">\n");
    
    for suggestion in &suggestions {
        html.push_str(&format!(
            "                        <li class=\"suggestion-item\">{}</li>\n",
            suggestion
        ));
    }
    
    if suggestions.is_empty() {
        html.push_str("                        <p style='text-align: center; color: #666;'>暂无个性化建议</p>\n");
    }
    
    html.push_str("                    </ul>\n                </div>\n            </div>\n            \n            <div class=\"footer\">\n                🦀 Rust 学习进度跟踪系统 - 让学习更高效，让进步看得见！\n            </div>\n        </div>\n    </div>\n</body>\n</html>");
    
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProgressTracker, LearningUnit, LearningUnitType, LearningStage, LearningUnitStatus};
    
    #[test]
    fn test_dashboard_renderer() {
        let tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
        let config = DashboardConfig::default();
        let renderer = DashboardRenderer::new(config);
        
        let dashboard = renderer.render(&tracker);
        
        assert!(dashboard.contains("Rust 学习进度跟踪系统"));
        assert!(dashboard.contains("测试学习者"));
        assert!(dashboard.contains("总体学习进度"));
        assert!(dashboard.contains("✨ 已解锁成就") || dashboard.contains("🏆 成就系统"));
        assert!(dashboard.contains("学习路径推荐"));
        assert!(dashboard.contains("个性化学习建议"));
    }
    
    #[test]
    fn test_html_dashboard_generation() {
        let tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
        
        let html = generate_html_dashboard(&tracker);
        
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Rust 学习进度跟踪系统"));
        assert!(html.contains("测试学习者"));
        assert!(html.contains("progress-fill"));
        assert!(html.contains("achievement-grid"));
        assert!(html.contains("recommendation-list"));
    }
}