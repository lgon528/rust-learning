# 学习工具集

本目录包含Rust学习计划的辅助工具，用于自我评估、代码质量检查和学习进度跟踪。

## 工具概览

### 📊 自我评估工具 (assessment)

**功能特点**:
- 每周学习评估
- 阶段性知识检查
- 个性化学习建议
- 学习效果分析

**使用场景**:
- 每周学习总结
- 阶段学习验收
- 学习计划调整
- 知识点查漏补缺

### 🔍 代码质量检查 (quality-check)

**功能特点**:
- 自动化代码审查
- 最佳实践检查
- 性能分析报告
- 安全漏洞扫描

**集成工具**:
- rustfmt: 代码格式化
- clippy: 代码检查
- cargo-audit: 安全审计
- cargo-tarpaulin: 测试覆盖率

### 📈 进度跟踪工具 (progress-tracker)

**功能特点**:
- 学习进度可视化
- 时间投入统计
- 成就系统
- 学习路径推荐

**数据维度**:
- 完成的练习数量
- 代码质量分数
- 学习时间投入
- 知识点掌握程度

## 快速开始

### 安装工具

```bash
# 进入工具目录
cd tools

# 构建所有工具
cargo build --release

# 安装到系统路径（可选）
cargo install --path assessment
cargo install --path quality-check
cargo install --path progress-tracker
```

### 初始化配置

```bash
# 初始化学习配置
cargo run --bin setup-learning-env

# 创建个人学习档案
cargo run --bin create-profile
```

## 使用指南

### 自我评估工具

#### 每周评估

```bash
# 开始新的一周评估
cargo run --bin weekly-assessment

# 查看评估历史
cargo run --bin assessment-history

# 生成学习报告
cargo run --bin learning-report --week 1
```

#### 阶段评估

```bash
# 阶段1评估
cargo run --bin stage-assessment -- --stage 1

# 查看阶段进度
cargo run --bin stage-progress

# 生成阶段报告
cargo run --bin stage-report -- --stage 1 --format pdf
```

### 代码质量检查

#### 单项检查

```bash
# 代码格式检查
cargo run --bin format-check -- --path ../exercises/stage1-basics

# 代码质量检查
cargo run --bin quality-check -- --path ../examples/stage2-ownership

# 安全审计
cargo run --bin security-audit -- --workspace
```

#### 综合检查

```bash
# 运行所有质量检查
cargo run --bin full-quality-check

# 生成质量报告
cargo run --bin quality-report -- --output report.html

# 设置质量门禁
cargo run --bin quality-gate -- --min-score 80
```

### 进度跟踪工具

#### 进度记录

```bash
# 记录练习完成
cargo run --bin track-exercise -- --exercise "stage1-basic-01" --status completed

# 记录学习时间
cargo run --bin track-time -- --activity "reading" --duration 120

# 记录知识点掌握
cargo run --bin track-knowledge -- --topic "ownership" --level "proficient"
```

#### 进度查看

```bash
# 查看整体进度
cargo run --bin progress-summary

# 查看详细统计
cargo run --bin progress-stats -- --period week

# 生成进度图表
cargo run --bin progress-chart -- --type line --output progress.png
```

## 配置文件

### 评估配置 (assessment/config.toml)

```toml
[assessment]
# 评估频率
weekly_reminder = true
stage_checkpoint = true

# 评估标准
[assessment.criteria]
code_quality_weight = 0.4
knowledge_test_weight = 0.3
practice_completion_weight = 0.3

# 通过标准
[assessment.thresholds]
stage_pass_score = 75
weekly_target_score = 80
```

### 质量检查配置 (quality-check/config.toml)

```toml
[quality]
# 检查工具配置
rustfmt_check = true
clippy_check = true
audit_check = true
coverage_check = true

# 质量标准
[quality.standards]
min_coverage = 80
max_complexity = 10
max_line_length = 100

# 忽略规则
[quality.ignore]
files = ["generated/*", "vendor/*"]
lints = ["clippy::module_inception"]
```

### 进度跟踪配置 (progress-tracker/config.toml)

```toml
[tracking]
# 数据存储
data_file = "progress.json"
backup_enabled = true

# 统计周期
[tracking.periods]
daily_summary = true
weekly_report = true
monthly_analysis = true

# 成就系统
[tracking.achievements]
exercise_streak = [7, 14, 30]
quality_score = [80, 90, 95]
learning_hours = [50, 100, 200]
```

## 数据格式

### 学习进度数据

```json
{
  "profile": {
    "name": "学习者姓名",
    "start_date": "2024-01-01",
    "target_completion": "2024-06-01",
    "weekly_hours": 7
  },
  "progress": {
    "current_stage": 2,
    "completed_exercises": 15,
    "total_exercises": 60,
    "completion_rate": 0.25
  },
  "quality_metrics": {
    "average_score": 85,
    "latest_score": 88,
    "trend": "improving"
  },
  "time_tracking": {
    "total_hours": 45,
    "this_week_hours": 8,
    "daily_average": 1.2
  }
}
```

### 评估报告格式

```json
{
  "assessment_id": "week-03-2024",
  "date": "2024-01-21",
  "type": "weekly",
  "scores": {
    "knowledge_test": 82,
    "code_quality": 88,
    "practice_completion": 90,
    "overall": 86
  },
  "feedback": {
    "strengths": ["所有权概念理解深入", "代码风格规范"],
    "improvements": ["错误处理需要加强", "测试覆盖率偏低"],
    "recommendations": ["多练习Result类型", "编写更多单元测试"]
  }
}
```

## 扩展开发

### 添加新的评估维度

```rust
// assessment/src/evaluator.rs
pub trait Evaluator {
    fn evaluate(&self, data: &LearningData) -> EvaluationResult;
    fn get_feedback(&self, score: f64) -> Vec<String>;
}

// 实现自定义评估器
pub struct CustomEvaluator {
    criteria: EvaluationCriteria,
}

impl Evaluator for CustomEvaluator {
    fn evaluate(&self, data: &LearningData) -> EvaluationResult {
        // 自定义评估逻辑
    }
}
```

### 添加新的质量检查

```rust
// quality-check/src/checker.rs
pub trait QualityChecker {
    fn check(&self, code_path: &Path) -> CheckResult;
    fn get_suggestions(&self, issues: &[Issue]) -> Vec<Suggestion>;
}

// 实现自定义检查器
pub struct CustomChecker {
    rules: Vec<QualityRule>,
}
```

### 添加新的跟踪指标

```rust
// progress-tracker/src/tracker.rs
pub trait ProgressTracker {
    fn track_event(&mut self, event: TrackingEvent);
    fn get_metrics(&self, period: TimePeriod) -> Metrics;
}

// 实现自定义跟踪器
pub struct CustomTracker {
    storage: Box<dyn Storage>,
    analyzers: Vec<Box<dyn Analyzer>>,
}
```

## 故障排除

### 常见问题

1. **工具无法启动**
   ```bash
   # 检查依赖
   cargo check
   
   # 重新构建
   cargo clean && cargo build
   ```

2. **数据文件损坏**
   ```bash
   # 恢复备份
   cargo run --bin restore-backup
   
   # 重置数据
   cargo run --bin reset-data --confirm
   ```

3. **权限问题**
   ```bash
   # 修复权限
   chmod +x target/release/*
   
   # 检查配置文件权限
   ls -la config/
   ```

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug cargo run --bin assessment

# 运行诊断工具
cargo run --bin diagnostic -- --full

# 生成调试报告
cargo run --bin debug-report
```