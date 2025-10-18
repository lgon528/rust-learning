//! Rust 学习进度跟踪系统
//! 
//! 提供学习进度跟踪、可视化、个性化推荐和成就系统功能。

pub mod dashboard;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};

/// 学习阶段定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LearningStage {
    Stage1Basics,
    Stage2Ownership,
    Stage3AdvancedConcepts,
    Stage4Ecosystem,
    Stage5Projects,
}

impl LearningStage {
    /// 获取所有学习阶段
    pub fn all_stages() -> Vec<LearningStage> {
        vec![
            LearningStage::Stage1Basics,
            LearningStage::Stage2Ownership,
            LearningStage::Stage3AdvancedConcepts,
            LearningStage::Stage4Ecosystem,
            LearningStage::Stage5Projects,
        ]
    }

    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            LearningStage::Stage1Basics => "阶段1: 基础入门",
            LearningStage::Stage2Ownership => "阶段2: 所有权系统",
            LearningStage::Stage3AdvancedConcepts => "阶段3: 高级概念",
            LearningStage::Stage4Ecosystem => "阶段4: 生态系统",
            LearningStage::Stage5Projects => "阶段5: 项目实战",
        }
    }

    /// 获取阶段描述
    pub fn description(&self) -> &'static str {
        match self {
            LearningStage::Stage1Basics => "Rust 基础语法、环境搭建、基本数据类型和控制流",
            LearningStage::Stage2Ownership => "Rust 的核心特性：所有权、借用、生命周期",
            LearningStage::Stage3AdvancedConcepts => "结构体、枚举、错误处理、泛型和特征",
            LearningStage::Stage4Ecosystem => "Cargo、常用库、异步编程、Web 框架",
            LearningStage::Stage5Projects => "实际项目开发：Web 应用、系统编程、区块链",
        }
    }

    /// 获取预计学习时间（周）
    pub fn estimated_weeks(&self) -> u32 {
        match self {
            LearningStage::Stage1Basics => 3,
            LearningStage::Stage2Ownership => 2,
            LearningStage::Stage3AdvancedConcepts => 2,
            LearningStage::Stage4Ecosystem => 2,
            LearningStage::Stage5Projects => 3,
        }
    }
}

/// 学习单元类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LearningUnitType {
    ContentReading,      // 内容阅读
    CodeExample,         // 代码示例
    Exercise,            // 练习题
    Project,             // 项目实战
    Assessment,          // 自我评估
}

impl LearningUnitType {
    /// 获取单元类型名称
    pub fn name(&self) -> &'static str {
        match self {
            LearningUnitType::ContentReading => "内容阅读",
            LearningUnitType::CodeExample => "代码示例",
            LearningUnitType::Exercise => "练习题",
            LearningUnitType::Project => "项目实战",
            LearningUnitType::Assessment => "自我评估",
        }
    }

    /// 获取单元类型权重（用于进度计算）
    pub fn weight(&self) -> f32 {
        match self {
            LearningUnitType::ContentReading => 0.15,
            LearningUnitType::CodeExample => 0.25,
            LearningUnitType::Exercise => 0.30,
            LearningUnitType::Project => 0.20,
            LearningUnitType::Assessment => 0.10,
        }
    }
}

/// 学习单元状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LearningUnitStatus {
    NotStarted,     // 未开始
    InProgress,     // 进行中
    Completed,      // 已完成
    Skipped,        // 已跳过
}

impl LearningUnitStatus {
    /// 获取状态名称
    pub fn name(&self) -> &'static str {
        match self {
            LearningUnitStatus::NotStarted => "未开始",
            LearningUnitStatus::InProgress => "进行中",
            LearningUnitStatus::Completed => "已完成",
            LearningUnitStatus::Skipped => "已跳过",
        }
    }

    /// 是否已完成
    pub fn is_completed(&self) -> bool {
        matches!(self, LearningUnitStatus::Completed)
    }
}

/// 学习单元定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningUnit {
    pub id: String,
    pub name: String,
    pub unit_type: LearningUnitType,
    pub stage: LearningStage,
    pub path: String,           // 文件路径或URL
    pub estimated_time_minutes: u32,
    pub status: LearningUnitStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub score: Option<f32>,     // 0.0 - 100.0
    pub notes: Option<String>,
}

impl LearningUnit {
    /// 创建新的学习单元
    pub fn new(id: String, name: String, unit_type: LearningUnitType, stage: LearningStage, path: String, estimated_time_minutes: u32) -> Self {
        Self {
            id,
            name,
            unit_type,
            stage,
            path,
            estimated_time_minutes,
            status: LearningUnitStatus::NotStarted,
            started_at: None,
            completed_at: None,
            score: None,
            notes: None,
        }
    }

    /// 开始学习单元
    pub fn start(&mut self) {
        self.status = LearningUnitStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    /// 完成学习单元
    pub fn complete(&mut self, score: Option<f32>) {
        self.status = LearningUnitStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.score = score;
    }

    /// 跳过学习单元
    pub fn skip(&mut self) {
        self.status = LearningUnitStatus::Skipped;
    }

    /// 获取实际学习时间（分钟）
    pub fn actual_time_minutes(&self) -> Option<u32> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => {
                let duration = end - start;
                Some(duration.num_minutes() as u32)
            }
            _ => None,
        }
    }
}

/// 学习进度统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressStats {
    pub total_units: usize,
    pub completed_units: usize,
    pub in_progress_units: usize,
    pub skipped_units: usize,
    pub overall_progress: f32,  // 0.0 - 100.0
    pub total_time_minutes: u32,
    pub completed_time_minutes: u32,
    pub average_score: Option<f32>,
    pub current_stage: LearningStage,
    pub stage_progress: HashMap<String, f32>,
}

/// 学习路径推荐
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathRecommendation {
    pub next_units: Vec<LearningUnit>,
    pub recommended_stage: LearningStage,
    pub estimated_time_minutes: u32,
    pub confidence_score: f32,  // 0.0 - 1.0
    pub reasoning: String,
}

/// 成就定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub condition: AchievementCondition,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub rarity: AchievementRarity,
}

/// 成就条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCondition {
    CompleteUnits { count: usize, unit_type: Option<LearningUnitType> },
    CompleteStage { stage: LearningStage },
    ScoreAverage { min_score: f32, unit_count: usize },
    StreakDays { days: u32 },
    TotalTime { hours: u32 },
}

/// 成就稀有度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AchievementRarity {
    Common,     // 普通
    Rare,       // 稀有
    Epic,       // 史诗
    Legendary,  // 传说
}

impl AchievementRarity {
    /// 获取稀有度名称
    pub fn name(&self) -> &'static str {
        match self {
            AchievementRarity::Common => "普通",
            AchievementRarity::Rare => "稀有",
            AchievementRarity::Epic => "史诗",
            AchievementRarity::Legendary => "传说",
        }
    }

    /// 获取稀有度颜色
    pub fn color(&self) -> &'static str {
        match self {
            AchievementRarity::Common => "#9CA3AF",
            AchievementRarity::Rare => "#3B82F6",
            AchievementRarity::Epic => "#8B5CF6",
            AchievementRarity::Legendary => "#F59E0B",
        }
    }
}

/// 学习进度跟踪器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressTracker {
    pub learner_id: String,
    pub learner_name: String,
    pub learning_units: Vec<LearningUnit>,
    pub achievements: Vec<Achievement>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl ProgressTracker {
    /// 创建新的进度跟踪器
    pub fn new(learner_id: String, learner_name: String) -> Self {
        let mut tracker = Self {
            learner_id,
            learner_name,
            learning_units: Vec::new(),
            achievements: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        // 初始化默认学习单元
        tracker.initialize_default_units();
        tracker.initialize_default_achievements();
        
        tracker
    }

    /// 从文件加载进度跟踪器
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let tracker: Self = serde_json::from_str(&content)?;
        Ok(tracker)
    }

    /// 保存到文件
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// 初始化默认学习单元
    fn initialize_default_units(&mut self) {
        // 这里应该根据实际项目结构初始化学习单元
        // 为了演示，我们先创建一些示例单元
        let units = vec![
            LearningUnit::new(
                "stage1-environment".to_string(),
                "环境搭建与基础配置".to_string(),
                LearningUnitType::ContentReading,
                LearningStage::Stage1Basics,
                "content/stage1-basics/01-environment".to_string(),
                60,
            ),
            LearningUnit::new(
                "stage1-syntax".to_string(),
                "基本语法与数据类型".to_string(),
                LearningUnitType::ContentReading,
                LearningStage::Stage1Basics,
                "content/stage1-basics/02-syntax".to_string(),
                120,
            ),
            LearningUnit::new(
                "stage1-syntax-demo".to_string(),
                "语法演示代码".to_string(),
                LearningUnitType::CodeExample,
                LearningStage::Stage1Basics,
                "examples/stage1-basics/02-syntax-demo".to_string(),
                45,
            ),
        ];

        self.learning_units.extend(units);
    }

    /// 初始化默认成就
    fn initialize_default_achievements(&mut self) {
        let achievements = vec![
            Achievement {
                id: "first_steps".to_string(),
                name: "初次尝试".to_string(),
                description: "完成第一个学习单元".to_string(),
                icon: "🎯".to_string(),
                condition: AchievementCondition::CompleteUnits { count: 1, unit_type: None },
                unlocked_at: None,
                rarity: AchievementRarity::Common,
            },
            Achievement {
                id: "stage1_master".to_string(),
                name: "基础大师".to_string(),
                description: "完成阶段1所有内容".to_string(),
                icon: "🌟".to_string(),
                condition: AchievementCondition::CompleteStage { stage: LearningStage::Stage1Basics },
                unlocked_at: None,
                rarity: AchievementRarity::Rare,
            },
            Achievement {
                id: "code_warrior".to_string(),
                name: "代码战士".to_string(),
                description: "完成10个代码示例".to_string(),
                icon: "⚔️".to_string(),
                condition: AchievementCondition::CompleteUnits { count: 10, unit_type: Some(LearningUnitType::CodeExample) },
                unlocked_at: None,
                rarity: AchievementRarity::Epic,
            },
            Achievement {
                id: "perfect_student".to_string(),
                name: "完美学生".to_string(),
                description: "连续5个练习得分90分以上".to_string(),
                icon: "🏆".to_string(),
                condition: AchievementCondition::ScoreAverage { min_score: 90.0, unit_count: 5 },
                unlocked_at: None,
                rarity: AchievementRarity::Legendary,
            },
        ];

        self.achievements.extend(achievements);
    }

    /// 获取学习单元
    pub fn get_unit(&self, unit_id: &str) -> Option<&LearningUnit> {
        self.learning_units.iter().find(|u| u.id == unit_id)
    }

    /// 获取可变的学习单元
    pub fn get_unit_mut(&mut self, unit_id: &str) -> Option<&mut LearningUnit> {
        self.learning_units.iter_mut().find(|u| u.id == unit_id)
    }

    /// 添加学习单元
    pub fn add_unit(&mut self, unit: LearningUnit) {
        self.learning_units.push(unit);
        self.last_updated = Utc::now();
    }

    /// 获取进度统计
    pub fn get_progress_stats(&self) -> ProgressStats {
        let total_units = self.learning_units.len();
        let completed_units = self.learning_units.iter()
            .filter(|u| u.status.is_completed())
            .count();
        let in_progress_units = self.learning_units.iter()
            .filter(|u| u.status == LearningUnitStatus::InProgress)
            .count();
        let skipped_units = self.learning_units.iter()
            .filter(|u| u.status == LearningUnitStatus::Skipped)
            .count();

        // 计算总体进度（加权）
        let total_weight: f32 = self.learning_units.iter()
            .map(|u| u.unit_type.weight())
            .sum();
        let completed_weight: f32 = self.learning_units.iter()
            .filter(|u| u.status.is_completed())
            .map(|u| u.unit_type.weight())
            .sum();
        
        let overall_progress = if total_weight > 0.0 {
            (completed_weight / total_weight) * 100.0
        } else {
            0.0
        };

        // 计算时间统计
        let total_time_minutes: u32 = self.learning_units.iter()
            .map(|u| u.estimated_time_minutes)
            .sum();
        let completed_time_minutes: u32 = self.learning_units.iter()
            .filter(|u| u.status.is_completed())
            .map(|u| u.estimated_time_minutes)
            .sum();

        // 计算平均分
        let completed_with_scores: Vec<&LearningUnit> = self.learning_units.iter()
            .filter(|u| u.status.is_completed() && u.score.is_some())
            .collect();
        
        let average_score = if !completed_with_scores.is_empty() {
            let total_score: f32 = completed_with_scores.iter()
                .map(|u| u.score.unwrap())
                .sum();
            Some(total_score / completed_with_scores.len() as f32)
        } else {
            None
        };

        // 计算各阶段进度
        let mut stage_progress = HashMap::new();
        for stage in LearningStage::all_stages() {
            let stage_units: Vec<&LearningUnit> = self.learning_units.iter()
                .filter(|u| u.stage == stage)
                .collect();
            
            if !stage_units.is_empty() {
                let completed_stage_units = stage_units.iter()
                    .filter(|u| u.status.is_completed())
                    .count();
                let progress = (completed_stage_units as f32 / stage_units.len() as f32) * 100.0;
                stage_progress.insert(format!("{:?}", stage), progress);
            }
        }

        // 确定当前阶段（第一个未完成的阶段）
        let current_stage = LearningStage::all_stages().into_iter()
            .find(|stage| {
                let stage_units: Vec<&LearningUnit> = self.learning_units.iter()
                    .filter(|u| u.stage == *stage)
                    .collect();
                
                if stage_units.is_empty() {
                    return false;
                }
                
                let completed_count = stage_units.iter()
                    .filter(|u| u.status.is_completed())
                    .count();
                
                completed_count < stage_units.len()
            })
            .unwrap_or(LearningStage::Stage5Projects);

        ProgressStats {
            total_units,
            completed_units,
            in_progress_units,
            skipped_units,
            overall_progress,
            total_time_minutes,
            completed_time_minutes,
            average_score,
            current_stage,
            stage_progress,
        }
    }

    /// 获取学习路径推荐
    pub fn get_learning_path_recommendation(&self) -> LearningPathRecommendation {
        let stats = self.get_progress_stats();
        let mut next_units = Vec::new();
        let mut estimated_time_minutes = 0;

        // 查找当前阶段的未完成单元
        let current_stage_units: Vec<&LearningUnit> = self.learning_units.iter()
            .filter(|u| u.stage == stats.current_stage)
            .filter(|u| !u.status.is_completed())
            .collect();

        // 推荐优先级：未开始的 > 进行中的，按类型权重排序
        let mut candidates: Vec<&LearningUnit> = current_stage_units.into_iter()
            .filter(|u| u.status != LearningUnitStatus::Skipped)
            .collect();

        candidates.sort_by(|a, b| {
            // 优先未开始的单元
            let status_cmp = match (&a.status, &b.status) {
                (LearningUnitStatus::NotStarted, LearningUnitStatus::InProgress) => std::cmp::Ordering::Less,
                (LearningUnitStatus::InProgress, LearningUnitStatus::NotStarted) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            };

            if status_cmp != std::cmp::Ordering::Equal {
                return status_cmp;
            }

            // 按类型权重排序（高权重优先）
            b.unit_type.weight().partial_cmp(&a.unit_type.weight()).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 选择前3-5个推荐单元
        for unit in candidates.into_iter().take(5) {
            next_units.push(unit.clone());
            estimated_time_minutes += unit.estimated_time_minutes;
        }

        // 计算置信度分数
        let confidence_score = if !next_units.is_empty() {
            let completed_ratio = stats.completed_units as f32 / stats.total_units as f32;
            let stage_progress = stats.stage_progress.get(&format!("{:?}", stats.current_stage))
                .copied()
                .unwrap_or(0.0) / 100.0;
            
            (completed_ratio + stage_progress) / 2.0
        } else {
            0.0
        };

        let reasoning = if next_units.is_empty() {
            "恭喜！您已完成所有学习单元。建议复习或开始实际项目练习。".to_string()
        } else {
            format!("基于您的学习进度，推荐您接下来完成 {} 的 {} 个学习单元，预计需要 {} 分钟。",
                stats.current_stage.name(),
                next_units.len(),
                estimated_time_minutes
            )
        };

        LearningPathRecommendation {
            next_units,
            recommended_stage: stats.current_stage.clone(),
            estimated_time_minutes,
            confidence_score,
            reasoning,
        }
    }

    /// 检查并解锁成就
    pub fn check_achievements(&mut self) -> Vec<String> {
        let mut newly_unlocked = Vec::new();
        let stats = self.get_progress_stats();

        for achievement in &mut self.achievements {
            if achievement.unlocked_at.is_some() {
                continue; // 已解锁
            }

            let should_unlock = match &achievement.condition {
                AchievementCondition::CompleteUnits { count, unit_type } => {
                    let completed_units = self.learning_units.iter()
                        .filter(|u| u.status.is_completed())
                        .filter(|u| {
                            if let Some(ut) = unit_type {
                                u.unit_type == *ut
                            } else {
                                true
                            }
                        })
                        .count();
                    
                    completed_units >= *count
                },
                AchievementCondition::CompleteStage { stage } => {
                    let stage_progress = stats.stage_progress.get(&format!("{:?}", stage))
                        .copied()
                        .unwrap_or(0.0);
                    stage_progress >= 100.0
                },
                AchievementCondition::ScoreAverage { min_score, unit_count } => {
                    if let Some(avg_score) = stats.average_score {
                        let completed_with_scores = self.learning_units.iter()
                            .filter(|u| u.status.is_completed() && u.score.is_some())
                            .count();
                        
                        avg_score >= *min_score && completed_with_scores >= *unit_count
                    } else {
                        false
                    }
                },
                AchievementCondition::StreakDays { days } => {
                    // 简化实现：检查是否有连续的学习记录
                    // 实际实现中需要更复杂的逻辑
                    let completed_recently = self.learning_units.iter()
                        .filter(|u| u.status.is_completed())
                        .filter(|u| {
                            if let Some(completed_at) = u.completed_at {
                                let duration = Utc::now() - completed_at;
                                duration.num_days() <= *days as i64
                            } else {
                                false
                            }
                        })
                        .count();
                    
                    completed_recently >= 3 // 简化条件
                },
                AchievementCondition::TotalTime { hours } => {
                    let total_hours = stats.completed_time_minutes / 60;
                    total_hours >= *hours
                },
            };

            if should_unlock {
                achievement.unlocked_at = Some(Utc::now());
                newly_unlocked.push(achievement.id.clone());
            }
        }

        if !newly_unlocked.is_empty() {
            self.last_updated = Utc::now();
        }

        newly_unlocked
    }

    /// 获取个性化学习建议
    pub fn get_personalized_suggestions(&self) -> Vec<String> {
        let stats = self.get_progress_stats();
        let mut suggestions = Vec::new();

        // 基于进度给出建议
        if stats.overall_progress < 20.0 {
            suggestions.push("🎯 刚开始学习 Rust，建议从基础语法开始，每天保持 30-60 分钟的学习时间。".to_string());
        } else if stats.overall_progress < 50.0 {
            suggestions.push("📈 学习进展良好！建议继续深入理解所有权系统，这是 Rust 的核心概念。".to_string());
        } else if stats.overall_progress < 80.0 {
            suggestions.push("🚀 已经掌握了 Rust 的基础知识，可以开始尝试一些实际项目来巩固所学内容。".to_string());
        } else {
            suggestions.push("🏆 恭喜！您已经完成了大部分学习内容，建议开始贡献开源项目或开发个人项目。".to_string());
        }

        // 基于平均分数给出建议
        if let Some(avg_score) = stats.average_score {
            if avg_score < 70.0 {
                suggestions.push("📚 建议多复习之前的内容，确保对基础概念有深入理解。".to_string());
            } else if avg_score >= 90.0 {
                suggestions.push("⭐ 您的学习成绩非常优秀！可以考虑挑战更高级的内容或帮助他人学习。".to_string());
            }
        }

        // 基于学习时间给出建议
        let total_hours = stats.completed_time_minutes / 60;
        if total_hours < 10 {
            suggestions.push("⏰ 建议增加学习时间，Rust 需要持续的练习才能掌握。".to_string());
        } else if total_hours > 100 {
            suggestions.push("💪 您已经投入了大量时间学习，坚持下去一定会取得成功！".to_string());
        }

        // 基于当前阶段给出具体建议
        match stats.current_stage {
            LearningStage::Stage1Basics => {
                suggestions.push("🔧 重点掌握 Rust 的基础语法和开发环境配置。".to_string());
            },
            LearningStage::Stage2Ownership => {
                suggestions.push("🔑 所有权系统是 Rust 的核心，建议多做练习加深理解。".to_string());
            },
            LearningStage::Stage3AdvancedConcepts => {
                suggestions.push("🎨 学习如何使用 Rust 的高级特性构建更复杂的程序。".to_string());
            },
            LearningStage::Stage4Ecosystem => {
                suggestions.push("🌐 了解 Rust 生态系统，学习使用常用的第三方库。".to_string());
            },
            LearningStage::Stage5Projects => {
                suggestions.push("💼 通过实际项目来综合运用所学知识，提升实战能力。".to_string());
            },
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_stage() {
        let stages = LearningStage::all_stages();
        assert_eq!(stages.len(), 5);
        assert_eq!(LearningStage::Stage1Basics.name(), "阶段1: 基础入门");
        assert_eq!(LearningStage::Stage1Basics.estimated_weeks(), 3);
    }

    #[test]
    fn test_learning_unit() {
        let mut unit = LearningUnit::new(
            "test-unit".to_string(),
            "测试单元".to_string(),
            LearningUnitType::ContentReading,
            LearningStage::Stage1Basics,
            "test/path".to_string(),
            60,
        );

        assert_eq!(unit.status, LearningUnitStatus::NotStarted);
        unit.start();
        assert_eq!(unit.status, LearningUnitStatus::InProgress);
        unit.complete(Some(85.0));
        assert_eq!(unit.status, LearningUnitStatus::Completed);
        assert_eq!(unit.score, Some(85.0));
    }

    #[test]
    fn test_progress_tracker() {
        let mut tracker = ProgressTracker::new("test-learner".to_string(), "测试学习者".to_string());
        assert_eq!(tracker.learning_units.len(), 3); // 示例单元
        assert_eq!(tracker.achievements.len(), 4); // 示例成就

        // 测试进度统计
        let stats = tracker.get_progress_stats();
        assert_eq!(stats.total_units, 3);
        assert_eq!(stats.completed_units, 0);
        assert_eq!(stats.overall_progress, 0.0);

        // 完成一个单元
        if let Some(unit) = tracker.get_unit_mut("stage1-environment") {
            unit.start();
            unit.complete(Some(90.0));
        }

        let stats = tracker.get_progress_stats();
        assert_eq!(stats.completed_units, 1);
        assert!(stats.overall_progress > 0.0);
    }
}