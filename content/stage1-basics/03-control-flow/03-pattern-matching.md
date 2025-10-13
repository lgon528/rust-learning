# 3.3 模式匹配

模式匹配是Rust最强大的特性之一，它允许你根据数据的结构和值来执行不同的代码分支。Rust的模式匹配比传统的switch语句更加强大和安全，它确保所有可能的情况都被处理，并且可以解构复杂的数据类型。本节将深入探讨match表达式、if let、while let以及各种模式的使用。

## 🎯 学习目标

- 掌握match表达式的语法和用法
- 理解模式匹配的穷尽性检查
- 学会使用各种模式（字面量、变量、通配符等）
- 掌握if let和while let的使用
- 理解模式匹配中的所有权和借用
- 学会在函数参数和let语句中使用模式

## 🔍 match表达式基础

### 基本match语法

```rust
fn main() {
    // 基本的match表达式
    let number = 3;
    
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 => println!("四"),
        5 => println!("五"),
        _ => println!("其他数字"),  // 通配符模式
    }
    
    // match作为表达式返回值
    let description = match number {
        1 => "第一",
        2 => "第二",
        3 => "第三",
        n if n > 10 => "大数字",  // 守卫条件
        _ => "小数字",
    };
    
    println!("数字{}是{}", number, description);
    
    // 匹配多个值
    let day = 3;
    
    match day {
        1 | 2 | 3 | 4 | 5 => println!("工作日"),
        6 | 7 => println!("周末"),
        _ => println!("无效的日期"),
    }
    
    // 匹配范围
    let score = 85;
    
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',  // 处理无效分数
    };
    
    println!("分数{}对应等级{}", score, grade);
    
    // 匹配字符
    let character = 'x';
    
    match character {
        'a'..='z' => println!("小写字母"),
        'A'..='Z' => println!("大写字母"),
        '0'..='9' => println!("数字字符"),
        _ => println!("其他字符"),
    }
}
```

### 复杂数据类型的匹配

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // 匹配枚举
    let msg = Message::Move { x: 10, y: 20 };
    
    match msg {
        Message::Quit => {
            println!("退出消息");
        },
        Message::Move { x, y } => {
            println!("移动到坐标 ({}, {})", x, y);
        },
        Message::Write(text) => {
            println!("写入文本: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!("改变颜色为 RGB({}, {}, {})", r, g, b);
        },
    }
    
    // 匹配Option
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    
    match some_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    match no_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    // 匹配Result
    let parse_result = "42".parse::<i32>();
    
    match parse_result {
        Ok(number) => println!("解析成功: {}", number),
        Err(error) => println!("解析失败: {}", error),
    }
    
    // 匹配元组
    let point = (3, 5);
    
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在Y轴上，y = {}", y),
        (x, 0) => println!("在X轴上，x = {}", x),
        (x, y) => println!("点({}, {})", x, y),
    }
    
    // 匹配数组和切片
    let array = [1, 2, 3];
    
    match array {
        [1, 2, 3] => println!("匹配 [1, 2, 3]"),
        [1, _, 3] => println!("第一个是1，第三个是3"),
        [first, .., last] => println!("第一个: {}, 最后一个: {}", first, last),
    }
    
    // 匹配向量的切片
    let vec = vec![1, 2, 3, 4, 5];
    
    match vec.as_slice() {
        [] => println!("空向量"),
        [x] => println!("只有一个元素: {}", x),
        [x, y] => println!("两个元素: {}, {}", x, y),
        [first, .., last] => println!("多个元素，首: {}, 尾: {}", first, last),
    }
    
    // 匹配IP地址
    let ip1 = IpAddr::V4(127, 0, 0, 1);
    let ip2 = IpAddr::V6("::1".to_string());
    
    for ip in [ip1, ip2] {
        match ip {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4地址: {}.{}.{}.{}", a, b, c, d);
            },
            IpAddr::V6(addr) => {
                println!("IPv6地址: {}", addr);
            },
        }
    }
}
```

## 🔧 模式匹配的高级特性

### 守卫条件（Match Guards）

```rust
fn main() {
    // 基本守卫条件
    let number = Some(4);
    
    match number {
        Some(x) if x < 5 => println!("小于5的数: {}", x),
        Some(x) => println!("大于等于5的数: {}", x),
        None => println!("没有数字"),
    }
    
    // 复杂的守卫条件
    let point = (3, 4);
    
    match point {
        (x, y) if x == y => println!("对角线上的点: ({}, {})", x, y),
        (x, y) if x > y => println!("x大于y: ({}, {})", x, y),
        (x, y) if x < y => println!("x小于y: ({}, {})", x, y),
        (x, y) => println!("其他情况: ({}, {})", x, y),
    }
    
    // 使用外部变量的守卫条件
    let threshold = 10;
    let value = Some(15);
    
    match value {
        Some(x) if x > threshold => println!("{} 大于阈值 {}", x, threshold),
        Some(x) => println!("{} 不大于阈值 {}", x, threshold),
        None => println!("没有值"),
    }
    
    // 多重条件的守卫
    let age = 25;
    let has_license = true;
    
    match age {
        x if x >= 18 && has_license => println!("可以开车"),
        x if x >= 18 => println!("有年龄但没有驾照"),
        _ => println!("年龄不够"),
    }
    
    // 在枚举匹配中使用守卫
    #[derive(Debug)]
    enum Temperature {
        Celsius(f64),
        Fahrenheit(f64),
    }
    
    let temp = Temperature::Celsius(25.0);
    
    match temp {
        Temperature::Celsius(c) if c > 30.0 => println!("热天: {}°C", c),
        Temperature::Celsius(c) if c < 0.0 => println!("冰点以下: {}°C", c),
        Temperature::Celsius(c) => println!("适中温度: {}°C", c),
        Temperature::Fahrenheit(f) if f > 86.0 => println!("热天: {}°F", f),
        Temperature::Fahrenheit(f) if f < 32.0 => println!("冰点以下: {}°F", f),
        Temperature::Fahrenheit(f) => println!("适中温度: {}°F", f),
    }
}
```

### @ 绑定

```rust
#[derive(Debug)]
enum Message {
    Hello { id: i32 },
    Goodbye,
}

fn main() {
    // 基本的@绑定
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("找到ID在范围内: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("找到ID在10-12范围内，但不需要使用值");
        },
        Message::Hello { id } => {
            println!("其他ID: {}", id);
        },
        Message::Goodbye => println!("再见"),
    }
    
    // 在Option中使用@绑定
    let numbers = vec![Some(1), Some(5), Some(10), None, Some(15)];
    
    for number in numbers {
        match number {
            Some(n @ 1..=10) => println!("小数字: {}", n),
            Some(n @ 11..=20) => println!("中等数字: {}", n),
            Some(n) => println!("大数字: {}", n),
            None => println!("没有数字"),
        }
    }
    
    // 复杂结构中的@绑定
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 3, y: 4 },
        Point { x: -1, y: 2 },
    ];
    
    for point in points {
        match point {
            Point { x: 0, y: 0 } => println!("原点"),
            Point { x: x @ 1..=5, y: y @ 1..=5 } => {
                println!("第一象限的小坐标点: ({}, {})", x, y);
            },
            Point { x, y } if x == y => {
                println!("对角线上的点: ({}, {})", x, y);
            },
            p @ Point { x, y } if x.abs() + y.abs() > 10 => {
                println!("距离原点较远的点: {:?}", p);
            },
            Point { x, y } => {
                println!("普通点: ({}, {})", x, y);
            },
        }
    }
    
    // 嵌套结构中的@绑定
    #[derive(Debug)]
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }
    
    let color = Color::Rgb(255, 0, 0);
    
    match color {
        Color::Rgb(r @ 200..=255, 0, 0) => {
            println!("高亮度红色，红色值: {}", r);
        },
        Color::Rgb(r, g, b) => {
            println!("RGB颜色: ({}, {}, {})", r, g, b);
        },
        Color::Hsv(h, s, v) => {
            println!("HSV颜色: ({}, {}, {})", h, s, v);
        },
    }
}
```

## 🔄 if let 和 while let

### if let 语法糖

```rust
fn main() {
    // 基本的if let用法
    let some_value = Some(3);
    
    // 使用match的方式
    match some_value {
        Some(3) => println!("找到数字3"),
        _ => {},  // 忽略其他情况
    }
    
    // 使用if let的简化方式
    if let Some(3) = some_value {
        println!("找到数字3（使用if let）");
    }
    
    // if let with else
    let number = Some(7);
    
    if let Some(x) = number {
        println!("数字是: {}", x);
    } else {
        println!("没有数字");
    }
    
    // 处理Result类型
    let parse_result = "42".parse::<i32>();
    
    if let Ok(number) = parse_result {
        println!("解析成功: {}", number);
    } else {
        println!("解析失败");
    }
    
    // 复杂枚举的if let
    #[derive(Debug)]
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        Quit,
    }
    
    let msg = Message::Move { x: 10, y: 20 };
    
    if let Message::Move { x, y } = msg {
        println!("移动到: ({}, {})", x, y);
    }
    
    // 嵌套的if let
    let nested_option = Some(Some(5));
    
    if let Some(inner_option) = nested_option {
        if let Some(value) = inner_option {
            println!("嵌套值: {}", value);
        }
    }
    
    // 或者使用模式匹配直接处理
    if let Some(Some(value)) = nested_option {
        println!("直接匹配嵌套值: {}", value);
    }
    
    // if let与守卫条件
    let point = Some((3, 4));
    
    if let Some((x, y)) = point {
        if x > 0 && y > 0 {
            println!("第一象限的点: ({}, {})", x, y);
        }
    }
    
    // 处理向量
    let vec = vec![1, 2, 3, 4, 5];
    
    if let [first, .., last] = vec.as_slice() {
        println!("第一个: {}, 最后一个: {}", first, last);
    }
    
    // 处理HashMap
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    if let Some(value) = map.get("key1") {
        println!("找到值: {}", value);
    }
    
    // 链式if let
    let result1 = Some("hello");
    let result2 = Some(42);
    
    if let Some(text) = result1 {
        if let Some(number) = result2 {
            println!("文本: {}, 数字: {}", text, number);
        }
    }
}
```

### while let 循环

```rust
fn main() {
    // 基本的while let用法
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("弹出栈中的元素:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // 处理迭代器
    let mut iter = vec!["a", "b", "c"].into_iter();
    
    println!("\n迭代元素:");
    while let Some(item) = iter.next() {
        println!("项目: {}", item);
    }
    
    // 处理字符迭代器
    let mut chars = "Hello".chars();
    
    println!("\n逐个字符:");
    while let Some(ch) = chars.next() {
        println!("字符: '{}'", ch);
    }
    
    // 处理Result序列
    let parse_attempts = vec!["1", "2", "abc", "4", "xyz"];
    let mut iter = parse_attempts.iter();
    
    println!("\n解析数字直到遇到错误:");
    while let Some(s) = iter.next() {
        match s.parse::<i32>() {
            Ok(num) => println!("解析成功: {}", num),
            Err(_) => {
                println!("解析失败: {}, 停止处理", s);
                break;
            }
        }
    }
    
    // 处理嵌套Option
    let mut nested_options = vec![Some(Some(1)), Some(None), Some(Some(2)), None];
    
    println!("\n处理嵌套Option:");
    while let Some(outer) = nested_options.pop() {
        match outer {
            Some(Some(value)) => println!("找到值: {}", value),
            Some(None) => println!("外层有值，内层为空"),
            None => println!("这不应该发生"),  // 因为while let已经匹配了Some
        }
    }
    
    // 模拟消息处理循环
    simulate_message_processing();
    
    // 模拟文件读取
    simulate_file_reading();
}

#[derive(Debug)]
enum Command {
    Move(i32, i32),
    Draw(String),
    Quit,
}

fn simulate_message_processing() {
    println!("\n=== 消息处理循环 ===");
    
    let mut commands = vec![
        Some(Command::Move(10, 20)),
        Some(Command::Draw("circle".to_string())),
        Some(Command::Move(30, 40)),
        None,  // 模拟没有命令的情况
        Some(Command::Quit),
        Some(Command::Draw("square".to_string())),  // 这个不会被处理
    ];
    
    while let Some(cmd_option) = commands.pop() {
        if let Some(cmd) = cmd_option {
            match cmd {
                Command::Move(x, y) => {
                    println!("移动到 ({}, {})", x, y);
                },
                Command::Draw(shape) => {
                    println!("绘制 {}", shape);
                },
                Command::Quit => {
                    println!("收到退出命令，停止处理");
                    break;
                }
            }
        } else {
            println!("收到空命令，跳过");
        }
    }
}

fn simulate_file_reading() {
    println!("\n=== 模拟文件读取 ===");
    
    // 模拟文件行
    let file_lines = vec![
        Ok("第一行内容".to_string()),
        Ok("第二行内容".to_string()),
        Err("读取错误"),
        Ok("第四行内容".to_string()),  // 这行不会被处理
    ];
    
    let mut line_iter = file_lines.into_iter();
    
    while let Some(line_result) = line_iter.next() {
        match line_result {
            Ok(content) => {
                println!("读取行: {}", content);
            },
            Err(error) => {
                println!("读取错误: {}, 停止读取", error);
                break;
            }
        }
    }
}
```

## 🔧 解构和模式

### 结构体解构

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 基本结构体解构
    let point = Point { x: 10, y: 20 };
    
    match point {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("在Y轴上，y = {}", y),
        Point { x, y: 0 } => println!("在X轴上，x = {}", x),
        Point { x, y } => println!("点({}, {})", x, y),
    }
    
    // 部分解构
    let Point { x, .. } = point;  // 只提取x，忽略其他字段
    println!("x坐标: {}", x);
    
    // 重命名字段
    let Point { x: coord_x, y: coord_y } = point;
    println!("坐标: ({}, {})", coord_x, coord_y);
    
    // 复杂结构体解构
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    };
    
    match person {
        Person { name, age, email: Some(email_addr) } => {
            println!("姓名: {}, 年龄: {}, 邮箱: {}", name, age, email_addr);
        },
        Person { name, age, email: None } => {
            println!("姓名: {}, 年龄: {}, 无邮箱", name, age);
        },
    }
    
    // 嵌套结构体解构
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 20, y: 0 },
    };
    
    match rect {
        Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } => {
            println!("矩形: 左上({}, {}), 右下({}, {})", x1, y1, x2, y2);
            let width = (x2 - x1).abs();
            let height = (y1 - y2).abs();
            println!("宽度: {}, 高度: {}", width, height);
        }
    }
    
    // 在函数参数中解构
    fn print_point_info(Point { x, y }: Point) {
        println!("点的坐标: ({}, {})", x, y);
    }
    
    print_point_info(Point { x: 5, y: 15 });
    
    // 在let语句中解构
    let Rectangle { top_left, bottom_right } = rect;
    println!("左上角: {:?}", top_left);
    println!("右下角: {:?}", bottom_right);
    
    // 解构引用
    let point_ref = &Point { x: 100, y: 200 };
    
    match point_ref {
        &Point { x, y } => println!("引用的点: ({}, {})", x, y),
    }
    
    // 或者使用ref模式
    match point_ref {
        Point { ref x, ref y } => println!("使用ref: ({}, {})", x, y),
    }
}
```

### 元组和数组解构

```rust
fn main() {
    // 元组解构
    let tuple = (1, "hello", 3.14, true);
    
    match tuple {
        (1, text, pi, flag) => {
            println!("匹配: 数字={}, 文本={}, π={}, 标志={}", 1, text, pi, flag);
        },
        (n, _, _, _) => {
            println!("第一个元素是: {}", n);
        },
    }
    
    // 在let中解构元组
    let (first, second, third, fourth) = tuple;
    println!("解构: {}, {}, {}, {}", first, second, third, fourth);
    
    // 忽略某些元素
    let (a, _, c, _) = tuple;
    println!("选择性解构: {}, {}", a, c);
    
    // 嵌套元组解构
    let nested = ((1, 2), (3, 4));
    
    match nested {
        ((a, b), (c, d)) => {
            println!("嵌套元组: ({}, {}), ({}, {})", a, b, c, d);
        }
    }
    
    let ((x1, y1), (x2, y2)) = nested;
    println!("解构嵌套: 点1({}, {}), 点2({}, {})", x1, y1, x2, y2);
    
    // 数组解构
    let array = [1, 2, 3, 4, 5];
    
    match array {
        [1, 2, 3, 4, 5] => println!("完全匹配数组"),
        [1, _, _, _, 5] => println!("首尾匹配"),
        [first, .., last] => println!("首: {}, 尾: {}", first, last),
    }
    
    // 切片模式
    let slice = &array[1..4];
    
    match slice {
        [a, b, c] => println!("三元素切片: {}, {}, {}", a, b, c),
        [first, rest @ ..] => {
            println!("第一个: {}, 其余: {:?}", first, rest);
        },
        [] => println!("空切片"),
    }
    
    // 向量解构（通过切片）
    let vec = vec![10, 20, 30, 40];
    
    match vec.as_slice() {
        [] => println!("空向量"),
        [x] => println!("单元素向量: {}", x),
        [x, y] => println!("双元素向量: {}, {}", x, y),
        [first, middle @ .., last] => {
            println!("多元素向量: 首={}, 中间={:?}, 尾={}", first, middle, last);
        }
    }
    
    // 字符串切片模式（需要转换为字节）
    let text = "hello";
    
    match text.as_bytes() {
        [b'h', rest @ ..] => {
            println!("以'h'开头，其余: {:?}", std::str::from_utf8(rest).unwrap());
        },
        _ => println!("其他模式"),
    }
    
    // 复杂的数组模式
    let matrix_row = [1, 0, 0, 1];
    
    match matrix_row {
        [1, 0, 0, 1] => println!("单位矩阵行"),
        [0, 0, 0, 0] => println!("零行"),
        [a, 0, 0, b] if a == b => println!("对角线相等: {}", a),
        [a, b, c, d] => println!("一般行: [{}, {}, {}, {}]", a, b, c, d),
    }
}
```

## 🔄 函数参数中的模式

### 函数参数解构

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

// 在函数参数中解构结构体
fn print_coordinates(Point { x, y }: Point) {
    println!("坐标: ({}, {})", x, y);
}

// 解构元组参数
fn calculate_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

// 解构Option参数
fn process_optional_value(opt: Option<(i32, String)>) {
    match opt {
        Some((number, text)) => {
            println!("处理: 数字={}, 文本={}", number, text);
        },
        None => {
            println!("没有值可处理");
        }
    }
}

// 使用引用模式
fn print_point_ref(point: &Point) {
    match point {
        Point { x, y } => println!("引用点: ({}, {})", x, y),
    }
}

// 可变引用模式
fn move_point(point: &mut Point, dx: i32, dy: i32) {
    match point {
        Point { x, y } => {
            *x += dx;
            *y += dy;
        }
    }
}

// 复杂枚举参数解构
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => {
            std::f64::consts::PI * radius * radius
        },
        Shape::Rectangle { width, height } => {
            width * height
        },
        Shape::Triangle { base, height } => {
            0.5 * base * height
        }
    }
}

// 嵌套解构
fn process_nested_data(data: (Point, Option<String>)) {
    match data {
        (Point { x, y }, Some(label)) => {
            println!("标记点 '{}' 在 ({}, {})", label, x, y);
        },
        (Point { x, y }, None) => {
            println!("未标记点在 ({}, {})", x, y);
        }
    }
}

// 使用守卫的函数
fn categorize_point(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => "原点".to_string(),
        Point { x, y } if x > 0 && y > 0 => "第一象限".to_string(),
        Point { x, y } if x < 0 && y > 0 => "第二象限".to_string(),
        Point { x, y } if x < 0 && y < 0 => "第三象限".to_string(),
        Point { x, y } if x > 0 && y < 0 => "第四象限".to_string(),
        Point { x: 0, y } if y != 0 => "Y轴".to_string(),
        Point { x, y: 0 } if x != 0 => "X轴".to_string(),
        _ => "未知位置".to_string(),
    }
}

// 处理多个参数的解构
fn compare_points((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> String {
    match (x1.cmp(&x2), y1.cmp(&y2)) {
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => "相同点".to_string(),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => "第一个点在左下".to_string(),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => "第一个点在右上".to_string(),
        _ => "其他位置关系".to_string(),
    }
}

fn main() {
    // 测试各种函数
    let point1 = Point { x: 3, y: 4 };
    let point2 = Point { x: 6, y: 8 };
    
    print_coordinates(point1);
    
    let distance = calculate_distance((3.0, 4.0), (6.0, 8.0));
    println!("距离: {:.2}", distance);
    
    process_optional_value(Some((42, "hello".to_string())));
    process_optional_value(None);
    
    print_point_ref(&point2);
    
    let mut movable_point = Point { x: 0, y: 0 };
    println!("移动前: {:?}", movable_point);
    move_point(&mut movable_point, 5, 3);
    println!("移动后: {:?}", movable_point);
    
    let circle = Shape::Circle { radius: 5.0 };
    let rectangle = Shape::Rectangle { width: 4.0, height: 6.0 };
    let triangle = Shape::Triangle { base: 8.0, height: 3.0 };
    
    println!("圆形面积: {:.2}", calculate_area(circle));
    println!("矩形面积: {:.2}", calculate_area(rectangle));
    println!("三角形面积: {:.2}", calculate_area(triangle));
    
    let labeled_point = (Point { x: 10, y: 20 }, Some("重要点".to_string()));
    let unlabeled_point = (Point { x: -5, y: 15 }, None);
    
    process_nested_data(labeled_point);
    process_nested_data(unlabeled_point);
    
    let test_points = vec![
        Point { x: 0, y: 0 },
        Point { x: 3, y: 4 },
        Point { x: -2, y: 5 },
        Point { x: -3, y: -4 },
        Point { x: 6, y: -2 },
        Point { x: 0, y: 7 },
        Point { x: -8, y: 0 },
    ];
    
    for point in test_points {
        let category = categorize_point(point);
        println!("点 {:?} 位于: {}", point, category);
    }
    
    let comparison = compare_points((1, 2), (3, 4));
    println!("点比较结果: {}", comparison);
}
```

## 🧪 实践练习

### 练习1：状态机实现

```rust
// TODO: 实现一个简单的状态机
// 要求：
// 1. 定义不同的状态（开始、处理中、完成、错误）
// 2. 定义状态转换事件
// 3. 实现状态转换逻辑
// 4. 处理无效的状态转换
// 5. 记录状态转换历史

#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    Processing { progress: u8 },
    Completed { result: String },
    Error { message: String },
    Paused { saved_progress: u8 },
}

#[derive(Debug, Clone)]
enum Event {
    Start,
    Progress(u8),
    Complete(String),
    Error(String),
    Pause,
    Resume,
    Reset,
}

#[derive(Debug)]
struct StateMachine {
    current_state: State,
    history: Vec<(State, Event)>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::Idle,
            history: Vec::new(),
        }
    }
    
    // TODO: 实现状态转换
    fn transition(&mut self, event: Event) -> Result<(), String> {
        todo!("实现状态转换逻辑")
    }
    
    // TODO: 检查转换是否有效
    fn is_valid_transition(&self, event: &Event) -> bool {
        todo!("实现转换有效性检查")
    }
    
    // TODO: 获取当前状态
    fn current_state(&self) -> &State {
        &self.current_state
    }
    
    // TODO: 获取历史记录
    fn get_history(&self) -> &[(State, Event)] {
        &self.history
    }
    
    // TODO: 重置状态机
    fn reset(&mut self) {
        todo!("实现状态机重置")
    }
}

fn main() {
    println!("=== 状态机测试 ===");
    
    let mut sm = StateMachine::new();
    println!("初始状态: {:?}", sm.current_state());
    
    // 测试状态转换序列
    let events = vec![
        Event::Start,
        Event::Progress(25),
        Event::Progress(50),
        Event::Pause,
        Event::Resume,
        Event::Progress(75),
        Event::Complete("任务完成".to_string()),
    ];
    
    for event in events {
        println!("\n处理事件: {:?}", event);
        
        match sm.transition(event.clone()) {
            Ok(()) => {
                println!("转换成功，新状态: {:?}", sm.current_state());
            },
            Err(error) => {
                println!("转换失败: {}", error);
            }
        }
    }
    
    println!("\n=== 状态转换历史 ===");
    for (i, (state, event)) in sm.get_history().iter().enumerate() {
        println!("{}: {:?} -> {:?}", i + 1, event, state);
    }
    
    // 测试错误情况
    println!("\n=== 测试无效转换 ===");
    let invalid_events = vec![
        Event::Start,  // 已经完成，不能重新开始
        Event::Progress(10),  // 已经完成，不能设置进度
    ];
    
    for event in invalid_events {
        println!("\n尝试无效事件: {:?}", event);
        match sm.transition(event) {
            Ok(()) => println!("意外成功"),
            Err(error) => println!("预期失败: {}", error),
        }
    }
}
```

### 练习2：配置解析器

```rust
use std::collections::HashMap;

// TODO: 实现一个配置文件解析器
// 要求：
// 1. 支持多种配置值类型（字符串、数字、布尔值、数组）
// 2. 支持嵌套配置
// 3. 提供类型安全的访问方法
// 4. 处理配置验证和默认值
// 5. 支持配置合并

#[derive(Debug, Clone, PartialEq)]
enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
    Null,
}

#[derive(Debug)]
struct Config {
    data: HashMap<String, ConfigValue>,
}

#[derive(Debug)]
enum ConfigError {
    KeyNotFound(String),
    TypeMismatch { expected: String, found: String },
    ValidationError(String),
}

impl Config {
    fn new() -> Self {
        Config {
            data: HashMap::new(),
        }
    }
    
    // TODO: 设置配置值
    fn set(&mut self, key: &str, value: ConfigValue) {
        todo!("实现配置设置")
    }
    
    // TODO: 获取字符串值
    fn get_string(&self, key: &str) -> Result<&str, ConfigError> {
        todo!("实现字符串获取")
    }
    
    // TODO: 获取整数值
    fn get_integer(&self, key: &str) -> Result<i64, ConfigError> {
        todo!("实现整数获取")
    }
    
    // TODO: 获取浮点数值
    fn get_float(&self, key: &str) -> Result<f64, ConfigError> {
        todo!("实现浮点数获取")
    }
    
    // TODO: 获取布尔值
    fn get_boolean(&self, key: &str) -> Result<bool, ConfigError> {
        todo!("实现布尔值获取")
    }
    
    // TODO: 获取数组
    fn get_array(&self, key: &str) -> Result<&Vec<ConfigValue>, ConfigError> {
        todo!("实现数组获取")
    }
    
    // TODO: 获取嵌套对象
    fn get_object(&self, key: &str) -> Result<&HashMap<String, ConfigValue>, ConfigError> {
        todo!("实现对象获取")
    }
    
    // TODO: 获取嵌套值（支持点号路径）
    fn get_nested(&self, path: &str) -> Result<&ConfigValue, ConfigError> {
        todo!("实现嵌套值获取")
    }
    
    // TODO: 合并配置
    fn merge(&mut self, other: Config) {
        todo!("实现配置合并")
    }
    
    // TODO: 验证配置
    fn validate(&self, schema: &ConfigSchema) -> Result<(), Vec<ConfigError>> {
        todo!("实现配置验证")
    }
    
    // TODO: 应用默认值
    fn apply_defaults(&mut self, defaults: &Config) {
        todo!("实现默认值应用")
    }
}

#[derive(Debug)]
struct ConfigSchema {
    required_keys: Vec<String>,
    type_constraints: HashMap<String, String>,
    validators: HashMap<String, Box<dyn Fn(&ConfigValue) -> bool>>,
}

// TODO: 辅助函数
fn parse_config_line(line: &str) -> Option<(String, ConfigValue)> {
    todo!("实现配置行解析")
}

fn config_value_type_name(value: &ConfigValue) -> &'static str {
    todo!("实现类型名称获取")
}

fn main() {
    println!("=== 配置解析器测试 ===");
    
    let mut config = Config::new();
    
    // 设置各种类型的配置
    config.set("app_name", ConfigValue::String("MyApp".to_string()));
    config.set("port", ConfigValue::Integer(8080));
    config.set("debug", ConfigValue::Boolean(true));
    config.set("timeout", ConfigValue::Float(30.5));
    
    // 设置数组配置
    let hosts = vec![
        ConfigValue::String("localhost".to_string()),
        ConfigValue::String("127.0.0.1".to_string()),
    ];
    config.set("allowed_hosts", ConfigValue::Array(hosts));
    
    // 设置嵌套对象配置
    let mut database_config = HashMap::new();
    database_config.insert("host".to_string(), ConfigValue::String("localhost".to_string()));
    database_config.insert("port".to_string(), ConfigValue::Integer(5432));
    database_config.insert("name".to_string(), ConfigValue::String("mydb".to_string()));
    config.set("database", ConfigValue::Object(database_config));
    
    // 测试配置访问
    println!("\n=== 配置访问测试 ===");
    
    match config.get_string("app_name") {
        Ok(name) => println!("应用名称: {}", name),
        Err(e) => println!("获取应用名称失败: {:?}", e),
    }
    
    match config.get_integer("port") {
        Ok(port) => println!("端口: {}", port),
        Err(e) => println!("获取端口失败: {:?}", e),
    }
    
    match config.get_boolean("debug") {
        Ok(debug) => println!("调试模式: {}", debug),
        Err(e) => println!("获取调试模式失败: {:?}", e),
    }
    
    // 测试嵌套访问
    match config.get_nested("database.host") {
        Ok(ConfigValue::String(host)) => println!("数据库主机: {}", host),
        Ok(other) => println!("数据库主机类型错误: {:?}", other),
        Err(e) => println!("获取数据库主机失败: {:?}", e),
    }
    
    // 测试类型错误
    println!("\n=== 类型错误测试 ===");
    match config.get_integer("app_name") {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的类型错误: {:?}", e),
    }
    
    // 测试不存在的键
    match config.get_string("nonexistent") {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的键不存在错误: {:?}", e),
    }
    
    // 创建默认配置
    let mut defaults = Config::new();
    defaults.set("timeout", ConfigValue::Float(60.0));
    defaults.set("max_connections", ConfigValue::Integer(100));
    defaults.set("log_level", ConfigValue::String("info".to_string()));
    
    println!("\n=== 应用默认值前 ===");
    match config.get_integer("max_connections") {
        Ok(val) => println!("最大连接数: {}", val),
        Err(_) => println!("最大连接数未设置"),
    }
    
    config.apply_defaults(&defaults);
    
    println!("\n=== 应用默认值后 ===");
    match config.get_integer("max_connections") {
        Ok(val) => println!("最大连接数: {}", val),
        Err(e) => println!("获取最大连接数失败: {:?}", e),
    }
}
```

### 练习3：表达式求值器

```rust
// TODO: 实现一个简单的数学表达式求值器
// 要求：
// 1. 支持基本运算（+、-、*、/）
// 2. 支持括号
// 3. 支持变量
// 4. 支持函数调用
// 5. 提供错误处理和调试信息

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
enum UnaryOperator {
    Negate,
    Abs,
}

#[derive(Debug)]
enum EvalError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    DivisionByZero,
    InvalidArgumentCount { expected: usize, got: usize },
    TypeError(String),
}

#[derive(Debug)]
struct Context {
    variables: HashMap<String, f64>,
    functions: HashMap<String, fn(&[f64]) -> Result<f64, EvalError>>,
}

impl Context {
    fn new() -> Self {
        let mut ctx = Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
        
        // 添加内置函数
        ctx.functions.insert("sin".to_string(), |args| {
            if args.len() != 1 {
                return Err(EvalError::InvalidArgumentCount { expected: 1, got: args.len() });
            }
            Ok(args[0].sin())
        });
        
        ctx.functions.insert("cos".to_string(), |args| {
            if args.len() != 1 {
                return Err(EvalError::InvalidArgumentCount { expected: 1, got: args.len() });
            }
            Ok(args[0].cos())
        });
        
        ctx.functions.insert("sqrt".to_string(), |args| {
            if args.len() != 1 {
                return Err(EvalError::InvalidArgumentCount { expected: 1, got: args.len() });
            }
            if args[0] < 0.0 {
                return Err(EvalError::TypeError("负数不能开平方根".to_string()));
            }
            Ok(args[0].sqrt())
        });
        
        ctx
    }
    
    fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}

// TODO: 实现表达式求值
fn evaluate(expr: &Expr, context: &Context) -> Result<f64, EvalError> {
    todo!("实现表达式求值")
}

// TODO: 实现二元运算
fn apply_binary_op(op: &BinaryOperator, left: f64, right: f64) -> Result<f64, EvalError> {
    todo!("实现二元运算")
}

// TODO: 实现一元运算
fn apply_unary_op(op: &UnaryOperator, operand: f64) -> Result<f64, EvalError> {
    todo!("实现一元运算")
}

// TODO: 简化表达式（可选的优化功能）
fn simplify(expr: Expr) -> Expr {
    todo!("实现表达式简化")
}

// TODO: 表达式转字符串（用于调试）
fn expr_to_string(expr: &Expr) -> String {
    todo!("实现表达式转字符串")
}

fn main() {
    println!("=== 表达式求值器测试 ===");
    
    let mut context = Context::new();
    context.set_variable("x".to_string(), 5.0);
    context.set_variable("y".to_string(), 3.0);
    context.set_variable("pi".to_string(), std::f64::consts::PI);
    
    // 测试表达式
    let expressions = vec![
        // 基本运算
        Expr::BinaryOp {
            left: Box::new(Expr::Number(10.0)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Number(5.0)),
        },
        
        // 变量运算
        Expr::BinaryOp {
            left: Box::new(Expr::Variable("x".to_string())),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Variable("y".to_string())),
        },
        
        // 复杂表达式: (x + y) * 2
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Variable("x".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Variable("y".to_string())),
            }),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Number(2.0)),
        },
        
        // 函数调用: sin(pi/2)
        Expr::FunctionCall {
            name: "sin".to_string(),
            args: vec![
                Expr::BinaryOp {
                    left: Box::new(Expr::Variable("pi".to_string())),
                    op: BinaryOperator::Divide,
                    right: Box::new(Expr::Number(2.0)),
                }
            ],
        },
        
        // 一元运算: -x
        Expr::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(Expr::Variable("x".to_string())),
        },
    ];
    
    for (i, expr) in expressions.iter().enumerate() {
        println!("\n表达式 {}: {}", i + 1, expr_to_string(expr));
        
        match evaluate(expr, &context) {
            Ok(result) => println!("结果: {}", result),
            Err(error) => println!("错误: {:?}", error),
        }
    }
    
    // 测试错误情况
    println!("\n=== 错误处理测试 ===");
    
    // 未定义变量
    let undefined_var = Expr::Variable("z".to_string());
    match evaluate(&undefined_var, &context) {
        Ok(_) => println!("意外成功"),
        Err(error) => println!("未定义变量错误: {:?}", error),
    }
    
    // 除零错误
    let division_by_zero = Expr::BinaryOp {
        left: Box::new(Expr::Number(10.0)),
        op: BinaryOperator::Divide,
        right: Box::new(Expr::Number(0.0)),
    };
    match evaluate(&division_by_zero, &context) {
        Ok(_) => println!("意外成功"),
        Err(error) => println!("除零错误: {:?}", error),
    }
    
    // 未定义函数
    let undefined_func = Expr::FunctionCall {
        name: "unknown".to_string(),
        args: vec![Expr::Number(1.0)],
    };
    match evaluate(&undefined_func, &context) {
        Ok(_) => println!("意外成功"),
        Err(error) => println!("未定义函数错误: {:?}", error),
    }
}
```

## 📚 与其他语言的比较

### 与C/C++的比较

```rust
// Rust的模式匹配
fn rust_pattern_matching(value: Option<i32>) {
    match value {
        Some(x) if x > 0 => println!("正数: {}", x),
        Some(x) if x < 0 => println!("负数: {}", x),
        Some(0) => println!("零"),
        None => println!("无值"),
    }
}

/*
C++等价代码（使用std::optional，C++17）:

void cpp_pattern_matching(std::optional<int> value) {
    if (value.has_value()) {
        int x = value.value();
        if (x > 0) {
            std::cout << "正数: " << x << std::endl;
        } else if (x < 0) {
            std::cout << "负数: " << x << std::endl;
        } else {
            std::cout << "零" << std::endl;
        }
    } else {
        std::cout << "无值" << std::endl;
    }
}

传统C代码需要使用switch和if语句的组合，
无法提供Rust模式匹配的表达力和安全性。
*/
```

### 与Python的比较

```rust
// Rust的结构化匹配
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn rust_shape_matching(shape: Shape) {
    match shape {
        Shape::Circle(r) if r > 0.0 => {
            println!("圆形，半径: {}, 面积: {:.2}", r, std::f64::consts::PI * r * r);
        },
        Shape::Rectangle(w, h) if w > 0.0 && h > 0.0 => {
            println!("矩形，宽: {}, 高: {}, 面积: {:.2}", w, h, w * h);
        },
        Shape::Triangle(a, b, c) if a > 0.0 && b > 0.0 && c > 0.0 => {
            let s = (a + b + c) / 2.0;
            let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
            println!("三角形，边长: {}, {}, {}, 面积: {:.2}", a, b, c, area);
        },
        _ => println!("无效的形状参数"),
    }
}

/*
Python 3.10+的结构化模式匹配:

def python_shape_matching(shape):
    match shape:
        case ('circle', r) if r > 0:
            print(f"圆形，半径: {r}, 面积: {3.14159 * r * r:.2f}")
        case ('rectangle', w, h) if w > 0 and h > 0:
            print(f"矩形，宽: {w}, 高: {h}, 面积: {w * h:.2f}")
        case ('triangle', a, b, c) if a > 0 and b > 0 and c > 0:
            s = (a + b + c) / 2
            area = (s * (s - a) * (s - b) * (s - c)) ** 0.5
            print(f"三角形，边长: {a}, {b}, {c}, 面积: {area:.2f}")
        case _:
            print("无效的形状参数")

# Python的模式匹配功能较新，语法相似但类型安全性不如Rust
*/
```

## 🎯 最佳实践

### 1. 穷尽性检查

```rust
#[derive(Debug)]
enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
}

// 好的做法：处理所有情况
fn handle_status_good(status: Status) {
    match status {
        Status::Pending => println!("等待中"),
        Status::InProgress => println!("进行中"),
        Status::Completed => println!("已完成"),
        Status::Failed => println!("失败"),
    }
}

// 不推荐：使用通配符可能遗漏新增的枚举值
fn handle_status_bad(status: Status) {
    match status {
        Status::Pending => println!("等待中"),
        Status::InProgress => println!("进行中"),
        _ => println!("其他状态"),  // 可能遗漏重要的状态处理
    }
}
```

### 2. 合理使用if let

```rust
// 好的做法：只关心一种情况时使用if let
fn process_some_value(opt: Option<i32>) {
    if let Some(value) = opt {
        println!("处理值: {}", value);
        // 只处理Some的情况，忽略None
    }
}

// 不好的做法：需要处理多种情况时不要用if let
fn process_all_cases_bad(opt: Option<i32>) {
    if let Some(value) = opt {
        println!("有值: {}", value);
    } else {
        println!("无值");
    }
    // 应该使用match更清晰
}

// 好的做法：处理多种情况使用match
fn process_all_cases_good(opt: Option<i32>) {
    match opt {
        Some(value) => println!("有值: {}", value),
        None => println!("无值"),
    }
}
```

### 3. 避免深度嵌套

```rust
// 不好的做法：深度嵌套
fn nested_bad(opt1: Option<Option<i32>>) {
    match opt1 {
        Some(opt2) => {
            match opt2 {
                Some(value) => println!("值: {}", value),
                None => println!("内层为空"),
            }
        },
        None => println!("外层为空"),
    }
}

// 好的做法：使用模式匹配直接处理
fn nested_good(opt1: Option<Option<i32>>) {
    match opt1 {
        Some(Some(value)) => println!("值: {}", value),
        Some(None) => println!("内层为空"),
        None => println!("外层为空"),
    }
}

// 或者使用flatten
fn nested_with_flatten(opt1: Option<Option<i32>>) {
    match opt1.flatten() {
        Some(value) => println!("值: {}", value),
        None => println!("没有值"),
    }
}
```

## ❌ 常见错误

### 1. 忘记处理所有情况

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,  // 新增的颜色
}

// 编译错误：没有处理所有情况
/*
fn bad_color_match(color: Color) {
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue => println!("蓝色"),
        // 缺少Yellow的处理，编译器会报错
    }
}
*/

// 正确的做法
fn good_color_match(color: Color) {
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue => println!("蓝色"),
        Color::Yellow => println!("黄色"),
    }
}
```

### 2. 模式顺序错误

```rust
// 错误：通配符模式应该放在最后
/*
fn bad_pattern_order(x: i32) {
    match x {
        _ => println!("任何数字"),  // 这会匹配所有情况
        1 => println!("一"),        // 永远不会执行
        2 => println!("二"),        // 永远不会执行
    }
}
*/

// 正确：具体模式在前，通配符在后
fn good_pattern_order(x: i32) {
    match x {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他数字"),
    }
}
```

### 3. 所有权问题

```rust
#[derive(Debug)]
struct Data {
    value: String,
}

// 错误：移动了所有权
/*
fn bad_ownership(data: Data) {
    match data {
        Data { value } => {
            println!("值: {}", value);
            // data的所有权被移动了
        }
    }
    // println!("{:?}", data);  // 编译错误：data已被移动
}
*/

// 正确：使用引用
fn good_ownership(data: &Data) {
    match data {
        Data { value } => {
            println!("值: {}", value);
            // 没有移动所有权
        }
    }
    println!("{:?}", data);  // 正常使用
}

// 或者使用ref模式
fn good_ownership_ref(data: Data) {
    match data {
        Data { ref value } => {
            println!("值: {}", value);
            // 使用引用，不移动所有权
        }
    }
    println!("{:?}", data);  // 正常使用
}
```

## ✅ 学习检查清单

- [ ] 理解match表达式的基本语法
- [ ] 掌握各种模式类型（字面量、变量、通配符、范围）
- [ ] 学会使用守卫条件
- [ ] 理解@绑定的用法
- [ ] 掌握if let和while let的使用场景
- [ ] 学会解构复杂数据类型（结构体、枚举、元组、数组）
- [ ] 理解模式匹配中的所有权规则
- [ ] 能够在函数参数中使用模式
- [ ] 掌握模式匹配的最佳实践
- [ ] 了解常见错误及其避免方法

## 📖 扩展阅读

- [Rust官方文档 - 模式和匹配](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference - 模式](https://doc.rust-lang.org/reference/patterns.html)
- [Rust by Example - 模式匹配](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [模式匹配最佳实践](https://rust-lang.github.io/api-guidelines/)

---

**下一节预告：** 在下一节中，我们将学习Rust的函数进阶特性，包括闭包、高阶函数和函数式编程概念。