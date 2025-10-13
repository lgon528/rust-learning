# Vector 动态数组

## 学习目标

通过本节学习，你将掌握：

- Vector 的创建和初始化方法
- Vector 的基本操作（增删改查）
- Vector 的内存管理和性能特性
- Vector 的迭代器使用
- Vector 的高级用法和最佳实践

## 基本概念

### 什么是 Vector

Vector（`Vec<T>`）是 Rust 标准库提供的动态数组类型，具有以下特点：

- **动态大小**：可以在运行时增长和缩小
- **连续内存**：元素在内存中连续存储，访问效率高
- **类型安全**：所有元素必须是同一类型
- **所有权管理**：遵循 Rust 的所有权规则

### Vector 的内存布局

```rust
// Vector 在堆上分配内存
// 栈上存储：指针、容量、长度
// 堆上存储：实际数据
let vec = vec![1, 2, 3, 4, 5];
// 栈: [ptr, capacity, length]
// 堆: [1, 2, 3, 4, 5, ...]
```

## Vector 的创建

### 1. 使用 `Vec::new()` 创建空 Vector

```rust
// 创建空的 i32 Vector
let mut numbers: Vec<i32> = Vec::new();

// 类型推断
let mut names = Vec::new();
names.push("Alice".to_string()); // 编译器推断为 Vec<String>
```

### 2. 使用 `vec!` 宏创建

```rust
// 创建包含初始值的 Vector
let numbers = vec![1, 2, 3, 4, 5];

// 创建重复值的 Vector
let zeros = vec![0; 10]; // 10 个 0

// 创建不同类型的 Vector
let mixed: Vec<Box<dyn std::fmt::Display>> = vec![
    Box::new(42),
    Box::new("hello"),
];
```

### 3. 使用 `Vec::with_capacity()` 预分配容量

```rust
// 预分配容量，避免频繁重新分配
let mut numbers = Vec::with_capacity(100);
println!("容量: {}, 长度: {}", numbers.capacity(), numbers.len());
// 输出: 容量: 100, 长度: 0
```

### 4. 从其他集合创建

```rust
// 从数组创建
let arr = [1, 2, 3, 4, 5];
let vec = Vec::from(arr);

// 从迭代器创建
let vec: Vec<i32> = (1..=5).collect();

// 从切片创建
let slice = &[1, 2, 3];
let vec = slice.to_vec();
```

## Vector 的基本操作

### 添加元素

```rust
let mut numbers = Vec::new();

// 在末尾添加元素
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 在指定位置插入元素
numbers.insert(1, 10); // 在索引 1 处插入 10
// numbers: [1, 10, 2, 3]

// 扩展 Vector
let more_numbers = vec![4, 5, 6];
numbers.extend(more_numbers);
// numbers: [1, 10, 2, 3, 4, 5, 6]

// 追加另一个 Vector
let mut other = vec![7, 8, 9];
numbers.append(&mut other);
// numbers: [1, 10, 2, 3, 4, 5, 6, 7, 8, 9]
// other: [] (被清空)
```

### 访问元素

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 使用索引访问（可能 panic）
let third = numbers[2]; // 3

// 使用 get 方法安全访问
match numbers.get(2) {
    Some(value) => println!("第三个元素: {}", value),
    None => println!("索引超出范围"),
}

// 获取第一个和最后一个元素
if let Some(first) = numbers.first() {
    println!("第一个元素: {}", first);
}

if let Some(last) = numbers.last() {
    println!("最后一个元素: {}", last);
}

// 获取可变引用
let mut numbers = vec![1, 2, 3, 4, 5];
if let Some(first_mut) = numbers.first_mut() {
    *first_mut = 10;
}
// numbers: [10, 2, 3, 4, 5]
```

### 修改元素

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

// 直接修改
numbers[0] = 10;

// 使用 get_mut 安全修改
if let Some(value) = numbers.get_mut(1) {
    *value = 20;
}

// 交换元素
numbers.swap(0, 4);
// numbers: [5, 20, 3, 4, 10]

// 反转 Vector
numbers.reverse();
// numbers: [10, 4, 3, 20, 5]

// 排序
numbers.sort();
// numbers: [3, 4, 5, 10, 20]
```

### 删除元素

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

// 删除末尾元素
if let Some(last) = numbers.pop() {
    println!("删除的元素: {}", last); // 5
}

// 删除指定位置的元素
let removed = numbers.remove(1); // 删除索引 1 的元素
println!("删除的元素: {}", removed); // 2
// numbers: [1, 3, 4]

// 删除并替换为最后一个元素（更高效）
let swapped = numbers.swap_remove(0);
println!("删除的元素: {}", swapped); // 1
// numbers: [4, 3]

// 清空 Vector
numbers.clear();
// numbers: []

// 保留满足条件的元素
let mut numbers = vec![1, 2, 3, 4, 5, 6];
numbers.retain(|&x| x % 2 == 0);
// numbers: [2, 4, 6]
```

## Vector 的迭代

### 基本迭代

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 不可变迭代
for number in &numbers {
    println!("{}", number);
}

// 可变迭代
let mut numbers = vec![1, 2, 3, 4, 5];
for number in &mut numbers {
    *number *= 2;
}
// numbers: [2, 4, 6, 8, 10]

// 获取所有权的迭代
for number in numbers {
    println!("{}", number);
}
// numbers 在此后不可用
```

### 迭代器方法

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map: 转换每个元素
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
// doubled: [2, 4, 6, 8, 10]

// filter: 过滤元素
let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
// evens: [&2, &4]

// find: 查找第一个匹配的元素
if let Some(found) = numbers.iter().find(|&&x| x > 3) {
    println!("找到: {}", found); // 4
}

// fold: 累积操作
let sum = numbers.iter().fold(0, |acc, x| acc + x);
println!("总和: {}", sum); // 15

// any/all: 检查条件
let has_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// enumerate: 获取索引和值
for (index, value) in numbers.iter().enumerate() {
    println!("索引 {}: 值 {}", index, value);
}
```

### 链式操作

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 复杂的链式操作
let result: Vec<String> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)        // 过滤偶数
    .map(|x| x * x)                   // 平方
    .filter(|&&x| x > 10)             // 过滤大于 10 的
    .map(|x| format!("数字: {}", x))   // 转换为字符串
    .collect();

println!("{:?}", result); // ["数字: 16", "数字: 36", "数字: 64", "数字: 100"]
```

## Vector 的高级用法

### 容量管理

```rust
let mut numbers = Vec::new();
println!("初始容量: {}", numbers.capacity()); // 0

// 预留容量
numbers.reserve(10);
println!("预留后容量: {}", numbers.capacity()); // >= 10

// 精确预留容量
numbers.reserve_exact(20);
println!("精确预留后容量: {}", numbers.capacity()); // >= 20

// 缩小容量到实际大小
numbers.push(1);
numbers.push(2);
numbers.shrink_to_fit();
println!("缩小后容量: {}", numbers.capacity()); // 接近 2
```

### 切片操作

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 获取切片
let slice = &numbers[2..5]; // [3, 4, 5]
let first_half = &numbers[..5]; // [1, 2, 3, 4, 5]
let second_half = &numbers[5..]; // [6, 7, 8, 9, 10]

// 分割
let (left, right) = numbers.split_at(5);
// left: [1, 2, 3, 4, 5], right: [6, 7, 8, 9, 10]

// 窗口迭代
for window in numbers.windows(3) {
    println!("{:?}", window);
}
// [1, 2, 3]
// [2, 3, 4]
// [3, 4, 5]
// ...

// 块迭代
for chunk in numbers.chunks(3) {
    println!("{:?}", chunk);
}
// [1, 2, 3]
// [4, 5, 6]
// [7, 8, 9]
// [10]
```

### 二分查找

```rust
let numbers = vec![1, 3, 5, 7, 9, 11, 13, 15];

// 二分查找（Vector 必须已排序）
match numbers.binary_search(&7) {
    Ok(index) => println!("找到 7 在索引 {}", index),
    Err(index) => println!("未找到，应插入到索引 {}", index),
}

// 自定义比较的二分查找
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

let mut people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Charlie".to_string(), age: 35 },
];

let target = Person { name: "Bob".to_string(), age: 30 };
match people.binary_search(&target) {
    Ok(index) => println!("找到 Bob 在索引 {}", index),
    Err(_) => println!("未找到 Bob"),
}
```

## 实际应用示例

### 1. 动态缓冲区

```rust
struct Buffer {
    data: Vec<u8>,
    position: usize,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            data: Vec::new(),
            position: 0,
        }
    }
    
    fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
    
    fn read(&mut self, len: usize) -> Option<&[u8]> {
        if self.position + len <= self.data.len() {
            let slice = &self.data[self.position..self.position + len];
            self.position += len;
            Some(slice)
        } else {
            None
        }
    }
    
    fn remaining(&self) -> usize {
        self.data.len() - self.position
    }
}
```

### 2. 简单的栈实现

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}
```

### 3. 批处理系统

```rust
struct BatchProcessor<T> {
    batch: Vec<T>,
    batch_size: usize,
}

impl<T> BatchProcessor<T> {
    fn new(batch_size: usize) -> Self {
        BatchProcessor {
            batch: Vec::with_capacity(batch_size),
            batch_size,
        }
    }
    
    fn add(&mut self, item: T) -> Option<Vec<T>> {
        self.batch.push(item);
        
        if self.batch.len() >= self.batch_size {
            Some(std::mem::replace(&mut self.batch, Vec::with_capacity(self.batch_size)))
        } else {
            None
        }
    }
    
    fn flush(&mut self) -> Vec<T> {
        std::mem::replace(&mut self.batch, Vec::with_capacity(self.batch_size))
    }
}
```

## 性能考虑

### 1. 容量预分配

```rust
// 低效：频繁重新分配
let mut numbers = Vec::new();
for i in 0..1000 {
    numbers.push(i); // 可能触发多次重新分配
}

// 高效：预分配容量
let mut numbers = Vec::with_capacity(1000);
for i in 0..1000 {
    numbers.push(i); // 不会重新分配
}
```

### 2. 批量操作

```rust
// 低效：逐个添加
let mut target = Vec::new();
let source = vec![1, 2, 3, 4, 5];
for item in source {
    target.push(item);
}

// 高效：批量扩展
let mut target = Vec::new();
let source = vec![1, 2, 3, 4, 5];
target.extend(source);
```

### 3. 避免不必要的克隆

```rust
// 低效：不必要的克隆
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x.clone() * 2).collect();

// 高效：直接解引用
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
```

## 最佳实践

### 1. 选择合适的初始化方法

```rust
// 已知大小时使用 with_capacity
let mut buffer = Vec::with_capacity(expected_size);

// 已知内容时使用 vec! 宏
let constants = vec![PI, E, SQRT_2];

// 需要重复值时
let zeros = vec![0.0; matrix_size];
```

### 2. 合理使用迭代器

```rust
// 推荐：使用迭代器链
let result: Vec<_> = data
    .iter()
    .filter(|&&x| x > 0)
    .map(|x| x * 2)
    .collect();

// 避免：手动循环
let mut result = Vec::new();
for &x in &data {
    if x > 0 {
        result.push(x * 2);
    }
}
```

### 3. 错误处理

```rust
// 推荐：使用 get 方法
fn safe_access(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()
}

// 避免：直接索引（可能 panic）
fn unsafe_access(vec: &Vec<i32>, index: usize) -> i32 {
    vec[index] // 可能 panic
}
```

## 常见错误

### 1. 借用检查器问题

```rust
// 错误：同时持有可变和不可变引用
let mut numbers = vec![1, 2, 3];
let first = &numbers[0]; // 不可变借用
numbers.push(4); // 错误：可变借用
println!("{}", first);

// 正确：分离借用作用域
let mut numbers = vec![1, 2, 3];
{
    let first = numbers[0]; // 复制值而不是借用
    numbers.push(4);
    println!("{}", first);
}
```

### 2. 索引越界

```rust
// 错误：可能越界
let numbers = vec![1, 2, 3];
let value = numbers[10]; // panic!

// 正确：安全访问
let numbers = vec![1, 2, 3];
if let Some(value) = numbers.get(10) {
    println!("值: {}", value);
} else {
    println!("索引超出范围");
}
```

### 3. 性能陷阱

```rust
// 低效：频繁的 insert 操作
let mut numbers = vec![1, 2, 3, 4, 5];
for i in 0..1000 {
    numbers.insert(0, i); // O(n) 操作
}

// 高效：使用 VecDeque 或先收集再反转
use std::collections::VecDeque;
let mut numbers = VecDeque::from(vec![1, 2, 3, 4, 5]);
for i in 0..1000 {
    numbers.push_front(i); // O(1) 操作
}
```

## 学习检查清单

- [ ] 理解 Vector 的内存布局和特性
- [ ] 掌握 Vector 的各种创建方法
- [ ] 熟练使用 Vector 的增删改查操作
- [ ] 理解 Vector 的迭代器使用
- [ ] 掌握 Vector 的容量管理
- [ ] 了解 Vector 的性能特性
- [ ] 能够选择合适的 Vector 操作方法
- [ ] 理解常见的性能陷阱和最佳实践

## 扩展阅读

- [Rust 官方文档 - Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Rust By Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [The Rust Programming Language - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust Performance Book - Collections](https://nnethercote.github.io/perf-book/standard-library-types.html)
- [Rust Nomicon - Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)