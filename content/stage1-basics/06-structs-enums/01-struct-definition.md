# 结构体定义和使用

## 学习目标

通过本节学习，你将能够：

- [ ] 理解结构体的概念和用途
- [ ] 掌握结构体的定义语法
- [ ] 学会创建和初始化结构体实例
- [ ] 理解字段访问和修改
- [ ] 掌握结构体的所有权规则
- [ ] 学会使用结构体更新语法
- [ ] 理解元组结构体和单元结构体

## 结构体基础概念

### 什么是结构体

结构体（struct）是一种自定义数据类型，允许你将多个相关的值组合在一起形成一个有意义的组合。结构体类似于其他语言中的对象或记录。

### 基本结构体定义

```rust
// 定义一个用户结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 定义一个矩形结构体
struct Rectangle {
    width: u32,
    height: u32,
}

// 定义一个点结构体
struct Point {
    x: f64,
    y: f64,
}
```

### 创建结构体实例

```rust
fn main() {
    // 创建用户实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 创建矩形实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 创建点实例
    let point1 = Point { x: 0.0, y: 0.0 };
    
    println!("用户名: {}", user1.username);
    println!("矩形面积: {}", rect1.width * rect1.height);
    println!("点坐标: ({}, {})", point1.x, point1.y);
}
```

## 字段访问和修改

### 访问结构体字段

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // 使用点号访问字段
    println!("姓名: {}", person.name);
    println!("年龄: {}", person.age);
}
```

### 修改结构体字段

```rust
struct Counter {
    value: i32,
}

fn main() {
    // 必须使用 mut 关键字使结构体可变
    let mut counter = Counter { value: 0 };
    
    println!("初始值: {}", counter.value);
    
    // 修改字段值
    counter.value += 1;
    println!("增加后: {}", counter.value);
    
    counter.value *= 2;
    println!("翻倍后: {}", counter.value);
}
```

### 部分字段可变性

```rust
// 注意：Rust 不支持部分字段可变性
// 整个结构体实例要么可变，要么不可变

struct Config {
    debug: bool,
    max_connections: u32,
}

fn main() {
    let mut config = Config {
        debug: true,
        max_connections: 100,
    };
    
    // 可以修改任何字段
    config.debug = false;
    config.max_connections = 200;
    
    println!("调试模式: {}", config.debug);
    println!("最大连接数: {}", config.max_connections);
}
```

## 结构体构造函数

### 使用函数创建结构体

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 构造函数
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 使用字段初始化简写语法
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,      // 等同于 email: email
        username,   // 等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    
    let user2 = build_user_shorthand(
        String::from("another@example.com"),
        String::from("anotherusername456"),
    );
    
    println!("用户1: {}", user1.username);
    println!("用户2: {}", user2.username);
}
```

## 结构体更新语法

### 基于现有实例创建新实例

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 使用结构体更新语法创建新实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        ..user1  // 其余字段从 user1 复制
    };
    
    // 注意：user1 的 String 字段被移动了，user1 不再有效
    // println!("{}", user1.username); // 这会导致编译错误
    
    println!("用户2邮箱: {}", user2.email);
    println!("用户2活跃状态: {}", user2.active);
}
```

### 所有权和借用注意事项

```rust
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point1 = Point { x: 1, y: 2 };
    
    // 对于实现了 Copy trait 的类型，更新语法不会移动所有权
    let point2 = Point {
        x: 5,
        ..point1
    };
    
    // point1 仍然有效，因为 i32 实现了 Copy
    println!("点1: ({}, {})", point1.x, point1.y);
    println!("点2: ({}, {})", point2.x, point2.y);
    
    // 使用 clone 避免所有权问题
    let point3 = Point {
        y: 10,
        ..point1.clone()
    };
    
    println!("点3: ({}, {})", point3.x, point3.y);
}
```

## 元组结构体

### 定义和使用元组结构体

```rust
// 元组结构体：有名称但字段没有名称
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 访问元组结构体的字段
    println!("黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 解构元组结构体
    let Color(r, g, b) = black;
    println!("红色分量: {}", r);
    
    // 元组结构体可以有不同的类型，即使字段相同
    // let mixed: Point = black; // 编译错误：类型不匹配
}
```

### 新类型模式

```rust
// 使用元组结构体创建新类型
struct Meters(f64);
struct Seconds(f64);

fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}

fn main() {
    let distance = Meters(100.0);
    let time = Seconds(10.0);
    
    let speed = calculate_speed(distance, time);
    println!("速度: {} m/s", speed);
    
    // 类型安全：不能混用不同的新类型
    // let wrong_speed = calculate_speed(time, distance); // 编译错误
}
```

## 单元结构体

### 定义和使用单元结构体

```rust
// 单元结构体：没有任何字段
struct AlwaysEqual;

// 实现 trait 的标记类型
struct FileMarker;
struct NetworkMarker;

fn main() {
    let subject = AlwaysEqual;
    let file_marker = FileMarker;
    let network_marker = NetworkMarker;
    
    // 单元结构体主要用于实现 trait 或作为标记类型
    println!("单元结构体创建成功");
}
```

## 结构体的调试输出

### 使用 Debug trait

```rust
// 自动派生 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let point1 = Point { x: 1.5, y: 2.5 };
    
    // 使用 {:?} 进行调试输出
    println!("矩形: {:?}", rect1);
    println!("点: {:?}", point1);
    
    // 使用 {:#?} 进行美化输出
    println!("矩形（美化）: {:#?}", rect1);
}
```

### 自定义调试输出

```rust
use std::fmt;

struct Person {
    name: String,
    age: u32,
}

// 手动实现 Debug trait
impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person {{ name: \"{}\", age: {} }}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("人员信息: {:?}", person);
}
```

## 实际应用示例

### 图书管理系统

```rust
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    pages: u32,
    available: bool,
}

#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<Book>,
}

impl Book {
    fn new(title: String, author: String, isbn: String, pages: u32) -> Book {
        Book {
            title,
            author,
            isbn,
            pages,
            available: true,
        }
    }
    
    fn borrow_book(&mut self) -> Result<(), String> {
        if self.available {
            self.available = false;
            Ok(())
        } else {
            Err("书籍已被借出".to_string())
        }
    }
    
    fn return_book(&mut self) {
        self.available = true;
    }
}

impl Library {
    fn new(name: String) -> Library {
        Library {
            name,
            books: Vec::new(),
        }
    }
    
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    fn find_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }
    
    fn available_books(&self) -> Vec<&Book> {
        self.books.iter().filter(|book| book.available).collect()
    }
}

fn main() {
    let mut library = Library::new("市图书馆".to_string());
    
    // 添加书籍
    let book1 = Book::new(
        "Rust程序设计语言".to_string(),
        "Steve Klabnik".to_string(),
        "978-1718500440".to_string(),
        552,
    );
    
    let book2 = Book::new(
        "深入浅出Rust".to_string(),
        "范长春".to_string(),
        "978-7111606888".to_string(),
        400,
    );
    
    library.add_book(book1);
    library.add_book(book2);
    
    // 查找书籍
    if let Some(book) = library.find_book_by_title("Rust程序设计语言") {
        println!("找到书籍: {:?}", book);
    }
    
    // 显示可用书籍
    let available = library.available_books();
    println!("可用书籍数量: {}", available.len());
    
    for book in available {
        println!("- {}", book.title);
    }
}
```

### 几何计算系统

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Circle {
    fn new(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    fn contains_point(&self, point: &Point) -> bool {
        self.center.distance_to(point) <= self.radius
    }
}

impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left,
            bottom_right,
        }
    }
    
    fn area(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
    
    fn perimeter(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        2.0 * (width + height)
    }
    
    fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y <= self.top_left.y
            && point.y >= self.bottom_right.y
    }
}

fn main() {
    // 创建几何图形
    let center = Point::new(0.0, 0.0);
    let circle = Circle::new(center, 5.0);
    
    let rect = Rectangle::new(
        Point::new(-3.0, 3.0),
        Point::new(3.0, -3.0),
    );
    
    // 计算属性
    println!("圆形面积: {:.2}", circle.area());
    println!("圆形周长: {:.2}", circle.circumference());
    
    println!("矩形面积: {:.2}", rect.area());
    println!("矩形周长: {:.2}", rect.perimeter());
    
    // 测试点包含
    let test_point = Point::new(2.0, 2.0);
    println!("点 ({}, {}) 在圆内: {}", 
             test_point.x, test_point.y, 
             circle.contains_point(&test_point));
    println!("点 ({}, {}) 在矩形内: {}", 
             test_point.x, test_point.y, 
             rect.contains_point(&test_point));
}
```

## 与其他语言的比较

### 与 C/C++ 的比较

```rust
// Rust 结构体
struct Person {
    name: String,    // 自动管理内存
    age: u32,
}

// C++ 等价代码（简化）:
// struct Person {
//     std::string name;  // 需要手动管理或使用智能指针
//     unsigned int age;
// };

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Rust 自动处理内存管理，无需手动释放
    println!("姓名: {}", person.name);
} // person 在这里自动释放
```

### 与 Python 的比较

```rust
// Rust 结构体（编译时类型检查）
struct Point {
    x: f64,
    y: f64,
}

// Python 等价代码:
// class Point:
//     def __init__(self, x, y):
//         self.x = x
//         self.y = y

fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    
    // Rust 在编译时检查类型
    // point.x = "invalid"; // 编译错误
    
    println!("坐标: ({}, {})", point.x, point.y);
}
```

## 最佳实践

### 1. 结构体设计原则

```rust
// 好的设计：字段相关且内聚
#[derive(Debug)]
struct User {
    id: u64,
    username: String,
    email: String,
    created_at: std::time::SystemTime,
}

// 避免：不相关的字段组合
// struct BadDesign {
//     user_name: String,
//     temperature: f64,  // 与用户无关
//     color: String,     // 与用户无关
// }
```

### 2. 使用构造函数

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 提供构造函数而不是直接创建
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 提供验证的构造函数
    fn new_validated(width: u32, height: u32) -> Result<Rectangle, String> {
        if width == 0 || height == 0 {
            Err("宽度和高度必须大于0".to_string())
        } else {
            Ok(Rectangle { width, height })
        }
    }
}

fn main() {
    let rect1 = Rectangle::new(10, 20);
    
    match Rectangle::new_validated(0, 10) {
        Ok(rect) => println!("创建成功: {:?}", rect),
        Err(e) => println!("创建失败: {}", e),
    }
}
```

### 3. 合理使用派生宏

```rust
// 根据需要派生 trait
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 对于包含不可复制类型的结构体，谨慎使用 Clone
#[derive(Debug)] // 不派生 Clone，因为 String 的克隆可能昂贵
struct User {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // 便宜的复制
    
    println!("点相等: {}", p1 == p2);
}
```

## 常见错误

### 1. 所有权和借用错误

```rust
struct Container {
    data: String,
}

fn main() {
    let container1 = Container {
        data: String::from("hello"),
    };
    
    // 错误：尝试部分移动
    // let container2 = Container {
    //     data: container1.data, // 移动了 data
    // };
    // println!("{}", container1.data); // 编译错误：值已被移动
    
    // 正确：克隆或借用
    let container2 = Container {
        data: container1.data.clone(),
    };
    
    println!("容器1: {}", container1.data); // 现在可以使用
    println!("容器2: {}", container2.data);
}
```

### 2. 可变性错误

```rust
struct Counter {
    value: i32,
}

fn main() {
    let counter = Counter { value: 0 };
    
    // 错误：尝试修改不可变结构体
    // counter.value += 1; // 编译错误
    
    // 正确：声明为可变
    let mut mutable_counter = Counter { value: 0 };
    mutable_counter.value += 1;
    
    println!("计数器值: {}", mutable_counter.value);
}
```

### 3. 字段初始化错误

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // 错误：缺少字段
    // let person = Person {
    //     name: String::from("Alice"),
    //     // 缺少 age 字段
    // };
    
    // 正确：提供所有字段
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("人员: {} ({}岁)", person.name, person.age);
}
```

## 学习检查清单

完成本节学习后，请确认你能够：

- [ ] 定义基本的结构体
- [ ] 创建和初始化结构体实例
- [ ] 访问和修改结构体字段
- [ ] 理解结构体的所有权规则
- [ ] 使用字段初始化简写语法
- [ ] 使用结构体更新语法
- [ ] 定义和使用元组结构体
- [ ] 定义和使用单元结构体
- [ ] 为结构体派生和实现 trait
- [ ] 编写结构体的构造函数
- [ ] 避免常见的所有权和可变性错误

## 扩展阅读

- [Rust Book - 结构体](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Reference - 结构体](https://doc.rust-lang.org/reference/items/structs.html)
- [Rust by Example - 结构体](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

---

**下一节预告**：在下一节中，我们将学习结构体的方法和关联函数，了解如何为结构体添加行为和功能。