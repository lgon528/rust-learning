# 练习题库

本目录包含Rust学习计划中所有阶段的练习题，按学习阶段和难度组织。

## 目录结构

```
exercises/
├── stage1-basics/          # 阶段1：基础入门练习
├── stage2-ownership/       # 阶段2：所有权系统练习
├── stage3-advanced/        # 阶段3：进阶特性练习
└── stage4-projects/        # 阶段4：综合项目练习
```

## 练习分类

每个阶段的练习按难度分为三类：

- **基础练习 (Basic)**: 巩固基本概念，适合初学者
- **应用练习 (Application)**: 实际应用场景，提升实战能力
- **挑战练习 (Challenge)**: 高难度题目，深入理解原理

## 使用方法

### 完成练习

```bash
# 进入练习目录
cd exercises/stage1-basics/basic-01-variables

# 查看练习要求
cat README.md

# 编辑代码文件
# 在 src/main.rs 或 src/lib.rs 中完成练习

# 运行测试检查答案
cargo test
```

### 查看参考答案

```bash
# 参考答案在 solutions/ 目录中
cd solutions
cargo run  # 或 cargo test
```

### 自动评分

```bash
# 使用自动评分工具
cargo run --bin exercise-checker -- --stage 1
```

## 练习特点

- **测试驱动**: 每个练习都有完整的测试用例
- **渐进难度**: 从简单到复杂，循序渐进
- **实用性强**: 练习内容贴近实际开发场景
- **即时反馈**: 通过测试获得即时反馈

## 学习建议

### 做题流程

1. **理解题意**: 仔细阅读练习说明和要求
2. **分析测试**: 查看测试用例，理解预期行为
3. **编写代码**: 实现功能，通过所有测试
4. **优化代码**: 考虑性能、可读性和最佳实践
5. **对比答案**: 查看参考答案，学习不同解法

### 遇到困难时

1. **查看提示**: 每个练习都有提示信息
2. **参考示例**: 回顾相关的代码示例
3. **查阅文档**: 使用 `cargo doc --open` 查看文档
4. **寻求帮助**: 在学习社区提问讨论

## 评分标准

练习评分基于以下维度：

- **功能正确性** (40%): 通过所有测试用例
- **代码质量** (30%): 遵循Rust最佳实践
- **性能效率** (20%): 算法和数据结构选择合理
- **代码风格** (10%): 符合Rust代码规范

## 进度跟踪

使用进度跟踪工具记录学习进度：

```bash
# 更新练习完成状态
cargo run --bin progress-tracker -- --exercise stage1-basic-01 --status completed

# 查看整体进度
cargo run --bin progress-tracker -- --summary
```