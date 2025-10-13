# 泛型基础

## 学习目标

通过本节学习，你将掌握：

- 理解泛型的概念和作用
- 掌握泛型函数的定义和使用
- 掌握泛型结构体和枚举的定义
- 理解泛型的约束和限制
- 掌握泛型的性能特点
- 学会在实际项目中应用泛型

## 基本概念

### 什么是泛型

泛型（Generics）是一种编程技术，允许我们编写可以处理多种类型的代码，而不需要为每种类型重复编写相同的逻辑。

**核心特点：**
- **类型参数化**：使用类型参数代替具体类型
- **编译时单态化**：编译器为每种具体类型生成专门的代码
- **零成本抽象**：运行时性能与手写的具体类型代码相同
- **类型安全**：编译时检查类型正确性

### 为什么需要泛型

```rust
// 没有泛型时，需要为每种类型写重复代码
fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// 使用泛型，一个函数处理多种类型
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## 泛型函数

### 基本语法

```rust
// 泛型函数的基本形式
fn function_name<T>(parameter: T) -> T {
    // 函数体
}

// 多个类型参数
fn function_name<T, U>(param1: T, param2: U) -> T {
    // 函数体
}
```

### 实际示例

```rust
// 交换两个值
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

// 获取切片的第一个元素
fn first<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}

// 创建包含两个相同元素的向量
fn duplicate<T: Clone>(item: T) -> Vec<T> {
    vec![item.clone(), item]
}

// 使用示例
fn main() {
    let mut x = 5;
    let mut y = 10;
    swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y); // x: 10, y: 5
    
    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(first_num) = first(&numbers) {
        println!("第一个数字: {}", first_num);
    }
    
    let duplicated = duplicate("hello".to_string());
    println!("{:?}", duplicated); // ["hello", "hello"]
}
```

## 泛型结构体

### 基本定义

```rust
// 单个类型参数
struct Point<T> {
    x: T,
    y: T,
}

// 多个类型参数
struct Pair<T, U> {
    first: T,
    second: U,
}

// 使用示例
fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    let pair = Pair {
        first: "hello",
        second: 42,
    };
}
```

### 泛型结构体的方法

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 构造函数
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    // 获取 x 坐标的引用
    fn x(&self) -> &T {
        &self.x
    }
    
    // 获取 y 坐标的引用
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 混合泛型方法
impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> Point<T> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

## 泛型枚举

### 标准库中的泛型枚举

```rust
// Option<T> - 可能有值或没有值
enum Option<T> {
    Some(T),
    None,
}

// Result<T, E> - 成功或失败
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 自定义泛型枚举

```rust
// 二叉树节点
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

// 链表节点
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// 使用示例
fn main() {
    let tree: BinaryTree<i32> = BinaryTree::Node {
        value: 5,
        left: Box::new(BinaryTree::Empty),
        right: Box::new(BinaryTree::Empty),
    };
    
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
                Box::new(List::Nil))))));
}
```

## 泛型约束

### Trait 约束

```rust
use std::fmt::Display;

// 要求类型实现 Display trait
fn print_it<T: Display>(item: T) {
    println!("{}", item);
}

// 多个约束
fn compare_and_print<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} 大于 {}", a, b);
    } else {
        println!("{} 小于等于 {}", a, b);
    }
}

// where 子句（更清晰的语法）
fn complex_function<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: Display + Debug,
{
    format!("a: {}, b: {:?}", a, b)
}
```

### 生命周期约束

```rust
// 泛型类型必须满足生命周期约束
fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if x > y { x } else { y }
}

// 结构体中的生命周期约束
struct Holder<'a, T> {
    value: &'a T,
}

impl<'a, T> Holder<'a, T>
where
    T: Display,
{
    fn print(&self) {
        println!("{}", self.value);
    }
}
```

## 实际应用示例

### 泛型容器

```rust
// 自定义栈实现
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
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

// 使用示例
fn main() {
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    while let Some(item) = int_stack.pop() {
        println!("弹出: {}", item);
    }
    
    let mut string_stack = Stack::new();
    string_stack.push("hello".to_string());
    string_stack.push("world".to_string());
}
```

### 泛型算法

```rust
// 泛型排序函数
fn bubble_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 泛型搜索函数
fn binary_search<T: PartialOrd>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] == target {
            return Some(mid);
        } else if &arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    None
}

// 使用示例
fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut numbers);
    println!("排序后: {:?}", numbers);
    
    if let Some(index) = binary_search(&numbers, &25) {
        println!("找到 25 在索引 {}", index);
    }
}
```

## 性能考虑

### 单态化（Monomorphization）

```rust
// 这个泛型函数
fn generic_function<T>(x: T) -> T {
    x
}

// 编译器会为每种使用的类型生成专门的版本
// fn generic_function_i32(x: i32) -> i32 { x }
// fn generic_function_String(x: String) -> String { x }

fn main() {
    let integer = generic_function(5i32);
    let string = generic_function(String::from("hello"));
}
```

### 编译时间 vs 运行时性能

```rust
// 优点：运行时零成本
// 缺点：可能增加编译时间和二进制大小

// 考虑使用 trait 对象来减少代码膨胀
use std::fmt::Display;

// 泛型版本 - 编译时单态化
fn print_generic<T: Display>(item: T) {
    println!("{}", item);
}

// trait 对象版本 - 运行时动态分发
fn print_dynamic(item: &dyn Display) {
    println!("{}", item);
}
```

## 最佳实践

### 1. 合理使用约束

```rust
// 好的做法：只添加必要的约束
fn process_data<T: Clone>(data: T) -> T {
    data.clone()
}

// 避免过度约束
// fn process_data<T: Clone + Display + Debug + PartialEq>(data: T) -> T {
//     data.clone() // 只用到了 Clone
// }
```

### 2. 使用有意义的类型参数名

```rust
// 好的做法
struct Database<Connection, Query, Result> {
    connection: Connection,
    _phantom: std::marker::PhantomData<(Query, Result)>,
}

// 避免无意义的名称
// struct Database<T, U, V> { ... }
```

### 3. 考虑默认类型参数

```rust
// 为常用情况提供默认类型
struct MyVec<T, A = std::alloc::Global> {
    data: Vec<T>,
    allocator: A,
}

// 大多数情况下可以简单使用
type IntVec = MyVec<i32>; // 使用默认分配器
```

## 常见错误

### 1. 忘记添加必要的约束

```rust
// 错误：T 没有实现 PartialOrd
// fn max<T>(a: T, b: T) -> T {
//     if a > b { a } else { b } // 编译错误
// }

// 正确：添加约束
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### 2. 类型推断问题

```rust
// 可能需要显式指定类型
fn main() {
    let numbers = vec![1, 2, 3];
    
    // 编译器可能无法推断类型
    // let collected = numbers.iter().collect(); // 错误
    
    // 显式指定类型
    let collected: Vec<&i32> = numbers.iter().collect();
    
    // 或者使用 turbofish 语法
    let collected = numbers.iter().collect::<Vec<&i32>>();
}
```

### 3. 生命周期和泛型的混淆

```rust
// 注意生命周期参数的位置
struct Wrapper<'a, T> {
    value: &'a T,
}

// 实现时要保持一致
impl<'a, T> Wrapper<'a, T> {
    fn new(value: &'a T) -> Self {
        Wrapper { value }
    }
}
```

## 学习检查清单

- [ ] 理解泛型的基本概念和作用
- [ ] 能够定义和使用泛型函数
- [ ] 能够定义泛型结构体和枚举
- [ ] 掌握泛型约束的使用
- [ ] 理解单态化的概念
- [ ] 能够在实际项目中合理使用泛型
- [ ] 了解泛型的性能特点
- [ ] 能够避免常见的泛型使用错误

## 扩展阅读

- [Rust Book - Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust Reference - Generics](https://doc.rust-lang.org/reference/items/generics.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [The Rustonomicon - Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)