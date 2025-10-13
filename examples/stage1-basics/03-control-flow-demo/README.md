# Control Flow Demo - Rust 控制流演示

本项目演示了 Rust 中的各种控制流结构，包括条件语句、循环、模式匹配和高级控制流技术。

## 项目结构

```
control-flow-demo/
├── src/
│   └── lib.rs          # 主要的控制流示例代码
├── Cargo.toml          # 项目配置
└── README.md           # 项目说明文档
```

## 涵盖的功能

### 1. 条件语句 (Conditionals)
- `if`、`else if`、`else` 语句
- 条件表达式
- 数值分类和年龄分组
- 绝对值计算

### 2. 循环 (Loops)
- `for` 循环遍历
- `while` 循环
- `loop` 无限循环
- 循环控制：`break` 和 `continue`
- 嵌套循环和标签

### 3. 模式匹配 (Pattern Matching)
- `match` 表达式
- `Option` 和 `Result` 处理
- 枚举匹配
- 守卫条件
- 解构匹配

### 4. 高级控制流 (Advanced Control Flow)
- 函数式编程风格
- 迭代器和闭包
- 错误传播 (`?` 操作符)
- 复杂的控制流组合

## 使用方法

### 运行测试

```bash
# 运行所有测试
cargo test -p control-flow-demo

# 运行文档测试
cargo test -p control-flow-demo --doc

# 运行单元测试
cargo test -p control-flow-demo --lib
```

### 查看示例

```rust
use control_flow_demo::{
    conditionals::*,
    loops::*,
    pattern_matching::*,
    advanced::*,
};

// 条件语句示例
let result = check_number(42);
let category = categorize_age(25);

// 循环示例
let count = count_to_n(5);
let fact = factorial(5);

// 模式匹配示例
let description = describe_value(Some(42));
let color_desc = describe_color(Color::Red);

// 高级控制流示例
let numbers = vec![1, 2, 3, 4, 5, 6];
let processed = process_numbers(&numbers);
```

## 学习要点

### 条件语句
- Rust 中的 `if` 是表达式，可以返回值
- 条件必须是 `bool` 类型
- 可以用于变量赋值和函数返回

### 循环结构
- `for` 循环适用于已知迭代次数
- `while` 循环适用于条件驱动的迭代
- `loop` 提供无限循环，需要显式 `break`
- 循环标签允许在嵌套循环中精确控制

### 模式匹配
- `match` 必须覆盖所有可能的情况
- 支持复杂的模式和守卫条件
- 是处理枚举和 `Option`/`Result` 的首选方式

### 高级技术
- 函数式编程风格提高代码简洁性
- `?` 操作符简化错误处理
- 迭代器链式调用提高性能

## 相关章节

本示例对应《Rust 学习指南》第三章：控制流
- 条件语句和分支
- 循环结构详解
- 模式匹配基础
- 高级控制流技术

## 测试信息

- **单元测试**: 5 个测试用例
- **文档测试**: 15 个示例测试
- **覆盖范围**: 所有主要控制流结构

## 扩展练习

1. **条件练习**：实现更复杂的条件逻辑，如多重条件判断
2. **循环练习**：尝试不同的循环模式，如双重循环查找
3. **匹配练习**：创建自定义枚举并实现复杂的模式匹配
4. **函数式练习**：使用迭代器和闭包重写传统循环代码

## 注意事项

- 所有示例都遵循 Rust 的所有权和借用规则
- 代码风格符合 Rust 官方规范
- 包含详细的文档注释和使用示例
- 测试覆盖了正常情况和边界条件