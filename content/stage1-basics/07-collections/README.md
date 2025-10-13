# 第六章：集合

## 章节概述

集合（Collections）是编程中最常用的数据结构之一。Rust 标准库提供了多种高效、安全的集合类型，每种都有其特定的使用场景和性能特征。本章将深入介绍 Rust 中的主要集合类型，帮助你选择和使用最适合的数据结构。

## 学习目标

通过本章学习，你将能够：

- 掌握 Vector 动态数组的使用和优化技巧
- 理解 HashMap 哈希映射的工作原理和最佳实践
- 熟悉其他集合类型的特点和应用场景
- 根据需求选择最合适的集合类型
- 理解各种集合的性能特征和内存使用
- 掌握集合操作的惯用模式和最佳实践

## 核心概念

### 1. Vector 动态数组

```rust
// 创建和使用 Vector
let mut numbers = vec![1, 2, 3, 4, 5];
numbers.push(6);

// 安全访问
if let Some(third) = numbers.get(2) {
    println!("第三个元素: {}", third);
}

// 迭代处理
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

### 2. HashMap 哈希映射

```rust
use std::collections::HashMap;

// 创建和使用 HashMap
let mut scores = HashMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);

// Entry API 的强大功能
let count = scores.entry("Charlie").or_insert(0);
*count += 10;

// 安全访问和迭代
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### 3. 其他重要集合

```rust
use std::collections::{BTreeMap, HashSet, VecDeque, BinaryHeap};

// BTreeMap - 有序映射
let mut btree = BTreeMap::new();
btree.insert(3, "three");
btree.insert(1, "one");
// 按键排序迭代

// HashSet - 唯一值集合
let mut set = HashSet::new();
set.insert("unique");
let is_new = set.insert("unique"); // false

// VecDeque - 双端队列
let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(0);

// BinaryHeap - 优先队列
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);
let max = heap.pop(); // Some(4)
```

## 章节结构

### [01-vectors.md](./01-vectors.md) - Vector 动态数组
- Vector 的基本概念和内存布局
- 创建和初始化方法
- 基本操作（增删改查）
- 迭代和遍历技巧
- 容量管理和性能优化
- 实际应用示例

### [02-hashmaps.md](./02-hashmaps.md) - HashMap 哈希映射
- HashMap 的工作原理
- 创建和基本操作
- Entry API 的高级用法
- 自定义键类型的要求
- 迭代和遍历模式
- 性能优化技巧
- 实际应用场景

### [03-other-collections.md](./03-other-collections.md) - 其他集合类型
- BTreeMap 有序映射
- HashSet 和 BTreeSet 集合
- VecDeque 双端队列
- BinaryHeap 优先队列
- LinkedList 链表（及其局限性）
- 集合类型选择指南

## 实践项目建议

### 初级项目

1. **单词统计器**
   - 使用 HashMap 统计文本中单词出现频率
   - 练习字符串处理和哈希映射操作

2. **学生成绩管理**
   - 使用 Vec 存储学生信息
   - 使用 HashMap 建立姓名到成绩的映射
   - 实现排序、查找、统计功能

3. **简单缓存系统**
   - 使用 HashMap 实现 LRU 缓存
   - 练习 Entry API 和容量管理

### 中级项目

1. **任务调度器**
   - 使用 BinaryHeap 实现优先队列
   - 使用 VecDeque 管理任务队列
   - 实现任务依赖关系处理

2. **图数据结构**
   - 使用 HashMap 存储邻接表
   - 实现图的遍历算法（BFS、DFS）
   - 使用 BTreeMap 实现有序的图操作

3. **配置管理系统**
   - 使用嵌套的 HashMap 存储配置
   - 实现配置的层级覆盖
   - 支持不同数据类型的配置值

### 高级项目

1. **内存数据库**
   - 使用多种集合类型实现索引
   - 实现查询优化和缓存策略
   - 支持事务和并发访问

2. **分布式一致性哈希**
   - 使用 BTreeMap 实现一致性哈希环
   - 实现节点的动态添加和删除
   - 处理数据迁移和负载均衡

## 性能对比和选择指南

### 时间复杂度对比

| 操作 | Vec | HashMap | BTreeMap | HashSet | VecDeque | BinaryHeap |
|------|-----|---------|----------|---------|----------|-----------|
| 访问 | O(1) | O(1)* | O(log n) | O(1)* | O(1) | O(1) |
| 插入 | O(1)* | O(1)* | O(log n) | O(1)* | O(1) | O(log n) |
| 删除 | O(n) | O(1)* | O(log n) | O(1)* | O(1) | O(log n) |
| 查找 | O(n) | O(1)* | O(log n) | O(1)* | O(n) | O(n) |

*平均情况，最坏情况可能退化到 O(n)

### 选择决策树

```
需要存储数据？
├─ 需要键值对映射？
│  ├─ 需要有序？ → BTreeMap
│  └─ 不需要有序？ → HashMap
├─ 需要唯一值集合？
│  ├─ 需要有序？ → BTreeSet
│  └─ 不需要有序？ → HashSet
├─ 需要优先队列？ → BinaryHeap
├─ 需要双端操作？ → VecDeque
└─ 一般序列存储？ → Vec
```

## 最佳实践总结

### 1. 容量预分配

```rust
// 已知大概大小时预分配容量
let mut vec = Vec::with_capacity(1000);
let mut map = HashMap::with_capacity(100);
```

### 2. 选择合适的迭代方式

```rust
// 不需要修改时使用不可变引用
for item in &collection {
    println!("{:?}", item);
}

// 需要修改时使用可变引用
for item in &mut collection {
    *item *= 2;
}

// 需要获取所有权时使用 into_iter
for item in collection {
    process_owned(item);
}
```

### 3. 使用 Entry API

```rust
// 推荐：使用 Entry API
map.entry(key).or_insert(default_value);

// 避免：先检查再插入
if !map.contains_key(&key) {
    map.insert(key, default_value);
}
```

### 4. 错误处理

```rust
// 安全访问，避免 panic
if let Some(value) = vec.get(index) {
    // 处理值
}

// 或提供默认值
let value = map.get(&key).unwrap_or(&default);
```

## 常见陷阱

1. **索引越界**：使用 `get()` 方法代替直接索引
2. **不必要的克隆**：合理使用引用和移动语义
3. **选择错误的集合类型**：根据使用模式选择最适合的类型
4. **忽略容量管理**：预分配容量以避免频繁重新分配
5. **借用检查器冲突**：理解借用规则，合理设计数据访问模式

## 进阶学习路径

1. **深入理解内存布局**
   - 学习各种集合的内部实现
   - 理解缓存友好性和内存局部性

2. **并发集合**
   - 学习 `Arc<Mutex<HashMap>>` 等并发模式
   - 了解无锁数据结构

3. **自定义集合**
   - 实现自己的数据结构
   - 学习 trait 的高级用法

4. **性能优化**
   - 使用 criterion.rs 进行基准测试
   - 学习 profiling 和性能分析工具

## 相关资源

- [Rust 官方文档 - Collections](https://doc.rust-lang.org/std/collections/index.html)
- [Rust By Example - Collections](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [The Rust Programming Language - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust Algorithm Club](https://github.com/EbTech/rust-algorithms)

---

通过本章的学习，你将掌握 Rust 中各种集合类型的使用方法，能够根据具体需求选择最合适的数据结构，并编写出高效、安全的 Rust 代码。