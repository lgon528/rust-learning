# 2.2 数据类型详解

Rust是一门静态类型语言，这意味着所有变量的类型在编译时都必须已知。本节将详细介绍Rust的类型系统，包括标量类型、复合类型以及类型推断机制。

## 🎯 学习目标

- 掌握Rust的标量类型（整数、浮点数、布尔值、字符）
- 理解复合类型（元组、数组）的使用
- 学会类型注解和类型推断
- 了解类型转换的方法
- 掌握字符串类型的基础知识

## 📊 类型系统概览

Rust的类型系统设计原则：
- **静态类型**：编译时确定所有类型
- **类型安全**：防止类型相关的运行时错误
- **零成本抽象**：类型检查不影响运行时性能
- **表达力强**：丰富的类型表达能力

```rust
fn main() {
    // 显式类型注解
    let x: i32 = 42;
    
    // 类型推断
    let y = 42;  // 编译器推断为i32
    
    // 类型必须明确的情况
    let parsed: u32 = "42".parse().expect("不是数字");
    
    println!("x: {}, y: {}, parsed: {}", x, y, parsed);
}
```

## 🔢 标量类型

标量类型代表单个值，Rust有四种主要的标量类型。

### 整数类型

#### 有符号整数

| 长度 | 有符号 | 范围 |
|------|--------|------|
| 8-bit | `i8` | -128 到 127 |
| 16-bit | `i16` | -32,768 到 32,767 |
| 32-bit | `i32` | -2,147,483,648 到 2,147,483,647 |
| 64-bit | `i64` | -9,223,372,036,854,775,808 到 9,223,372,036,854,775,807 |
| 128-bit | `i128` | -170,141,183,460,469,231,731,687,303,715,884,105,728 到 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| arch | `isize` | 取决于架构（32位或64位） |

#### 无符号整数

| 长度 | 无符号 | 范围 |
|------|--------|------|
| 8-bit | `u8` | 0 到 255 |
| 16-bit | `u16` | 0 到 65,535 |
| 32-bit | `u32` | 0 到 4,294,967,295 |
| 64-bit | `u64` | 0 到 18,446,744,073,709,551,615 |
| 128-bit | `u128` | 0 到 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| arch | `usize` | 取决于架构（32位或64位） |

#### 整数字面量

```rust
fn main() {
    // 十进制
    let decimal = 98_222;        // 下划线提高可读性
    
    // 十六进制
    let hex = 0xff;
    
    // 八进制
    let octal = 0o77;
    
    // 二进制
    let binary = 0b1111_0000;
    
    // 字节（仅限u8）
    let byte = b'A';
    
    // 类型后缀
    let typed_int = 42u32;
    let another_typed = 100_i64;
    
    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);
    println!("typed_int: {}", typed_int);
    println!("another_typed: {}", another_typed);
}
```

#### 整数溢出

```rust
fn main() {
    // Debug模式下会panic，Release模式下会环绕
    let mut x: u8 = 255;
    println!("x = {}", x);
    
    // 显式处理溢出
    match x.checked_add(1) {
        Some(result) => println!("结果: {}", result),
        None => println!("溢出了！"),
    }
    
    // 环绕加法
    let wrapped = x.wrapping_add(1);
    println!("环绕结果: {}", wrapped);  // 0
    
    // 饱和加法
    let saturated = x.saturating_add(1);
    println!("饱和结果: {}", saturated);  // 255
    
    // 溢出加法（返回结果和是否溢出）
    let (result, overflowed) = x.overflowing_add(1);
    println!("结果: {}, 是否溢出: {}", result, overflowed);
}
```

### 浮点数类型

```rust
fn main() {
    // f32：单精度浮点数
    let x: f32 = 3.14159;
    
    // f64：双精度浮点数（默认）
    let y = 2.71828;  // 默认为f64
    let z: f64 = 1.41421;
    
    // 科学记数法
    let scientific = 1e6;      // 1,000,000.0
    let small = 1e-6;          // 0.000001
    
    // 特殊值
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("scientific: {}", scientific);
    println!("small: {}", small);
    println!("infinity: {}", infinity);
    println!("neg_infinity: {}", neg_infinity);
    println!("nan: {}", nan);
    
    // 浮点数比较
    let a = 0.1 + 0.2;
    let b = 0.3;
    println!("a: {}, b: {}", a, b);
    println!("a == b: {}", a == b);  // false！
    
    // 正确的浮点数比较
    let epsilon = f64::EPSILON;
    println!("近似相等: {}", (a - b).abs() < epsilon);
}
```

### 布尔类型

```rust
fn main() {
    let t = true;
    let f: bool = false;  // 显式类型注解
    
    // 布尔运算
    let and_result = t && f;    // false
    let or_result = t || f;     // true
    let not_result = !t;        // false
    
    println!("t: {}, f: {}", t, f);
    println!("t && f: {}", and_result);
    println!("t || f: {}", or_result);
    println!("!t: {}", not_result);
    
    // 布尔值在条件语句中的使用
    if t {
        println!("t是真的");
    }
    
    // 布尔值转换
    let bool_as_int = t as i32;  // true -> 1, false -> 0
    println!("布尔值转整数: {}", bool_as_int);
    
    // 比较运算产生布尔值
    let comparison = 5 > 3;
    println!("5 > 3: {}", comparison);
}
```

### 字符类型

```rust
fn main() {
    // char类型：4字节Unicode标量值
    let c = 'z';
    let z: char = 'ℤ';  // 数学符号
    let heart_eyed_cat = '😻';  // emoji
    let chinese = '中';
    
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    println!("chinese: {}", chinese);
    
    // 字符的Unicode值
    println!("'A'的Unicode值: {}", 'A' as u32);
    println!("'中'的Unicode值: {}", '中' as u32);
    
    // 转义字符
    let newline = '\n';
    let tab = '\t';
    let backslash = '\\';
    let single_quote = '\'';
    
    println!("转义字符演示:");
    print!("第一行{}第二行", newline);
    print!("制表符{}对齐", tab);
    println!("反斜杠: {}", backslash);
    println!("单引号: {}", single_quote);
    
    // Unicode转义
    let unicode_char = '\u{1F60A}';  // 😊
    println!("Unicode转义: {}", unicode_char);
}
```

## 📦 复合类型

复合类型可以将多个值组合成一个类型。

### 元组类型

```rust
fn main() {
    // 基本元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // 解构元组
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // 通过索引访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    
    println!("索引访问: {}, {}, {}", five_hundred, six_point_four, one);
    
    // 单元类型（空元组）
    let unit = ();
    println!("单元类型: {:?}", unit);
    
    // 嵌套元组
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("嵌套元组: a={}, b={}, c={}, d={}", a, b, c, d);
    
    // 不同类型的元组
    let mixed = ("hello", 42, true, 3.14);
    println!("混合类型元组: {:?}", mixed);
    
    // 元组作为函数返回值
    let (sum, product) = calculate(10, 20);
    println!("和: {}, 积: {}", sum, product);
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
```

### 数组类型

```rust
fn main() {
    // 基本数组
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];
    
    // 显式类型和长度
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 初始化相同值
    let c = [3; 5];  // [3, 3, 3, 3, 3]
    
    println!("数组a: {:?}", a);
    println!("数组b: {:?}", b);
    println!("数组c: {:?}", c);
    
    // 访问数组元素
    let first = a[0];
    let second = a[1];
    println!("第一个元素: {}, 第二个元素: {}", first, second);
    
    // 数组长度
    println!("数组a的长度: {}", a.len());
    println!("months数组的长度: {}", months.len());
    
    // 数组切片
    let slice = &a[1..4];  // [2, 3, 4]
    println!("切片: {:?}", slice);
    
    // 遍历数组
    println!("遍历数组a:");
    for element in &a {
        println!("元素: {}", element);
    }
    
    // 带索引遍历
    println!("带索引遍历:");
    for (index, element) in a.iter().enumerate() {
        println!("索引 {}: 值 {}", index, element);
    }
    
    // 数组边界检查
    let index = 10;
    match a.get(index) {
        Some(value) => println!("索引 {} 的值: {}", index, value),
        None => println!("索引 {} 超出边界", index),
    }
}
```

## 🔄 类型转换

### 显式类型转换（as）

```rust
fn main() {
    // 数值类型转换
    let a = 42i32;
    let b = a as f64;      // i32 -> f64
    let c = b as u32;      // f64 -> u32
    
    println!("a: {}, b: {}, c: {}", a, b, c);
    
    // 可能丢失精度的转换
    let large_number = 300u16;
    let small_number = large_number as u8;  // 可能溢出
    println!("large: {}, small: {}", large_number, small_number);
    
    // 字符和数值转换
    let char_a = 'A';
    let ascii_value = char_a as u8;
    println!("字符 '{}' 的ASCII值: {}", char_a, ascii_value);
    
    // 布尔值转换
    let bool_val = true;
    let int_val = bool_val as i32;
    println!("布尔值 {} 转为整数: {}", bool_val, int_val);
}
```

### 安全类型转换

```rust
use std::convert::TryFrom;

fn main() {
    // 使用TryFrom进行安全转换
    let a: i32 = 42;
    
    match u8::try_from(a) {
        Ok(b) => println!("转换成功: {}", b),
        Err(e) => println!("转换失败: {}", e),
    }
    
    // 转换可能失败的情况
    let large: i32 = 300;
    match u8::try_from(large) {
        Ok(b) => println!("转换成功: {}", b),
        Err(e) => println!("转换失败: {}", e),
    }
    
    // 字符串解析
    let s = "42";
    match s.parse::<i32>() {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }
    
    let invalid = "not_a_number";
    match invalid.parse::<i32>() {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }
}
```

## 📝 字符串类型基础

```rust
fn main() {
    // 字符串字面量（&str）
    let s1 = "hello";
    let s2: &str = "world";
    
    // String类型
    let s3 = String::from("hello");
    let s4 = "world".to_string();
    
    println!("s1: {}, s2: {}", s1, s2);
    println!("s3: {}, s4: {}", s3, s4);
    
    // 字符串拼接
    let combined = format!("{} {}", s1, s2);
    println!("拼接结果: {}", combined);
    
    // 字符串长度
    println!("s1长度: {} 字节", s1.len());
    println!("s1长度: {} 字符", s1.chars().count());
    
    // 中文字符串
    let chinese = "你好世界";
    println!("中文字符串: {}", chinese);
    println!("字节长度: {}", chinese.len());        // 12字节
    println!("字符长度: {}", chinese.chars().count()); // 4字符
    
    // 字符串切片（小心Unicode边界）
    let hello = "Hello";
    let slice = &hello[0..2];  // "He"
    println!("切片: {}", slice);
    
    // 遍历字符
    println!("遍历字符:");
    for c in chinese.chars() {
        println!("字符: {}", c);
    }
}
```

## 🔍 类型推断和注解

### 类型推断

```rust
fn main() {
    // 编译器可以推断的情况
    let x = 42;          // 推断为i32
    let y = 3.14;        // 推断为f64
    let z = true;        // 推断为bool
    let s = "hello";     // 推断为&str
    
    // 从使用上下文推断
    let mut vec = Vec::new();  // 类型未知
    vec.push(42);              // 现在推断为Vec<i32>
    
    // 从函数返回类型推断
    let parsed = "42".parse().expect("解析失败");  // 需要类型注解
    let parsed: i32 = "42".parse().expect("解析失败");  // 明确类型
    
    println!("推断的类型值: {}, {}, {}, {}", x, y, z, s);
    println!("向量: {:?}", vec);
    println!("解析结果: {}", parsed);
}
```

### 类型注解

```rust
fn main() {
    // 必须使用类型注解的情况
    
    // 1. 多种可能的类型
    let parsed: u32 = "42".parse().expect("解析失败");
    
    // 2. 集合类型
    let numbers: Vec<i32> = Vec::new();
    
    // 3. 函数参数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 4. 复杂类型
    let complex: (i32, Vec<String>, bool) = (42, vec!["hello".to_string()], true);
    
    // 5. 泛型类型参数
    let result: Result<i32, std::num::ParseIntError> = "42".parse();
    
    println!("解析结果: {}", parsed);
    println!("数字向量: {:?}", numbers);
    println!("加法结果: {}", add(10, 20));
    println!("复杂类型: {:?}", complex);
    println!("Result类型: {:?}", result);
}
```

## 🧪 实践练习

### 练习1：类型探索

```rust
fn main() {
    // TODO: 声明不同类型的变量
    // 整数类型：i8, i16, i32, i64, u8, u16, u32, u64
    // 浮点类型：f32, f64
    // 布尔类型：bool
    // 字符类型：char
    
    // TODO: 打印每个变量的值和类型大小
    // 提示：使用std::mem::size_of::<T>()
}
```

### 练习2：数组和元组操作

```rust
fn main() {
    // TODO: 创建一个包含5个整数的数组
    
    // TODO: 创建一个包含姓名、年龄、身高的元组
    
    // TODO: 计算数组所有元素的和
    
    // TODO: 解构元组并打印每个字段
    
    // TODO: 创建一个二维数组（数组的数组）
}
```

### 练习3：类型转换练习

```rust
fn main() {
    let numbers = ["42", "3.14", "true", "hello"];
    
    // TODO: 尝试将每个字符串转换为不同类型
    // 整数、浮点数、布尔值
    // 处理转换失败的情况
    
    // TODO: 实现一个函数，安全地将f64转换为i32
    
    // TODO: 实现温度转换（摄氏度 <-> 华氏度）
}
```

### 练习4：字符串处理

```rust
fn main() {
    let text = "Rust编程语言";
    
    // TODO: 统计字符串的字节长度和字符长度
    
    // TODO: 遍历字符串的每个字符
    
    // TODO: 提取字符串的一部分（注意Unicode边界）
    
    // TODO: 将字符串转换为大写/小写
}
```

## 🔧 最佳实践

### 1. 选择合适的整数类型

```rust
// 好的做法：根据用途选择类型
fn good_integer_usage() {
    let age: u8 = 25;           // 年龄不会超过255
    let population: u64 = 1_000_000;  // 人口数可能很大
    let index: usize = 0;       // 数组索引使用usize
    let temperature: i16 = -40; // 温度可能为负
}

// 避免：不必要的大类型
fn avoid_oversized_types() {
    let small_number: i64 = 5;  // 浪费内存
    let counter: i128 = 0;      // 过度设计
}
```

### 2. 浮点数比较

```rust
fn floating_point_comparison() {
    let a = 0.1 + 0.2;
    let b = 0.3;
    
    // 错误的比较
    // if a == b { ... }  // 可能失败
    
    // 正确的比较
    const EPSILON: f64 = 1e-10;
    if (a - b).abs() < EPSILON {
        println!("数值近似相等");
    }
    
    // 或使用专门的库
    // use approx::assert_relative_eq;
    // assert_relative_eq!(a, b, epsilon = 1e-10);
}
```

### 3. 类型注解的使用

```rust
fn type_annotation_best_practices() {
    // 明确意图时使用类型注解
    let user_id: u64 = 12345;
    let price: f32 = 19.99;  // 明确使用单精度
    
    // 避免冗余的类型注解
    let x: i32 = 42;  // 冗余，编译器可以推断
    let y = 42;       // 更简洁
    
    // 复杂类型时使用注解提高可读性
    let config: HashMap<String, Vec<i32>> = HashMap::new();
}

use std::collections::HashMap;
```

### 4. 数组vs向量的选择

```rust
fn array_vs_vector() {
    // 使用数组：大小固定，编译时已知
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    
    // 使用向量：大小可变，运行时确定
    let mut shopping_list = Vec::new();
    shopping_list.push("苹果");
    shopping_list.push("香蕉");
    
    // 性能考虑：数组在栈上，向量在堆上
    let stack_array = [1, 2, 3, 4, 5];     // 栈分配，快速
    let heap_vector = vec![1, 2, 3, 4, 5]; // 堆分配，灵活
}
```

## 🚨 常见错误

### 1. 整数溢出

```rust
fn integer_overflow_examples() {
    let mut x: u8 = 255;
    
    // 在debug模式下会panic
    // x += 1;  // 溢出！
    
    // 正确处理溢出
    match x.checked_add(1) {
        Some(result) => println!("结果: {}", result),
        None => println!("溢出了，使用默认值"),
    }
}
```

### 2. 浮点数精度问题

```rust
fn floating_point_precision() {
    let x = 0.1;
    let y = 0.2;
    let z = 0.3;
    
    // 错误：直接比较
    if x + y == z {
        println!("相等");  // 可能不会执行
    }
    
    // 正确：使用epsilon比较
    if (x + y - z).abs() < f64::EPSILON {
        println!("近似相等");
    }
}
```

### 3. 数组越界

```rust
fn array_bounds_checking() {
    let arr = [1, 2, 3, 4, 5];
    
    // 编译时已知的越界会被检测
    // let x = arr[10];  // 编译错误
    
    // 运行时越界会panic
    let index = 10;
    // let y = arr[index];  // 运行时panic
    
    // 安全的访问方式
    match arr.get(index) {
        Some(value) => println!("值: {}", value),
        None => println!("索引越界"),
    }
}
```

### 4. 字符串切片边界

```rust
fn string_slicing_errors() {
    let s = "你好世界";
    
    // 错误：可能切在Unicode字符中间
    // let slice = &s[0..2];  // 可能panic
    
    // 正确：使用字符边界
    let mut char_indices = s.char_indices();
    if let Some((_, _)) = char_indices.next() {
        if let Some((end_idx, _)) = char_indices.next() {
            let slice = &s[0..end_idx];
            println!("安全切片: {}", slice);
        }
    }
    
    // 或者使用chars()迭代器
    let first_two_chars: String = s.chars().take(2).collect();
    println!("前两个字符: {}", first_two_chars);
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 理解并使用所有标量类型（整数、浮点数、布尔值、字符）
- [ ] 掌握复合类型（元组、数组）的创建和操作
- [ ] 正确使用类型注解和理解类型推断
- [ ] 安全地进行类型转换
- [ ] 处理整数溢出和浮点数精度问题
- [ ] 避免数组越界和字符串切片错误
- [ ] 选择合适的数据类型
- [ ] 理解不同类型的内存布局和性能特征

## 📚 延伸阅读

- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [The Rustonomicon - Data Layout](https://doc.rust-lang.org/nomicon/data.html)

---

**数据类型掌握完成！** 🎯 你现在理解了Rust的类型系统基础。

[← 上一节：变量和可变性](./01-variables.md) | [下一节：函数定义和调用 →](./03-functions.md)