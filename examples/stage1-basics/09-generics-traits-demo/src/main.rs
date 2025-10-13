//! 泛型、特征和生命周期综合示例
//! 
//! 本示例展示了 Rust 中泛型、Trait 和生命周期的各种用法，
//! 包括基础概念、高级特性和实际应用场景。

use std::fmt::{Debug, Display};
// use std::cmp::PartialOrd;
use std::collections::HashMap;

// ============================================================================
// 1. 泛型基础示例
// ============================================================================

/// 泛型函数：交换两个值
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

/// 泛型函数：找到切片中的最大值
fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max().copied()
}

/// 泛型结构体：二维点
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn _x(&self) -> &T {
        &self.x
    }
    
    fn _y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 泛型枚举：自定义 Result 类型
#[derive(Debug)]
enum MyResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> MyResult<T, E> {
    fn is_success(&self) -> bool {
        matches!(self, MyResult::Success(_))
    }
    
    fn _unwrap(self) -> T 
    where 
        E: Debug 
    {
        match self {
            MyResult::Success(value) => value,
            MyResult::Failure(err) => panic!("调用 unwrap 失败: {:?}", err),
        }
    }
}

// ============================================================================
// 2. Trait 系统示例
// ============================================================================

/// 基础 Trait：可描述的
trait Describable {
    fn describe(&self) -> String;
    
    // 默认实现
    fn _short_description(&self) -> String {
        let desc = self.describe();
        if desc.len() > 50 {
            format!("{}...", &desc[..47])
        } else {
            desc
        }
    }
}

/// 几何形状 Trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    
    // 关联类型
    type Coordinate;
    
    // 关联常量
    const DIMENSIONS: u32;
    
    // 默认实现
    fn info(&self) -> String {
        format!("面积: {:.2}, 周长: {:.2}", self.area(), self.perimeter())
    }
}

/// 可绘制 Trait
trait Drawable {
    fn draw(&self);
}

/// 矩形结构体
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    type Coordinate = (f64, f64);
    const DIMENSIONS: u32 = 2;
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!("矩形: 宽 {}, 高 {}, {}", self.width, self.height, self.info())
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形: {}x{}", self.width, self.height);
    }
}

/// 圆形结构体
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Circle {
    type Coordinate = (f64, f64);
    const DIMENSIONS: u32 = 2;
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("圆形: 半径 {}, {}", self.radius, self.info())
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形: 半径 {}", self.radius);
    }
}

/// Trait 继承示例
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

trait Pet: Animal + Clone {
    fn owner(&self) -> &str;
    fn is_friendly(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
struct Dog {
    name: String,
    owner: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("{} 说: 汪汪!", self.name);
    }
}

impl Pet for Dog {
    fn owner(&self) -> &str {
        &self.owner
    }
}

// ============================================================================
// 3. 生命周期示例
// ============================================================================

/// 生命周期基础：返回较长的字符串
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 包含引用的结构体
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    _part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(part: &'a str) -> Self {
        ImportantExcerpt { _part: part }
    }
    
    fn level(&self) -> i32 {
        3
    }
    
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self._part
    }
}

/// 字符串解析器（零拷贝）
struct StringParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> StringParser<'a> {
    fn new(input: &'a str) -> Self {
        StringParser { input, position: 0 }
    }
    
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        Some(ch)
    }
    
    fn parse_word(&mut self) -> Option<&'a str> {
        let start = self.position;
        
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }
        
        if start == self.position {
            None
        } else {
            Some(&self.input[start..self.position])
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

// ============================================================================
// 4. 高级特性组合示例
// ============================================================================

/// 序列化 Trait
trait Serialize {
    fn serialize(&self) -> String;
}

trait Deserialize: Sized {
    fn deserialize(data: &str) -> Result<Self, String>;
}

/// 用户结构体
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    fn new(id: u32, name: String, email: String) -> Self {
        User { id, name, email }
    }
}

impl Serialize for User {
    fn serialize(&self) -> String {
        format!("{}|{}|{}", self.id, self.name, self.email)
    }
}

impl Deserialize for User {
    fn deserialize(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.split('|').collect();
        if parts.len() != 3 {
            return Err("格式错误：需要3个字段".to_string());
        }
        
        let id = parts[0].parse()
            .map_err(|_| "ID 解析错误".to_string())?;
        
        Ok(User {
            id,
            name: parts[1].to_string(),
            email: parts[2].to_string(),
        })
    }
}

/// 泛型缓存系统
struct Cache<'a, K, V> 
where 
    K: Eq + std::hash::Hash + Clone,
{
    data: HashMap<K, CacheEntry<'a, V>>,
}

struct CacheEntry<'a, V> {
    value: &'a V,
    access_count: usize,
}

impl<'a, K, V> Cache<'a, K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: K, value: &'a V) {
        let entry = CacheEntry {
            value,
            access_count: 0,
        };
        self.data.insert(key, entry);
    }
    
    fn get(&mut self, key: &K) -> Option<&'a V> {
        if let Some(entry) = self.data.get_mut(key) {
            entry.access_count += 1;
            Some(entry.value)
        } else {
            None
        }
    }
    
    fn stats(&self, key: &K) -> Option<usize> {
        self.data.get(key).map(|entry| entry.access_count)
    }
}

/// 泛型容器：栈
#[derive(Debug)]
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
    
    fn _is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn _len(&self) -> usize {
        self.items.len()
    }
}

// 为 Stack 实现 Display trait
impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack [")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// ============================================================================
// 5. 实用函数
// ============================================================================

/// 使用 Trait 约束的泛型函数
fn _print_info<T>(item: &T) 
where 
    T: Debug + Display,
{
    println!("Debug: {:?}", item);
    println!("Display: {}", item);
}

/// 处理可序列化对象的泛型函数
fn save_to_string<T: Serialize>(item: &T) -> String {
    item.serialize()
}

fn load_from_string<T: Deserialize>(data: &str) -> Result<T, String> {
    T::deserialize(data)
}

/// 使用 Trait 对象的函数
fn draw_shapes(shapes: &[Box<dyn Drawable>]) {
    for shape in shapes {
        shape.draw();
    }
}

fn describe_shapes(shapes: &[&dyn Describable]) {
    for shape in shapes {
        println!("{}", shape.describe());
    }
}

// ============================================================================
// 6. 主函数和测试
// ============================================================================

fn main() {
    println!("=== 泛型、特征和生命周期综合示例 ===");
    
    // 泛型基础示例
    println!("\n1. 泛型基础示例:");
    demonstrate_generics();
    
    // Trait 系统示例
    println!("\n2. Trait 系统示例:");
    demonstrate_traits();
    
    // 生命周期示例
    println!("\n3. 生命周期示例:");
    demonstrate_lifetimes();
    
    // 高级特性组合
    println!("\n4. 高级特性组合:");
    demonstrate_advanced_features();
    
    // 实际应用示例
    println!("\n5. 实际应用示例:");
    demonstrate_practical_examples();
}

fn demonstrate_generics() {
    // 泛型函数
    let mut a = 5;
    let mut b = 10;
    println!("交换前: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("交换后: a = {}, b = {}", a, b);
    
    // 找最大值
    let numbers = [1, 5, 3, 9, 2];
    if let Some(max) = find_max(&numbers) {
        println!("最大值: {}", max);
    }
    
    // 泛型结构体
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    println!("整数点: {:?}", int_point);
    println!("浮点数点: {:?}, 距离原点: {:.2}", 
             float_point, float_point.distance_from_origin());
    
    // 泛型枚举
    let success: MyResult<i32, &str> = MyResult::Success(42);
    let failure: MyResult<i32, &str> = MyResult::Failure("错误信息");
    println!("成功结果: {:?}, 是否成功: {}", success, success.is_success());
    println!("失败结果: {:?}, 是否成功: {}", failure, failure.is_success());
}

fn demonstrate_traits() {
    // 创建形状
    let rectangle = Rectangle::new(5.0, 3.0);
    let circle = Circle::new(2.0);
    
    // 使用 Trait 方法
    println!("矩形面积: {:.2}", rectangle.area());
    println!("圆形周长: {:.2}", circle.perimeter());
    
    // 使用 Trait 对象
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(rectangle),
        Box::new(circle),
    ];
    draw_shapes(&shapes);
    
    // Trait 继承
    let dog = Dog {
        name: "旺财".to_string(),
        owner: "小明".to_string(),
    };
    dog.make_sound();
    println!("{} 的主人是 {}", dog.name(), dog.owner());
    println!("是否友好: {}", dog.is_friendly());
}

fn demonstrate_lifetimes() {
    // 基础生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("较长的字符串: {}", result);
    
    // 结构体中的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt::new(first_sentence);
    println!("摘录: {:?}", excerpt);
    println!("级别: {}", excerpt.level());
    
    // 字符串解析器
    let input = "hello world rust programming";
    let mut parser = StringParser::new(input);
    
    println!("解析单词:");
    while parser.position < input.len() {
        parser.skip_whitespace();
        if let Some(word) = parser.parse_word() {
            println!("  {}", word);
        } else {
            break;
        }
    }
}

fn demonstrate_advanced_features() {
    // 序列化和反序列化
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    let serialized = save_to_string(&user);
    println!("序列化用户: {}", serialized);
    
    let deserialized: User = load_from_string(&serialized).unwrap();
    println!("反序列化用户: {:?}", deserialized);
    
    // 泛型缓存
    let data1 = String::from("重要数据1");
    let data2 = String::from("重要数据2");
    
    let mut cache = Cache::new();
    cache.insert("key1", &data1);
    cache.insert("key2", &data2);
    
    if let Some(value) = cache.get(&"key1") {
        println!("缓存获取: {}", value);
    }
    
    if let Some(count) = cache.stats(&"key1") {
        println!("访问次数: {}", count);
    }
}

fn demonstrate_practical_examples() {
    // 泛型栈
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈: {}", stack);
    println!("栈顶: {:?}", stack.peek());
    println!("弹出: {:?}", stack.pop());
    println!("栈: {}", stack);
    
    // 字符串栈
    let mut string_stack = Stack::new();
    string_stack.push("first".to_string());
    string_stack.push("second".to_string());
    string_stack.push("third".to_string());
    
    println!("字符串栈: {}", string_stack);
    
    // 形状描述
    let rectangle = Rectangle::new(4.0, 6.0);
    let circle = Circle::new(3.0);
    
    let describable_shapes: Vec<&dyn Describable> = vec![&rectangle, &circle];
    describe_shapes(&describable_shapes);
}

// ============================================================================
// 7. 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_swap() {
        let mut a = 5;
        let mut b = 10;
        swap(&mut a, &mut b);
        assert_eq!(a, 10);
        assert_eq!(b, 5);
    }
    
    #[test]
    fn test_find_max() {
        let numbers = [1, 5, 3, 9, 2];
        assert_eq!(find_max(&numbers), Some(9));
        
        let empty: [i32; 0] = [];
        assert_eq!(find_max(&empty), None);
    }
    
    #[test]
    fn test_point() {
        let point = Point::new(3.0, 4.0);
        assert_eq!(point.distance_from_origin(), 5.0);
    }
    
    #[test]
    fn test_my_result() {
        let success: MyResult<i32, &str> = MyResult::Success(42);
        assert!(success.is_success());
        assert_eq!(success._unwrap(), 42);
        
        let failure: MyResult<i32, &str> = MyResult::Failure("error");
        assert!(!failure.is_success());
    }
    
    #[test]
    fn test_shapes() {
        let rectangle = Rectangle::new(5.0, 3.0);
        assert_eq!(rectangle.area(), 15.0);
        assert_eq!(rectangle.perimeter(), 16.0);
        
        let circle = Circle::new(2.0);
        assert!((circle.area() - (std::f64::consts::PI * 4.0)).abs() < 1e-10);
    }
    
    #[test]
    fn test_longest() {
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("same", "size"), "size"); // 相等时返回第二个
    }
    
    #[test]
    fn test_user_serialization() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        let serialized = user.serialize();
        let deserialized = User::deserialize(&serialized).unwrap();
        assert_eq!(user, deserialized);
    }
    
    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack._is_empty());
        
        stack.push(1);
        stack.push(2);
        assert_eq!(stack._len(), 2);
        assert_eq!(stack.peek(), Some(&2));
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack._is_empty());
    }
    
    #[test]
    fn test_string_parser() {
        let input = "hello world";
        let mut parser = StringParser::new(input);
        
        assert_eq!(parser.parse_word(), Some("hello"));
        parser.skip_whitespace();
        assert_eq!(parser.parse_word(), Some("world"));
        assert_eq!(parser.parse_word(), None);
    }
    
    #[test]
    fn test_cache() {
        let data = String::from("test data");
        let mut cache = Cache::new();
        
        cache.insert("key", &data);
        assert_eq!(cache.get(&"key"), Some(&data));
        assert_eq!(cache.stats(&"key"), Some(1));
        
        cache.get(&"key"); // 再次访问
        assert_eq!(cache.stats(&"key"), Some(2));
    }
}
