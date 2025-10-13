//! 集合类型综合示例
//! 
//! 本示例演示了 Rust 中各种集合类型的使用方法，包括：
//! - Vector 动态数组
//! - HashMap 哈希映射
//! - BTreeMap 有序映射
//! - HashSet 和 BTreeSet 集合
//! - VecDeque 双端队列
//! - BinaryHeap 优先队列

use std::collections::{
    HashMap, BTreeMap, HashSet, BTreeSet, 
    VecDeque, BinaryHeap
};
use std::cmp::Reverse;

fn main() {
    println!("=== Rust 集合类型示例 ===");
    
    // Vector 示例
    println!("\n1. Vector 动态数组示例:");
    vector_examples();
    
    // HashMap 示例
    println!("\n2. HashMap 哈希映射示例:");
    hashmap_examples();
    
    // BTreeMap 示例
    println!("\n3. BTreeMap 有序映射示例:");
    btreemap_examples();
    
    // HashSet 和 BTreeSet 示例
    println!("\n4. Set 集合示例:");
    set_examples();
    
    // VecDeque 示例
    println!("\n5. VecDeque 双端队列示例:");
    vecdeque_examples();
    
    // BinaryHeap 示例
    println!("\n6. BinaryHeap 优先队列示例:");
    binaryheap_examples();
    
    // 实际应用示例
    println!("\n7. 实际应用示例:");
    practical_examples();
}

/// Vector 动态数组示例
fn vector_examples() {
    // 基本创建和操作
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    numbers.extend([7, 8, 9]);
    
    println!("原始数组: {:?}", numbers);
    
    // 安全访问
    match numbers.get(2) {
        Some(value) => println!("索引 2 的值: {}", value),
        None => println!("索引 2 不存在"),
    }
    
    // 迭代器操作
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("翻倍后: {:?}", doubled);
    
    // 过滤操作
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // 容量管理
    let vec_with_capacity: Vec<i32> = Vec::with_capacity(10);
    println!("预分配容量: {}", vec_with_capacity.capacity());
    
    // 切片操作
    let slice = &numbers[1..4];
    println!("切片 [1..4]: {:?}", slice);
}

/// HashMap 哈希映射示例
fn hashmap_examples() {
    // 基本创建和操作
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    println!("学生成绩: {:?}", scores);
    
    // Entry API 使用
    let alice_score = scores.entry("Alice").or_insert(0);
    *alice_score += 5; // Alice 加分
    
    scores.entry("Diana").or_insert(88); // 新学生
    
    // 安全访问
    match scores.get("Bob") {
        Some(score) => println!("Bob 的成绩: {}", score),
        None => println!("Bob 不在记录中"),
    }
    
    // 迭代
    println!("所有学生成绩:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }
    
    // 统计示例：单词计数
    let text = "hello world hello rust world rust";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("单词统计: {:?}", word_count);
}

/// BTreeMap 有序映射示例
fn btreemap_examples() {
    let mut btree = BTreeMap::new();
    btree.insert(3, "three");
    btree.insert(1, "one");
    btree.insert(4, "four");
    btree.insert(2, "two");
    btree.insert(5, "five");
    
    println!("BTreeMap (自动排序): {:?}", btree);
    
    // 范围查询
    println!("范围查询 [2..=4]:");
    for (k, v) in btree.range(2..=4) {
        println!("  {}: {}", k, v);
    }
    
    // 获取第一个和最后一个
    if let Some((first_key, first_value)) = btree.first_key_value() {
        println!("第一个元素: {}: {}", first_key, first_value);
    }
    
    if let Some((last_key, last_value)) = btree.last_key_value() {
        println!("最后一个元素: {}: {}", last_key, last_value);
    }
    
    // 分割操作
    let mut btree_copy = btree.clone();
    let right_part = btree_copy.split_off(&3);
    println!("分割后左半部分: {:?}", btree_copy);
    println!("分割后右半部分: {:?}", right_part);
}

/// Set 集合示例
fn set_examples() {
    // HashSet 示例
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("集合1: {:?}", set1);
    println!("集合2: {:?}", set2);
    
    // 集合操作
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("交集: {:?}", intersection);
    
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("并集: {:?}", union);
    
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("差集 (set1 - set2): {:?}", difference);
    
    // 去重示例
    let numbers_with_duplicates = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique_numbers: HashSet<_> = numbers_with_duplicates.into_iter().collect();
    println!("去重后: {:?}", unique_numbers);
    
    // BTreeSet 示例（有序）
    let mut btree_set = BTreeSet::new();
    btree_set.insert(5);
    btree_set.insert(2);
    btree_set.insert(8);
    btree_set.insert(1);
    
    println!("BTreeSet (有序): {:?}", btree_set);
    
    // 范围查询
    println!("BTreeSet 范围查询 [2..=5]:");
    for item in btree_set.range(2..=5) {
        println!("  {}", item);
    }
}

/// VecDeque 双端队列示例
fn vecdeque_examples() {
    let mut deque = VecDeque::new();
    
    // 从两端添加元素
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_front(-1);
    
    println!("双端队列: {:?}", deque);
    
    // 从两端移除元素
    if let Some(front) = deque.pop_front() {
        println!("从前端移除: {}", front);
    }
    
    if let Some(back) = deque.pop_back() {
        println!("从后端移除: {}", back);
    }
    
    println!("剩余队列: {:?}", deque);
    
    // 旋转操作
    let mut rotate_deque: VecDeque<i32> = (1..=5).collect();
    println!("原始队列: {:?}", rotate_deque);
    
    rotate_deque.rotate_left(2);
    println!("左旋转 2 位: {:?}", rotate_deque);
    
    rotate_deque.rotate_right(1);
    println!("右旋转 1 位: {:?}", rotate_deque);
}

/// BinaryHeap 优先队列示例
fn binaryheap_examples() {
    // 最大堆（默认）
    let mut max_heap = BinaryHeap::new();
    max_heap.push(3);
    max_heap.push(1);
    max_heap.push(4);
    max_heap.push(1);
    max_heap.push(5);
    
    println!("最大堆: {:?}", max_heap);
    
    println!("按优先级弹出（最大值优先）:");
    while let Some(max) = max_heap.pop() {
        println!("  弹出: {}", max);
    }
    
    // 最小堆（使用 Reverse）
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(4));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(5));
    
    println!("\n最小堆弹出（最小值优先）:");
    while let Some(Reverse(min)) = min_heap.pop() {
        println!("  弹出: {}", min);
    }
}

/// 实际应用示例
fn practical_examples() {
    // 1. 学生成绩管理系统
    println!("\n--- 学生成绩管理系统 ---");
    student_grade_system();
    
    // 2. 任务调度系统
    println!("\n--- 任务调度系统 ---");
    task_scheduler_system();
    
    // 3. 缓存系统
    println!("\n--- 简单缓存系统 ---");
    simple_cache_system();
}

/// 学生成绩管理系统
fn student_grade_system() {
    #[derive(Debug, Clone)]
    struct Student {
        _id: u32,
        name: String,
        grades: Vec<f64>,
    }
    
    impl Student {
        fn new(id: u32, name: String) -> Self {
            Student {
                _id: id,
                name,
                grades: Vec::new(),
            }
        }
        
        fn add_grade(&mut self, grade: f64) {
            self.grades.push(grade);
        }
        
        fn average_grade(&self) -> Option<f64> {
            if self.grades.is_empty() {
                None
            } else {
                Some(self.grades.iter().sum::<f64>() / self.grades.len() as f64)
            }
        }
    }
    
    struct GradeSystem {
        students: HashMap<u32, Student>,
        name_to_id: HashMap<String, u32>,
    }
    
    impl GradeSystem {
        fn new() -> Self {
            GradeSystem {
                students: HashMap::new(),
                name_to_id: HashMap::new(),
            }
        }
        
        fn add_student(&mut self, id: u32, name: String) {
            let student = Student::new(id, name.clone());
            self.name_to_id.insert(name, id);
            self.students.insert(id, student);
        }
        
        fn add_grade_by_name(&mut self, name: &str, grade: f64) -> Result<(), String> {
            if let Some(&id) = self.name_to_id.get(name) {
                if let Some(student) = self.students.get_mut(&id) {
                    student.add_grade(grade);
                    Ok(())
                } else {
                    Err("学生不存在".to_string())
                }
            } else {
                Err("学生姓名未找到".to_string())
            }
        }
        
        fn get_top_students(&self, n: usize) -> Vec<(String, f64)> {
            let mut student_averages: Vec<_> = self.students
                .values()
                .filter_map(|student| {
                    student.average_grade().map(|avg| (student.name.clone(), avg))
                })
                .collect();
            
            student_averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            student_averages.into_iter().take(n).collect()
        }
    }
    
    // 使用示例
    let mut system = GradeSystem::new();
    system.add_student(1, "Alice".to_string());
    system.add_student(2, "Bob".to_string());
    system.add_student(3, "Charlie".to_string());
    
    let _ = system.add_grade_by_name("Alice", 95.0);
    let _ = system.add_grade_by_name("Alice", 87.0);
    let _ = system.add_grade_by_name("Bob", 92.0);
    let _ = system.add_grade_by_name("Bob", 88.0);
    let _ = system.add_grade_by_name("Charlie", 78.0);
    let _ = system.add_grade_by_name("Charlie", 85.0);
    
    let top_students = system.get_top_students(3);
    println!("前三名学生:");
    for (i, (name, avg)) in top_students.iter().enumerate() {
        println!("  {}. {}: {:.2}", i + 1, name, avg);
    }
}

/// 任务调度系统
fn task_scheduler_system() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Task {
        id: u32,
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
    
    struct TaskScheduler {
        high_priority: BinaryHeap<Task>,
        normal_queue: VecDeque<Task>,
        completed: Vec<u32>,
    }
    
    impl TaskScheduler {
        fn new() -> Self {
            TaskScheduler {
                high_priority: BinaryHeap::new(),
                normal_queue: VecDeque::new(),
                completed: Vec::new(),
            }
        }
        
        fn add_task(&mut self, task: Task) {
            if task.priority >= 5 {
                self.high_priority.push(task);
            } else {
                self.normal_queue.push_back(task);
            }
        }
        
        fn get_next_task(&mut self) -> Option<Task> {
            // 优先处理高优先级任务
            if let Some(task) = self.high_priority.pop() {
                Some(task)
            } else {
                self.normal_queue.pop_front()
            }
        }
        
        fn complete_task(&mut self, task_id: u32) {
            self.completed.push(task_id);
        }
        
        fn status(&self) -> (usize, usize, usize) {
            (self.high_priority.len(), self.normal_queue.len(), self.completed.len())
        }
    }
    
    // 使用示例
    let mut scheduler = TaskScheduler::new();
    
    scheduler.add_task(Task {
        id: 1,
        priority: 3,
        description: "普通任务1".to_string(),
    });
    
    scheduler.add_task(Task {
        id: 2,
        priority: 8,
        description: "紧急任务".to_string(),
    });
    
    scheduler.add_task(Task {
        id: 3,
        priority: 2,
        description: "普通任务2".to_string(),
    });
    
    scheduler.add_task(Task {
        id: 4,
        priority: 6,
        description: "重要任务".to_string(),
    });
    
    println!("执行任务:");
    while let Some(task) = scheduler.get_next_task() {
        println!("  执行: {} (优先级: {})", task.description, task.priority);
        scheduler.complete_task(task.id);
    }
    
    let (high, normal, completed) = scheduler.status();
    println!("任务状态 - 高优先级: {}, 普通: {}, 已完成: {}", high, normal, completed);
}

/// 简单缓存系统
fn simple_cache_system() {
    struct SimpleCache<K, V> {
        data: HashMap<K, V>,
        access_order: VecDeque<K>,
        capacity: usize,
    }
    
    impl<K, V> SimpleCache<K, V>
    where
        K: std::hash::Hash + Eq + Clone,
    {
        fn new(capacity: usize) -> Self {
            SimpleCache {
                data: HashMap::with_capacity(capacity),
                access_order: VecDeque::with_capacity(capacity),
                capacity,
            }
        }
        
        fn get(&mut self, key: &K) -> Option<&V> {
            if self.data.contains_key(key) {
                // 更新访问顺序
                self.access_order.retain(|k| k != key);
                self.access_order.push_back(key.clone());
                self.data.get(key)
            } else {
                None
            }
        }
        
        fn put(&mut self, key: K, value: V) {
            if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
                // 移除最久未使用的项
                if let Some(oldest_key) = self.access_order.pop_front() {
                    self.data.remove(&oldest_key);
                }
            }
            
            // 如果键已存在，更新访问顺序
            if self.data.contains_key(&key) {
                self.access_order.retain(|k| k != &key);
            }
            
            self.data.insert(key.clone(), value);
            self.access_order.push_back(key);
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
    }
    
    // 使用示例
    let mut cache = SimpleCache::new(3);
    
    cache.put("key1", "value1");
    cache.put("key2", "value2");
    cache.put("key3", "value3");
    
    println!("缓存大小: {}", cache.len());
    
    // 访问 key1
    if let Some(value) = cache.get(&"key1") {
        println!("获取 key1: {}", value);
    }
    
    // 添加新项，应该移除最久未使用的 key2
    cache.put("key4", "value4");
    
    // 尝试获取 key2（应该不存在）
    match cache.get(&"key2") {
        Some(value) => println!("获取 key2: {}", value),
        None => println!("key2 已被移除（LRU）"),
    }
    
    println!("最终缓存大小: {}", cache.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_operations() {
        let mut vec = vec![1, 2, 3];
        vec.push(4);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[3], 4);
        
        let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8]);
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", 10);
        map.insert("key2", 20);
        
        assert_eq!(map.get("key1"), Some(&10));
        assert_eq!(map.get("key3"), None);
        
        let count = map.entry("key3").or_insert(0);
        *count += 5;
        assert_eq!(map.get("key3"), Some(&5));
    }
    
    #[test]
    fn test_btreemap_ordering() {
        let mut btree = BTreeMap::new();
        btree.insert(3, "three");
        btree.insert(1, "one");
        btree.insert(2, "two");
        
        let keys: Vec<_> = btree.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3]);
    }
    
    #[test]
    fn test_set_operations() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
        
        let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&2));
        assert!(intersection.contains(&3));
    }
    
    #[test]
    fn test_vecdeque_operations() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_front(0);
        deque.push_back(2);
        
        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.len(), 1);
    }
    
    #[test]
    fn test_binary_heap_operations() {
        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(2);
        
        assert_eq!(heap.pop(), Some(4)); // 最大值
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.len(), 2);
    }
}
