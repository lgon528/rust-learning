# Rust 语法基础演示

这个项目演示了 Rust 编程语言的基础语法特性，对应《Rust 学习指南》第二章内容。

## 项目结构

```
syntax-demo/
├── src/
│   └── lib.rs          # 主要的语法演示代码
├── Cargo.toml          # 项目配置文件
└── README.md           # 项目说明文档
```

## 涵盖的语法特性

### 1. 变量和可变性
- 不可变变量声明
- 可变变量声明 (`mut` 关键字)
- 变量遮蔽 (shadowing)
- 常量声明

### 2. 数据类型
- **标量类型**：
  - 整数类型 (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`)
  - 浮点类型 (`f32`, `f64`)
  - 布尔类型 (`bool`)
  - 字符类型 (`char`)
- **复合类型**：
  - 元组 (tuple)
  - 数组 (array)

### 3. 函数
- 函数定义和调用
- 参数传递
- 返回值
- 表达式 vs 语句

### 4. 注释
- 单行注释 (`//`)
- 多行注释 (`/* */`)
- 文档注释 (`///`)
- 内部文档注释 (`//!`)

## 使用方法

### 运行示例

```bash
# 进入项目目录
cd examples/stage1-basics/syntax-demo

# 运行测试
cargo test

# 查看文档
cargo doc --open
```

### 代码示例

```rust
use syntax_demo::variables::demonstrate_variables;
use syntax_demo::data_types::demonstrate_data_types;
use syntax_demo::functions::greet;
use syntax_demo::comments::demonstrate_comments;

fn main() {
    // 演示变量
    demonstrate_variables();
    
    // 演示数据类型
    demonstrate_data_types();
    
    // 演示函数
    let message = greet("Rust");
    println!("{}", message);
    
    // 演示注释
    demonstrate_comments();
}
```

## 学习要点

### 变量和可变性
- Rust 中变量默认是不可变的
- 使用 `mut` 关键字声明可变变量
- 变量遮蔽允许重新绑定变量名

### 类型系统
- Rust 是静态类型语言
- 编译器可以进行类型推断
- 显式类型注解提高代码可读性

### 函数设计
- 函数参数必须声明类型
- 返回值类型在箭头后声明
- 最后一个表达式作为返回值

### 注释最佳实践
- 使用文档注释为公共 API 编写文档
- 包含使用示例和参数说明
- 内部文档注释用于模块级别说明

## 相关章节

- [第二章：Rust 语法基础](../../content/stage1-basics/02-syntax/)
- [第一章：环境搭建](../../content/stage1-basics/01-environment/)
- [第三章：控制流](../../content/stage1-basics/03-control-flow/)

## 测试

项目包含完整的单元测试和文档测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_greet_function

# 运行文档测试
cargo test --doc
```

## 扩展练习

1. **变量练习**：尝试不同的变量声明方式
2. **类型转换**：练习不同数据类型之间的转换
3. **函数设计**：编写更复杂的函数示例
4. **文档编写**：为自己的函数添加文档注释

## 注意事项

- 确保理解不可变性的概念
- 注意整数溢出的处理
- 理解表达式和语句的区别
- 养成编写文档注释的习惯