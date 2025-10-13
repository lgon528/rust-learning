# 2.3 函数定义和调用

函数是Rust程序的基本构建块。本节将详细介绍Rust中函数的定义、调用、参数传递、返回值以及一些高级特性。

## 🎯 学习目标

- 掌握函数的定义和调用语法
- 理解参数传递机制
- 学会使用返回值和表达式
- 了解函数的作用域和生命周期
- 掌握高阶函数和闭包基础

## 📝 函数基础

### 函数定义语法

```rust
// 基本函数定义
fn function_name(parameter: Type) -> ReturnType {
    // 函数体
    // 返回值（表达式或return语句）
}

// 示例
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let greeting = greet("Alice");
    println!("{}", greeting);
}
```

### 函数命名约定

```rust
// 好的函数命名（snake_case）
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn get_user_input() -> String {
    // 实现获取用户输入
    String::from("user input")
}

// 动词开头的函数名
fn process_data(data: &[i32]) -> Vec<i32> {
    data.iter().map(|x| x * 2).collect()
}

fn validate_email(email: &str) -> bool {
    email.contains('@')
}

fn main() {
    let area = calculate_area(10.0, 20.0);
    println!("面积: {}", area);
    
    println!("4是偶数: {}", is_even(4));
    println!("5是偶数: {}", is_even(5));
    
    let input = get_user_input();
    println!("用户输入: {}", input);
    
    let numbers = [1, 2, 3, 4, 5];
    let processed = process_data(&numbers);
    println!("处理后的数据: {:?}", processed);
    
    println!("邮箱有效: {}", validate_email("user@example.com"));
}
```

## 📥 参数传递

### 值传递

```rust
fn take_ownership(s: String) {
    println!("函数内部: {}", s);
    // s在函数结束时被销毁
}

fn copy_value(x: i32) {
    println!("复制的值: {}", x);
    // i32实现了Copy trait，所以是复制而不是移动
}

fn main() {
    let s = String::from("hello");
    take_ownership(s);  // s的所有权移动到函数内
    // println!("{}", s);  // 错误：s已经被移动
    
    let x = 5;
    copy_value(x);      // x被复制
    println!("x仍然可用: {}", x);  // OK：x仍然有效
}
```

### 引用传递

```rust
fn borrow_string(s: &String) -> usize {
    s.len()  // 借用s，不获取所有权
}

fn borrow_str(s: &str) -> usize {
    s.len()  // 更通用的字符串切片
}

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let s = String::from("hello");
    
    // 不可变借用
    let len = borrow_string(&s);
    println!("字符串长度: {}", len);
    println!("原字符串: {}", s);  // s仍然可用
    
    // 字符串切片更通用
    let len2 = borrow_str(&s);
    let len3 = borrow_str("literal string");
    println!("长度: {}, {}", len2, len3);
    
    // 可变借用
    let mut s2 = String::from("hello");
    modify_string(&mut s2);
    println!("修改后: {}", s2);
}
```

### 多个参数

```rust
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn calculate_circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn format_person_info(name: &str, age: u32, city: &str) -> String {
    format!("{} is {} years old and lives in {}", name, age, city)
}

// 使用元组传递多个相关参数
fn calculate_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// 使用结构体传递复杂参数
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn calculate_struct_area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

fn main() {
    println!("矩形面积: {}", calculate_rectangle_area(10.0, 20.0));
    println!("圆形面积: {:.2}", calculate_circle_area(5.0));
    
    let info = format_person_info("Alice", 30, "Beijing");
    println!("{}", info);
    
    let p1 = (0.0, 0.0);
    let p2 = (3.0, 4.0);
    println!("距离: {}", calculate_distance(p1, p2));
    
    let rect = Rectangle { width: 10.0, height: 20.0 };
    println!("结构体面积: {}", calculate_struct_area(&rect));
}
```

## 📤 返回值

### 表达式返回

```rust
// 表达式返回（推荐）
fn add(a: i32, b: i32) -> i32 {
    a + b  // 没有分号，这是一个表达式
}

// return语句
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;  // 显式return
}

// 条件返回
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a  // 表达式
    } else {
        b  // 表达式
    }
}

// 复杂逻辑的返回
fn classify_number(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 0 && n <= 10 => "small positive",
        _ => "large positive",
    }
}

fn main() {
    println!("加法: {}", add(5, 3));
    println!("减法: {}", subtract(10, 4));
    println!("最大值: {}", max(15, 8));
    println!("分类: {}", classify_number(-5));
    println!("分类: {}", classify_number(0));
    println!("分类: {}", classify_number(7));
    println!("分类: {}", classify_number(100));
}
```

### 多返回值（元组）

```rust
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn get_name_and_age() -> (String, u32) {
    (String::from("Alice"), 30)
}

fn analyze_string(s: &str) -> (usize, usize, bool) {
    let char_count = s.chars().count();
    let byte_count = s.len();
    let has_spaces = s.contains(' ');
    (char_count, byte_count, has_spaces)
}

// 解构返回值
fn main() {
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
    
    let (name, age) = get_name_and_age();
    println!("姓名: {}, 年龄: {}", name, age);
    
    let text = "Hello, 世界!";
    let (chars, bytes, spaces) = analyze_string(text);
    println!("字符数: {}, 字节数: {}, 包含空格: {}", chars, bytes, spaces);
}
```

### Option和Result返回

```rust
// 可能失败的操作返回Option
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

// 可能出错的操作返回Result
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// 字符串解析
fn parse_positive_integer(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(num) if num > 0 => Ok(num),
        Ok(_) => Err(String::from("数字必须为正数")),
        Err(_) => Err(String::from("无效的数字格式")),
    }
}

fn main() {
    let numbers = [1, 3, 5, 8, 9, 12];
    match find_first_even(&numbers) {
        Some(even) => println!("第一个偶数: {}", even),
        None => println!("没有找到偶数"),
    }
    
    match safe_divide(10.0, 3.0) {
        Ok(result) => println!("除法结果: {:.2}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("除法结果: {:.2}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    let inputs = ["42", "0", "-5", "abc"];
    for input in &inputs {
        match parse_positive_integer(input) {
            Ok(num) => println!("解析成功: {}", num),
            Err(error) => println!("解析 '{}' 失败: {}", input, error),
        }
    }
}
```

## 🔄 函数作为值

### 函数指针

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 接受函数作为参数
fn apply_operation(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

// 返回函数指针
fn get_operation(op_type: &str) -> fn(i32, i32) -> i32 {
    match op_type {
        "add" => add,
        "multiply" => multiply,
        _ => add,  // 默认操作
    }
}

fn main() {
    let result1 = apply_operation(5, 3, add);
    let result2 = apply_operation(5, 3, multiply);
    
    println!("加法结果: {}", result1);
    println!("乘法结果: {}", result2);
    
    let op = get_operation("multiply");
    let result3 = op(4, 6);
    println!("动态操作结果: {}", result3);
    
    // 函数指针数组
    let operations: [fn(i32, i32) -> i32; 2] = [add, multiply];
    for (i, op) in operations.iter().enumerate() {
        let result = op(10, 2);
        println!("操作 {} 结果: {}", i, result);
    }
}
```

### 闭包基础

```rust
fn main() {
    // 基本闭包
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // 显式类型注解的闭包
    let multiply: fn(i32, i32) -> i32 = |a, b| a * b;
    println!("3 * 4 = {}", multiply(3, 4));
    
    // 捕获环境变量
    let factor = 10;
    let scale = |x| x * factor;  // 捕获factor
    println!("5 * {} = {}", factor, scale(5));
    
    // 可变捕获
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    
    println!("计数器: {}", increment());
    println!("计数器: {}", increment());
    println!("计数器: {}", increment());
    
    // 闭包作为参数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("翻倍: {:?}", doubled);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
}
```

## 🎯 高级函数特性

### 泛型函数

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 多个泛型参数
fn pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// 泛型函数与trait约束
fn print_debug<T: std::fmt::Debug>(item: T) {
    println!("Debug: {:?}", item);
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("最大的数字: {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("最大的字符: {}", result);
    
    let p1 = pair("hello", 42);
    let p2 = pair(3.14, true);
    println!("配对1: {:?}", p1);
    println!("配对2: {:?}", p2);
    
    print_debug("Hello");
    print_debug(vec![1, 2, 3]);
    print_debug(("tuple", 42));
}
```

### 递归函数

```rust
// 阶乘
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// 斐波那契数列
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 尾递归优化版本的斐波那契
fn fibonacci_tail_recursive(n: u32) -> u64 {
    fn fib_helper(n: u32, a: u64, b: u64) -> u64 {
        match n {
            0 => a,
            _ => fib_helper(n - 1, b, a + b),
        }
    }
    fib_helper(n, 0, 1)
}

// 二分查找（递归版本）
fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left > right {
        return None;
    }
    
    let mid = left + (right - left) / 2;
    
    match arr[mid].cmp(&target) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Greater => {
            if mid == 0 {
                None
            } else {
                binary_search_recursive(arr, target, left, mid - 1)
            }
        },
        std::cmp::Ordering::Less => binary_search_recursive(arr, target, mid + 1, right),
    }
}

fn main() {
    // 测试阶乘
    for i in 0..=10 {
        println!("{}! = {}", i, factorial(i));
    }
    
    // 测试斐波那契
    println!("\n斐波那契数列:");
    for i in 0..=10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
    
    // 测试尾递归版本
    println!("\n尾递归斐波那契:");
    for i in 0..=20 {
        println!("fib_tail({}) = {}", i, fibonacci_tail_recursive(i));
    }
    
    // 测试二分查找
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 7;
    match binary_search_recursive(&arr, target, 0, arr.len() - 1) {
        Some(index) => println!("\n找到 {} 在索引 {}", target, index),
        None => println!("\n未找到 {}", target),
    }
}
```

## 🧪 实践练习

### 练习1：基础函数编写

```rust
fn main() {
    // TODO: 实现以下函数
    
    // 1. 计算两个数的最大公约数
    fn gcd(a: u32, b: u32) -> u32 {
        // 使用欧几里得算法
        todo!()
    }
    
    // 2. 判断一个数是否为质数
    fn is_prime(n: u32) -> bool {
        todo!()
    }
    
    // 3. 计算数组的平均值
    fn average(numbers: &[f64]) -> Option<f64> {
        // 空数组返回None
        todo!()
    }
    
    // 4. 反转字符串
    fn reverse_string(s: &str) -> String {
        todo!()
    }
    
    // 测试你的实现
    println!("gcd(48, 18) = {}", gcd(48, 18));
    println!("is_prime(17) = {}", is_prime(17));
    println!("is_prime(18) = {}", is_prime(18));
    
    let nums = [1.0, 2.0, 3.0, 4.0, 5.0];
    if let Some(avg) = average(&nums) {
        println!("平均值: {}", avg);
    }
    
    println!("反转 'hello': {}", reverse_string("hello"));
}
```

### 练习2：高阶函数

```rust
fn main() {
    // TODO: 实现一个通用的映射函数
    fn map_vec<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        todo!()
    }
    
    // TODO: 实现一个过滤函数
    fn filter_vec<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        todo!()
    }
    
    // TODO: 实现一个折叠（reduce）函数
    fn fold_vec<T, U, F>(vec: Vec<T>, init: U, f: F) -> U
    where
        F: Fn(U, T) -> U,
    {
        todo!()
    }
    
    // 测试你的实现
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled = map_vec(numbers.clone(), |x| x * 2);
    println!("翻倍: {:?}", doubled);
    
    let evens = filter_vec(numbers.clone(), |&x| x % 2 == 0);
    println!("偶数: {:?}", evens);
    
    let sum = fold_vec(numbers, 0, |acc, x| acc + x);
    println!("总和: {}", sum);
}
```

### 练习3：错误处理

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput,
}

fn main() {
    // TODO: 实现安全的数学运算函数
    
    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        todo!()
    }
    
    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        todo!()
    }
    
    fn safe_log(x: f64) -> Result<f64, MathError> {
        todo!()
    }
    
    // TODO: 实现一个组合函数，计算 sqrt(a/b)
    fn sqrt_divide(a: f64, b: f64) -> Result<f64, MathError> {
        // 使用 ? 操作符链式处理错误
        todo!()
    }
    
    // 测试你的实现
    let test_cases = [
        (16.0, 4.0),
        (10.0, 0.0),  // 除零错误
        (-4.0, 2.0),  // 负数开方错误
        (9.0, 3.0),
    ];
    
    for (a, b) in test_cases {
        match sqrt_divide(a, b) {
            Ok(result) => println!("sqrt({}/{}) = {:.2}", a, b, result),
            Err(error) => println!("错误: {:?}", error),
        }
    }
}
```

### 练习4：递归和迭代

```rust
fn main() {
    // TODO: 实现汉诺塔问题的递归解法
    fn hanoi(n: u32, from: char, to: char, aux: char) {
        todo!()
    }
    
    // TODO: 实现快速排序
    fn quicksort(arr: &mut [i32]) {
        todo!()
    }
    
    // TODO: 实现迭代版本的阶乘
    fn factorial_iterative(n: u64) -> u64 {
        todo!()
    }
    
    // TODO: 实现帕斯卡三角形的某一行
    fn pascal_triangle_row(n: u32) -> Vec<u32> {
        todo!()
    }
    
    // 测试你的实现
    println!("汉诺塔 (3个盘子):");
    hanoi(3, 'A', 'C', 'B');
    
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("\n排序前: {:?}", arr);
    quicksort(&mut arr);
    println!("排序后: {:?}", arr);
    
    println!("\n阶乘比较:");
    for i in 0..=10 {
        println!("{}! = {} (迭代)", i, factorial_iterative(i));
    }
    
    println!("\n帕斯卡三角形前几行:");
    for i in 0..=5 {
        println!("第{}行: {:?}", i, pascal_triangle_row(i));
    }
}
```

## 🔧 最佳实践

### 1. 函数设计原则

```rust
// 好的做法：单一职责
fn calculate_tax(income: f64, rate: f64) -> f64 {
    income * rate
}

fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

// 避免：函数做太多事情
fn bad_calculate_and_format_tax(income: f64, rate: f64) -> String {
    let tax = income * rate;
    format!("${:.2}", tax)  // 混合了计算和格式化
}

// 好的做法：纯函数（无副作用）
fn add_pure(a: i32, b: i32) -> i32 {
    a + b  // 只依赖输入参数
}

// 避免：有副作用的函数
static mut GLOBAL_COUNTER: i32 = 0;

fn add_with_side_effect(a: i32, b: i32) -> i32 {
    unsafe {
        GLOBAL_COUNTER += 1;  // 副作用
    }
    a + b
}

fn main() {
    let income = 50000.0;
    let rate = 0.2;
    
    let tax = calculate_tax(income, rate);
    let formatted = format_currency(tax);
    
    println!("税额: {}", formatted);
    
    println!("纯函数结果: {}", add_pure(5, 3));
}
```

### 2. 错误处理

```rust
// 好的做法：使用Result处理可能失败的操作
fn parse_age(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(age) if age <= 150 => Ok(age),
        Ok(_) => Err("年龄不能超过150".to_string()),
        Err(_) => Err("无效的年龄格式".to_string()),
    }
}

// 好的做法：使用Option处理可能为空的情况
fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        Some(*numbers.iter().max().unwrap())
    }
}

// 避免：使用panic处理预期的错误
fn bad_parse_age(input: &str) -> u8 {
    input.parse().expect("必须是有效年龄")  // 不好：调用者无法处理错误
}

fn main() {
    match parse_age("25") {
        Ok(age) => println!("年龄: {}", age),
        Err(error) => println!("错误: {}", error),
    }
    
    let numbers = [1, 5, 3, 9, 2];
    match find_max(&numbers) {
        Some(max) => println!("最大值: {}", max),
        None => println!("数组为空"),
    }
}
```

### 3. 参数设计

```rust
// 好的做法：使用借用而不是获取所有权
fn process_string(s: &str) -> usize {  // 接受&str更通用
    s.len()
}

fn process_numbers(numbers: &[i32]) -> i32 {  // 接受切片更灵活
    numbers.iter().sum()
}

// 好的做法：使用结构体组织相关参数
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

fn connect_with_config(config: &Config) -> Result<(), String> {
    println!("连接到 {}:{}, 超时: {}s", config.host, config.port, config.timeout);
    Ok(())
}

// 避免：过多的参数
fn bad_connect(host: &str, port: u16, timeout: u64, retry: u32, ssl: bool, auth: &str) {
    // 参数太多，难以使用和维护
}

fn main() {
    let text = "Hello, world!";
    println!("字符串长度: {}", process_string(text));
    
    let numbers = vec![1, 2, 3, 4, 5];
    println!("数字总和: {}", process_numbers(&numbers));
    
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        timeout: 30,
    };
    
    if let Err(e) = connect_with_config(&config) {
        println!("连接失败: {}", e);
    }
}
```

## 🚨 常见错误

### 1. 所有权和借用错误

```rust
fn main() {
    let s = String::from("hello");
    
    // 错误：移动后使用
    // take_ownership(s);
    // println!("{}", s);  // 错误：s已被移动
    
    // 正确：使用借用
    borrow_string(&s);
    println!("{}", s);  // OK：s仍然有效
    
    // 错误：可变和不可变借用冲突
    let mut s2 = String::from("hello");
    let r1 = &s2;        // 不可变借用
    // let r2 = &mut s2;    // 错误：不能同时有可变和不可变借用
    // println!("{}, {}", r1, r2);
    
    // 正确：借用作用域不重叠
    println!("{}", r1);  // r1的最后使用
    let r2 = &mut s2;    // OK：r1不再使用
    r2.push_str(", world");
    println!("{}", r2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &String) {
    println!("{}", s);
}
```

### 2. 返回值错误

```rust
fn main() {
    // 错误：返回局部变量的引用
    // let result = return_local_reference();
    
    // 正确：返回拥有所有权的值
    let result = return_owned_string();
    println!("{}", result);
    
    // 错误：忘记返回值
    // let sum = add_without_return(5, 3);
    
    // 正确：使用表达式返回
    let sum = add_with_return(5, 3);
    println!("和: {}", sum);
}

// 错误：返回局部变量的引用
// fn return_local_reference() -> &str {
//     let s = String::from("hello");
//     &s  // 错误：s在函数结束时被销毁
// }

// 正确：返回拥有所有权的值
fn return_owned_string() -> String {
    let s = String::from("hello");
    s  // OK：移动所有权给调用者
}

// 错误：忘记返回值
fn add_without_return(a: i32, b: i32) -> i32 {
    let result = a + b;
    // 忘记返回result
    // 实际返回() (unit type)
    result;  // 注意分号！这使它成为语句而不是表达式
}

// 正确：返回表达式
fn add_with_return(a: i32, b: i32) -> i32 {
    a + b  // 表达式，没有分号
}
```

### 3. 递归错误

```rust
fn main() {
    // 测试改进的递归函数
    println!("阶乘: {}", safe_factorial(5));
    
    // 测试可能栈溢出的情况
    match safe_factorial(20) {
        result => println!("20! = {}", result),
    }
}

// 错误：没有基础情况，会导致栈溢出
// fn bad_factorial(n: u64) -> u64 {
//     n * bad_factorial(n - 1)  // 没有停止条件
// }

// 错误：基础情况不正确
// fn bad_factorial2(n: u64) -> u64 {
//     if n == 1 {  // 错误：n=0时会继续递归
//         1
//     } else {
//         n * bad_factorial2(n - 1)
//     }
// }

// 正确：有正确的基础情况
fn safe_factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,  // 正确的基础情况
        _ => n * safe_factorial(n - 1),
    }
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 正确定义和调用函数
- [ ] 理解参数传递的所有权规则
- [ ] 使用不同的返回值类型（单值、元组、Option、Result）
- [ ] 编写纯函数和避免不必要的副作用
- [ ] 使用函数指针和基础闭包
- [ ] 实现递归函数并避免栈溢出
- [ ] 处理函数中的错误情况
- [ ] 设计清晰、可维护的函数接口
- [ ] 避免常见的所有权和借用错误

## 📚 延伸阅读

- [Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust Reference - Functions](https://doc.rust-lang.org/reference/items/functions.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

---

**函数基础掌握完成！** 🎯 你现在理解了Rust函数系统的核心概念。

[← 上一节：数据类型详解](./02-data-types.md) | [下一节：注释和文档 →](./04-comments.md)