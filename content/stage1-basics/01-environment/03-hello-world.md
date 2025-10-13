# 1.3 第一个Rust程序

编写第一个Rust程序是学习之旅的重要里程碑。本节将详细介绍如何创建、编译和运行Rust程序，并深入理解程序的结构。

## 🎯 学习目标

- 创建第一个Rust程序
- 理解Rust程序的基本结构
- 掌握编译和运行流程
- 了解Rust与其他语言的差异
- 学会基本的调试方法

## 🚀 Hello World程序

### 方法1：使用rustc直接编译

#### 创建源文件

```bash
# 创建项目目录
mkdir hello_rust
cd hello_rust

# 创建源文件
touch main.rs
```

#### 编写代码

```rust
// main.rs
fn main() {
    println!("Hello, world!");
}
```

#### 编译和运行

```bash
# 编译
rustc main.rs

# 运行（Linux/macOS）
./main

# 运行（Windows）
.\main.exe
```

### 方法2：使用Cargo（推荐）

#### 创建新项目

```bash
# 创建新的Cargo项目
cargo new hello_cargo
cd hello_cargo

# 查看项目结构
tree .
# 或者
ls -la
```

项目结构：
```
hello_cargo/
├── Cargo.toml      # 项目配置文件
├── src/
│   └── main.rs     # 源代码文件
└── .gitignore      # Git忽略文件
```

#### 查看生成的代码

```rust
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```

#### 构建和运行

```bash
# 构建项目
cargo build

# 运行项目
cargo run

# 检查代码（不生成可执行文件）
cargo check
```

## 📋 程序结构详解

### 基本语法分析

```rust
fn main() {                    // 1. 函数定义
    println!("Hello, world!"); // 2. 宏调用
}                              // 3. 代码块结束
```

#### 1. 函数定义 `fn main()`

- `fn`：函数定义关键字
- `main`：函数名，程序入口点
- `()`：参数列表（空）
- `{}`：函数体

#### 2. 宏调用 `println!`

- `println!`：打印宏（注意感叹号！）
- 宏与函数的区别：宏在编译时展开
- 格式化输出功能

#### 3. 语句结束

- 分号`;`表示语句结束
- Rust区分语句(statement)和表达式(expression)

### 与其他语言对比

#### C/C++对比

```c
// C语言
#include <stdio.h>

int main() {
    printf("Hello, world!\n");
    return 0;
}
```

```rust
// Rust
fn main() {
    println!("Hello, world!");
    // 无需显式return 0
    // 无需包含头文件
}
```

**主要差异**：
- Rust无需包含头文件
- Rust的main函数无需返回值
- Rust使用宏而非函数进行格式化输出

#### Golang对比

```go
// Go语言
package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
```

```rust
// Rust
fn main() {
    println!("Hello, world!");
}
```

**主要差异**：
- Rust无需包声明
- Rust的println!是内置宏
- Rust使用fn而非func

## 🔧 Cargo项目详解

### Cargo.toml文件

```toml
[package]
name = "hello_cargo"        # 项目名称
version = "0.1.0"           # 版本号
edition = "2021"            # Rust版本

# 可选配置
authors = ["Your Name <your.email@example.com>"]
description = "A simple hello world program"
license = "MIT"
repository = "https://github.com/username/hello_cargo"

[dependencies]
# 依赖包列表（目前为空）
```

### 项目结构说明

```
hello_cargo/
├── Cargo.toml          # 项目元数据和依赖
├── Cargo.lock          # 依赖版本锁定（自动生成）
├── src/                # 源代码目录
│   ├── main.rs         # 二进制程序入口
│   └── lib.rs          # 库入口（可选）
├── tests/              # 集成测试（可选）
├── examples/           # 示例代码（可选）
├── benches/            # 性能测试（可选）
└── target/             # 编译输出目录
    ├── debug/          # 调试版本
    └── release/        # 发布版本
```

### Cargo命令详解

```bash
# 项目管理
cargo new project_name      # 创建新项目
cargo init                  # 在当前目录初始化项目

# 构建相关
cargo build                 # 构建调试版本
cargo build --release       # 构建发布版本
cargo run                   # 构建并运行
cargo check                 # 检查代码（快速）

# 测试相关
cargo test                  # 运行测试
cargo bench                 # 运行性能测试

# 文档相关
cargo doc                   # 生成文档
cargo doc --open            # 生成并打开文档

# 清理
cargo clean                 # 清理构建文件
```

## 🎨 扩展Hello World

### 1. 格式化输出

```rust
fn main() {
    let name = "Rust";
    let version = "1.75";
    
    // 基本格式化
    println!("Hello, {}!", name);
    
    // 多个参数
    println!("Hello, {}! Version: {}", name, version);
    
    // 位置参数
    println!("{0} is awesome! {0} version {1}", name, version);
    
    // 命名参数
    println!("Hello, {language}! Version: {ver}", 
             language = name, ver = version);
    
    // 格式化选项
    let number = 42;
    println!("Number: {:>5}", number);      // 右对齐，宽度5
    println!("Number: {:<5}", number);      // 左对齐，宽度5
    println!("Number: {:^5}", number);      // 居中，宽度5
    println!("Number: {:05}", number);      // 零填充
    println!("Hex: {:x}", number);          // 十六进制
    println!("Binary: {:b}", number);       // 二进制
}
```

### 2. 用户输入

```rust
use std::io;

fn main() {
    println!("What's your name?");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim());
}
```

### 3. 命令行参数

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        println!("Hello, {}!", args[1]);
    } else {
        println!("Hello, world!");
    }
    
    // 显示所有参数
    println!("Arguments: {:?}", args);
}
```

运行示例：
```bash
cargo run Alice Bob
# 输出：
# Hello, Alice!
# Arguments: ["target/debug/hello_cargo", "Alice", "Bob"]
```

## 🐛 调试技巧

### 1. 使用println!调试

```rust
fn main() {
    let x = 5;
    let y = 10;
    
    println!("x = {}, y = {}", x, y);  // 基本调试
    println!("x = {:?}", x);           // Debug格式
    
    let result = x + y;
    println!("result = {}", result);
}
```

### 2. 使用dbg!宏

```rust
fn main() {
    let x = 5;
    let y = dbg!(x * 2);  // 打印表达式和结果
    
    dbg!(x, y);           // 打印多个变量
}
```

### 3. 编译时调试信息

```bash
# 启用调试信息
cargo build

# 使用调试器（需要安装gdb或lldb）
# Linux
gdb target/debug/hello_cargo

# macOS
lldb target/debug/hello_cargo
```

## 🔍 常见错误和解决方案

### 1. 编译错误

#### 错误：忘记分号

```rust
// 错误代码
fn main() {
    println!("Hello, world!")  // 缺少分号
}
```

**错误信息**：
```
error: expected `;`, found `}`
```

**解决方案**：添加分号
```rust
fn main() {
    println!("Hello, world!");  // 添加分号
}
```

#### 错误：拼写错误

```rust
// 错误代码
fn main() {
    printl!("Hello, world!");  // println! 拼写错误
}
```

**错误信息**：
```
error: cannot find macro `printl` in this scope
```

### 2. 运行时错误

#### 错误：数组越界

```rust
fn main() {
    let arr = [1, 2, 3];
    println!("{}", arr[5]);  // 越界访问
}
```

**错误信息**：
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'
```

### 3. Cargo错误

#### 错误：项目名称无效

```bash
cargo new 123-project  # 数字开头无效
```

**解决方案**：使用有效的项目名称
```bash
cargo new my_project   # 使用下划线
cargo new my-project   # 使用连字符
```

## 🧪 实践练习

### 练习1：基础Hello World

创建一个程序，输出你的姓名和当前日期：

```rust
fn main() {
    // TODO: 输出你的姓名
    // TODO: 输出当前日期（可以硬编码）
}
```

### 练习2：格式化输出

创建一个程序，展示不同的格式化选项：

```rust
fn main() {
    let name = "Rust";
    let year = 2023;
    let pi = 3.14159;
    
    // TODO: 使用不同的格式化选项输出这些变量
}
```

### 练习3：交互式程序

创建一个简单的交互式程序：

```rust
use std::io;

fn main() {
    // TODO: 询问用户姓名
    // TODO: 询问用户年龄
    // TODO: 输出个性化问候
}
```

### 练习4：命令行工具

创建一个简单的命令行工具：

```rust
use std::env;

fn main() {
    // TODO: 处理命令行参数
    // TODO: 根据参数执行不同操作
}
```

## 📊 性能对比

### 编译时间对比

```bash
# 测试编译时间
time rustc main.rs          # 直接编译
time cargo build            # Cargo构建
time cargo build --release  # 发布版本构建
```

### 可执行文件大小

```bash
# 查看文件大小
ls -lh main                    # rustc编译的文件
ls -lh target/debug/hello_cargo    # debug版本
ls -lh target/release/hello_cargo  # release版本
```

典型结果：
- Debug版本：~3MB（包含调试信息）
- Release版本：~300KB（优化后）
- Strip后：~200KB（移除符号表）

### 运行时性能

```bash
# 性能测试
time ./target/debug/hello_cargo
time ./target/release/hello_cargo
```

## 📚 深入理解

### 编译过程

```
源代码(.rs) → 词法分析 → 语法分析 → 语义分析 → 
中间代码 → 优化 → 机器码 → 可执行文件
```

### 内存布局

```rust
fn main() {
    // 栈上的数据
    let x = 42;              // 存储在栈上
    let s = "Hello";         // 字符串字面量存储在只读内存
    
    // 堆上的数据
    let heap_string = String::from("World");  // 存储在堆上
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 使用rustc直接编译Rust程序
- [ ] 使用Cargo创建和管理项目
- [ ] 理解Rust程序的基本结构
- [ ] 掌握println!宏的使用
- [ ] 了解Cargo.toml的基本配置
- [ ] 能够调试简单的编译错误
- [ ] 理解debug和release版本的区别
- [ ] 能够处理用户输入和命令行参数

## 📖 延伸阅读

- [Rust Book - Hello World](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [println!宏文档](https://doc.rust-lang.org/std/macro.println.html)

---

**恭喜！** 🎉 你已经成功创建并运行了第一个Rust程序。

[← 上一节：开发工具选择](./02-dev-tools.md) | [下一节：Cargo包管理器 →](./04-cargo-basics.md)