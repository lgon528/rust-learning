# 其他集合类型

## 学习目标

通过本节学习，你将掌握：

- BTreeMap 的特性和使用场景
- HashSet 和 BTreeSet 的使用
- VecDeque 双端队列的应用
- BinaryHeap 优先队列的使用
- LinkedList 链表的特点
- 各种集合类型的性能对比和选择原则

## BTreeMap 有序映射

### 基本概念

BTreeMap 是基于 B 树实现的有序键值对集合：

- **有序存储**：按键的顺序存储元素
- **对数时间复杂度**：查找、插入、删除都是 O(log n)
- **范围查询**：支持高效的范围查询
- **键必须实现 Ord**：用于排序

```rust
use std::collections::BTreeMap;

// 创建 BTreeMap
let mut scores = BTreeMap::new();
scores.insert("Charlie", 85);
scores.insert("Alice", 95);
scores.insert("Bob", 90);

// 按键的顺序迭代
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
// 输出：Alice: 95, Bob: 90, Charlie: 85
```

### BTreeMap 的高级功能

```rust
use std::collections::BTreeMap;
use std::ops::Bound::*;

let mut map = BTreeMap::new();
map.insert(1, "one");
map.insert(3, "three");
map.insert(5, "five");
map.insert(7, "seven");
map.insert(9, "nine");

// 范围查询
for (k, v) in map.range(3..=7) {
    println!("{}: {}", k, v);
}
// 输出：3: three, 5: five, 7: seven

// 使用 Bound 进行更复杂的范围查询
for (k, v) in map.range((Included(3), Excluded(7))) {
    println!("{}: {}", k, v);
}
// 输出：3: three, 5: five

// 分割操作
let mut right_map = map.split_off(&5);
println!("左半部分: {:?}", map);      // {1: "one", 3: "three"}
println!("右半部分: {:?}", right_map); // {5: "five", 7: "seven", 9: "nine"}

// 获取第一个和最后一个元素
map.insert(5, "five");
map.insert(7, "seven");
map.insert(9, "nine");

if let Some((first_key, first_value)) = map.first_key_value() {
    println!("第一个: {}: {}", first_key, first_value);
}

if let Some((last_key, last_value)) = map.last_key_value() {
    println!("最后一个: {}: {}", last_key, last_value);
}
```

### BTreeMap vs HashMap 对比

```rust
use std::collections::{HashMap, BTreeMap};

// 性能对比示例
fn performance_comparison() {
    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();
    
    // HashMap: O(1) 平均时间复杂度
    hash_map.insert("key", "value"); // 快速插入
    let value = hash_map.get("key");  // 快速查找
    
    // BTreeMap: O(log n) 时间复杂度
    btree_map.insert("key", "value"); // 较慢插入，但有序
    let value = btree_map.get("key");  // 较慢查找，但支持范围查询
    
    // BTreeMap 的优势：有序迭代和范围查询
    for (k, v) in &btree_map {
        println!("{}: {}", k, v); // 按键排序输出
    }
}
```

## HashSet 和 BTreeSet

### HashSet 哈希集合

```rust
use std::collections::HashSet;

// 创建 HashSet
let mut set1: HashSet<i32> = HashSet::new();
set1.insert(1);
set1.insert(2);
set1.insert(3);

let mut set2 = HashSet::new();
set2.insert(2);
set2.insert(3);
set2.insert(4);

// 集合操作
let intersection: HashSet<_> = set1.intersection(&set2).collect();
println!("交集: {:?}", intersection); // {2, 3}

let union: HashSet<_> = set1.union(&set2).collect();
println!("并集: {:?}", union); // {1, 2, 3, 4}

let difference: HashSet<_> = set1.difference(&set2).collect();
println!("差集: {:?}", difference); // {1}

let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).collect();
println!("对称差集: {:?}", symmetric_difference); // {1, 4}

// 子集检查
let small_set: HashSet<_> = [2, 3].iter().collect();
if small_set.is_subset(&set1) {
    println!("small_set 是 set1 的子集");
}
```

### BTreeSet 有序集合

```rust
use std::collections::BTreeSet;

// 创建 BTreeSet
let mut set = BTreeSet::new();
set.insert(5);
set.insert(2);
set.insert(8);
set.insert(1);

// 有序迭代
for item in &set {
    println!("{}", item);
}
// 输出：1, 2, 5, 8

// 范围查询
for item in set.range(2..=5) {
    println!("{}", item);
}
// 输出：2, 5

// 分割操作
let right_set = set.split_off(&5);
println!("左半部分: {:?}", set);       // {1, 2}
println!("右半部分: {:?}", right_set); // {5, 8}
```

### 实际应用：去重和集合运算

```rust
use std::collections::HashSet;

// 数据去重
fn remove_duplicates<T>(vec: Vec<T>) -> Vec<T>
where
    T: std::hash::Hash + Eq + Clone,
{
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

// 用户权限系统
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

struct User {
    name: String,
    permissions: HashSet<Permission>,
}

impl User {
    fn new(name: String) -> Self {
        User {
            name,
            permissions: HashSet::new(),
        }
    }
    
    fn grant_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }
    
    fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
    
    fn has_all_permissions(&self, required: &HashSet<Permission>) -> bool {
        required.is_subset(&self.permissions)
    }
    
    fn common_permissions(&self, other: &User) -> HashSet<Permission> {
        self.permissions.intersection(&other.permissions).cloned().collect()
    }
}

// 使用示例
let mut alice = User::new("Alice".to_string());
alice.grant_permission(Permission::Read);
alice.grant_permission(Permission::Write);

let mut bob = User::new("Bob".to_string());
bob.grant_permission(Permission::Read);
bob.grant_permission(Permission::Execute);

let common = alice.common_permissions(&bob);
println!("共同权限: {:?}", common); // {Read}
```

## VecDeque 双端队列

### 基本概念和操作

```rust
use std::collections::VecDeque;

// 创建双端队列
let mut deque = VecDeque::new();

// 从两端添加元素
deque.push_back(1);    // [1]
deque.push_front(2);   // [2, 1]
deque.push_back(3);    // [2, 1, 3]
deque.push_front(4);   // [4, 2, 1, 3]

println!("队列: {:?}", deque);

// 从两端移除元素
if let Some(front) = deque.pop_front() {
    println!("从前端移除: {}", front); // 4
}

if let Some(back) = deque.pop_back() {
    println!("从后端移除: {}", back);   // 3
}

println!("剩余队列: {:?}", deque); // [2, 1]

// 访问两端元素（不移除）
if let Some(front) = deque.front() {
    println!("前端元素: {}", front);
}

if let Some(back) = deque.back() {
    println!("后端元素: {}", back);
}
```

### VecDeque 的高级功能

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = (1..=5).collect();
println!("原始队列: {:?}", deque); // [1, 2, 3, 4, 5]

// 旋转操作
deque.rotate_left(2);
println!("左旋转 2 位: {:?}", deque); // [3, 4, 5, 1, 2]

deque.rotate_right(1);
println!("右旋转 1 位: {:?}", deque); // [2, 3, 4, 5, 1]

// 分割操作
let right_deque = deque.split_off(3);
println!("左半部分: {:?}", deque);       // [2, 3, 4]
println!("右半部分: {:?}", right_deque); // [5, 1]

// 在任意位置插入和删除
deque.insert(1, 10);
println!("插入后: {:?}", deque); // [2, 10, 3, 4]

if let Some(removed) = deque.remove(1) {
    println!("移除的元素: {}", removed); // 10
}
println!("移除后: {:?}", deque); // [2, 3, 4]
```

### 实际应用：滑动窗口

```rust
use std::collections::VecDeque;

struct SlidingWindow<T> {
    window: VecDeque<T>,
    capacity: usize,
}

impl<T> SlidingWindow<T> {
    fn new(capacity: usize) -> Self {
        SlidingWindow {
            window: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    
    fn add(&mut self, item: T) {
        if self.window.len() == self.capacity {
            self.window.pop_front();
        }
        self.window.push_back(item);
    }
    
    fn current_window(&self) -> &VecDeque<T> {
        &self.window
    }
    
    fn is_full(&self) -> bool {
        self.window.len() == self.capacity
    }
}

// 计算滑动平均值
struct MovingAverage {
    window: SlidingWindow<f64>,
    sum: f64,
}

impl MovingAverage {
    fn new(window_size: usize) -> Self {
        MovingAverage {
            window: SlidingWindow::new(window_size),
            sum: 0.0,
        }
    }
    
    fn add_value(&mut self, value: f64) {
        if self.window.is_full() {
            if let Some(old_value) = self.window.window.front() {
                self.sum -= old_value;
            }
        }
        
        self.window.add(value);
        self.sum += value;
    }
    
    fn average(&self) -> Option<f64> {
        if self.window.window.is_empty() {
            None
        } else {
            Some(self.sum / self.window.window.len() as f64)
        }
    }
}

// 使用示例
let mut ma = MovingAverage::new(3);
ma.add_value(10.0);
ma.add_value(20.0);
ma.add_value(30.0);
println!("平均值: {:?}", ma.average()); // Some(20.0)

ma.add_value(40.0); // 移除 10.0，添加 40.0
println!("平均值: {:?}", ma.average()); // Some(30.0)
```

## BinaryHeap 优先队列

### 基本概念和操作

```rust
use std::collections::BinaryHeap;

// 创建最大堆（默认）
let mut heap = BinaryHeap::new();

// 添加元素
heap.push(3);
heap.push(1);
heap.push(4);
heap.push(1);
heap.push(5);

println!("堆: {:?}", heap);

// 获取最大元素（不移除）
if let Some(max) = heap.peek() {
    println!("最大元素: {}", max); // 5
}

// 移除并返回最大元素
while let Some(max) = heap.pop() {
    println!("弹出: {}", max);
}
// 输出：5, 4, 3, 1, 1
```

### 自定义优先级

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// 最小堆（使用 Reverse 包装）
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(4));

while let Some(Reverse(min)) = min_heap.pop() {
    println!("最小值: {}", min);
}
// 输出：1, 3, 4

// 自定义结构体的优先级
#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 任务优先队列
let mut task_queue = BinaryHeap::new();
task_queue.push(Task {
    priority: 1,
    description: "低优先级任务".to_string(),
});
task_queue.push(Task {
    priority: 5,
    description: "高优先级任务".to_string(),
});
task_queue.push(Task {
    priority: 3,
    description: "中优先级任务".to_string(),
});

while let Some(task) = task_queue.pop() {
    println!("执行任务: {} (优先级: {})", task.description, task.priority);
}
```

### 实际应用：Dijkstra 算法

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Distance(u32);

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0) // 反向比较，实现最小堆
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    distance: Distance,
    node: NodeId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    edges: HashMap<NodeId, Vec<(NodeId, u32)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: u32) {
        self.edges.entry(from).or_insert_with(Vec::new).push((to, weight));
    }
    
    fn dijkstra(&self, start: NodeId) -> HashMap<NodeId, u32> {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        distances.insert(start, 0);
        heap.push(State {
            distance: Distance(0),
            node: start,
        });
        
        while let Some(State { distance, node }) = heap.pop() {
            if distance.0 > *distances.get(&node).unwrap_or(&u32::MAX) {
                continue;
            }
            
            if let Some(neighbors) = self.edges.get(&node) {
                for &(neighbor, weight) in neighbors {
                    let new_distance = distance.0 + weight;
                    let current_distance = *distances.get(&neighbor).unwrap_or(&u32::MAX);
                    
                    if new_distance < current_distance {
                        distances.insert(neighbor, new_distance);
                        heap.push(State {
                            distance: Distance(new_distance),
                            node: neighbor,
                        });
                    }
                }
            }
        }
        
        distances
    }
}
```

## LinkedList 链表

### 基本概念

LinkedList 是双向链表实现，但在 Rust 中很少使用：

```rust
use std::collections::LinkedList;

// 创建链表
let mut list = LinkedList::new();

// 添加元素
list.push_back(1);
list.push_back(2);
list.push_front(0);

println!("链表: {:?}", list); // [0, 1, 2]

// 分割链表
let mut right_list = list.split_off(1);
println!("左半部分: {:?}", list);       // [0]
println!("右半部分: {:?}", right_list); // [1, 2]

// 合并链表
list.append(&mut right_list);
println!("合并后: {:?}", list); // [0, 1, 2]
```

### 为什么很少使用 LinkedList

```rust
// LinkedList 的问题：
// 1. 缓存局部性差
// 2. 内存开销大（每个节点都有指针）
// 3. 大多数情况下 Vec 性能更好

// 推荐：使用 Vec 代替 LinkedList
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
vec.insert(0, 0); // 在开头插入，虽然是 O(n)，但通常比链表快
```

## 集合类型选择指南

### 性能对比表

| 操作 | Vec | VecDeque | LinkedList | HashMap | BTreeMap | HashSet | BTreeSet | BinaryHeap |
|------|-----|----------|------------|---------|----------|---------|----------|-----------|
| 索引访问 | O(1) | O(1) | O(n) | - | - | - | - | - |
| 头部插入 | O(n) | O(1) | O(1) | - | - | - | - | - |
| 尾部插入 | O(1)* | O(1) | O(1) | - | - | - | - | O(log n) |
| 查找 | O(n) | O(n) | O(n) | O(1)* | O(log n) | O(1)* | O(log n) | O(1) |
| 插入 | O(n) | O(n) | O(1) | O(1)* | O(log n) | O(1)* | O(log n) | O(log n) |
| 删除 | O(n) | O(n) | O(1) | O(1)* | O(log n) | O(1)* | O(log n) | O(log n) |
| 有序迭代 | ✓ | ✓ | ✓ | ✗ | ✓ | ✗ | ✓ | ✗ |
| 范围查询 | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✓ | ✗ |

*平均情况，最坏情况可能是 O(n)

### 选择原则

```rust
// 1. 需要索引访问 -> Vec
let mut data = vec![1, 2, 3, 4, 5];
let third = data[2]; // O(1) 访问

// 2. 需要从两端操作 -> VecDeque
use std::collections::VecDeque;
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);

// 3. 需要快速查找和无序存储 -> HashMap/HashSet
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value"); // O(1) 平均

// 4. 需要有序存储和范围查询 -> BTreeMap/BTreeSet
use std::collections::BTreeMap;
let mut btree = BTreeMap::new();
btree.insert(3, "three");
btree.insert(1, "one");
for (k, v) in btree.range(1..=2) {
    println!("{}: {}", k, v);
}

// 5. 需要优先队列 -> BinaryHeap
use std::collections::BinaryHeap;
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);
let max = heap.pop(); // 获取最大值

// 6. 需要频繁的中间插入/删除 -> LinkedList（很少使用）
// 通常 Vec 或 VecDeque 是更好的选择
```

### 内存使用对比

```rust
use std::mem;
use std::collections::*;

// 比较不同集合的内存开销
fn memory_comparison() {
    println!("Vec<i32> 大小: {} 字节", mem::size_of::<Vec<i32>>());
    println!("VecDeque<i32> 大小: {} 字节", mem::size_of::<VecDeque<i32>>());
    println!("LinkedList<i32> 大小: {} 字节", mem::size_of::<LinkedList<i32>>());
    println!("HashMap<i32, i32> 大小: {} 字节", mem::size_of::<HashMap<i32, i32>>());
    println!("BTreeMap<i32, i32> 大小: {} 字节", mem::size_of::<BTreeMap<i32, i32>>());
    println!("HashSet<i32> 大小: {} 字节", mem::size_of::<HashSet<i32>>());
    println!("BTreeSet<i32> 大小: {} 字节", mem::size_of::<BTreeSet<i32>>());
    println!("BinaryHeap<i32> 大小: {} 字节", mem::size_of::<BinaryHeap<i32>>());
}
```

## 实际应用综合示例

### 任务调度系统

```rust
use std::collections::{HashMap, BinaryHeap, VecDeque};
use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    id: u64,
    priority: u32,
    description: String,
    dependencies: Vec<u64>,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskScheduler {
    tasks: HashMap<u64, Task>,
    ready_queue: BinaryHeap<Task>,
    waiting_queue: VecDeque<Task>,
    completed: Vec<u64>,
}

impl TaskScheduler {
    fn new() -> Self {
        TaskScheduler {
            tasks: HashMap::new(),
            ready_queue: BinaryHeap::new(),
            waiting_queue: VecDeque::new(),
            completed: Vec::new(),
        }
    }
    
    fn add_task(&mut self, task: Task) {
        let task_id = task.id;
        self.tasks.insert(task_id, task.clone());
        
        if task.dependencies.is_empty() {
            self.ready_queue.push(task);
        } else {
            self.waiting_queue.push_back(task);
        }
    }
    
    fn complete_task(&mut self, task_id: u64) {
        self.completed.push(task_id);
        
        // 检查等待队列中的任务是否可以执行
        let mut i = 0;
        while i < self.waiting_queue.len() {
            let task = &self.waiting_queue[i];
            let dependencies_met = task.dependencies.iter()
                .all(|dep_id| self.completed.contains(dep_id));
            
            if dependencies_met {
                let task = self.waiting_queue.remove(i).unwrap();
                self.ready_queue.push(task);
            } else {
                i += 1;
            }
        }
    }
    
    fn get_next_task(&mut self) -> Option<Task> {
        self.ready_queue.pop()
    }
    
    fn status(&self) -> (usize, usize, usize) {
        (self.ready_queue.len(), self.waiting_queue.len(), self.completed.len())
    }
}

// 使用示例
let mut scheduler = TaskScheduler::new();

scheduler.add_task(Task {
    id: 1,
    priority: 5,
    description: "高优先级任务".to_string(),
    dependencies: vec![],
});

scheduler.add_task(Task {
    id: 2,
    priority: 3,
    description: "依赖任务1的任务".to_string(),
    dependencies: vec![1],
});

scheduler.add_task(Task {
    id: 3,
    priority: 4,
    description: "中优先级任务".to_string(),
    dependencies: vec![],
});

// 执行任务
while let Some(task) = scheduler.get_next_task() {
    println!("执行任务: {} (优先级: {})", task.description, task.priority);
    scheduler.complete_task(task.id);
}

let (ready, waiting, completed) = scheduler.status();
println!("状态 - 就绪: {}, 等待: {}, 完成: {}", ready, waiting, completed);
```

## 最佳实践

### 1. 选择合适的集合类型

```rust
// 根据使用模式选择

// 频繁随机访问 -> Vec
let data: Vec<i32> = (0..1000).collect();
let value = data[500]; // O(1)

// 频繁头尾操作 -> VecDeque
use std::collections::VecDeque;
let mut buffer: VecDeque<String> = VecDeque::new();
buffer.push_back("new".to_string());
buffer.pop_front();

// 需要唯一性检查 -> HashSet
use std::collections::HashSet;
let mut seen: HashSet<String> = HashSet::new();
if seen.insert("item".to_string()) {
    println!("首次见到这个项目");
}

// 需要排序和范围查询 -> BTreeMap
use std::collections::BTreeMap;
let mut scores: BTreeMap<String, i32> = BTreeMap::new();
for (name, score) in scores.range("A".to_string().."M".to_string()) {
    println!("{}: {}", name, score);
}
```

### 2. 预分配容量

```rust
use std::collections::{HashMap, HashSet, VecDeque};

// 已知大概大小时预分配
let mut map = HashMap::with_capacity(1000);
let mut set = HashSet::with_capacity(500);
let mut deque = VecDeque::with_capacity(100);
```

### 3. 避免不必要的克隆

```rust
use std::collections::HashMap;

// 低效：不必要的克隆
let mut map = HashMap::new();
let key = "expensive_key".to_string();
map.insert(key.clone(), "value"); // 克隆了键

// 高效：移动所有权
let mut map = HashMap::new();
let key = "expensive_key".to_string();
map.insert(key, "value"); // 移动键的所有权
```

## 学习检查清单

- [ ] 理解 BTreeMap 的特性和使用场景
- [ ] 掌握 HashSet 和 BTreeSet 的集合操作
- [ ] 熟练使用 VecDeque 进行双端操作
- [ ] 理解 BinaryHeap 的优先队列特性
- [ ] 了解 LinkedList 的特点和局限性
- [ ] 能够根据需求选择合适的集合类型
- [ ] 理解各种集合的性能特征
- [ ] 掌握集合类型的最佳实践

## 扩展阅读

- [Rust 官方文档 - Collections](https://doc.rust-lang.org/std/collections/index.html)
- [Rust By Example - Collections](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [The Rust Performance Book - Collections](https://nnethercote.github.io/perf-book/standard-library-types.html)
- [Rust Algorithm Club](https://github.com/EbTech/rust-algorithms)
- [Data Structures and Algorithms in Rust](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust)