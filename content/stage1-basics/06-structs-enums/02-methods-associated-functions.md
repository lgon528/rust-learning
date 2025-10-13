# 方法和关联函数

## 学习目标

通过本节学习，你将能够：

- [ ] 理解方法和关联函数的概念
- [ ] 掌握 `impl` 块的使用
- [ ] 学会定义和调用方法
- [ ] 理解 `self`、`&self` 和 `&mut self` 的区别
- [ ] 掌握关联函数的定义和调用
- [ ] 学会方法链式调用
- [ ] 理解多个 `impl` 块的使用
- [ ] 掌握方法的最佳实践

## 方法基础概念

### 什么是方法

方法（method）是与特定类型关联的函数。方法的第一个参数总是 `self`，它代表调用该方法的结构体实例。

### 基本方法定义

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用 impl 块为 Rectangle 定义方法
impl Rectangle {
    // 方法：计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 方法：检查是否为正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // 方法：获取周长
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形面积: {}", rect.area());
    println!("是否为正方形: {}", rect.is_square());
    println!("矩形周长: {}", rect.perimeter());
}
```

## self 参数的不同形式

### `&self` - 不可变借用

```rust
struct Book {
    title: String,
    pages: u32,
    available: bool,
}

impl Book {
    // 只读访问，不修改结构体
    fn get_title(&self) -> &str {
        &self.title
    }
    
    fn get_pages(&self) -> u32 {
        self.pages
    }
    
    fn is_available(&self) -> bool {
        self.available
    }
    
    // 计算阅读时间（假设每页2分钟）
    fn estimated_reading_time(&self) -> u32 {
        self.pages * 2
    }
}

fn main() {
    let book = Book {
        title: String::from("Rust程序设计语言"),
        pages: 552,
        available: true,
    };
    
    println!("书名: {}", book.get_title());
    println!("页数: {}", book.get_pages());
    println!("预计阅读时间: {}分钟", book.estimated_reading_time());
    
    // book 仍然可用，因为方法只是借用了它
    println!("是否可用: {}", book.is_available());
}
```

### `&mut self` - 可变借用

```rust
struct Counter {
    value: i32,
}

impl Counter {
    // 修改结构体的方法需要 &mut self
    fn increment(&mut self) {
        self.value += 1;
    }
    
    fn decrement(&mut self) {
        self.value -= 1;
    }
    
    fn add(&mut self, amount: i32) {
        self.value += amount;
    }
    
    fn reset(&mut self) {
        self.value = 0;
    }
    
    // 只读方法仍然使用 &self
    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut counter = Counter { value: 0 };
    
    println!("初始值: {}", counter.get_value());
    
    counter.increment();
    println!("增加后: {}", counter.get_value());
    
    counter.add(5);
    println!("添加5后: {}", counter.get_value());
    
    counter.reset();
    println!("重置后: {}", counter.get_value());
}
```

### `self` - 获取所有权

```rust
struct Message {
    content: String,
    processed: bool,
}

impl Message {
    // 消费 self，返回处理后的内容
    fn process(mut self) -> String {
        self.processed = true;
        format!("已处理: {}", self.content)
    }
    
    // 消费 self，转换为另一种类型
    fn into_string(self) -> String {
        self.content
    }
    
    // 只读方法
    fn is_processed(&self) -> bool {
        self.processed
    }
}

fn main() {
    let message = Message {
        content: String::from("Hello, World!"),
        processed: false,
    };
    
    println!("处理前: {}", message.is_processed());
    
    // process 方法消费了 message
    let processed_content = message.process();
    println!("{}", processed_content);
    
    // message 在这里不再可用
    // println!("{}", message.content); // 编译错误
    
    // 创建新的消息用于演示 into_string
    let another_message = Message {
        content: String::from("Another message"),
        processed: false,
    };
    
    let content = another_message.into_string();
    println!("提取的内容: {}", content);
}
```

## 关联函数

### 定义关联函数

关联函数是与类型关联但不以 `self` 作为第一个参数的函数，通常用作构造函数。

```rust
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    // 关联函数：构造函数
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    
    // 关联函数：创建单位圆
    fn unit_circle() -> Circle {
        Circle { radius: 1.0 }
    }
    
    // 关联函数：从直径创建圆
    fn from_diameter(diameter: f64) -> Circle {
        Circle {
            radius: diameter / 2.0,
        }
    }
    
    // 方法：计算面积
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    // 方法：计算周长
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn main() {
    // 使用关联函数创建实例
    let circle1 = Circle::new(5.0);
    let circle2 = Circle::unit_circle();
    let circle3 = Circle::from_diameter(10.0);
    
    println!("圆1: {:?}, 面积: {:.2}", circle1, circle1.area());
    println!("圆2: {:?}, 面积: {:.2}", circle2, circle2.area());
    println!("圆3: {:?}, 面积: {:.2}", circle3, circle3.area());
}
```

### 带验证的构造函数

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 基本构造函数
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    // 带验证的构造函数
    fn new_validated(name: String, age: u32) -> Result<Person, String> {
        if name.trim().is_empty() {
            return Err("姓名不能为空".to_string());
        }
        
        if age > 150 {
            return Err("年龄不能超过150岁".to_string());
        }
        
        Ok(Person { name, age })
    }
    
    // 默认构造函数
    fn default() -> Person {
        Person {
            name: String::from("未知"),
            age: 0,
        }
    }
    
    // 方法：获取信息
    fn get_info(&self) -> String {
        format!("{} ({}岁)", self.name, self.age)
    }
}

fn main() {
    // 使用不同的构造函数
    let person1 = Person::new("Alice".to_string(), 30);
    println!("人员1: {}", person1.get_info());
    
    // 使用验证构造函数
    match Person::new_validated("Bob".to_string(), 25) {
        Ok(person) => println!("人员2: {}", person.get_info()),
        Err(e) => println!("创建失败: {}", e),
    }
    
    // 测试验证失败
    match Person::new_validated("".to_string(), 200) {
        Ok(person) => println!("人员3: {}", person.get_info()),
        Err(e) => println!("创建失败: {}", e),
    }
    
    // 使用默认构造函数
    let person4 = Person::default();
    println!("人员4: {}", person4.get_info());
}
```

## 方法链式调用

### 设计支持链式调用的方法

```rust
#[derive(Debug)]
struct StringBuilder {
    content: String,
}

impl StringBuilder {
    fn new() -> StringBuilder {
        StringBuilder {
            content: String::new(),
        }
    }
    
    // 返回 self 以支持链式调用
    fn append(mut self, text: &str) -> Self {
        self.content.push_str(text);
        self
    }
    
    fn append_line(mut self, text: &str) -> Self {
        self.content.push_str(text);
        self.content.push('\n');
        self
    }
    
    fn append_char(mut self, ch: char) -> Self {
        self.content.push(ch);
        self
    }
    
    fn clear(mut self) -> Self {
        self.content.clear();
        self
    }
    
    // 终结方法：返回最终结果
    fn build(self) -> String {
        self.content
    }
    
    // 查看方法：不消费 self
    fn preview(&self) -> &str {
        &self.content
    }
}

fn main() {
    // 链式调用
    let result = StringBuilder::new()
        .append("Hello, ")
        .append("World!")
        .append_line("")
        .append_line("这是第二行")
        .append_char('!')
        .build();
    
    println!("构建结果:\n{}", result);
    
    // 中间查看
    let builder = StringBuilder::new()
        .append("正在构建...")
        .append_line("");
    
    println!("中间预览: {}", builder.preview());
    
    let final_result = builder
        .append("完成!")
        .build();
    
    println!("最终结果: {}", final_result);
}
```

### 可变引用的链式调用

```rust
#[derive(Debug)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }
    
    // 返回 &mut self 以支持可变链式调用
    fn translate(&mut self, dx: f64, dy: f64) -> &mut Self {
        self.x += dx;
        self.y += dy;
        self
    }
    
    fn scale(&mut self, factor: f64) -> &mut Self {
        self.x *= factor;
        self.y *= factor;
        self
    }
    
    fn normalize(&mut self) -> &mut Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
        }
        self
    }
    
    fn rotate(&mut self, angle: f64) -> &mut Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let new_x = self.x * cos_a - self.y * sin_a;
        let new_y = self.x * sin_a + self.y * cos_a;
        self.x = new_x;
        self.y = new_y;
        self
    }
    
    // 只读方法
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let mut vector = Vector2D::new(3.0, 4.0);
    println!("初始向量: {:?}, 长度: {:.2}", vector, vector.length());
    
    // 链式调用修改向量
    vector
        .translate(1.0, 1.0)
        .scale(2.0)
        .rotate(std::f64::consts::PI / 4.0);
    
    println!("变换后向量: {:?}, 长度: {:.2}", vector, vector.length());
    
    // 继续链式调用
    vector.normalize();
    println!("标准化后向量: {:?}, 长度: {:.2}", vector, vector.length());
}
```

## 多个 impl 块

### 组织相关功能

```rust
#[derive(Debug)]
struct BankAccount {
    account_number: String,
    balance: f64,
    is_active: bool,
}

// 基本操作
impl BankAccount {
    fn new(account_number: String, initial_balance: f64) -> BankAccount {
        BankAccount {
            account_number,
            balance: initial_balance,
            is_active: true,
        }
    }
    
    fn get_balance(&self) -> f64 {
        self.balance
    }
    
    fn is_active(&self) -> bool {
        self.is_active
    }
}

// 交易操作
impl BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if !self.is_active {
            return Err("账户已被冻结".to_string());
        }
        
        if amount <= 0.0 {
            return Err("存款金额必须大于0".to_string());
        }
        
        self.balance += amount;
        Ok(())
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if !self.is_active {
            return Err("账户已被冻结".to_string());
        }
        
        if amount <= 0.0 {
            return Err("取款金额必须大于0".to_string());
        }
        
        if amount > self.balance {
            return Err("余额不足".to_string());
        }
        
        self.balance -= amount;
        Ok(())
    }
    
    fn transfer(&mut self, to_account: &mut BankAccount, amount: f64) -> Result<(), String> {
        self.withdraw(amount)?;
        match to_account.deposit(amount) {
            Ok(()) => Ok(()),
            Err(e) => {
                // 如果存款失败，回滚取款
                self.balance += amount;
                Err(format!("转账失败: {}", e))
            }
        }
    }
}

// 账户管理
impl BankAccount {
    fn freeze(&mut self) {
        self.is_active = false;
    }
    
    fn unfreeze(&mut self) {
        self.is_active = true;
    }
    
    fn close_account(self) -> f64 {
        // 返回剩余余额并消费账户
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount::new("ACC001".to_string(), 1000.0);
    let mut account2 = BankAccount::new("ACC002".to_string(), 500.0);
    
    println!("账户1余额: {:.2}", account1.get_balance());
    println!("账户2余额: {:.2}", account2.get_balance());
    
    // 存款
    match account1.deposit(200.0) {
        Ok(()) => println!("存款成功"),
        Err(e) => println!("存款失败: {}", e),
    }
    
    // 转账
    match account1.transfer(&mut account2, 300.0) {
        Ok(()) => println!("转账成功"),
        Err(e) => println!("转账失败: {}", e),
    }
    
    println!("转账后账户1余额: {:.2}", account1.get_balance());
    println!("转账后账户2余额: {:.2}", account2.get_balance());
}
```

## 实际应用示例

### 任务管理系统

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
    priority: u8, // 1-5, 5 是最高优先级
}

#[derive(Debug)]
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

// 任务操作
impl Task {
    fn new(title: String, description: String, priority: u8) -> Task {
        Task {
            id: 0, // 将由 TaskManager 设置
            title,
            description,
            status: TaskStatus::Todo,
            priority: priority.min(5).max(1), // 限制在 1-5 范围内
        }
    }
    
    fn start(&mut self) -> Result<(), String> {
        match self.status {
            TaskStatus::Todo => {
                self.status = TaskStatus::InProgress;
                Ok(())
            }
            _ => Err("只能开始待办任务".to_string()),
        }
    }
    
    fn complete(&mut self) -> Result<(), String> {
        match self.status {
            TaskStatus::InProgress => {
                self.status = TaskStatus::Done;
                Ok(())
            }
            TaskStatus::Todo => {
                self.status = TaskStatus::Done;
                Ok(())
            }
            TaskStatus::Done => Err("任务已完成".to_string()),
        }
    }
    
    fn reset(&mut self) {
        self.status = TaskStatus::Todo;
    }
    
    fn update_priority(&mut self, priority: u8) {
        self.priority = priority.min(5).max(1);
    }
}

// 任务管理器操作
impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
    
    fn add_task(&mut self, mut task: Task) -> u32 {
        let id = self.next_id;
        task.id = id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }
    
    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }
    
    fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }
    
    fn remove_task(&mut self, id: u32) -> Option<Task> {
        self.tasks.remove(&id)
    }
    
    fn list_tasks_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| matches!(task.status, ref s if std::mem::discriminant(s) == std::mem::discriminant(&status)))
            .collect()
    }
    
    fn list_high_priority_tasks(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.priority >= 4)
            .collect()
    }
}

// 便捷操作
impl TaskManager {
    fn start_task(&mut self, id: u32) -> Result<(), String> {
        match self.get_task_mut(id) {
            Some(task) => task.start(),
            None => Err("任务不存在".to_string()),
        }
    }
    
    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        match self.get_task_mut(id) {
            Some(task) => task.complete(),
            None => Err("任务不存在".to_string()),
        }
    }
    
    fn update_task_priority(&mut self, id: u32, priority: u8) -> Result<(), String> {
        match self.get_task_mut(id) {
            Some(task) => {
                task.update_priority(priority);
                Ok(())
            }
            None => Err("任务不存在".to_string()),
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    
    // 添加任务
    let task1 = Task::new(
        "学习 Rust".to_string(),
        "完成 Rust 基础教程".to_string(),
        5,
    );
    
    let task2 = Task::new(
        "写文档".to_string(),
        "为项目编写 README".to_string(),
        3,
    );
    
    let id1 = task_manager.add_task(task1);
    let id2 = task_manager.add_task(task2);
    
    println!("添加了任务 {} 和 {}", id1, id2);
    
    // 开始任务
    match task_manager.start_task(id1) {
        Ok(()) => println!("任务 {} 已开始", id1),
        Err(e) => println!("开始任务失败: {}", e),
    }
    
    // 完成任务
    match task_manager.complete_task(id1) {
        Ok(()) => println!("任务 {} 已完成", id1),
        Err(e) => println!("完成任务失败: {}", e),
    }
    
    // 列出高优先级任务
    let high_priority = task_manager.list_high_priority_tasks();
    println!("高优先级任务数量: {}", high_priority.len());
    
    for task in high_priority {
        println!("- {} (优先级: {})", task.title, task.priority);
    }
}
```

## 最佳实践

### 1. 方法命名约定

```rust
struct User {
    name: String,
    email: String,
    active: bool,
}

impl User {
    // 构造函数使用 new
    fn new(name: String, email: String) -> User {
        User {
            name,
            email,
            active: true,
        }
    }
    
    // 获取器使用 get_ 前缀或直接使用字段名
    fn name(&self) -> &str {
        &self.name
    }
    
    fn get_email(&self) -> &str {
        &self.email
    }
    
    // 布尔值获取器使用 is_ 前缀
    fn is_active(&self) -> bool {
        self.active
    }
    
    // 设置器使用 set_ 前缀
    fn set_email(&mut self, email: String) {
        self.email = email;
    }
    
    // 状态改变使用动词
    fn activate(&mut self) {
        self.active = true;
    }
    
    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

### 2. 错误处理

```rust
#[derive(Debug)]
enum ValidationError {
    EmptyName,
    InvalidEmail,
    AgeTooYoung,
    AgeTooOld,
}

struct Person {
    name: String,
    email: String,
    age: u8,
}

impl Person {
    fn new(name: String, email: String, age: u8) -> Result<Person, ValidationError> {
        if name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }
        
        if !email.contains('@') {
            return Err(ValidationError::InvalidEmail);
        }
        
        if age < 13 {
            return Err(ValidationError::AgeTooYoung);
        }
        
        if age > 120 {
            return Err(ValidationError::AgeTooOld);
        }
        
        Ok(Person { name, email, age })
    }
    
    fn update_email(&mut self, email: String) -> Result<(), ValidationError> {
        if !email.contains('@') {
            return Err(ValidationError::InvalidEmail);
        }
        
        self.email = email;
        Ok(())
    }
}
```

### 3. 文档注释

```rust
/// 表示一个二维点
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// 创建一个新的点
    /// 
    /// # 参数
    /// 
    /// * `x` - X 坐标
    /// * `y` - Y 坐标
    /// 
    /// # 示例
    /// 
    /// ```
    /// let point = Point::new(3.0, 4.0);
    /// assert_eq!(point.x, 3.0);
    /// assert_eq!(point.y, 4.0);
    /// ```
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    /// 计算到另一个点的距离
    /// 
    /// # 参数
    /// 
    /// * `other` - 另一个点的引用
    /// 
    /// # 返回值
    /// 
    /// 返回两点之间的欧几里得距离
    /// 
    /// # 示例
    /// 
    /// ```
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.distance_to(&p2), 5.0);
    /// ```
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
```

## 常见错误

### 1. self 参数类型错误

```rust
struct Counter {
    value: i32,
}

impl Counter {
    // 错误：尝试在不可变方法中修改
    // fn increment(&self) {
    //     self.value += 1; // 编译错误
    // }
    
    // 正确：使用可变引用
    fn increment(&mut self) {
        self.value += 1;
    }
    
    // 错误：不必要地获取所有权
    // fn get_value(self) -> i32 {
    //     self.value
    // }
    
    // 正确：使用借用
    fn get_value(&self) -> i32 {
        self.value
    }
}
```

### 2. 方法调用语法错误

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    
    // 正确：方法调用
    let area1 = rect.area();
    
    // 也正确：函数调用语法
    let area2 = Rectangle::area(&rect);
    
    // 错误：混淆方法和关联函数
    // let area3 = rect::new(10, 20); // 编译错误
    // let area4 = Rectangle.area(); // 编译错误
    
    println!("面积: {}, {}", area1, area2);
}
```

## 学习检查清单

完成本节学习后，请确认你能够：

- [ ] 理解方法和关联函数的区别
- [ ] 正确使用 `impl` 块定义方法
- [ ] 掌握 `&self`、`&mut self` 和 `self` 的使用场景
- [ ] 定义和调用关联函数
- [ ] 实现方法的链式调用
- [ ] 使用多个 `impl` 块组织代码
- [ ] 为方法编写适当的文档注释
- [ ] 在方法中正确处理错误
- [ ] 避免常见的 self 参数和调用语法错误

## 扩展阅读

- [Rust Book - 方法语法](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust Reference - 实现](https://doc.rust-lang.org/reference/items/implementations.html)
- [Rust by Example - 方法](https://doc.rust-lang.org/rust-by-example/fn/methods.html)

---

**下一节预告**：在下一节中，我们将学习枚举类型的定义和使用，了解如何使用枚举来表示不同的状态和选项。