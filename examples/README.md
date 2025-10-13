# 代码示例库

本目录包含Rust学习计划中所有阶段的代码示例，按学习阶段组织。

## 目录结构

```
examples/
├── stage1-basics/          # 阶段1：基础入门示例
├── stage2-ownership/       # 阶段2：所有权系统示例
├── stage3-advanced/        # 阶段3：进阶特性示例
└── stage4-projects/        # 阶段4：项目级示例
```

## 使用方法

### 运行单个示例

```bash
# 进入具体示例目录
cd examples/stage1-basics/hello-world

# 运行示例
cargo run
```

### 运行所有示例

```bash
# 在项目根目录运行
cargo run --bin example-runner
```

### 测试示例

```bash
# 测试特定阶段的所有示例
cargo test --package stage1-basics

# 测试所有示例
cargo test
```

## 示例特点

- **可运行性**: 所有示例都可以直接编译和运行
- **注释完整**: 每个示例都包含详细的中文注释
- **最佳实践**: 体现Rust编程的最佳实践
- **渐进式**: 从简单到复杂，循序渐进

## 学习建议

1. **按顺序学习**: 建议按阶段顺序学习示例
2. **动手实践**: 不要只看代码，要亲自运行和修改
3. **理解原理**: 重点理解每个示例背后的Rust概念
4. **举一反三**: 尝试基于示例创建自己的变体

## 贡献指南

如果你发现示例中的问题或有改进建议，欢迎提交Issue或Pull Request。