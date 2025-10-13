# 模式匹配高级用法

## 学习目标

通过本节学习，你将能够：

- [ ] 掌握复杂模式匹配语法
- [ ] 理解和使用守卫条件（match guards）
- [ ] 学会解构赋值和绑定
- [ ] 掌握 `if let` 和 `while let` 语法
- [ ] 理解模式匹配的穷尽性检查
- [ ] 学会使用 `@` 绑定操作符
- [ ] 掌握范围模式和通配符
- [ ] 理解模式匹配的性能特点

## 复杂模式匹配

### 嵌套模式匹配

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出程序"),
        
        // 解构结构体字段
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        
        // 解构字符串
        Message::Write(text) => println!("写入: {}", text),
        
        // 嵌套模式匹配
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("改变颜色为 RGB({}, {}, {})", r, g, b);
        }
        
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("改变颜色为 HSV({}, {}, {})", h, s, v);
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(Color::Rgb(255, 0, 0)),
        Message::ChangeColor(Color::Hsv(180, 100, 50)),
    ];
    
    for msg in messages {
        process_message(msg);
    }
}
```

### 复杂数据结构的模式匹配

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point },
    Triangle(Point, Point, Point),
}

fn analyze_shape(shape: &Shape) {
    match shape {
        // 匹配圆形，并解构中心点
        Shape::Circle { center: Point { x, y }, radius } => {
            println!("圆形: 中心({}, {}), 半径{}", x, y, radius);
        }
        
        // 匹配矩形，解构两个点
        Shape::Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } => {
            let width = (x2 - x1).abs();
            let height = (y2 - y1).abs();
            println!("矩形: 左上({}, {}), 右下({}, {}), 尺寸{}x{}", x1, y1, x2, y2, width, height);
        }
        
        // 匹配三角形，使用元组解构
        Shape::Triangle(
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 },
            Point { x: x3, y: y3 },
        ) => {
            println!("三角形: 顶点({}, {}), ({}, {}), ({}, {})", x1, y1, x2, y2, x3, y3);
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle {
            center: Point { x: 0, y: 0 },
            radius: 5.0,
        },
        Shape::Rectangle {
            top_left: Point { x: 0, y: 10 },
            bottom_right: Point { x: 20, y: 0 },
        },
        Shape::Triangle(
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 5, y: 10 },
        ),
    ];
    
    for shape in &shapes {
        analyze_shape(shape);
    }
}
```

## 守卫条件（Match Guards）

### 基本守卫条件

```rust
fn categorize_number(x: i32) {
    match x {
        n if n < 0 => println!("{} 是负数", n),
        0 => println!("零"),
        n if n > 0 && n <= 10 => println!("{} 是小正数", n),
        n if n > 10 && n <= 100 => println!("{} 是中等正数", n),
        n => println!("{} 是大正数", n),
    }
}

fn main() {
    let numbers = [-5, 0, 3, 15, 150];
    
    for num in numbers {
        categorize_number(num);
    }
}
```

### 复杂守卫条件

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

#[derive(Debug)]
enum UserAction {
    Login(User),
    Logout(String), // 用户名
    UpdateProfile(User),
    DeleteAccount(String),
}

fn process_user_action(action: UserAction) {
    match action {
        // 守卫条件检查用户状态
        UserAction::Login(user) if user.active && user.age >= 18 => {
            println!("成年活跃用户 {} 登录成功", user.name);
        }
        
        UserAction::Login(user) if !user.active => {
            println!("用户 {} 账户已被禁用", user.name);
        }
        
        UserAction::Login(user) if user.age < 18 => {
            println!("未成年用户 {} 需要监护人同意", user.name);
        }
        
        UserAction::Login(user) => {
            println!("用户 {} 登录，但状态异常", user.name);
        }
        
        // 守卫条件检查用户名长度
        UserAction::Logout(username) if username.len() > 0 => {
            println!("用户 {} 登出", username);
        }
        
        UserAction::Logout(_) => {
            println!("无效的登出请求");
        }
        
        // 守卫条件检查更新权限
        UserAction::UpdateProfile(user) if user.active => {
            println!("更新用户 {} 的资料", user.name);
        }
        
        UserAction::UpdateProfile(user) => {
            println!("用户 {} 无权更新资料（账户未激活）", user.name);
        }
        
        // 守卫条件检查删除权限
        UserAction::DeleteAccount(username) if username != "admin" => {
            println!("删除用户账户: {}", username);
        }
        
        UserAction::DeleteAccount(username) => {
            println!("无法删除管理员账户: {}", username);
        }
    }
}

fn main() {
    let actions = vec![
        UserAction::Login(User {
            name: "Alice".to_string(),
            age: 25,
            active: true,
        }),
        UserAction::Login(User {
            name: "Bob".to_string(),
            age: 16,
            active: true,
        }),
        UserAction::Login(User {
            name: "Charlie".to_string(),
            age: 30,
            active: false,
        }),
        UserAction::Logout("Alice".to_string()),
        UserAction::UpdateProfile(User {
            name: "Alice".to_string(),
            age: 25,
            active: true,
        }),
        UserAction::DeleteAccount("admin".to_string()),
        UserAction::DeleteAccount("Bob".to_string()),
    ];
    
    for action in actions {
        process_user_action(action);
    }
}
```

## @ 绑定操作符

### 基本 @ 绑定

```rust
#[derive(Debug)]
enum Message {
    Hello { id: i32 },
    Goodbye,
}

fn process_message(msg: Message) {
    match msg {
        // 使用 @ 绑定整个值，同时匹配内部结构
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("找到中等ID的Hello消息: {}", id_var);
        }
        
        Message::Hello { id: id_var @ 10..=12 } => {
            println!("找到高ID的Hello消息: {}", id_var);
        }
        
        Message::Hello { id } => {
            println!("其他Hello消息，ID: {}", id);
        }
        
        Message::Goodbye => println!("再见消息"),
    }
}

fn main() {
    let messages = vec![
        Message::Hello { id: 1 },
        Message::Hello { id: 5 },
        Message::Hello { id: 11 },
        Message::Hello { id: 15 },
        Message::Goodbye,
    ];
    
    for msg in messages {
        process_message(msg);
    }
}
```

### 复杂 @ 绑定

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Location {
    Origin,
    Point(Point),
    Area { top_left: Point, bottom_right: Point },
}

fn analyze_location(loc: Location) {
    match loc {
        Location::Origin => println!("原点"),
        
        // 绑定整个Point，同时检查坐标范围
        Location::Point(p @ Point { x: 0..=10, y: 0..=10 }) => {
            println!("小区域内的点: {:?}", p);
        }
        
        Location::Point(p @ Point { x, y }) if x == y => {
            println!("对角线上的点: {:?}", p);
        }
        
        Location::Point(p) => {
            println!("其他点: {:?}", p);
        }
        
        // 复杂的 @ 绑定
        Location::Area {
            top_left: tl @ Point { x: x1, y: y1 },
            bottom_right: br @ Point { x: x2, y: y2 },
        } if (x2 - x1) * (y2 - y1) <= 100 => {
            println!("小区域: 左上{:?}, 右下{:?}, 面积{}", tl, br, (x2 - x1) * (y2 - y1));
        }
        
        Location::Area { top_left, bottom_right } => {
            let area = (bottom_right.x - top_left.x) * (bottom_right.y - top_left.y);
            println!("大区域: 面积{}", area);
        }
    }
}

fn main() {
    let locations = vec![
        Location::Origin,
        Location::Point(Point { x: 5, y: 3 }),
        Location::Point(Point { x: 7, y: 7 }),
        Location::Point(Point { x: 15, y: 20 }),
        Location::Area {
            top_left: Point { x: 0, y: 10 },
            bottom_right: Point { x: 5, y: 5 },
        },
        Location::Area {
            top_left: Point { x: 0, y: 20 },
            bottom_right: Point { x: 30, y: 0 },
        },
    ];
    
    for loc in locations {
        analyze_location(loc);
    }
}
```

## if let 和 while let

### if let 语法

```rust
fn main() {
    let some_option = Some(5);
    let none_option: Option<i32> = None;
    
    // 传统的 match 方式
    match some_option {
        Some(value) => println!("值是: {}", value),
        None => (),
    }
    
    // 使用 if let 简化
    if let Some(value) = some_option {
        println!("if let: 值是: {}", value);
    }
    
    // if let 与 else
    if let Some(value) = none_option {
        println!("值是: {}", value);
    } else {
        println!("没有值");
    }
    
    // 复杂的 if let
    #[derive(Debug)]
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        Quit,
    }
    
    let msg = Message::Move { x: 10, y: 20 };
    
    if let Message::Move { x, y } = msg {
        println!("移动到: ({}, {})", x, y);
    } else if let Message::Write(text) = msg {
        println!("写入: {}", text);
    } else {
        println!("其他消息");
    }
}
```

### while let 语法

```rust
fn main() {
    // 使用 while let 处理 Vec
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈内容:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // 使用 while let 处理迭代器
    let mut iter = vec![1, 2, 3, 4, 5].into_iter();
    
    println!("\n迭代器内容:");
    while let Some(value) = iter.next() {
        if value > 3 {
            break;
        }
        println!("值: {}", value);
    }
    
    // 复杂的 while let
    #[derive(Debug)]
    enum Task {
        Process(String),
        Wait(u32),
        Done,
    }
    
    let mut tasks = vec![
        Task::Process("任务1".to_string()),
        Task::Wait(100),
        Task::Process("任务2".to_string()),
        Task::Done,
        Task::Process("任务3".to_string()), // 这个不会被处理
    ];
    
    println!("\n处理任务:");
    while let Some(task) = tasks.pop() {
        match task {
            Task::Process(name) => println!("处理: {}", name),
            Task::Wait(ms) => println!("等待: {}ms", ms),
            Task::Done => {
                println!("任务完成，停止处理");
                break;
            }
        }
    }
}
```

## 范围模式和通配符

### 范围模式

```rust
fn classify_char(c: char) {
    match c {
        'a'..='z' => println!("'{}' 是小写字母", c),
        'A'..='Z' => println!("'{}' 是大写字母", c),
        '0'..='9' => println!("'{}' 是数字", c),
        ' ' | '\t' | '\n' => println!("'{}' 是空白字符", c),
        _ => println!("'{}' 是其他字符", c),
    }
}

fn classify_number(n: i32) {
    match n {
        i32::MIN..=-1 => println!("{} 是负数", n),
        0 => println!("{} 是零", n),
        1..=10 => println!("{} 是小正数", n),
        11..=100 => println!("{} 是中等正数", n),
        101..=i32::MAX => println!("{} 是大正数", n),
    }
}

fn main() {
    let chars = ['a', 'Z', '5', ' ', '@'];
    for c in chars {
        classify_char(c);
    }
    
    println!();
    
    let numbers = [-10, 0, 5, 50, 500];
    for n in numbers {
        classify_number(n);
    }
}
```

### 通配符和忽略模式

```rust
#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
enum Message {
    Move(Point3D),
    Write(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn process_message(msg: Message) {
    match msg {
        // 只关心 x 坐标
        Message::Move(Point3D { x, y: _, z: _ }) => {
            println!("移动，x坐标: {}", x);
        }
        
        // 使用 .. 忽略剩余字段
        // Message::Move(Point3D { x, .. }) => {
        //     println!("移动，x坐标: {}", x);
        // }
        
        // 只关心字符串长度
        Message::Write(text) if text.len() > 10 => {
            println!("长消息: {} 字符", text.len());
        }
        
        Message::Write(_) => {
            println!("短消息");
        }
        
        // 只关心红色分量
        Message::ChangeColor(r, _, _) if r > 200 => {
            println!("高红色值: {}", r);
        }
        
        Message::ChangeColor(_, _, _) => {
            println!("其他颜色");
        }
        
        Message::Quit => println!("退出"),
    }
}

fn main() {
    let messages = vec![
        Message::Move(Point3D { x: 10, y: 20, z: 30 }),
        Message::Write("Hello".to_string()),
        Message::Write("This is a very long message".to_string()),
        Message::ChangeColor(255, 100, 50),
        Message::ChangeColor(100, 200, 150),
        Message::Quit,
    ];
    
    for msg in messages {
        process_message(msg);
    }
}
```

## 解构赋值

### 元组解构

```rust
fn main() {
    // 基本元组解构
    let tuple = (1, "hello", 3.14);
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // 忽略某些值
    let (a, _, c) = tuple;
    println!("a: {}, c: {}", a, c);
    
    // 嵌套元组解构
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    
    // 函数返回值解构
    fn get_coordinates() -> (i32, i32) {
        (10, 20)
    }
    
    let (x, y) = get_coordinates();
    println!("坐标: ({}, {})", x, y);
}
```

### 结构体解构

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    
    // 完全解构
    let Person { name, age, email } = person;
    println!("姓名: {}, 年龄: {}, 邮箱: {}", name, age, email);
    
    // 重新创建用于后续示例
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };
    
    // 部分解构，重命名字段
    let Person { name: person_name, age: person_age, .. } = person;
    println!("人员: {}, 年龄: {}", person_name, person_age);
    
    // 嵌套结构体解构
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }
    
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 20, y: 0 },
    };
    
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    
    println!("矩形: ({}, {}) 到 ({}, {})", x1, y1, x2, y2);
}
```

### 数组和切片解构

```rust
fn main() {
    // 数组解构
    let arr = [1, 2, 3, 4, 5];
    
    // 完全解构（需要知道确切长度）
    let [a, b, c, d, e] = arr;
    println!("数组元素: {}, {}, {}, {}, {}", a, b, c, d, e);
    
    // 部分解构
    let [first, second, ..] = arr;
    println!("前两个元素: {}, {}", first, second);
    
    let [.., last] = arr;
    println!("最后一个元素: {}", last);
    
    let [first, .., last] = arr;
    println!("第一个和最后一个: {}, {}", first, last);
    
    // 切片解构
    let slice = &arr[1..4];
    match slice {
        [x] => println!("单元素切片: {}", x),
        [x, y] => println!("双元素切片: {}, {}", x, y),
        [x, y, z] => println!("三元素切片: {}, {}, {}", x, y, z),
        _ => println!("其他长度的切片"),
    }
    
    // 动态切片模式匹配
    fn analyze_slice(slice: &[i32]) {
        match slice {
            [] => println!("空切片"),
            [x] => println!("单元素: {}", x),
            [x, y] => println!("两元素: {}, {}", x, y),
            [x, .., y] => println!("多元素，首尾: {}, {}", x, y),
        }
    }
    
    analyze_slice(&[]);
    analyze_slice(&[1]);
    analyze_slice(&[1, 2]);
    analyze_slice(&[1, 2, 3, 4, 5]);
}
```

## 穷尽性检查

### 编译器的穷尽性检查

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

// 这个函数会编译失败，因为没有处理所有变体
// fn incomplete_match(color: Color) {
//     match color {
//         Color::Red => println!("红色"),
//         Color::Green => println!("绿色"),
//         // 缺少 Blue 和 Yellow
//     }
// }

// 完整的匹配
fn complete_match(color: Color) {
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue => println!("蓝色"),
        Color::Yellow => println!("黄色"),
    }
}

// 使用通配符处理剩余情况
fn wildcard_match(color: Color) {
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        _ => println!("其他颜色"),
    }
}

fn main() {
    let colors = vec![Color::Red, Color::Green, Color::Blue, Color::Yellow];
    
    for color in colors {
        complete_match(color);
    }
}
```

### 处理未来的枚举扩展

```rust
#[derive(Debug)]
enum ApiResponse {
    Success { data: String },
    Error { code: u32, message: String },
    Timeout,
    // 未来可能添加更多变体
}

// 使用 #[non_exhaustive] 属性（在库中使用）
// #[non_exhaustive]
// enum FutureEnum {
//     VariantA,
//     VariantB,
// }

fn handle_response(response: ApiResponse) {
    match response {
        ApiResponse::Success { data } => {
            println!("成功: {}", data);
        }
        ApiResponse::Error { code, message } => {
            println!("错误 {}: {}", code, message);
        }
        ApiResponse::Timeout => {
            println!("请求超时");
        }
        // 如果枚举标记为 #[non_exhaustive]，需要通配符
        // _ => println!("未知响应类型"),
    }
}

// 更灵活的处理方式
fn flexible_handle_response(response: ApiResponse) {
    match response {
        ApiResponse::Success { data } => {
            println!("处理成功数据: {}", data);
        }
        error_or_timeout => {
            // 处理所有错误情况
            match error_or_timeout {
                ApiResponse::Error { code, message } => {
                    eprintln!("API错误 {}: {}", code, message);
                }
                ApiResponse::Timeout => {
                    eprintln!("请求超时，请重试");
                }
                _ => unreachable!(), // 这里不应该到达
            }
        }
    }
}

fn main() {
    let responses = vec![
        ApiResponse::Success {
            data: "用户数据".to_string(),
        },
        ApiResponse::Error {
            code: 404,
            message: "未找到".to_string(),
        },
        ApiResponse::Timeout,
    ];
    
    for response in responses {
        handle_response(response);
    }
}
```

## 性能考虑

### 模式匹配的性能特点

```rust
use std::time::Instant;

#[derive(Debug, Clone)]
enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

// 使用 match 的版本
fn calculate_match(op: &Operation) -> Option<i32> {
    match op {
        Operation::Add(a, b) => Some(a + b),
        Operation::Subtract(a, b) => Some(a - b),
        Operation::Multiply(a, b) => Some(a * b),
        Operation::Divide(a, b) if *b != 0 => Some(a / b),
        Operation::Divide(_, _) => None,
    }
}

// 使用 if let 链的版本（性能较差）
fn calculate_if_let(op: &Operation) -> Option<i32> {
    if let Operation::Add(a, b) = op {
        Some(a + b)
    } else if let Operation::Subtract(a, b) = op {
        Some(a - b)
    } else if let Operation::Multiply(a, b) = op {
        Some(a * b)
    } else if let Operation::Divide(a, b) = op {
        if *b != 0 {
            Some(a / b)
        } else {
            None
        }
    } else {
        None
    }
}

fn benchmark_pattern_matching() {
    let operations: Vec<Operation> = (0..1_000_000)
        .map(|i| match i % 4 {
            0 => Operation::Add(i, i + 1),
            1 => Operation::Subtract(i, i - 1),
            2 => Operation::Multiply(i, 2),
            _ => Operation::Divide(i, if i == 0 { 1 } else { i }),
        })
        .collect();
    
    // 测试 match 性能
    let start = Instant::now();
    let mut sum = 0;
    for op in &operations {
        if let Some(result) = calculate_match(op) {
            sum += result;
        }
    }
    let match_duration = start.elapsed();
    println!("Match 结果: {}, 耗时: {:?}", sum, match_duration);
    
    // 测试 if let 性能
    let start = Instant::now();
    let mut sum = 0;
    for op in &operations {
        if let Some(result) = calculate_if_let(op) {
            sum += result;
        }
    }
    let if_let_duration = start.elapsed();
    println!("If let 结果: {}, 耗时: {:?}", sum, if_let_duration);
    
    println!(
        "Match 比 if let 快 {:.2}x",
        if_let_duration.as_nanos() as f64 / match_duration.as_nanos() as f64
    );
}

fn main() {
    benchmark_pattern_matching();
}
```

### 优化模式匹配

```rust
// 避免不必要的克隆
#[derive(Debug)]
enum Data {
    Text(String),
    Number(i32),
    List(Vec<i32>),
}

// 低效的方式：不必要的克隆
fn process_data_inefficient(data: Data) -> String {
    match data {
        Data::Text(text) => text.clone(), // 不必要的克隆
        Data::Number(n) => n.to_string(),
        Data::List(list) => format!("{:?}", list.clone()), // 不必要的克隆
    }
}

// 高效的方式：使用引用
fn process_data_efficient(data: &Data) -> String {
    match data {
        Data::Text(text) => text.clone(), // 这里的克隆是必要的
        Data::Number(n) => n.to_string(),
        Data::List(list) => format!("{:?}", list), // 不需要克隆
    }
}

// 更高效：返回引用或借用
fn get_text_ref(data: &Data) -> Option<&str> {
    match data {
        Data::Text(text) => Some(text),
        _ => None,
    }
}

// 使用 matches! 宏进行简单检查
fn is_text(data: &Data) -> bool {
    matches!(data, Data::Text(_))
}

fn main() {
    let data_items = vec![
        Data::Text("Hello".to_string()),
        Data::Number(42),
        Data::List(vec![1, 2, 3]),
    ];
    
    for data in &data_items {
        println!("处理结果: {}", process_data_efficient(data));
        
        if let Some(text) = get_text_ref(data) {
            println!("文本内容: {}", text);
        }
        
        println!("是否为文本: {}", is_text(data));
    }
}
```

## 实际应用示例

### 状态机与模式匹配

```rust
#[derive(Debug, Clone)]
enum State {
    Idle,
    Loading { progress: u8 },
    Success { data: String },
    Error { message: String, retry_count: u32 },
}

#[derive(Debug)]
enum Event {
    Start,
    Progress(u8),
    Complete(String),
    Fail(String),
    Retry,
    Reset,
}

struct StateMachine {
    state: State,
    max_retries: u32,
}

impl StateMachine {
    fn new(max_retries: u32) -> Self {
        StateMachine {
            state: State::Idle,
            max_retries,
        }
    }
    
    fn handle_event(&mut self, event: Event) -> Result<(), String> {
        let new_state = match (&self.state, event) {
            // 从空闲状态开始
            (State::Idle, Event::Start) => State::Loading { progress: 0 },
            
            // 加载过程中的进度更新
            (State::Loading { .. }, Event::Progress(progress)) => {
                State::Loading { progress }
            }
            
            // 加载完成
            (State::Loading { .. }, Event::Complete(data)) => {
                State::Success { data }
            }
            
            // 加载失败
            (State::Loading { .. }, Event::Fail(message)) => {
                State::Error {
                    message,
                    retry_count: 0,
                }
            }
            
            // 错误状态下重试
            (State::Error { message, retry_count }, Event::Retry) => {
                if *retry_count < self.max_retries {
                    State::Error {
                        message: message.clone(),
                        retry_count: retry_count + 1,
                    }
                } else {
                    return Err("超过最大重试次数".to_string());
                }
            }
            
            // 重置到空闲状态
            (_, Event::Reset) => State::Idle,
            
            // 无效的状态转换
            (current_state, event) => {
                return Err(format!(
                    "无效的状态转换: {:?} + {:?}",
                    current_state, event
                ));
            }
        };
        
        self.state = new_state;
        Ok(())
    }
    
    fn get_state(&self) -> &State {
        &self.state
    }
    
    fn can_start(&self) -> bool {
        matches!(self.state, State::Idle | State::Success { .. } | State::Error { .. })
    }
    
    fn is_loading(&self) -> bool {
        matches!(self.state, State::Loading { .. })
    }
    
    fn get_progress(&self) -> Option<u8> {
        match &self.state {
            State::Loading { progress } => Some(*progress),
            _ => None,
        }
    }
}

fn main() {
    let mut sm = StateMachine::new(3);
    
    println!("初始状态: {:?}", sm.get_state());
    
    // 模拟状态转换
    let events = vec![
        Event::Start,
        Event::Progress(25),
        Event::Progress(50),
        Event::Progress(75),
        Event::Fail("网络错误".to_string()),
        Event::Retry,
        Event::Progress(100),
        Event::Complete("数据加载完成".to_string()),
        Event::Reset,
    ];
    
    for event in events {
        println!("\n处理事件: {:?}", event);
        match sm.handle_event(event) {
            Ok(()) => {
                println!("新状态: {:?}", sm.get_state());
                if let Some(progress) = sm.get_progress() {
                    println!("进度: {}%", progress);
                }
            }
            Err(e) => println!("错误: {}", e),
        }
    }
}
```

## 最佳实践

### 1. 模式匹配的可读性

```rust
// 好的做法：清晰的模式匹配
#[derive(Debug)]
enum HttpResponse {
    Ok { body: String },
    NotFound,
    ServerError { code: u16, message: String },
}

fn handle_response_good(response: HttpResponse) {
    match response {
        HttpResponse::Ok { body } => {
            println!("成功响应，内容长度: {}", body.len());
        }
        HttpResponse::NotFound => {
            println!("资源未找到");
        }
        HttpResponse::ServerError { code, message } => {
            eprintln!("服务器错误 {}: {}", code, message);
        }
    }
}

// 避免：过于复杂的嵌套匹配
fn handle_response_complex(response: Option<Result<HttpResponse, String>>) {
    match response {
        Some(Ok(HttpResponse::Ok { body })) if body.len() > 100 => {
            println!("大响应体");
        }
        Some(Ok(HttpResponse::Ok { body })) => {
            println!("小响应体: {}", body);
        }
        Some(Ok(HttpResponse::NotFound)) => {
            println!("未找到");
        }
        Some(Ok(HttpResponse::ServerError { code, message })) => {
            eprintln!("服务器错误: {} - {}", code, message);
        }
        Some(Err(e)) => {
            eprintln!("请求错误: {}", e);
        }
        None => {
            println!("无响应");
        }
    }
}

// 更好的做法：分层处理
fn handle_response_layered(response: Option<Result<HttpResponse, String>>) {
    let response = match response {
        Some(Ok(resp)) => resp,
        Some(Err(e)) => {
            eprintln!("请求错误: {}", e);
            return;
        }
        None => {
            println!("无响应");
            return;
        }
    };
    
    handle_response_good(response);
}
```

### 2. 性能优化

```rust
// 使用引用避免不必要的移动
fn process_efficiently(data: &[String]) {
    for item in data {
        match item.as_str() {
            "start" => println!("开始处理"),
            "end" => println!("结束处理"),
            text if text.starts_with("data:") => {
                println!("处理数据: {}", &text[5..]);
            }
            _ => println!("未知命令: {}", item),
        }
    }
}

// 使用 matches! 宏进行简单检查
fn filter_commands(data: &[String]) -> Vec<&String> {
    data.iter()
        .filter(|item| matches!(item.as_str(), "start" | "end" | s if s.starts_with("data:")))
        .collect()
}
```

## 学习检查清单

完成本节学习后，请确认你能够：

- [ ] 理解和使用复杂的嵌套模式匹配
- [ ] 掌握守卫条件的使用方法
- [ ] 熟练使用 `@` 绑定操作符
- [ ] 理解 `if let` 和 `while let` 的适用场景
- [ ] 掌握范围模式和通配符的使用
- [ ] 理解编译器的穷尽性检查
- [ ] 能够进行各种形式的解构赋值
- [ ] 了解模式匹配的性能特点
- [ ] 能够设计清晰、高效的模式匹配代码
- [ ] 避免常见的模式匹配陷阱

## 扩展阅读

- [Rust Book - 模式和匹配](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference - 模式](https://doc.rust-lang.org/reference/patterns.html)
- [Rust by Example - 模式匹配](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [The Rust Programming Language - 高级模式](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

---

**下一节预告**：在下一节中，我们将学习泛型编程的基础，了解如何编写可重用的代码。