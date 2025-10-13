# HashMap 哈希映射

## 学习目标

通过本节学习，你将掌握：

- HashMap 的基本概念和特性
- HashMap 的创建和初始化方法
- HashMap 的基本操作（增删改查）
- HashMap 的迭代和遍历
- HashMap 的高级用法和性能优化
- 自定义键类型的要求和实现

## 基本概念

### 什么是 HashMap

HashMap（`HashMap<K, V>`）是 Rust 标准库提供的哈希表实现，具有以下特点：

- **键值对存储**：存储键值对（key-value pairs）
- **快速查找**：平均 O(1) 时间复杂度的查找、插入和删除
- **无序存储**：不保证元素的插入顺序
- **唯一键**：每个键只能出现一次
- **所有权管理**：遵循 Rust 的所有权规则

### HashMap 的内存布局

```rust
use std::collections::HashMap;

// HashMap 内部结构（简化）
// - 哈希表：存储键值对的桶数组
// - 哈希函数：将键映射到桶索引
// - 负载因子：控制何时扩容
let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");
```

## HashMap 的创建

### 1. 使用 `HashMap::new()` 创建空 HashMap

```rust
use std::collections::HashMap;

// 创建空的 HashMap
let mut scores: HashMap<String, i32> = HashMap::new();

// 类型推断
let mut ages = HashMap::new();
ages.insert("Alice", 25); // 编译器推断为 HashMap<&str, i32>
```

### 2. 使用 `HashMap::with_capacity()` 预分配容量

```rust
use std::collections::HashMap;

// 预分配容量，减少重新哈希
let mut map = HashMap::with_capacity(100);
println!("容量: {}", map.capacity()); // >= 100
```

### 3. 从迭代器创建

```rust
use std::collections::HashMap;

// 从元组迭代器创建
let teams = vec![
    ("Blue".to_string(), 10),
    ("Yellow".to_string(), 50),
    ("Red".to_string(), 30),
];
let scores: HashMap<String, i32> = teams.into_iter().collect();

// 从两个 Vec 创建
let keys = vec!["a", "b", "c"];
let values = vec![1, 2, 3];
let map: HashMap<&str, i32> = keys.into_iter().zip(values).collect();
```

### 4. 使用宏创建（需要外部 crate）

```rust
// 使用 maplit crate 的 hashmap! 宏
// Cargo.toml: maplit = "1.0"
use maplit::hashmap;

let map = hashmap! {
    "key1" => "value1",
    "key2" => "value2",
    "key3" => "value3",
};
```

## HashMap 的基本操作

### 插入和更新

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// 插入新键值对
scores.insert("Blue".to_string(), 10);
scores.insert("Yellow".to_string(), 50);

// 更新已存在的键
scores.insert("Blue".to_string(), 25); // 覆盖原值

// 只在键不存在时插入
scores.entry("Red".to_string()).or_insert(30);
scores.entry("Blue".to_string()).or_insert(100); // 不会插入，因为键已存在

// 基于旧值更新
let count = scores.entry("Green".to_string()).or_insert(0);
*count += 10;

println!("{:?}", scores);
// 可能输出: {"Yellow": 50, "Blue": 25, "Red": 30, "Green": 10}
```

### 访问元素

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Yellow", 50);

// 使用 get 方法安全访问
match scores.get("Blue") {
    Some(score) => println!("蓝队得分: {}", score),
    None => println!("蓝队未找到"),
}

// 使用 get 和 unwrap_or 提供默认值
let blue_score = scores.get("Blue").unwrap_or(&0);
let green_score = scores.get("Green").unwrap_or(&0);

// 获取可变引用
if let Some(score) = scores.get_mut("Blue") {
    *score += 10;
}

// 检查键是否存在
if scores.contains_key("Yellow") {
    println!("黄队存在");
}

// 获取键值对的数量
println!("队伍数量: {}", scores.len());
```

### 删除元素

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Yellow", 50);
scores.insert("Red", 30);

// 删除指定键的元素
if let Some(removed) = scores.remove("Blue") {
    println!("删除了蓝队，得分: {}", removed);
}

// 删除键值对并返回
if let Some((key, value)) = scores.remove_entry("Yellow") {
    println!("删除了 {}: {}", key, value);
}

// 清空所有元素
scores.clear();
println!("清空后长度: {}", scores.len()); // 0

// 检查是否为空
if scores.is_empty() {
    println!("HashMap 为空");
}
```

## Entry API 的高级用法

Entry API 提供了更高效和灵活的方式来操作 HashMap：

### 基本 Entry 操作

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// or_insert: 键不存在时插入
map.entry("key1").or_insert("default_value");
map.entry("key1").or_insert("another_value"); // 不会覆盖

// or_insert_with: 使用闭包延迟计算默认值
map.entry("key2").or_insert_with(|| {
    println!("计算默认值"); // 只在键不存在时执行
    "computed_value"
});

// and_modify: 修改已存在的值
map.entry("key1")
    .and_modify(|v| *v = "modified")
    .or_insert("default");

println!("{:?}", map);
```

### 复杂的 Entry 操作

```rust
use std::collections::HashMap;

// 统计单词出现次数
let text = "hello world hello rust world";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", word_count);
// {"hello": 2, "world": 2, "rust": 1}

// 更复杂的例子：按首字母分组
let words = vec!["apple", "banana", "cherry", "apricot", "blueberry"];
let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

for word in words {
    if let Some(first_char) = word.chars().next() {
        groups.entry(first_char)
            .or_insert_with(Vec::new)
            .push(word);
    }
}

println!("{:?}", groups);
// {'a': ["apple", "apricot"], 'b': ["banana", "blueberry"], 'c': ["cherry"]}
```

## HashMap 的迭代

### 基本迭代

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

// 迭代键值对
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// 只迭代键
for key in map.keys() {
    println!("键: {}", key);
}

// 只迭代值
for value in map.values() {
    println!("值: {}", value);
}

// 可变迭代值
for value in map.values_mut() {
    *value *= 2;
}

// 获取所有权的迭代
for (key, value) in map {
    println!("{}: {}", key, value);
}
// map 在此后不可用
```

### 迭代器方法

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);
scores.insert("Charlie", 92);
scores.insert("Diana", 88);

// 过滤高分学生
let high_scores: HashMap<&str, &i32> = scores
    .iter()
    .filter(|(_, &score)| score >= 90)
    .collect();

// 转换为不同格式
let score_strings: Vec<String> = scores
    .iter()
    .map(|(name, score)| format!("{}: {}", name, score))
    .collect();

// 查找最高分
if let Some((name, score)) = scores.iter().max_by_key(|(_, score)| *score) {
    println!("最高分: {} - {}", name, score);
}

// 计算平均分
let average: f64 = scores.values().sum::<i32>() as f64 / scores.len() as f64;
println!("平均分: {:.2}", average);
```

## 自定义键类型

### 键的要求

要作为 HashMap 的键，类型必须实现：
- `Hash` trait：用于计算哈希值
- `Eq` trait：用于比较键的相等性

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

// 手动实现 Hash
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

// 使用自定义类型作为键
let mut people_scores = HashMap::new();
let alice = Person {
    name: "Alice".to_string(),
    age: 25,
};
people_scores.insert(alice, 95);
```

### 使用 derive 宏

```rust
use std::collections::HashMap;

// 自动派生 Hash 和 Eq
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

let mut grid = HashMap::new();
grid.insert(Coordinate { x: 0, y: 0 }, "origin");
grid.insert(Coordinate { x: 1, y: 1 }, "northeast");

if let Some(value) = grid.get(&Coordinate { x: 0, y: 0 }) {
    println!("原点处的值: {}", value);
}
```

### 复合键

```rust
use std::collections::HashMap;

// 使用元组作为复合键
type UserSession = (String, u64); // (user_id, session_id)

let mut sessions: HashMap<UserSession, String> = HashMap::new();
sessions.insert(("alice".to_string(), 12345), "active".to_string());
sessions.insert(("bob".to_string(), 67890), "inactive".to_string());

// 查找特定用户会话
let session_key = ("alice".to_string(), 12345);
if let Some(status) = sessions.get(&session_key) {
    println!("会话状态: {}", status);
}
```

## 实际应用示例

### 1. 缓存系统

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

struct Cache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new(default_ttl: Duration) -> Self {
        Cache {
            data: HashMap::new(),
            default_ttl,
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + self.default_ttl,
        };
        self.data.insert(key, entry);
    }
    
    fn get(&mut self, key: &K) -> Option<V> {
        // 清理过期条目
        let now = Instant::now();
        if let Some(entry) = self.data.get(key) {
            if entry.expires_at > now {
                Some(entry.value.clone())
            } else {
                self.data.remove(key);
                None
            }
        } else {
            None
        }
    }
    
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, entry| entry.expires_at > now);
    }
}
```

### 2. 配置管理器

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<ConfigValue>),
}

struct ConfigManager {
    config: HashMap<String, ConfigValue>,
    defaults: HashMap<String, ConfigValue>,
}

impl ConfigManager {
    fn new() -> Self {
        ConfigManager {
            config: HashMap::new(),
            defaults: HashMap::new(),
        }
    }
    
    fn set_default(&mut self, key: String, value: ConfigValue) {
        self.defaults.insert(key, value);
    }
    
    fn set(&mut self, key: String, value: ConfigValue) {
        self.config.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.config.get(key).or_else(|| self.defaults.get(key))
    }
    
    fn get_string(&self, key: &str) -> Option<&String> {
        match self.get(key) {
            Some(ConfigValue::String(s)) => Some(s),
            _ => None,
        }
    }
    
    fn get_integer(&self, key: &str) -> Option<i64> {
        match self.get(key) {
            Some(ConfigValue::Integer(i)) => Some(*i),
            _ => None,
        }
    }
    
    fn get_boolean(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some(ConfigValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }
}
```

### 3. 简单的数据库索引

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Record {
    id: u64,
    name: String,
    email: String,
    age: u32,
}

struct Database {
    records: HashMap<u64, Record>,
    name_index: HashMap<String, u64>,
    email_index: HashMap<String, u64>,
}

impl Database {
    fn new() -> Self {
        Database {
            records: HashMap::new(),
            name_index: HashMap::new(),
            email_index: HashMap::new(),
        }
    }
    
    fn insert(&mut self, record: Record) -> Result<(), String> {
        // 检查邮箱是否已存在
        if self.email_index.contains_key(&record.email) {
            return Err("邮箱已存在".to_string());
        }
        
        let id = record.id;
        
        // 更新索引
        self.name_index.insert(record.name.clone(), id);
        self.email_index.insert(record.email.clone(), id);
        
        // 插入记录
        self.records.insert(id, record);
        
        Ok(())
    }
    
    fn get_by_id(&self, id: u64) -> Option<&Record> {
        self.records.get(&id)
    }
    
    fn get_by_name(&self, name: &str) -> Option<&Record> {
        self.name_index.get(name)
            .and_then(|id| self.records.get(id))
    }
    
    fn get_by_email(&self, email: &str) -> Option<&Record> {
        self.email_index.get(email)
            .and_then(|id| self.records.get(id))
    }
    
    fn delete(&mut self, id: u64) -> Option<Record> {
        if let Some(record) = self.records.remove(&id) {
            // 清理索引
            self.name_index.remove(&record.name);
            self.email_index.remove(&record.email);
            Some(record)
        } else {
            None
        }
    }
}
```

## 性能优化

### 1. 容量预分配

```rust
use std::collections::HashMap;

// 低效：频繁重新哈希
let mut map = HashMap::new();
for i in 0..1000 {
    map.insert(i, i * 2);
}

// 高效：预分配容量
let mut map = HashMap::with_capacity(1000);
for i in 0..1000 {
    map.insert(i, i * 2);
}
```

### 2. 选择合适的哈希器

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::collections::hash_map::DefaultHasher;

// 默认哈希器（安全但较慢）
let map1: HashMap<i32, String> = HashMap::new();

// 使用 FxHash（更快但不抗 DoS 攻击）
// 需要添加 rustc-hash crate
// use rustc_hash::FxHashMap;
// let map2: FxHashMap<i32, String> = FxHashMap::default();

// 对于已知安全的场景，可以使用更快的哈希器
```

### 3. 避免不必要的克隆

```rust
use std::collections::HashMap;

// 低效：不必要的字符串克隆
let mut map = HashMap::new();
let key = "expensive_key".to_string();
map.insert(key.clone(), "value"); // 克隆了键

// 高效：直接移动所有权
let mut map = HashMap::new();
let key = "expensive_key".to_string();
map.insert(key, "value"); // 移动键的所有权

// 或者使用引用作为键（如果生命周期允许）
let mut map: HashMap<&str, &str> = HashMap::new();
map.insert("key", "value");
```

### 4. 批量操作

```rust
use std::collections::HashMap;

// 低效：逐个插入
let mut map = HashMap::new();
let data = vec![("a", 1), ("b", 2), ("c", 3)];
for (k, v) in data {
    map.insert(k, v);
}

// 高效：使用 extend
let mut map = HashMap::new();
let data = vec![("a", 1), ("b", 2), ("c", 3)];
map.extend(data);

// 或者直接从迭代器创建
let data = vec![("a", 1), ("b", 2), ("c", 3)];
let map: HashMap<&str, i32> = data.into_iter().collect();
```

## 最佳实践

### 1. 选择合适的键类型

```rust
use std::collections::HashMap;

// 推荐：使用 &str 作为键（如果生命周期允许）
fn process_data(data: &HashMap<&str, i32>) {
    // 处理数据
}

// 或者使用 String（如果需要拥有所有权）
fn store_data() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), 42);
    map
}

// 避免：不必要的复杂键类型
// 除非确实需要，否则避免使用复杂的结构体作为键
```

### 2. 合理使用 Entry API

```rust
use std::collections::HashMap;

// 推荐：使用 Entry API
let mut map = HashMap::new();
map.entry("key").or_insert(0);

// 避免：先检查再插入
let mut map = HashMap::new();
if !map.contains_key("key") {
    map.insert("key", 0); // 进行了两次哈希查找
}
```

### 3. 错误处理

```rust
use std::collections::HashMap;

// 推荐：使用 get 方法
fn safe_lookup(map: &HashMap<String, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()
}

// 推荐：提供默认值
fn lookup_with_default(map: &HashMap<String, i32>, key: &str) -> i32 {
    map.get(key).copied().unwrap_or(0)
}

// 避免：直接索引（HashMap 不支持索引）
// let value = map[key]; // 编译错误
```

## 常见错误

### 1. 借用检查器问题

```rust
use std::collections::HashMap;

// 错误：同时持有可变和不可变引用
let mut map = HashMap::new();
map.insert("key", "value");
let value = map.get("key"); // 不可变借用
map.insert("key2", "value2"); // 错误：可变借用
// println!("{:?}", value);

// 正确：分离借用作用域
let mut map = HashMap::new();
map.insert("key", "value");
{
    let value = map.get("key").copied(); // 复制值
    map.insert("key2", "value2");
    if let Some(v) = value {
        println!("{}", v);
    }
}
```

### 2. 键的生命周期问题

```rust
use std::collections::HashMap;

// 错误：键的生命周期不够长
fn create_map() -> HashMap<&str, i32> {
    let mut map = HashMap::new();
    let key = String::from("key");
    // map.insert(&key, 42); // 错误：key 在函数结束时被销毁
    map
}

// 正确：使用拥有所有权的键
fn create_map_correct() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), 42);
    map
}
```

### 3. 哈希和相等性不一致

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// 错误：Hash 和 PartialEq 不一致
#[derive(Debug)]
struct BadKey {
    id: u32,
    name: String,
}

impl PartialEq for BadKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // 只比较 id
    }
}

impl Eq for BadKey {}

impl Hash for BadKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state); // 错误：哈希了 name 但相等性检查中没有
    }
}

// 正确：Hash 和 PartialEq 保持一致
#[derive(Debug, PartialEq, Eq, Hash)]
struct GoodKey {
    id: u32,
    name: String,
}
```

## 学习检查清单

- [ ] 理解 HashMap 的基本概念和特性
- [ ] 掌握 HashMap 的各种创建方法
- [ ] 熟练使用 HashMap 的增删改查操作
- [ ] 理解和使用 Entry API
- [ ] 掌握 HashMap 的迭代方法
- [ ] 了解自定义键类型的要求
- [ ] 理解 HashMap 的性能特性
- [ ] 掌握 HashMap 的最佳实践
- [ ] 能够避免常见的使用错误

## 扩展阅读

- [Rust 官方文档 - HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Rust By Example - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [The Rust Programming Language - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust Performance Book - HashMap](https://nnethercote.github.io/perf-book/standard-library-types.html#hashmap)
- [Hash, Hasher, and BuildHasher](https://doc.rust-lang.org/std/hash/index.html)