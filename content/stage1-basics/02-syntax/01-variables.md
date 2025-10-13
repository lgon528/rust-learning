# 2.1 变量和可变性

变量是编程的基础概念，但Rust的变量系统有其独特之处。本节将深入探讨Rust中的变量声明、可变性、作用域和遮蔽等概念。

## 🎯 学习目标

- 理解Rust变量的默认不可变性
- 掌握可变变量的声明和使用
- 了解变量遮蔽（shadowing）机制
- 理解常量与变量的区别
- 掌握变量的作用域规则

## 🔒 变量的不可变性

### 默认不可变

Rust中的变量**默认是不可变的**，这是Rust安全性设计的核心特性之一。

```rust
fn main() {
    let x = 5;
    println!("x的值是: {}", x);
    
    // 这行代码会导致编译错误！
    // x = 6;  // error: cannot assign twice to immutable variable
}
```

**编译错误信息**：
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

### 与其他语言对比

| 语言 | 默认可变性 | 不可变声明 |
|------|------------|------------|
| **Rust** | 不可变 | `let x = 5;` |
| **C/C++** | 可变 | `const int x = 5;` |
| **JavaScript** | 可变 | `const x = 5;` |
| **Python** | 可变 | 无内置支持 |
| **Go** | 可变 | 无内置支持 |

### 不可变性的优势

1. **并发安全**：不可变数据天然线程安全
2. **推理简单**：值不会意外改变
3. **优化机会**：编译器可以进行更多优化
4. **减少bug**：避免意外修改导致的错误

## 🔄 可变变量

### 声明可变变量

使用`mut`关键字声明可变变量：

```rust
fn main() {
    let mut x = 5;
    println!("x的值是: {}", x);
    
    x = 6;  // 现在可以修改了
    println!("x的值是: {}", x);
}
```

### 可变性的传播

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;  // 可变引用
    *y = 10;
    println!("x的值是: {}", x);  // 输出: 10
    
    let z = &x;      // 不可变引用
    // *z = 15;      // 错误：不能通过不可变引用修改
}
```

### 何时使用可变变量

```rust
fn main() {
    // 累加器模式
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }
    println!("总和: {}", sum);
    
    // 状态机模式
    let mut state = "初始状态";
    match some_condition() {
        true => state = "状态A",
        false => state = "状态B",
    }
    
    // 集合操作
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
}

fn some_condition() -> bool {
    true
}
```

## 👥 变量遮蔽（Shadowing）

### 基本遮蔽

```rust
fn main() {
    let x = 5;
    println!("第一个x: {}", x);
    
    let x = x + 1;  // 遮蔽前一个x
    println!("第二个x: {}", x);
    
    {
        let x = x * 2;  // 在内部作用域遮蔽
        println!("内部作用域的x: {}", x);
    }
    
    println!("外部作用域的x: {}", x);
}
```

输出：
```
第一个x: 5
第二个x: 6
内部作用域的x: 12
外部作用域的x: 6
```

### 遮蔽vs可变性

```rust
fn main() {
    // 遮蔽：可以改变类型
    let spaces = "   ";
    let spaces = spaces.len();  // 从&str变为usize
    println!("空格数量: {}", spaces);
    
    // 可变变量：不能改变类型
    let mut spaces2 = "   ";
    // spaces2 = spaces2.len();  // 错误：类型不匹配
}
```

### 遮蔽的应用场景

#### 1. 类型转换

```rust
fn main() {
    let input = "42";
    let input: i32 = input.parse().expect("不是有效数字");
    let input = input * 2;
    println!("结果: {}", input);
}
```

#### 2. 数据处理管道

```rust
fn main() {
    let data = "hello world";
    let data = data.to_uppercase();        // String
    let data = data.replace(" ", "_");      // String
    let data = data.as_bytes();            // &[u8]
    println!("处理后的数据: {:?}", data);
}
```

#### 3. 配置处理

```rust
use std::env;

fn main() {
    let config = "default_value";
    let config = env::var("MY_CONFIG").unwrap_or(config.to_string());
    let config = config.trim();
    println!("最终配置: {}", config);
}
```

## 📏 常量

### 常量声明

```rust
// 全局常量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MAX_POINTS: u32 = 100_000;

fn main() {
    // 局部常量
    const LOCAL_CONST: i32 = 42;
    
    println!("三小时的秒数: {}", THREE_HOURS_IN_SECONDS);
    println!("最大分数: {}", MAX_POINTS);
    println!("局部常量: {}", LOCAL_CONST);
}
```

### 常量vs变量

| 特性 | 常量 | 不可变变量 |
|------|------|------------|
| **关键字** | `const` | `let` |
| **类型注解** | 必须 | 可选 |
| **初始化** | 编译时常量表达式 | 运行时值 |
| **作用域** | 全局或局部 | 局部 |
| **遮蔽** | 不支持 | 支持 |
| **命名约定** | SCREAMING_SNAKE_CASE | snake_case |

```rust
const PI: f64 = 3.14159;           // 编译时常量

fn main() {
    let radius = 5.0;               // 运行时值
    let area = PI * radius * radius;
    
    // const RUNTIME_CONST: f64 = area;  // 错误：不是编译时常量
    
    let pi = 3.14;                  // 可以遮蔽（不同标识符）
    println!("面积: {}", area);
}
```

## 🎯 作用域和生命周期

### 块作用域

```rust
fn main() {
    let x = 1;
    
    {
        let y = 2;
        let x = 3;  // 遮蔽外部的x
        println!("内部: x={}, y={}", x, y);
    }  // y在这里被销毁
    
    println!("外部: x={}", x);  // x恢复为1
    // println!("y={}", y);     // 错误：y不在作用域内
}
```

### 函数作用域

```rust
fn main() {
    let x = 1;
    
    let result = {
        let y = 2;
        x + y  // 表达式，返回值
    };  // y在这里被销毁
    
    println!("结果: {}", result);
}
```

### 循环作用域

```rust
fn main() {
    for i in 0..3 {
        let x = i * 2;  // 每次迭代都创建新的x
        println!("i={}, x={}", i, x);
    }  // i和x都在这里被销毁
    
    // println!("i={}", i);  // 错误：i不在作用域内
}
```

## 🔍 高级概念

### 解构赋值

```rust
fn main() {
    // 元组解构
    let (x, y) = (1, 2);
    println!("x={}, y={}", x, y);
    
    // 数组解构
    let [a, b, c] = [1, 2, 3];
    println!("a={}, b={}, c={}", a, b, c);
    
    // 部分解构
    let (first, .., last) = (1, 2, 3, 4, 5);
    println!("first={}, last={}", first, last);
    
    // 可变解构
    let (mut x, y) = (1, 2);
    x += 10;
    println!("x={}, y={}", x, y);
}
```

### 模式匹配中的变量

```rust
fn main() {
    let value = Some(42);
    
    match value {
        Some(x) => println!("值是: {}", x),  // x绑定到42
        None => println!("没有值"),
    }
    
    // 使用if let
    if let Some(x) = value {
        println!("使用if let: {}", x);
    }
    
    // 守卫条件
    match value {
        Some(x) if x > 40 => println!("大于40: {}", x),
        Some(x) => println!("小于等于40: {}", x),
        None => println!("没有值"),
    }
}
```

## 🧪 实践练习

### 练习1：基础变量操作

```rust
fn main() {
    // TODO: 声明一个不可变变量x，值为10
    
    // TODO: 尝试修改x的值（观察编译错误）
    
    // TODO: 声明一个可变变量y，值为20
    
    // TODO: 修改y的值为30
    
    // TODO: 使用遮蔽重新定义x为字符串"hello"
    
    println!("x: {}, y: {}", x, y);
}
```

### 练习2：作用域实验

```rust
fn main() {
    let x = 1;
    
    {
        // TODO: 在内部作用域声明变量y
        // TODO: 遮蔽外部的x
        // TODO: 打印内部作用域的变量
    }
    
    // TODO: 打印外部作用域的变量
    // TODO: 尝试访问内部作用域的变量（观察编译错误）
}
```

### 练习3：类型转换链

```rust
fn main() {
    // TODO: 从字符串"123"开始
    // TODO: 转换为数字
    // TODO: 乘以2
    // TODO: 转换回字符串
    // TODO: 每一步都使用遮蔽
    
    println!("最终结果: {}", result);
}
```

### 练习4：解构和模式匹配

```rust
fn main() {
    let point = (3, 4);
    let numbers = [1, 2, 3, 4, 5];
    let option_value = Some(42);
    
    // TODO: 解构point获取x和y坐标
    
    // TODO: 解构numbers获取第一个和最后一个元素
    
    // TODO: 使用match处理option_value
    
    // TODO: 计算点到原点的距离
}
```

## 🔧 最佳实践

### 1. 优先使用不可变变量

```rust
// 好的做法
fn calculate_area(radius: f64) -> f64 {
    let pi = 3.14159;
    pi * radius * radius
}

// 避免不必要的可变性
fn bad_example() {
    let mut x = 5;  // 如果x不需要改变，不要使用mut
    println!("x: {}", x);
}
```

### 2. 合理使用遮蔽

```rust
// 好的使用：类型转换
fn parse_input(input: &str) -> Result<i32, std::num::ParseIntError> {
    let input = input.trim();           // &str -> &str
    let input = input.parse::<i32>()?;  // &str -> i32
    Ok(input * 2)                       // 处理后返回
}

// 避免过度遮蔽
fn confusing_example() {
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    let x = x - 1;
    let x = x / 2;  // 太多遮蔽，难以理解
}
```

### 3. 明确的变量命名

```rust
// 好的命名
fn process_user_data() {
    let user_input = get_user_input();
    let cleaned_input = user_input.trim();
    let parsed_number = cleaned_input.parse::<i32>().unwrap();
    let doubled_result = parsed_number * 2;
}

// 避免无意义的名称
fn bad_naming() {
    let x = get_user_input();
    let x = x.trim();
    let x = x.parse::<i32>().unwrap();
    let x = x * 2;
}

fn get_user_input() -> String {
    "42".to_string()
}
```

## 🚨 常见错误

### 1. 忘记mut关键字

```rust
fn main() {
    let x = 5;
    // x = 6;  // 错误：cannot assign twice to immutable variable
    
    // 正确做法
    let mut y = 5;
    y = 6;  // OK
}
```

### 2. 在错误的作用域使用变量

```rust
fn main() {
    let x;
    {
        let y = 5;
        x = y;  // OK：y在赋值时仍在作用域内
    }
    // println!("{}", y);  // 错误：y已经超出作用域
    println!("{}", x);     // OK：x获得了y的值
}
```

### 3. 混淆遮蔽和可变性

```rust
fn main() {
    // 遮蔽：创建新变量
    let x = 5;
    let x = "hello";  // OK：新变量，不同类型
    
    // 可变性：修改现有变量
    let mut y = 5;
    // y = "hello";   // 错误：不能改变类型
    y = 10;           // OK：相同类型
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 理解Rust变量默认不可变的设计理念
- [ ] 正确使用`let`和`let mut`声明变量
- [ ] 理解变量遮蔽的概念和应用场景
- [ ] 区分常量和不可变变量
- [ ] 掌握变量的作用域规则
- [ ] 使用解构赋值和模式匹配
- [ ] 避免常见的变量使用错误
- [ ] 编写清晰、安全的变量操作代码

## 📚 延伸阅读

- [Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Reference - Variables](https://doc.rust-lang.org/reference/variables.html)
- [Rust by Example - Variable Bindings](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)

---

**变量基础掌握完成！** 🎯 你现在理解了Rust独特的变量系统。

[← 上一节：Cargo包管理器](../01-environment/04-cargo-basics.md) | [下一节：数据类型详解 →](./02-data-types.md)