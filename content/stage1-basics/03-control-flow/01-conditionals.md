# 3.1 条件语句

条件语句是程序控制流的基础，允许程序根据不同的条件执行不同的代码分支。Rust提供了`if`、`else if`、`else`等条件语句，以及强大的模式匹配功能。本节将详细介绍Rust中条件语句的使用方法和最佳实践。

## 🎯 学习目标

- 掌握if、else if、else的基本语法
- 理解条件表达式的概念
- 学会使用if let进行模式匹配
- 掌握条件语句的最佳实践
- 了解与其他语言的差异

## 🔀 基本if语句

### 简单条件判断

```rust
fn main() {
    let number = 6;
    
    // 基本if语句
    if number < 5 {
        println!("条件为真");
    }
    
    // if-else语句
    if number % 4 == 0 {
        println!("数字能被4整除");
    } else if number % 3 == 0 {
        println!("数字能被3整除");
    } else if number % 2 == 0 {
        println!("数字能被2整除");
    } else {
        println!("数字不能被4、3或2整除");
    }
    
    // 注意：条件必须是bool类型
    let condition = true;
    if condition {
        println!("条件为真");
    }
    
    // 错误示例：Rust不会自动转换类型
    // let number = 3;
    // if number {  // 编译错误！
    //     println!("这不会工作");
    // }
    
    // 正确的方式
    if number != 0 {
        println!("数字不为零");
    }
}
```

### 复杂条件表达式

```rust
fn main() {
    let age = 25;
    let has_license = true;
    let has_car = false;
    
    // 使用逻辑运算符组合条件
    if age >= 18 && has_license {
        println!("可以开车");
    }
    
    if age >= 21 || (age >= 18 && has_license) {
        println!("满足某些条件");
    }
    
    // 使用括号明确优先级
    if (age >= 18 && has_license) && !has_car {
        println!("有驾照但没有车");
    }
    
    // 复杂的条件逻辑
    let score = 85;
    let attendance = 0.9;
    let extra_credit = true;
    
    if (score >= 90) || (score >= 80 && attendance >= 0.8) || (score >= 70 && extra_credit) {
        println!("通过考试");
    } else {
        println!("考试不及格");
    }
    
    // 使用函数简化复杂条件
    if is_passing_grade(score, attendance, extra_credit) {
        println!("使用函数判断：通过考试");
    }
}

fn is_passing_grade(score: i32, attendance: f64, extra_credit: bool) -> bool {
    (score >= 90) || 
    (score >= 80 && attendance >= 0.8) || 
    (score >= 70 && extra_credit)
}
```

## 🎭 if作为表达式

### 基本表达式用法

```rust
fn main() {
    let condition = true;
    
    // if作为表达式
    let number = if condition { 5 } else { 6 };
    println!("number的值是: {}", number);
    
    // 更复杂的例子
    let weather = "sunny";
    let activity = if weather == "sunny" {
        "去公园"
    } else if weather == "rainy" {
        "在家读书"
    } else {
        "看电影"
    };
    println!("今天的活动: {}", activity);
    
    // 在函数中使用
    let grade = get_letter_grade(85);
    println!("成绩等级: {}", grade);
    
    // 用于初始化复杂数据
    let user_type = "admin";
    let permissions = if user_type == "admin" {
        vec!["read", "write", "delete", "admin"]
    } else if user_type == "editor" {
        vec!["read", "write"]
    } else {
        vec!["read"]
    };
    println!("用户权限: {:?}", permissions);
}

fn get_letter_grade(score: i32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}
```

### 表达式的类型一致性

```rust
fn main() {
    let condition = true;
    
    // 正确：两个分支返回相同类型
    let number = if condition { 5 } else { 6 };
    println!("数字: {}", number);
    
    // 错误示例：类型不匹配
    // let value = if condition { 5 } else { "six" };  // 编译错误！
    
    // 正确的处理方式：使用枚举或统一类型
    let value = if condition {
        "five".to_string()
    } else {
        "six".to_string()
    };
    println!("值: {}", value);
    
    // 使用Option处理可能的空值
    let maybe_number = if condition {
        Some(42)
    } else {
        None
    };
    
    match maybe_number {
        Some(n) => println!("数字是: {}", n),
        None => println!("没有数字"),
    }
    
    // 使用Result处理可能的错误
    let result = if condition {
        Ok("成功")
    } else {
        Err("失败")
    };
    
    match result {
        Ok(msg) => println!("结果: {}", msg),
        Err(err) => println!("错误: {}", err),
    }
}
```

## 🎯 if let模式匹配

### 基础if let用法

```rust
fn main() {
    // 处理Option类型
    let some_number = Some(5);
    let none_number: Option<i32> = None;
    
    // 使用if let简化Option处理
    if let Some(value) = some_number {
        println!("数字是: {}", value);
    } else {
        println!("没有数字");
    }
    
    if let Some(value) = none_number {
        println!("数字是: {}", value);
    } else {
        println!("确实没有数字");
    }
    
    // 处理Result类型
    let parse_result = "42".parse::<i32>();
    
    if let Ok(number) = parse_result {
        println!("解析成功: {}", number);
    } else {
        println!("解析失败");
    }
    
    let parse_error = "not_a_number".parse::<i32>();
    
    if let Err(error) = parse_error {
        println!("解析错误: {}", error);
    }
    
    // 处理枚举类型
    let message = Message::Write(String::from("hello"));
    
    if let Message::Write(text) = message {
        println!("写入消息: {}", text);
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### 复杂的if let模式

```rust
fn main() {
    // 嵌套的数据结构
    let data = Some(vec![1, 2, 3, 4, 5]);
    
    if let Some(ref numbers) = data {
        if numbers.len() > 3 {
            println!("向量有{}个元素，前三个是: {:?}", 
                    numbers.len(), &numbers[0..3]);
        }
    }
    
    // 组合条件
    let user = User {
        name: String::from("Alice"),
        age: 30,
        email: Some(String::from("alice@example.com")),
    };
    
    if let Some(ref email) = user.email {
        if user.age >= 18 {
            println!("成年用户 {} 的邮箱是: {}", user.name, email);
        }
    }
    
    // 使用守卫条件
    let number = Some(4);
    
    if let Some(x) = number {
        if x < 5 {
            println!("小于5的数字: {}", x);
        }
    }
    
    // 处理复杂枚举
    let messages = vec![
        Message::Write(String::from("Hello")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    
    for msg in messages {
        process_message(msg);
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

fn process_message(msg: Message) {
    if let Message::Write(text) = msg {
        println!("处理写入消息: {}", text);
    } else if let Message::Move { x, y } = msg {
        println!("移动到坐标: ({}, {})", x, y);
    } else if let Message::ChangeColor(r, g, b) = msg {
        println!("改变颜色为: RGB({}, {}, {})", r, g, b);
    } else {
        println!("处理其他消息: {:?}", msg);
    }
}
```

## 🔄 条件语句的最佳实践

### 代码组织和可读性

```rust
fn main() {
    let user_input = "admin";
    let user_age = 25;
    
    // 好的实践：使用早期返回减少嵌套
    if let Some(user) = authenticate_user(user_input) {
        if user.is_active() {
            if user.has_permission("admin") {
                println!("管理员用户已登录");
                // 执行管理员操作
            } else {
                println!("权限不足");
            }
        } else {
            println!("用户账户已禁用");
        }
    } else {
        println!("认证失败");
    }
    
    // 更好的实践：使用函数分解复杂逻辑
    handle_user_login(user_input);
    
    // 使用常量提高可读性
    const MIN_AGE: u32 = 18;
    const MAX_AGE: u32 = 65;
    
    if user_age >= MIN_AGE && user_age <= MAX_AGE {
        println!("用户年龄在有效范围内");
    }
    
    // 使用枚举替代魔法数字
    let status = UserStatus::Active;
    
    match status {
        UserStatus::Active => println!("用户活跃"),
        UserStatus::Inactive => println!("用户不活跃"),
        UserStatus::Suspended => println!("用户被暂停"),
    }
}

#[derive(Debug, PartialEq)]
enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug)]
struct AuthUser {
    name: String,
    active: bool,
    permissions: Vec<String>,
}

impl AuthUser {
    fn is_active(&self) -> bool {
        self.active
    }
    
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}

fn authenticate_user(input: &str) -> Option<AuthUser> {
    // 模拟用户认证
    if input == "admin" {
        Some(AuthUser {
            name: String::from("Administrator"),
            active: true,
            permissions: vec![String::from("admin"), String::from("read"), String::from("write")],
        })
    } else {
        None
    }
}

fn handle_user_login(user_input: &str) {
    let user = match authenticate_user(user_input) {
        Some(u) => u,
        None => {
            println!("认证失败");
            return;
        }
    };
    
    if !user.is_active() {
        println!("用户账户已禁用");
        return;
    }
    
    if !user.has_permission("admin") {
        println!("权限不足");
        return;
    }
    
    println!("管理员用户 {} 已成功登录", user.name);
    // 执行管理员操作
}
```

### 错误处理和边界情况

```rust
fn main() {
    // 处理用户输入
    let inputs = vec!["5", "0", "-3", "abc", ""];
    
    for input in inputs {
        process_user_input(input);
    }
    
    // 处理集合操作
    let numbers = vec![1, 2, 3, 4, 5];
    let empty_vec: Vec<i32> = vec![];
    
    process_numbers(&numbers);
    process_numbers(&empty_vec);
    
    // 处理配置选项
    let config = Config {
        debug: Some(true),
        max_connections: None,
        timeout: Some(30),
    };
    
    apply_config(&config);
}

#[derive(Debug)]
struct Config {
    debug: Option<bool>,
    max_connections: Option<u32>,
    timeout: Option<u32>,
}

fn process_user_input(input: &str) {
    // 处理空输入
    if input.is_empty() {
        println!("输入为空，请提供有效输入");
        return;
    }
    
    // 尝试解析数字
    match input.parse::<i32>() {
        Ok(number) => {
            if number > 0 {
                println!("正数: {}", number);
            } else if number == 0 {
                println!("零");
            } else {
                println!("负数: {}", number);
            }
        },
        Err(_) => {
            println!("'{}' 不是有效的数字", input);
        }
    }
}

fn process_numbers(numbers: &[i32]) {
    if numbers.is_empty() {
        println!("数组为空，无法处理");
        return;
    }
    
    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;
    
    println!("数组: {:?}", numbers);
    println!("总和: {}, 平均值: {:.2}", sum, avg);
    
    // 查找最大值和最小值
    if let (Some(max), Some(min)) = (numbers.iter().max(), numbers.iter().min()) {
        println!("最大值: {}, 最小值: {}", max, min);
    }
}

fn apply_config(config: &Config) {
    // 使用默认值处理可选配置
    let debug_mode = config.debug.unwrap_or(false);
    let max_conn = config.max_connections.unwrap_or(100);
    let timeout_secs = config.timeout.unwrap_or(60);
    
    println!("配置应用:");
    println!("  调试模式: {}", debug_mode);
    println!("  最大连接数: {}", max_conn);
    println!("  超时时间: {}秒", timeout_secs);
    
    // 验证配置的合理性
    if max_conn > 1000 {
        println!("警告: 最大连接数过高，可能影响性能");
    }
    
    if timeout_secs < 5 {
        println!("警告: 超时时间过短，可能导致连接失败");
    }
}
```

## 🔄 与其他语言的比较

### Rust vs C/C++

```rust
// Rust: 条件必须是bool类型
fn rust_example() {
    let x = 5;
    
    // 正确
    if x != 0 {
        println!("x不为零");
    }
    
    // 错误：不能直接使用数字
    // if x {  // 编译错误
    //     println!("这不会编译");
    // }
    
    // Rust的if是表达式
    let result = if x > 0 { "positive" } else { "non-positive" };
    println!("结果: {}", result);
}

/*
C/C++对比:

int x = 5;

// C/C++中可以直接使用数字作为条件
if (x) {  // 非零即为真
    printf("x不为零\n");
}

// C/C++中if不是表达式，需要使用三元运算符
const char* result = (x > 0) ? "positive" : "non-positive";
*/
```

### Rust vs Python/JavaScript

```rust
// Rust: 严格的类型检查
fn rust_vs_dynamic() {
    let value: Option<i32> = Some(42);
    
    // Rust使用模式匹配处理可选值
    if let Some(num) = value {
        println!("值是: {}", num);
    }
    
    // 或者使用match
    match value {
        Some(num) => println!("匹配到值: {}", num),
        None => println!("没有值"),
    }
}

/*
Python对比:

value = 42  # 或者 None

# Python中可以直接检查None
if value is not None:
    print(f"值是: {value}")

# 或者利用truthiness
if value:
    print(f"值是: {value}")

JavaScript对比:

let value = 42;  // 或者 null/undefined

// JavaScript中的truthiness检查
if (value) {
    console.log(`值是: ${value}`);
}

// 严格检查
if (value !== null && value !== undefined) {
    console.log(`值是: ${value}`);
}
*/
```

## 🧪 实践练习

### 练习1：成绩评定系统

```rust
// TODO: 完成以下函数，实现成绩评定系统
// 要求：
// 1. 90-100: A
// 2. 80-89: B  
// 3. 70-79: C
// 4. 60-69: D
// 5. 0-59: F
// 6. 处理无效输入（负数或超过100）

fn calculate_grade(score: i32) -> Result<char, String> {
    // 在这里实现你的代码
    todo!("实现成绩计算逻辑")
}

fn get_grade_description(grade: char) -> &'static str {
    // 返回成绩描述
    // A: "优秀", B: "良好", C: "中等", D: "及格", F: "不及格"
    todo!("实现成绩描述")
}

fn main() {
    let test_scores = vec![95, 87, 76, 65, 45, -5, 105];
    
    for score in test_scores {
        match calculate_grade(score) {
            Ok(grade) => {
                let description = get_grade_description(grade);
                println!("分数 {}: {} ({})", score, grade, description);
            },
            Err(error) => {
                println!("分数 {}: 错误 - {}", score, error);
            }
        }
    }
}
```

### 练习2：用户权限系统

```rust
// TODO: 实现一个用户权限检查系统

#[derive(Debug, PartialEq)]
enum UserRole {
    Guest,
    User,
    Moderator,
    Admin,
}

#[derive(Debug)]
struct User {
    name: String,
    role: UserRole,
    is_active: bool,
    login_attempts: u32,
}

impl User {
    fn new(name: String, role: UserRole) -> Self {
        User {
            name,
            role,
            is_active: true,
            login_attempts: 0,
        }
    }
    
    // TODO: 实现权限检查方法
    fn can_read(&self) -> bool {
        todo!("实现读取权限检查")
    }
    
    fn can_write(&self) -> bool {
        todo!("实现写入权限检查")
    }
    
    fn can_delete(&self) -> bool {
        todo!("实现删除权限检查")
    }
    
    fn can_admin(&self) -> bool {
        todo!("实现管理权限检查")
    }
    
    // TODO: 实现登录尝试逻辑
    fn attempt_login(&mut self, password: &str) -> Result<(), String> {
        // 模拟密码检查（简单示例）
        // 正确密码："password123"
        // 超过3次失败尝试后锁定账户
        todo!("实现登录逻辑")
    }
}

fn main() {
    let mut users = vec![
        User::new("Alice".to_string(), UserRole::Admin),
        User::new("Bob".to_string(), UserRole::Moderator),
        User::new("Charlie".to_string(), UserRole::User),
        User::new("David".to_string(), UserRole::Guest),
    ];
    
    // 测试权限
    for user in &users {
        println!("\n用户: {} ({:?})", user.name, user.role);
        println!("  读取权限: {}", user.can_read());
        println!("  写入权限: {}", user.can_write());
        println!("  删除权限: {}", user.can_delete());
        println!("  管理权限: {}", user.can_admin());
    }
    
    // 测试登录
    let passwords = vec!["wrong", "password123", "wrong", "wrong", "wrong", "password123"];
    
    for password in passwords {
        match users[0].attempt_login(password) {
            Ok(()) => println!("用户 {} 登录成功", users[0].name),
            Err(error) => println!("用户 {} 登录失败: {}", users[0].name, error),
        }
    }
}
```

### 练习3：配置验证器

```rust
// TODO: 实现一个配置验证系统

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: Option<u32>,
    timeout: Option<u32>,
    ssl_enabled: bool,
    debug_mode: bool,
}

impl ServerConfig {
    fn new(host: String, port: u16) -> Self {
        ServerConfig {
            host,
            port,
            max_connections: None,
            timeout: None,
            ssl_enabled: false,
            debug_mode: false,
        }
    }
    
    // TODO: 实现配置验证方法
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // 验证规则：
        // 1. host不能为空
        // 2. port必须在1024-65535范围内
        // 3. max_connections如果设置，必须在1-10000范围内
        // 4. timeout如果设置，必须在1-3600范围内
        // 5. 如果启用SSL，端口应该是443或8443
        
        todo!("实现配置验证逻辑")
    }
    
    // TODO: 实现配置应用方法
    fn apply_defaults(&mut self) {
        // 应用默认值：
        // max_connections默认为100
        // timeout默认为30秒
        
        todo!("实现默认值应用")
    }
    
    // TODO: 实现配置摘要方法
    fn summary(&self) -> String {
        // 返回配置的可读摘要
        todo!("实现配置摘要")
    }
}

fn main() {
    let test_configs = vec![
        ServerConfig::new("localhost".to_string(), 8080),
        ServerConfig {
            host: "".to_string(),
            port: 80,
            max_connections: Some(50000),
            timeout: Some(5000),
            ssl_enabled: true,
            debug_mode: true,
        },
        ServerConfig {
            host: "example.com".to_string(),
            port: 443,
            max_connections: Some(500),
            timeout: Some(60),
            ssl_enabled: true,
            debug_mode: false,
        },
    ];
    
    for (i, mut config) in test_configs.into_iter().enumerate() {
        println!("\n=== 配置 {} ===", i + 1);
        println!("原始配置: {:?}", config);
        
        // 应用默认值
        config.apply_defaults();
        println!("应用默认值后: {:?}", config);
        
        // 验证配置
        match config.validate() {
            Ok(()) => {
                println!("✅ 配置验证通过");
                println!("配置摘要: {}", config.summary());
            },
            Err(errors) => {
                println!("❌ 配置验证失败:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
    }
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 正确使用if、else if、else语句
- [ ] 理解条件表达式必须是bool类型
- [ ] 使用if作为表达式进行赋值
- [ ] 掌握if let模式匹配的用法
- [ ] 处理Option和Result类型
- [ ] 编写清晰、可读的条件逻辑
- [ ] 避免过度嵌套的条件语句
- [ ] 正确处理边界情况和错误
- [ ] 理解Rust与其他语言在条件语句上的差异

## 📚 延伸阅读

- [Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust Book - if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [Rust Reference - if expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
- [Rust by Example - if/else](https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html)

---

**条件语句掌握完成！** 🎯 你现在能够熟练使用Rust的条件控制结构。

[← 上一节：注释和文档](../02-syntax/04-comments.md) | [下一节：循环语句 →](./02-loops.md)