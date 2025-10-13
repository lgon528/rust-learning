# 结构体和枚举的高级特性

## 学习目标

通过本节学习，你将能够：

- [ ] 掌握结构体和枚举的生命周期参数
- [ ] 理解和使用泛型结构体和枚举
- [ ] 学会实现和使用 trait 对象
- [ ] 掌握自定义 derive 宏的使用
- [ ] 理解内存布局和优化
- [ ] 学会使用 newtype 模式
- [ ] 掌握零成本抽象的设计
- [ ] 理解类型安全的设计模式

## 生命周期参数

### 结构体中的生命周期

```rust
// 包含引用的结构体需要生命周期参数
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    // 生命周期省略规则适用
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
    
    // 显式生命周期参数
    fn longest_with_announcement<'b>(
        &self,
        other: &'b str,
        announcement: &str,
    ) -> &'a str
    where
        'a: 'b, // 'a 必须比 'b 活得更久
    {
        println!("注意！{}", announcement);
        if self.part.len() > other.len() {
            self.part
        } else {
            // 这里不能返回 other，因为它的生命周期是 'b
            self.part
        }
    }
}

// 多个生命周期参数
#[derive(Debug)]
struct DoubleRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> DoubleRef<'a, 'b> {
    fn get_first(&self) -> &'a str {
        self.first
    }
    
    fn get_second(&self) -> &'b str {
        self.second
    }
    
    // 返回较短的生命周期
    fn get_shorter(&self) -> &str {
        if self.first.len() < self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let novel = String::from("很久很久以前，在一个遥远的星系中...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {:?}", excerpt);
    println!("等级: {}", excerpt.level());
    
    let announcement = "请注意以下重要信息";
    let returned_part = excerpt.announce_and_return_part(announcement);
    println!("返回的部分: {}", returned_part);
    
    // 多生命周期示例
    let text1 = "第一段文本";
    let text2 = "第二段更长的文本内容";
    
    let double_ref = DoubleRef {
        first: text1,
        second: text2,
    };
    
    println!("双引用: {:?}", double_ref);
    println!("第一个: {}", double_ref.get_first());
    println!("第二个: {}", double_ref.get_second());
    println!("较短的: {}", double_ref.get_shorter());
}
```

### 枚举中的生命周期

```rust
#[derive(Debug)]
enum Cow<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl<'a> Cow<'a> {
    fn to_owned(self) -> Cow<'static> {
        match self {
            Cow::Borrowed(s) => Cow::Owned(s.to_string()),
            Cow::Owned(s) => Cow::Owned(s),
        }
    }
    
    fn as_str(&self) -> &str {
        match self {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s,
        }
    }
}

// 复杂的生命周期枚举
#[derive(Debug)]
enum Reference<'a, 'b> {
    Single(&'a str),
    Double(&'a str, &'b str),
    None,
}

impl<'a, 'b> Reference<'a, 'b> {
    fn get_first(&self) -> Option<&'a str> {
        match self {
            Reference::Single(s) => Some(s),
            Reference::Double(s, _) => Some(s),
            Reference::None => None,
        }
    }
    
    fn get_second(&self) -> Option<&'b str> {
        match self {
            Reference::Double(_, s) => Some(s),
            _ => None,
        }
    }
}

fn main() {
    // Cow 示例
    let borrowed = Cow::Borrowed("借用的字符串");
    let owned = Cow::Owned(String::from("拥有的字符串"));
    
    println!("借用的: {}", borrowed.as_str());
    println!("拥有的: {}", owned.as_str());
    
    let owned_cow = borrowed.to_owned();
    println!("转换为拥有的: {}", owned_cow.as_str());
    
    // Reference 示例
    let text1 = "第一个文本";
    let text2 = "第二个文本";
    
    let refs = vec![
        Reference::Single(text1),
        Reference::Double(text1, text2),
        Reference::None,
    ];
    
    for (i, r) in refs.iter().enumerate() {
        println!("引用 {}: {:?}", i, r);
        if let Some(first) = r.get_first() {
            println!("  第一个: {}", first);
        }
        if let Some(second) = r.get_second() {
            println!("  第二个: {}", second);
        }
    }
}
```

## 泛型结构体和枚举

### 泛型结构体

```rust
use std::fmt::Display;

// 基本泛型结构体
#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 带约束的泛型实现
impl<T> Point<T>
where
    T: Display + Copy,
{
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

// 多个泛型参数
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
    
    fn as_tuple(&self) -> (&T, &U) {
        (&self.first, &self.second)
    }
}

// 复杂的泛型结构体
#[derive(Debug)]
struct Container<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T, const N: usize> Container<T, N>
where
    T: Default + Copy,
{
    fn new() -> Self {
        Container {
            data: [T::default(); N],
            len: 0,
        }
    }
    
    fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len >= N {
            Err("容器已满")
        } else {
            self.data[self.len] = item;
            self.len += 1;
            Ok(())
        }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    // 基本泛型使用
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("整数点: {:?}", int_point);
    println!("浮点数点: {:?}", float_point);
    
    int_point.print();
    float_point.print();
    
    println!("距离原点: {}", float_point.distance_from_origin());
    
    // 多泛型参数
    let pair = Pair::new("Hello", 42);
    println!("配对: {:?}", pair);
    
    let (text, number) = pair.into_tuple();
    println!("解构: {} 和 {}", text, number);
    
    // 常量泛型
    let mut container: Container<i32, 5> = Container::new();
    
    for i in 0..7 {
        match container.push(i) {
            Ok(()) => println!("成功添加: {}", i),
            Err(e) => println!("添加失败: {}, 错误: {}", i, e),
        }
    }
    
    println!("容器长度: {}", container.len());
    for i in 0..container.len() {
        if let Some(value) = container.get(i) {
            println!("索引 {}: {}", i, value);
        }
    }
}
```

### 泛型枚举

```rust
use std::fmt::Debug;

// 基本泛型枚举
#[derive(Debug, Clone)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 自定义泛型枚举
#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T> Tree<T>
where
    T: Ord + Clone + Debug,
{
    fn new() -> Self {
        Tree::Empty
    }
    
    fn insert(&mut self, value: T) {
        match self {
            Tree::Empty => {
                *self = Tree::Node {
                    value,
                    left: Box::new(Tree::Empty),
                    right: Box::new(Tree::Empty),
                };
            }
            Tree::Node {
                value: node_value,
                left,
                right,
            } => {
                if value <= *node_value {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            }
        }
    }
    
    fn contains(&self, value: &T) -> bool {
        match self {
            Tree::Empty => false,
            Tree::Node {
                value: node_value,
                left,
                right,
            } => {
                if value == node_value {
                    true
                } else if value < node_value {
                    left.contains(value)
                } else {
                    right.contains(value)
                }
            }
        }
    }
    
    fn inorder_traversal(&self, result: &mut Vec<T>) {
        match self {
            Tree::Empty => {}
            Tree::Node { value, left, right } => {
                left.inorder_traversal(result);
                result.push(value.clone());
                right.inorder_traversal(result);
            }
        }
    }
}

// 复杂的泛型枚举
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    
    fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }
    
    fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
    
    fn map_left<F, T>(self, f: F) -> Either<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }
    
    fn map_right<F, T>(self, f: F) -> Either<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }
}

fn main() {
    // 二叉搜索树示例
    let mut tree = Tree::new();
    let values = vec![5, 3, 7, 2, 4, 6, 8];
    
    for value in values {
        tree.insert(value);
    }
    
    println!("树结构: {:?}", tree);
    
    println!("包含 4: {}", tree.contains(&4));
    println!("包含 9: {}", tree.contains(&9));
    
    let mut sorted = Vec::new();
    tree.inorder_traversal(&mut sorted);
    println!("中序遍历: {:?}", sorted);
    
    // Either 示例
    let values: Vec<Either<String, i32>> = vec![
        Either::Left("Hello".to_string()),
        Either::Right(42),
        Either::Left("World".to_string()),
        Either::Right(100),
    ];
    
    for (i, value) in values.into_iter().enumerate() {
        println!("值 {}: {:?}", i, value);
        
        let transformed = value
            .map_left(|s| s.to_uppercase())
            .map_right(|n| n * 2);
        
        match transformed {
            Either::Left(s) => println!("  转换后的字符串: {}", s),
            Either::Right(n) => println!("  转换后的数字: {}", n),
        }
    }
}
```

## Trait 对象和动态分发

### 基本 Trait 对象

```rust
use std::fmt::Debug;

// 定义 trait
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

// 实现 trait 的结构体
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
    }
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!("绘制三角形，底: {}, 高: {}", self.base, self.height);
    }
    
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 使用 trait 对象的函数
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
    println!("面积: {:.2}", shape.area());
}

fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

// 返回 trait 对象的函数
fn create_shape(shape_type: &str, size: f64) -> Box<dyn Drawable> {
    match shape_type {
        "circle" => Box::new(Circle { radius: size }),
        "square" => Box::new(Rectangle {
            width: size,
            height: size,
        }),
        "triangle" => Box::new(Triangle {
            base: size,
            height: size,
        }),
        _ => Box::new(Circle { radius: 1.0 }),
    }
}

fn main() {
    // 创建不同的形状
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    
    // 使用 trait 对象
    println!("=== 直接调用 ===");
    draw_shape(&circle);
    draw_shape(&rectangle);
    draw_shape(&triangle);
    
    // 存储在集合中
    println!("\n=== 集合中的 trait 对象 ===");
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 2.0,
            height: 5.0,
        }),
        Box::new(Triangle {
            base: 4.0,
            height: 3.0,
        }),
    ];
    
    for (i, shape) in shapes.iter().enumerate() {
        println!("形状 {}:", i + 1);
        draw_shape(shape.as_ref());
        println!();
    }
    
    println!("总面积: {:.2}", total_area(&shapes));
    
    // 动态创建形状
    println!("\n=== 动态创建 ===");
    let dynamic_shapes = vec![
        create_shape("circle", 2.0),
        create_shape("square", 3.0),
        create_shape("triangle", 4.0),
    ];
    
    for shape in &dynamic_shapes {
        draw_shape(shape.as_ref());
    }
}
```

### 复杂的 Trait 对象

```rust
use std::any::Any;
use std::fmt::Debug;

// 复杂的 trait 定义
trait Component: Debug + Send + Sync {
    fn name(&self) -> &str;
    fn update(&mut self, delta_time: f32);
    fn render(&self);
    
    // 提供默认实现
    fn is_active(&self) -> bool {
        true
    }
    
    // 用于类型转换
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// 具体的组件实现
#[derive(Debug)]
struct TransformComponent {
    x: f32,
    y: f32,
    rotation: f32,
}

impl Component for TransformComponent {
    fn name(&self) -> &str {
        "Transform"
    }
    
    fn update(&mut self, delta_time: f32) {
        self.rotation += 90.0 * delta_time; // 每秒旋转90度
        if self.rotation >= 360.0 {
            self.rotation -= 360.0;
        }
    }
    
    fn render(&self) {
        println!(
            "渲染变换: 位置({:.1}, {:.1}), 旋转{:.1}°",
            self.x, self.y, self.rotation
        );
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
struct RenderComponent {
    color: String,
    visible: bool,
}

impl Component for RenderComponent {
    fn name(&self) -> &str {
        "Render"
    }
    
    fn update(&mut self, _delta_time: f32) {
        // 渲染组件通常不需要更新
    }
    
    fn render(&self) {
        if self.visible {
            println!("渲染: 颜色 {}", self.color);
        } else {
            println!("对象不可见");
        }
    }
    
    fn is_active(&self) -> bool {
        self.visible
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
struct PhysicsComponent {
    velocity_x: f32,
    velocity_y: f32,
    mass: f32,
}

impl Component for PhysicsComponent {
    fn name(&self) -> &str {
        "Physics"
    }
    
    fn update(&mut self, delta_time: f32) {
        // 简单的重力模拟
        self.velocity_y -= 9.8 * delta_time;
        println!(
            "物理更新: 速度({:.1}, {:.1}), 质量{:.1}",
            self.velocity_x, self.velocity_y, self.mass
        );
    }
    
    fn render(&self) {
        println!("物理组件不需要渲染");
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// 实体系统
#[derive(Debug)]
struct Entity {
    id: u32,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    fn new(id: u32) -> Self {
        Entity {
            id,
            components: Vec::new(),
        }
    }
    
    fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
    
    fn get_component<T: Component + 'static>(&self) -> Option<&T> {
        self.components
            .iter()
            .find_map(|c| c.as_any().downcast_ref::<T>())
    }
    
    fn get_component_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        self.components
            .iter_mut()
            .find_map(|c| c.as_any_mut().downcast_mut::<T>())
    }
    
    fn update(&mut self, delta_time: f32) {
        for component in &mut self.components {
            if component.is_active() {
                component.update(delta_time);
            }
        }
    }
    
    fn render(&self) {
        println!("渲染实体 {}:", self.id);
        for component in &self.components {
            if component.is_active() {
                component.render();
            }
        }
    }
}

fn main() {
    // 创建实体和组件
    let mut entity = Entity::new(1);
    
    entity.add_component(Box::new(TransformComponent {
        x: 10.0,
        y: 20.0,
        rotation: 0.0,
    }));
    
    entity.add_component(Box::new(RenderComponent {
        color: "红色".to_string(),
        visible: true,
    }));
    
    entity.add_component(Box::new(PhysicsComponent {
        velocity_x: 5.0,
        velocity_y: 0.0,
        mass: 1.0,
    }));
    
    println!("初始状态:");
    entity.render();
    
    // 模拟游戏循环
    println!("\n=== 游戏循环 ===");
    for frame in 1..=3 {
        println!("\n帧 {}:", frame);
        entity.update(0.016); // 60 FPS
        entity.render();
        
        // 访问特定组件
        if let Some(transform) = entity.get_component::<TransformComponent>() {
            println!("当前旋转: {:.1}°", transform.rotation);
        }
    }
    
    // 修改组件
    if let Some(render) = entity.get_component_mut::<RenderComponent>() {
        render.color = "蓝色".to_string();
        println!("\n颜色已更改为蓝色");
    }
    
    entity.render();
}
```

## 自定义 Derive 宏

### 常用的 Derive 宏

```rust
// 基本的 derive 宏
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 自定义 PartialEq 实现
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        // 只比较 name 和 email，忽略 age
        self.name == other.name && self.email == other.email
    }
}

impl Eq for Person {}

// 自定义 Hash 实现
use std::hash::{Hash, Hasher};

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 只对 name 和 email 进行哈希
        self.name.hash(state);
        self.email.hash(state);
    }
}

// 复杂的 derive 示例
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending { since: String },
    Suspended { reason: String, until: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Account {
    user: User,
    status: Status,
    created_at: String,
}

// 自定义 Display 实现
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.email)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "活跃"),
            Status::Inactive => write!(f, "非活跃"),
            Status::Pending { since } => write!(f, "待处理(自{})", since),
            Status::Suspended { reason, until } => {
                if let Some(until) = until {
                    write!(f, "暂停({}, 直到{})", reason, until)
                } else {
                    write!(f, "暂停({})", reason)
                }
            }
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "账户[用户: {}, 状态: {}, 创建于: {}]",
            self.user, self.status, self.created_at
        )
    }
}

fn main() {
    // 测试基本 derive 功能
    let user1 = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    let user2 = user1.clone();
    let user3 = User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };
    
    println!("用户1: {:?}", user1);
    println!("用户1 == 用户2: {}", user1 == user2);
    println!("用户1 == 用户3: {}", user1 == user3);
    
    // 测试自定义 PartialEq
    let person1 = Person {
        name: "Charlie".to_string(),
        age: 25,
        email: "charlie@example.com".to_string(),
    };
    
    let person2 = Person {
        name: "Charlie".to_string(),
        age: 30, // 不同的年龄
        email: "charlie@example.com".to_string(),
    };
    
    println!("\n人员1: {:?}", person1);
    println!("人员2: {:?}", person2);
    println!("人员1 == 人员2: {}", person1 == person2); // true，因为忽略年龄
    
    // 测试 Hash
    use std::collections::HashMap;
    let mut person_map = HashMap::new();
    person_map.insert(person1.clone(), "第一个Charlie");
    person_map.insert(person2.clone(), "第二个Charlie");
    
    println!("HashMap 大小: {}", person_map.len()); // 1，因为哈希相同
    
    // 测试复杂结构
    let account = Account {
        user: user1.clone(),
        status: Status::Pending {
            since: "2024-01-01".to_string(),
        },
        created_at: "2023-12-01".to_string(),
    };
    
    println!("\n账户: {}", account);
    
    // 测试状态变化
    let statuses = vec![
        Status::Active,
        Status::Inactive,
        Status::Pending {
            since: "2024-01-15".to_string(),
        },
        Status::Suspended {
            reason: "违规行为".to_string(),
            until: Some("2024-02-01".to_string()),
        },
        Status::Suspended {
            reason: "账户审核".to_string(),
            until: None,
        },
    ];
    
    println!("\n所有状态:");
    for (i, status) in statuses.iter().enumerate() {
        println!("{}: {}", i + 1, status);
    }
}
```

## 内存布局优化

### 结构体内存布局

```rust
use std::mem;

// 未优化的结构体
#[derive(Debug)]
struct UnoptimizedStruct {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes
    c: u8,   // 1 byte
    d: u32,  // 4 bytes
    e: u8,   // 1 byte
}

// 优化后的结构体（手动重排）
#[derive(Debug)]
struct OptimizedStruct {
    b: u64,  // 8 bytes
    d: u32,  // 4 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte
    e: u8,   // 1 byte
    // 编译器会添加 1 byte 的填充
}

// 使用 repr 属性控制布局
#[repr(C)]
#[derive(Debug)]
struct CLayoutStruct {
    a: u8,
    b: u64,
    c: u8,
}

#[repr(packed)]
#[derive(Debug)]
struct PackedStruct {
    a: u8,
    b: u64,
    c: u8,
}

// 对齐优化
#[repr(align(16))]
#[derive(Debug)]
struct AlignedStruct {
    data: [u8; 10],
}

// 枚举的内存优化
#[derive(Debug)]
enum UnoptimizedEnum {
    Small(u8),
    Large([u8; 100]),
}

// 使用 Box 优化大变体
#[derive(Debug)]
enum OptimizedEnum {
    Small(u8),
    Large(Box<[u8; 100]>),
}

// 零大小类型
#[derive(Debug)]
struct ZeroSized;

#[derive(Debug)]
struct WithZeroSized {
    data: u32,
    marker: ZeroSized,
}

fn analyze_memory_layout() {
    println!("=== 内存布局分析 ===");
    
    // 结构体大小比较
    println!("UnoptimizedStruct 大小: {} bytes", mem::size_of::<UnoptimizedStruct>());
    println!("OptimizedStruct 大小: {} bytes", mem::size_of::<OptimizedStruct>());
    println!("CLayoutStruct 大小: {} bytes", mem::size_of::<CLayoutStruct>());
    println!("PackedStruct 大小: {} bytes", mem::size_of::<PackedStruct>());
    println!("AlignedStruct 大小: {} bytes", mem::size_of::<AlignedStruct>());
    
    // 对齐信息
    println!("\n=== 对齐信息 ===");
    println!("UnoptimizedStruct 对齐: {} bytes", mem::align_of::<UnoptimizedStruct>());
    println!("OptimizedStruct 对齐: {} bytes", mem::align_of::<OptimizedStruct>());
    println!("AlignedStruct 对齐: {} bytes", mem::align_of::<AlignedStruct>());
    
    // 枚举大小比较
    println!("\n=== 枚举大小比较 ===");
    println!("UnoptimizedEnum 大小: {} bytes", mem::size_of::<UnoptimizedEnum>());
    println!("OptimizedEnum 大小: {} bytes", mem::size_of::<OptimizedEnum>());
    
    // 零大小类型
    println!("\n=== 零大小类型 ===");
    println!("ZeroSized 大小: {} bytes", mem::size_of::<ZeroSized>());
    println!("WithZeroSized 大小: {} bytes", mem::size_of::<WithZeroSized>());
    
    // 字段偏移量（需要 unsafe）
    println!("\n=== 字段偏移量 ===");
    unsafe {
        let unopt = UnoptimizedStruct {
            a: 0, b: 0, c: 0, d: 0, e: 0,
        };
        let base = &unopt as *const _ as usize;
        println!("UnoptimizedStruct 字段偏移:");
        println!("  a: {} bytes", &unopt.a as *const _ as usize - base);
        println!("  b: {} bytes", &unopt.b as *const _ as usize - base);
        println!("  c: {} bytes", &unopt.c as *const _ as usize - base);
        println!("  d: {} bytes", &unopt.d as *const _ as usize - base);
        println!("  e: {} bytes", &unopt.e as *const _ as usize - base);
    }
}

fn main() {
    analyze_memory_layout();
    
    // 创建实例测试
    let unopt = UnoptimizedStruct {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };
    
    let opt = OptimizedStruct {
        b: 2,
        d: 4,
        a: 1,
        c: 3,
        e: 5,
    };
    
    println!("\n=== 实例测试 ===");
    println!("未优化结构体: {:?}", unopt);
    println!("优化结构体: {:?}", opt);
    
    // 枚举测试
    let small_enum = OptimizedEnum::Small(42);
    let large_enum = OptimizedEnum::Large(Box::new([0u8; 100]));
    
    println!("\n=== 枚举测试 ===");
    println!("小变体: {:?}", small_enum);
    println!("大变体大小: {} bytes", mem::size_of_val(&large_enum));
}
```

## Newtype 模式

### 基本 Newtype 模式

```rust
use std::fmt;
use std::ops::{Add, Deref, DerefMut};

// 基本的 newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(u32);

#[derive(Debug, Clone, PartialEq)]
struct Email(String);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Price(f64);

// 为 newtype 实现方法
impl UserId {
    fn new(id: u32) -> Self {
        UserId(id)
    }
    
    fn value(&self) -> u32 {
        self.0
    }
}

impl Email {
    fn new(email: String) -> Result<Self, &'static str> {
        if email.contains('@') && email.contains('.') {
            Ok(Email(email))
        } else {
            Err("无效的邮箱格式")
        }
    }
    
    fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl Price {
    fn new(amount: f64) -> Result<Self, &'static str> {
        if amount >= 0.0 {
            Ok(Price(amount))
        } else {
            Err("价格不能为负数")
        }
    }
    
    fn with_tax(&self, tax_rate: f64) -> Self {
        Price(self.0 * (1.0 + tax_rate))
    }
    
    fn discount(&self, percentage: f64) -> Self {
        Price(self.0 * (1.0 - percentage / 100.0))
    }
}

// 实现 Display
impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "用户#{}", self.0)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "¥{:.2}", self.0)
    }
}

// 实现运算符重载
impl Add for Price {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Price(self.0 + other.0)
    }
}

// 实现 Deref 以便访问内部值
impl Deref for Email {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 复杂的 newtype 示例
#[derive(Debug, Clone)]
struct SecureString(String);

impl SecureString {
    fn new(s: String) -> Self {
        SecureString(s)
    }
    
    fn len(&self) -> usize {
        self.0.len()
    }
    
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    // 安全的比较，防止时序攻击
    fn secure_eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (a, b) in self.0.bytes().zip(other.0.bytes()) {
            result |= a ^ b;
        }
        result == 0
    }
}

// 不实现 Display 以防止意外泄露
impl fmt::Display for SecureString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED:{}字符]", self.0.len())
    }
}

// 单位类型的 newtype
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Feet(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Seconds(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }
    
    fn to_feet(&self) -> Feet {
        Feet(self.0 * 3.28084)
    }
}

impl Feet {
    fn new(value: f64) -> Self {
        Feet(value)
    }
    
    fn to_meters(&self) -> Meters {
        Meters(self.0 / 3.28084)
    }
}

impl Seconds {
    fn new(value: f64) -> Self {
        Seconds(value)
    }
}

// 速度类型，组合其他单位
#[derive(Debug, Clone, Copy, PartialEq)]
struct Speed {
    meters_per_second: f64,
}

impl Speed {
    fn new(distance: Meters, time: Seconds) -> Self {
        Speed {
            meters_per_second: distance.0 / time.0,
        }
    }
    
    fn kmh(&self) -> f64 {
        self.meters_per_second * 3.6
    }
    
    fn mph(&self) -> f64 {
        self.meters_per_second * 2.237
    }
}

fn main() {
    // 基本 newtype 使用
    let user_id = UserId::new(12345);
    let email = Email::new("user@example.com".to_string()).unwrap();
    let price = Price::new(99.99).unwrap();
    
    println!("用户ID: {}", user_id);
    println!("邮箱: {}", email);
    println!("价格: {}", price);
    println!("邮箱域名: {}", email.domain());
    
    // 价格计算
    let tax_price = price.with_tax(0.1);
    let discount_price = price.discount(20.0);
    let total = tax_price + discount_price;
    
    println!("含税价格: {}", tax_price);
    println!("折扣价格: {}", discount_price);
    println!("总价: {}", total);
    
    // 类型安全演示
    println!("\n=== 类型安全演示 ===");
    
    // 这些会编译失败，因为类型不匹配：
    // let wrong1 = user_id + price;  // 错误：不能将 UserId 和 Price 相加
    // let wrong2 = email == user_id; // 错误：不能比较 Email 和 UserId
    
    // 安全字符串
    let password1 = SecureString::new("secret123".to_string());
    let password2 = SecureString::new("secret123".to_string());
    let password3 = SecureString::new("different".to_string());
    
    println!("密码1: {}", password1);
    println!("密码1 == 密码2: {}", password1.secure_eq(&password2));
    println!("密码1 == 密码3: {}", password1.secure_eq(&password3));
    
    // 单位转换
    println!("\n=== 单位转换 ===");
    let distance = Meters::new(100.0);
    let time = Seconds::new(10.0);
    let speed = Speed::new(distance, time);
    
    println!("距离: {:.1} 米", distance.0);
    println!("距离: {:.1} 英尺", distance.to_feet().0);
    println!("时间: {:.1} 秒", time.0);
    println!("速度: {:.1} m/s", speed.meters_per_second);
    println!("速度: {:.1} km/h", speed.kmh());
    println!("速度: {:.1} mph", speed.mph());
    
    // 类型安全的单位计算
    // let wrong_speed = Speed::new(time, distance); // 编译错误：参数顺序错误
}
```

## 零成本抽象

### 编译时计算

```rust
// 编译时常量计算
const fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;
            while i <= n {
                let temp = a + b;
                a = b;
                b = temp;
                i += 1;
            }
            b
        }
    }
}

// 编译时计算的数组
const FIBONACCI_SEQUENCE: [u64; 20] = {
    let mut arr = [0u64; 20];
    let mut i = 0;
    while i < 20 {
        arr[i] = fibonacci(i as u32);
        i += 1;
    }
    arr
};

// 零成本的状态机
#[derive(Debug, Clone, Copy)]
struct Idle;

#[derive(Debug, Clone, Copy)]
struct Running;

#[derive(Debug, Clone, Copy)]
struct Stopped;

#[derive(Debug)]
struct StateMachine<State> {
    state: std::marker::PhantomData<State>,
    data: String,
}

impl StateMachine<Idle> {
    fn new(data: String) -> Self {
        StateMachine {
            state: std::marker::PhantomData,
            data,
        }
    }
    
    fn start(self) -> StateMachine<Running> {
        println!("启动状态机: {}", self.data);
        StateMachine {
            state: std::marker::PhantomData,
            data: self.data,
        }
    }
}

impl StateMachine<Running> {
    fn process(&mut self) {
        println!("处理数据: {}", self.data);
        self.data.push_str(" [已处理]");
    }
    
    fn stop(self) -> StateMachine<Stopped> {
        println!("停止状态机: {}", self.data);
        StateMachine {
            state: std::marker::PhantomData,
            data: self.data,
        }
    }
}

impl StateMachine<Stopped> {
    fn restart(self) -> StateMachine<Running> {
        println!("重启状态机: {}", self.data);
        StateMachine {
            state: std::marker::PhantomData,
            data: self.data,
        }
    }
    
    fn get_result(self) -> String {
        self.data
    }
}

// 零成本的单位系统
trait Unit {
    const SCALE: f64;
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy)]
struct Meter;

#[derive(Debug, Clone, Copy)]
struct Kilometer;

#[derive(Debug, Clone, Copy)]
struct Mile;

impl Unit for Meter {
    const SCALE: f64 = 1.0;
    const NAME: &'static str = "米";
}

impl Unit for Kilometer {
    const SCALE: f64 = 1000.0;
    const NAME: &'static str = "千米";
}

impl Unit for Mile {
    const SCALE: f64 = 1609.344;
    const NAME: &'static str = "英里";
}

#[derive(Debug, Clone, Copy)]
struct Distance<U: Unit> {
    value: f64,
    _unit: std::marker::PhantomData<U>,
}

impl<U: Unit> Distance<U> {
    fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: std::marker::PhantomData,
        }
    }
    
    fn value(&self) -> f64 {
        self.value
    }
    
    fn to_meters(&self) -> Distance<Meter> {
        Distance::new(self.value * U::SCALE)
    }
    
    fn convert_to<T: Unit>(&self) -> Distance<T> {
        let meters = self.value * U::SCALE;
        Distance::new(meters / T::SCALE)
    }
}

impl<U: Unit> std::fmt::Display for Distance<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.value, U::NAME)
    }
}

// 零成本的构建器模式
#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Debug, Default)]
struct HttpRequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self::default()
    }
    
    fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    
    fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    
    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
    
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    
    fn build(self) -> Result<HttpRequest, &'static str> {
        let url = self.url.ok_or("URL 是必需的")?;
        let method = self.method.unwrap_or_else(|| "GET".to_string());
        
        Ok(HttpRequest {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}

fn main() {
    // 编译时计算
    println!("=== 编译时计算 ===");
    println!("第10个斐波那契数: {}", fibonacci(10));
    println!("斐波那契序列: {:?}", &FIBONACCI_SEQUENCE[..10]);
    
    // 零成本状态机
    println!("\n=== 零成本状态机 ===");
    let machine = StateMachine::new("初始数据".to_string());
    let mut running_machine = machine.start();
    running_machine.process();
    let stopped_machine = running_machine.stop();
    let result = stopped_machine.get_result();
    println!("最终结果: {}", result);
    
    // 这些会编译失败，因为状态不匹配：
    // let idle_machine = running_machine.start(); // 错误：Running 状态不能调用 start
    // let wrong = stopped_machine.process();     // 错误：Stopped 状态不能调用 process
    
    // 零成本单位系统
    println!("\n=== 零成本单位系统 ===");
    let distance_m = Distance::<Meter>::new(1000.0);
    let distance_km = Distance::<Kilometer>::new(1.0);
    let distance_mile = Distance::<Mile>::new(0.621371);
    
    println!("距离（米）: {}", distance_m);
    println!("距离（千米）: {}", distance_km);
    println!("距离（英里）: {}", distance_mile);
    
    // 单位转换
    let km_to_m = distance_km.to_meters();
    let mile_to_km: Distance<Kilometer> = distance_mile.convert_to();
    
    println!("1千米 = {}", km_to_m);
    println!("0.621371英里 = {}", mile_to_km);
    
    // 零成本构建器
    println!("\n=== 零成本构建器 ===");
    let request = HttpRequestBuilder::new()
        .url("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice", "email": "alice@example.com"}"#)
        .build()
        .unwrap();
    
    println!("HTTP请求: {:?}", request);
}
```

## 类型安全的设计模式

### 幻影类型（Phantom Types）

```rust
use std::marker::PhantomData;

// 幻影类型用于编译时类型检查
#[derive(Debug)]
struct Database<State> {
    connection_string: String,
    _state: PhantomData<State>,
}

// 状态标记
#[derive(Debug)]
struct Connected;

#[derive(Debug)]
struct Disconnected;

impl Database<Disconnected> {
    fn new(connection_string: String) -> Self {
        Database {
            connection_string,
            _state: PhantomData,
        }
    }
    
    fn connect(self) -> Result<Database<Connected>, &'static str> {
        println!("连接到数据库: {}", self.connection_string);
        // 模拟连接逻辑
        if self.connection_string.starts_with("valid://") {
            Ok(Database {
                connection_string: self.connection_string,
                _state: PhantomData,
            })
        } else {
            Err("无效的连接字符串")
        }
    }
}

impl Database<Connected> {
    fn query(&self, sql: &str) -> Vec<String> {
        println!("执行查询: {}", sql);
        vec!["结果1".to_string(), "结果2".to_string()]
    }
    
    fn execute(&self, sql: &str) -> usize {
        println!("执行命令: {}", sql);
        1 // 受影响的行数
    }
    
    fn disconnect(self) -> Database<Disconnected> {
        println!("断开数据库连接");
        Database {
            connection_string: self.connection_string,
            _state: PhantomData,
        }
    }
}

// 类型级别的权限系统
#[derive(Debug)]
struct User<Permission> {
    name: String,
    _permission: PhantomData<Permission>,
}

#[derive(Debug)]
struct ReadOnly;

#[derive(Debug)]
struct ReadWrite;

#[derive(Debug)]
struct Admin;

impl<P> User<P> {
    fn name(&self) -> &str {
        &self.name
    }
}

impl User<ReadOnly> {
    fn new_reader(name: String) -> Self {
        User {
            name,
            _permission: PhantomData,
        }
    }
    
    fn read_data(&self) -> Vec<String> {
        println!("{} 读取数据", self.name);
        vec!["数据1".to_string(), "数据2".to_string()]
    }
    
    fn upgrade_to_writer(self) -> User<ReadWrite> {
        println!("{} 升级为写入权限", self.name);
        User {
            name: self.name,
            _permission: PhantomData,
        }
    }
}

impl User<ReadWrite> {
    fn new_writer(name: String) -> Self {
        User {
            name,
            _permission: PhantomData,
        }
    }
    
    fn read_data(&self) -> Vec<String> {
        println!("{} 读取数据", self.name);
        vec!["数据1".to_string(), "数据2".to_string()]
    }
    
    fn write_data(&self, data: &str) {
        println!("{} 写入数据: {}", self.name, data);
    }
    
    fn upgrade_to_admin(self) -> User<Admin> {
        println!("{} 升级为管理员", self.name);
        User {
            name: self.name,
            _permission: PhantomData,
        }
    }
    
    fn downgrade_to_reader(self) -> User<ReadOnly> {
        println!("{} 降级为只读权限", self.name);
        User {
            name: self.name,
            _permission: PhantomData,
        }
    }
}

impl User<Admin> {
    fn new_admin(name: String) -> Self {
        User {
            name,
            _permission: PhantomData,
        }
    }
    
    fn read_data(&self) -> Vec<String> {
        println!("{} (管理员) 读取数据", self.name);
        vec!["数据1".to_string(), "数据2".to_string(), "敏感数据".to_string()]
    }
    
    fn write_data(&self, data: &str) {
        println!("{} (管理员) 写入数据: {}", self.name, data);
    }
    
    fn delete_data(&self, id: u32) {
        println!("{} (管理员) 删除数据 ID: {}", self.name, id);
    }
    
    fn manage_users(&self) {
        println!("{} (管理员) 管理用户", self.name);
    }
}

fn main() {
    // 数据库连接状态管理
    println!("=== 数据库状态管理 ===");
    let db = Database::new("valid://localhost:5432/mydb".to_string());
    
    // 这会编译失败：
    // db.query("SELECT * FROM users"); // 错误：未连接状态不能查询
    
    let connected_db = db.connect().unwrap();
    let results = connected_db.query("SELECT * FROM users");
    println!("查询结果: {:?}", results);
    
    let affected_rows = connected_db.execute("UPDATE users SET active = true");
    println!("受影响的行数: {}", affected_rows);
    
    let disconnected_db = connected_db.disconnect();
    
    // 这会编译失败：
    // disconnected_db.query("SELECT * FROM users"); // 错误：已断开连接
    
    // 权限系统
    println!("\n=== 权限系统 ===");
    let reader = User::new_reader("Alice".to_string());
    let data = reader.read_data();
    println!("读取的数据: {:?}", data);
    
    // 这会编译失败：
    // reader.write_data("新数据"); // 错误：只读用户不能写入
    
    let writer = reader.upgrade_to_writer();
    writer.write_data("新数据");
    
    let admin = writer.upgrade_to_admin();
    admin.manage_users();
    admin.delete_data(123);
    
    // 权限降级
    let downgraded = admin.downgrade_to_reader();
    let _ = downgraded.read_data();
    
    // 这会编译失败：
    // downgraded.delete_data(456); // 错误：只读用户不能删除数据
}
```

### 类型级别的配置

```rust
use std::marker::PhantomData;

// 配置特征
trait Config {
    const DEBUG: bool;
    const MAX_CONNECTIONS: usize;
    const TIMEOUT_MS: u64;
}

// 不同的配置类型
#[derive(Debug)]
struct Development;

#[derive(Debug)]
struct Production;

#[derive(Debug)]
struct Testing;

impl Config for Development {
    const DEBUG: bool = true;
    const MAX_CONNECTIONS: usize = 10;
    const TIMEOUT_MS: u64 = 5000;
}

impl Config for Production {
    const DEBUG: bool = false;
    const MAX_CONNECTIONS: usize = 100;
    const TIMEOUT_MS: u64 = 30000;
}

impl Config for Testing {
    const DEBUG: bool = true;
    const MAX_CONNECTIONS: usize = 5;
    const TIMEOUT_MS: u64 = 1000;
}

// 使用配置的服务
#[derive(Debug)]
struct WebServer<C: Config> {
    name: String,
    _config: PhantomData<C>,
}

impl<C: Config> WebServer<C> {
    fn new(name: String) -> Self {
        if C::DEBUG {
            println!("[DEBUG] 创建服务器: {}", name);
        }
        
        WebServer {
            name,
            _config: PhantomData,
        }
    }
    
    fn start(&self) {
        if C::DEBUG {
            println!("[DEBUG] 启动服务器: {}", self.name);
            println!("[DEBUG] 最大连接数: {}", C::MAX_CONNECTIONS);
            println!("[DEBUG] 超时时间: {}ms", C::TIMEOUT_MS);
        } else {
            println!("启动服务器: {}", self.name);
        }
    }
    
    fn handle_request(&self, request: &str) {
        if C::DEBUG {
            println!("[DEBUG] 处理请求: {}", request);
        }
        
        // 模拟处理时间
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        println!("请求已处理: {}", request);
    }
    
    fn get_max_connections(&self) -> usize {
        C::MAX_CONNECTIONS
    }
    
    fn get_timeout(&self) -> u64 {
        C::TIMEOUT_MS
    }
}

// 编译时特化的算法
trait Algorithm {
    const NAME: &'static str;
    fn process(data: &[i32]) -> Vec<i32>;
}

#[derive(Debug)]
struct BubbleSort;

#[derive(Debug)]
struct QuickSort;

#[derive(Debug)]
struct MergeSort;

impl Algorithm for BubbleSort {
    const NAME: &'static str = "冒泡排序";
    
    fn process(data: &[i32]) -> Vec<i32> {
        let mut result = data.to_vec();
        let len = result.len();
        
        for i in 0..len {
            for j in 0..len - 1 - i {
                if result[j] > result[j + 1] {
                    result.swap(j, j + 1);
                }
            }
        }
        
        result
    }
}

impl Algorithm for QuickSort {
    const NAME: &'static str = "快速排序";
    
    fn process(data: &[i32]) -> Vec<i32> {
        if data.len() <= 1 {
            return data.to_vec();
        }
        
        let mut result = data.to_vec();
        result.sort_unstable();
        result
    }
}

impl Algorithm for MergeSort {
    const NAME: &'static str = "归并排序";
    
    fn process(data: &[i32]) -> Vec<i32> {
        if data.len() <= 1 {
            return data.to_vec();
        }
        
        let mut result = data.to_vec();
        result.sort();
        result
    }
}

#[derive(Debug)]
struct Sorter<A: Algorithm> {
    _algorithm: PhantomData<A>,
}

impl<A: Algorithm> Sorter<A> {
    fn new() -> Self {
        println!("创建排序器: {}", A::NAME);
        Sorter {
            _algorithm: PhantomData,
        }
    }
    
    fn sort(&self, data: &[i32]) -> Vec<i32> {
        println!("使用 {} 排序数据: {:?}", A::NAME, data);
        let result = A::process(data);
        println!("排序结果: {:?}", result);
        result
    }
    
    fn algorithm_name(&self) -> &'static str {
        A::NAME
    }
}

fn main() {
    // 不同环境的服务器配置
    println!("=== 服务器配置 ===");
    
    let dev_server = WebServer::<Development>::new("开发服务器".to_string());
    let prod_server = WebServer::<Production>::new("生产服务器".to_string());
    let test_server = WebServer::<Testing>::new("测试服务器".to_string());
    
    println!("\n开发环境:");
    dev_server.start();
    dev_server.handle_request("GET /api/users");
    println!("最大连接数: {}", dev_server.get_max_connections());
    
    println!("\n生产环境:");
    prod_server.start();
    prod_server.handle_request("POST /api/orders");
    println!("超时时间: {}ms", prod_server.get_timeout());
    
    println!("\n测试环境:");
    test_server.start();
    test_server.handle_request("DELETE /api/cache");
    
    // 编译时算法选择
    println!("\n=== 编译时算法选择 ===");
    
    let data = vec![64, 34, 25, 12, 22, 11, 90];
    
    let bubble_sorter = Sorter::<BubbleSort>::new();
    let quick_sorter = Sorter::<QuickSort>::new();
    let merge_sorter = Sorter::<MergeSort>::new();
    
    println!("\n原始数据: {:?}", data);
    
    bubble_sorter.sort(&data);
    quick_sorter.sort(&data);
    merge_sorter.sort(&data);
    
    println!("\n算法名称:");
    println!("- {}", bubble_sorter.algorithm_name());
    println!("- {}", quick_sorter.algorithm_name());
    println!("- {}", merge_sorter.algorithm_name());
}
```

## 最佳实践

### 1. 生命周期设计原则

- **最小化生命周期参数**：只在必要时使用生命周期参数
- **利用生命周期省略规则**：让编译器自动推导
- **避免过度复杂的生命周期关系**：保持简单和清晰

### 2. 泛型设计指导

- **合理使用约束**：为泛型参数添加必要的 trait 约束
- **避免单态化爆炸**：谨慎使用泛型以避免编译时间过长
- **提供具体类型的特化实现**：为常用类型提供优化实现

### 3. 内存布局优化

- **字段重排序**：将大字段放在前面，小字段放在后面
- **使用 `#[repr(C)]`**：需要与 C 代码交互时
- **考虑缓存友好性**：频繁访问的字段放在一起

### 4. Newtype 模式应用

- **类型安全**：防止混淆相似类型的值
- **单位系统**：确保物理量的正确性
- **API 设计**：提供更清晰的接口

### 5. 零成本抽象设计

- **编译时计算**：尽可能在编译时完成计算
- **状态机**：使用类型系统确保状态转换的正确性
- **构建器模式**：提供流畅的 API 同时保持零成本

## 常见错误

### 1. 生命周期错误

```rust
// 错误：返回悬垂引用
// fn get_string() -> &str {
//     let s = String::from("hello");
//     &s // 错误：s 在函数结束时被销毁
// }

// 正确：返回拥有的值
fn get_string() -> String {
    String::from("hello")
}
```

### 2. 泛型约束不足

```rust
// 错误：缺少必要的约束
// fn print_and_clone<T>(value: T) {
//     println!("{:?}", value); // 错误：T 没有实现 Debug
//     let cloned = value.clone(); // 错误：T 没有实现 Clone
// }

// 正确：添加必要的约束
fn print_and_clone<T>(value: T) -> T
where
    T: std::fmt::Debug + Clone,
{
    println!("{:?}", value);
    value.clone()
}
```

### 3. 内存布局假设

```rust
// 危险：假设字段顺序
// #[repr(C)]
// struct Point {
//     x: f64,
//     y: f64,
// }
// 
// // 不要假设 x 在 y 之前，除非使用 #[repr(C)]
```

## 学习检查清单

- [ ] 理解生命周期参数的作用和语法
- [ ] 掌握泛型结构体和枚举的定义
- [ ] 学会使用 trait 对象进行动态分发
- [ ] 了解常用的 derive 宏及其作用
- [ ] 理解内存布局和对齐的概念
- [ ] 掌握 newtype 模式的应用场景
- [ ] 学会设计零成本抽象
- [ ] 理解幻影类型的用途
- [ ] 掌握类型级别的配置和算法选择
- [ ] 能够识别和避免常见的设计错误

## 扩展阅读

- [Rust Book - Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)
- [Rust Reference - Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
- [Rust Nomicon - PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [Zero Cost Abstractions](https://blog.rust-lang.org/2015/05/11/traits.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)