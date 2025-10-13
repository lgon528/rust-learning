# Stage 1 - Rust 基础示例

本目录包含 Rust 基础阶段的代码示例项目，旨在帮助初学者掌握 Rust 的核心概念和基本语法。

## 📋 示例项目列表

### 1. [environment-demo](./environment-demo/) - 环境搭建和验证

**学习目标**: 验证 Rust 开发环境，了解系统信息获取

**核心概念**:
- 环境变量操作 (`std::env`)
- 编译时信息获取 (`env!`, `option_env!`)
- 条件编译 (`cfg!`)
- 系统平台检测

**运行示例**:
```bash
cargo test -p environment-demo
cargo run -p environment-demo --example system_info
```

### 2. [control-flow-demo](./control-flow-demo/) - 控制流结构

**学习目标**: 掌握 Rust 的控制流语句和模式匹配

**核心概念**:
- 条件语句 (`if`/`else`)
- 循环结构 (`loop`, `while`, `for`)
- 模式匹配 (`match`)
- 循环控制 (`break`, `continue`)
- 标签循环和嵌套控制

**运行示例**:
```bash
cargo test -p control-flow-demo
cargo run -p control-flow-demo --example basic_loops
```

### 3. [ownership-intro-demo](./ownership-intro-demo/) - 所有权入门

**学习目标**: 理解 Rust 独特的所有权系统基础

**核心概念**:
- 所有权规则和转移
- 借用和引用 (`&`, `&mut`)
- 可变性控制 (`mut`)
- 生命周期基础
- 字符串切片和数组切片

**运行示例**:
```bash
cargo test -p ownership-intro-demo
cargo run -p ownership-intro-demo --example ownership_basics
```

### 4. [comparisons-demo](./comparisons-demo/) - 比较操作

**学习目标**: 掌握 Rust 中的比较运算和排序

**核心概念**:
- 基本比较运算符
- `PartialEq`, `Eq`, `PartialOrd`, `Ord` trait
- 自定义类型的比较实现
- 排序算法 (冒泡排序、选择排序)
- 搜索算法 (二分搜索)

**运行示例**:
```bash
cargo test -p comparisons-demo
cargo run -p comparisons-demo --example sorting_demo
```

## 🎯 学习路径建议

### 推荐学习顺序

1. **environment-demo** - 首先验证开发环境
2. **control-flow-demo** - 学习基本控制结构
3. **ownership-intro-demo** - 理解所有权概念
4. **comparisons-demo** - 掌握比较和排序

### 学习方法

1. **阅读代码**: 仔细阅读每个项目的 `src/lib.rs` 文件
2. **运行测试**: 使用 `cargo test -p <project-name>` 运行测试
3. **查看文档**: 使用 `cargo doc --open -p <project-name>` 查看文档
4. **实验修改**: 尝试修改代码，观察编译器反馈
5. **完成练习**: 查看各项目 README 中的扩展练习

## 🧪 测试和验证

### 运行所有基础示例测试

```bash
# 运行所有 stage1-basics 项目的测试
cargo test -p environment-demo -p control-flow-demo -p ownership-intro-demo -p comparisons-demo

# 或者运行整个 workspace 的测试
cargo test --workspace
```

### 生成文档

```bash
# 为所有基础项目生成文档
cargo doc --open --workspace

# 为特定项目生成文档
cargo doc --open -p ownership-intro-demo
```

## 📚 相关学习资源

### 对应教程章节

- **environment-demo**: 对应《Rust 程序设计语言》第 1 章 - 入门指南
- **control-flow-demo**: 对应第 3 章 - 常见编程概念
- **ownership-intro-demo**: 对应第 4 章 - 认识所有权
- **comparisons-demo**: 对应第 5 章 - 使用结构体组织相关联的数据

### 扩展阅读

- [Rust Book - 控制流](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust Book - 所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - 流程控制](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- [Rust by Example - 作用域规则](https://doc.rust-lang.org/rust-by-example/scope.html)

## 🔧 开发工具使用

### 代码格式化和检查

```bash
# 格式化代码
cargo fmt

# 运行 Clippy 检查
cargo clippy -- -D warnings

# 检查代码但不构建
cargo check --workspace
```

### 性能分析

```bash
# 运行 benchmark (如果有的话)
cargo bench -p comparisons-demo

# 查看编译优化后的代码
cargo build --release
```

## 🎓 学习检查点

完成本阶段学习后，你应该能够：

- ✅ 配置和验证 Rust 开发环境
- ✅ 使用 Rust 的基本控制流结构
- ✅ 理解所有权、借用和生命周期的基本概念
- ✅ 实现自定义类型的比较操作
- ✅ 编写基本的 Rust 函数和测试
- ✅ 使用 Cargo 管理项目和依赖

## 🚀 下一步

完成基础阶段后，建议继续学习：

1. **Stage 2 - 所有权系统**: 深入学习所有权、借用检查器和生命周期
2. **错误处理**: 学习 `Result` 和 `Option` 类型
3. **结构体和枚举**: 掌握自定义数据类型
4. **模块系统**: 了解代码组织和可见性

## 💡 常见问题

### Q: 编译错误怎么办？
A: Rust 编译器提供详细的错误信息，仔细阅读错误提示。常见问题包括所有权转移、借用检查失败等。

### Q: 如何调试 Rust 代码？
A: 使用 `println!` 宏进行简单调试，或使用 `dbg!` 宏查看变量值。IDE 如 VS Code 配合 rust-analyzer 提供更好的调试体验。

### Q: 性能如何优化？
A: 在学习阶段专注于正确性，性能优化可以后续考虑。Rust 的零成本抽象保证了基本的性能。

---

**开始探索 Rust 的世界吧！** 🦀

记住：Rust 的学习曲线可能比较陡峭，但一旦掌握了所有权系统，你就能写出既安全又高效的代码。不要害怕编译器错误，它们是你最好的老师！