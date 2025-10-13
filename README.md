# Rust Learning Project

一个全面的 Rust 学习项目，包含系统化的学习内容、实践示例、练习题目和实际项目。本项目旨在帮助开发者从零开始掌握 Rust 编程语言，并能够应用到实际的系统编程和 Web 开发中。

## 🎯 项目目标

- 提供结构化的 Rust 学习路径
- 通过实践示例加深对 Rust 概念的理解
- 提供渐进式的练习和项目实战
- 建立 Rust 最佳实践的参考资源

## 📁 项目结构

```
rust-learning/
├── content/                    # 学习内容和教程
│   ├── stage1-basics/         # 基础语法和概念
│   ├── stage2-ownership/      # 所有权和借用系统
│   ├── stage3-advanced/       # 高级特性和模式
│   └── stage4-projects/       # 项目实战指导
├── examples/                   # 代码示例和演示
│   ├── stage1-basics/         # 基础示例项目
│   ├── stage2-ownership/      # 所有权相关示例
│   ├── stage3-advanced/       # 高级特性示例
│   └── stage4-projects/       # 项目示例
├── exercises/                  # 练习题目和解答
│   ├── stage1-basics/         # 基础练习
│   ├── stage2-ownership/      # 所有权练习
│   ├── stage3-advanced/       # 高级练习
│   └── stage4-projects/       # 项目练习
├── projects/                   # 实际项目实现
│   ├── blockchain/            # 区块链项目
│   ├── system-programming/    # 系统编程项目
│   ├── web-application/       # Web 应用项目
│   └── shared/               # 共享库和工具
├── tools/                      # 开发工具和脚本
│   ├── assessment/            # 学习评估工具
│   ├── progress-tracker/      # 进度跟踪工具
│   └── quality-check/         # 代码质量检查
└── docs/                       # 项目文档
    └── rust-learning-plan/    # 学习计划文档
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+ (推荐使用最新稳定版)
- Cargo (Rust 包管理器)
- Git (版本控制)

### 安装和设置

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd rust-learning
   ```

2. **验证 Rust 环境**
   ```bash
   cargo test -p environment-demo
   ```

3. **运行示例项目**
   ```bash
   # 运行所有基础示例测试
   cargo test --workspace
   
   # 运行特定示例
   cargo test -p control-flow-demo
   cargo test -p ownership-intro-demo
   cargo test -p comparisons-demo
   ```

## 📚 学习路径

### 阶段 1: Rust 基础 (Stage 1 - Basics)

**目标**: 掌握 Rust 基本语法和核心概念

#### 已完成的示例项目:

1. **[environment-demo](./examples/stage1-basics/environment-demo/)** - 环境搭建和验证
   - 系统信息检查
   - 环境变量操作
   - 编译时信息获取
   - 平台特性检查

2. **[control-flow-demo](./examples/stage1-basics/control-flow-demo/)** - 控制流结构
   - 条件语句 (if/else)
   - 循环结构 (loop, while, for)
   - 模式匹配 (match)
   - 高级控制流技术

3. **[ownership-intro-demo](./examples/stage1-basics/ownership-intro-demo/)** - 所有权入门
   - 所有权基本概念
   - 借用和引用
   - 可变性规则
   - 生命周期基础

4. **[comparisons-demo](./examples/stage1-basics/comparisons-demo/)** - 比较操作
   - 基本比较运算
   - 自定义类型比较
   - 排序和搜索算法
   - 高级比较技术

#### 学习建议:
1. 按顺序学习各个示例项目
2. 运行测试了解功能
3. 阅读代码和文档注释
4. 尝试修改代码进行实验

### 阶段 2: 所有权系统 (Stage 2 - Ownership)

**目标**: 深入理解 Rust 的所有权、借用和生命周期系统

*（规划中，将包含更深入的所有权示例）*

### 阶段 3: 高级特性 (Stage 3 - Advanced)

**目标**: 掌握 Rust 的高级特性和编程模式

*（规划中，将包含 trait、泛型、宏等高级主题）*

### 阶段 4: 项目实战 (Stage 4 - Projects)

**目标**: 通过实际项目应用所学知识

*（规划中，将包含完整的应用项目）*

## 🛠️ 开发工具

### 代码质量

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 运行所有测试
cargo test

# 生成文档
cargo doc --open
```

### 项目管理

- **Workspace 配置**: 使用 Cargo workspace 管理多个相关项目
- **依赖管理**: 统一管理依赖版本和特性
- **测试策略**: 单元测试、集成测试和文档测试

## 📖 学习资源

### 官方资源
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例学习
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - 高级主题

### 推荐阅读
- [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/) - 中文版官方教程
- [Rust 语言圣经](https://course.rs/) - 中文学习资源
- [Rust 设计模式](https://rust-unofficial.github.io/patterns/) - 最佳实践

## 🤝 贡献指南

### 如何贡献

1. **报告问题**: 发现 bug 或有改进建议时创建 issue
2. **提交代码**: Fork 项目，创建分支，提交 PR
3. **完善文档**: 改进注释、README 或添加示例
4. **分享经验**: 分享学习心得和最佳实践

### 代码规范

- 遵循 Rust 官方代码风格 (`cargo fmt`)
- 通过 Clippy 检查 (`cargo clippy`)
- 为公共 API 编写文档注释
- 为新功能添加相应的测试

## 📊 项目状态

### 当前进度

- ✅ **环境搭建示例** - 完成
- ✅ **控制流示例** - 完成  
- ✅ **所有权入门示例** - 完成
- ✅ **比较操作示例** - 完成
- 🔄 **高级所有权示例** - 规划中
- 🔄 **错误处理示例** - 规划中
- 🔄 **泛型和 trait 示例** - 规划中

### 测试覆盖

```bash
# 查看测试覆盖情况
cargo test --workspace --verbose
```

当前所有基础示例项目都有完整的单元测试和文档测试覆盖。

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

感谢 Rust 社区提供的优秀学习资源和工具，特别是：
- Rust 官方团队
- 《Rust 程序设计语言》作者
- 所有贡献者和学习者

---

**开始你的 Rust 学习之旅吧！** 🦀

如果你是 Rust 新手，建议从 `examples/stage1-basics/environment-demo` 开始，验证你的开发环境是否正确配置。然后按顺序学习其他示例项目。

有问题或建议？欢迎创建 issue 或参与讨论！