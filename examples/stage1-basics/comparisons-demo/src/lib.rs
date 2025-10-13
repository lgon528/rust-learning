//! # Comparisons Demo - Rust 比较操作演示
//!
//! 本库演示了 Rust 中各种比较操作的使用方法，包括基本比较、自定义比较、
//! 排序算法以及比较相关的 trait 实现。
//!
//! ## 主要功能
//!
//! - 基本数据类型的比较操作
//! - 自定义类型的比较实现
//! - 排序和搜索算法
//! - 比较相关的 trait（Eq, Ord, PartialEq, PartialOrd）
//!
//! ## 使用示例
//!
//! ```
//! use comparisons_demo::basic::*;
//! use comparisons_demo::custom::*;
//!
//! // 基本比较
//! assert!(compare_numbers(5, 3) == std::cmp::Ordering::Greater);
//!
//! // 自定义类型比较
//! let person1 = Person::new("Alice", 30);
//! let person2 = Person::new("Bob", 25);
//! assert!(person1 > person2); // 按年龄比较
//! ```

/// 基本比较操作模块
pub mod basic {
    use std::cmp::Ordering;

    /// 比较两个数字并返回排序结果
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::basic::compare_numbers;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(compare_numbers(5, 3), Ordering::Greater);
    /// assert_eq!(compare_numbers(2, 7), Ordering::Less);
    /// assert_eq!(compare_numbers(4, 4), Ordering::Equal);
    /// ```
    pub fn compare_numbers(a: i32, b: i32) -> Ordering {
        a.cmp(&b)
    }

    /// 比较两个字符串（忽略大小写）
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::basic::compare_strings_ignore_case;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(compare_strings_ignore_case("Hello", "hello"), Ordering::Equal);
    /// assert_eq!(compare_strings_ignore_case("apple", "Banana"), Ordering::Less);
    /// ```
    pub fn compare_strings_ignore_case(a: &str, b: &str) -> Ordering {
        a.to_lowercase().cmp(&b.to_lowercase())
    }

    /// 查找数组中的最大值
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::basic::find_max;
    ///
    /// let numbers = vec![3, 7, 2, 9, 1];
    /// assert_eq!(find_max(&numbers), Some(&9));
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(find_max(&empty), None);
    /// ```
    pub fn find_max<T: Ord>(slice: &[T]) -> Option<&T> {
        slice.iter().max()
    }

    /// 查找数组中的最小值
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::basic::find_min;
    ///
    /// let numbers = vec![3, 7, 2, 9, 1];
    /// assert_eq!(find_min(&numbers), Some(&1));
    /// ```
    pub fn find_min<T: Ord>(slice: &[T]) -> Option<&T> {
        slice.iter().min()
    }

    /// 检查数组是否已排序
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::basic::is_sorted;
    ///
    /// assert!(is_sorted(&[1, 2, 3, 4, 5]));
    /// assert!(!is_sorted(&[1, 3, 2, 4, 5]));
    /// assert!(is_sorted(&[5])); // 单个元素总是排序的
    /// assert!(is_sorted(&[] as &[i32])); // 空数组总是排序的
    /// ```
    pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }
}

/// 自定义类型比较模块
pub mod custom {
    use std::cmp::Ordering;

    /// 表示一个人的结构体
    #[derive(Debug, Clone)]
    pub struct Person {
        pub name: String,
        pub age: u32,
    }

    impl Person {
        /// 创建新的 Person 实例
        ///
        /// # 示例
        ///
        /// ```
        /// use comparisons_demo::custom::Person;
        ///
        /// let person = Person::new("Alice", 30);
        /// assert_eq!(person.name, "Alice");
        /// assert_eq!(person.age, 30);
        /// ```
        pub fn new(name: &str, age: u32) -> Self {
            Person {
                name: name.to_string(),
                age,
            }
        }
    }

    // 实现 PartialEq，按年龄比较
    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age
        }
    }

    // 实现 Eq
    impl Eq for Person {}

    // 实现 PartialOrd，按年龄比较
    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // 实现 Ord，按年龄比较
    impl Ord for Person {
        fn cmp(&self, other: &Self) -> Ordering {
            self.age.cmp(&other.age)
        }
    }

    /// 表示一个产品的结构体
    #[derive(Debug, Clone, PartialEq)]
    pub struct Product {
        pub name: String,
        pub price: f64,
        pub rating: f32,
    }

    impl Product {
        /// 创建新的 Product 实例
        ///
        /// # 示例
        ///
        /// ```
        /// use comparisons_demo::custom::Product;
        ///
        /// let product = Product::new("Laptop", 999.99, 4.5);
        /// assert_eq!(product.name, "Laptop");
        /// assert!((product.price - 999.99).abs() < f64::EPSILON);
        /// ```
        pub fn new(name: &str, price: f64, rating: f32) -> Self {
            Product {
                name: name.to_string(),
                price,
                rating,
            }
        }
    }

    // 实现 PartialOrd，按评分比较（注意浮点数比较）
    impl PartialOrd for Product {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.rating.partial_cmp(&other.rating)
        }
    }

    /// 按价格比较产品
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::custom::{Product, compare_by_price};
    /// use std::cmp::Ordering;
    ///
    /// let product1 = Product::new("Laptop", 999.99, 4.5);
    /// let product2 = Product::new("Mouse", 29.99, 4.2);
    /// assert_eq!(compare_by_price(&product1, &product2), Ordering::Greater);
    /// ```
    pub fn compare_by_price(a: &Product, b: &Product) -> Ordering {
        a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal)
    }

    /// 按名称比较产品
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::custom::{Product, compare_by_name};
    /// use std::cmp::Ordering;
    ///
    /// let product1 = Product::new("Laptop", 999.99, 4.5);
    /// let product2 = Product::new("Mouse", 29.99, 4.2);
    /// assert_eq!(compare_by_name(&product1, &product2), Ordering::Less);
    /// ```
    pub fn compare_by_name(a: &Product, b: &Product) -> Ordering {
        a.name.cmp(&b.name)
    }
}

/// 排序和搜索算法模块
pub mod algorithms {
    use std::cmp::Ordering;

    /// 冒泡排序实现
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::algorithms::bubble_sort;
    ///
    /// let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    /// bubble_sort(&mut numbers);
    /// assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);
    /// ```
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    /// 选择排序实现
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::algorithms::selection_sort;
    ///
    /// let mut numbers = vec![64, 25, 12, 22, 11];
    /// selection_sort(&mut numbers);
    /// assert_eq!(numbers, vec![11, 12, 22, 25, 64]);
    /// ```
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            if min_idx != i {
                arr.swap(i, min_idx);
            }
        }
    }

    /// 二分搜索实现
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::algorithms::binary_search;
    ///
    /// let numbers = vec![1, 3, 5, 7, 9, 11, 13];
    /// assert_eq!(binary_search(&numbers, &7), Some(3));
    /// assert_eq!(binary_search(&numbers, &4), None);
    /// ```
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].cmp(target) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }
        None
    }

    /// 使用自定义比较函数排序
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::algorithms::sort_by;
    ///
    /// let mut words = vec!["apple", "pie", "a", "longer"];
    /// sort_by(&mut words, |a, b| a.len().cmp(&b.len()));
    /// assert_eq!(words, vec!["a", "pie", "apple", "longer"]);
    /// ```
    pub fn sort_by<T, F>(arr: &mut [T], mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        arr.sort_by(&mut compare);
    }

    /// 查找第 k 小的元素（快速选择算法）
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::algorithms::find_kth_smallest;
    ///
    /// let mut numbers = vec![3, 6, 8, 10, 1, 2, 1];
    /// assert_eq!(find_kth_smallest(&mut numbers, 3), Some(2));
    /// ```
    pub fn find_kth_smallest<T: Ord + Clone>(arr: &mut [T], k: usize) -> Option<T> {
        if k == 0 || k > arr.len() {
            return None;
        }
        
        arr.sort();
        Some(arr[k - 1].clone())
    }
}

/// 高级比较操作模块
pub mod advanced {
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;

    /// 多字段比较结构体
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Student {
        pub name: String,
        pub grade: u32,
        pub age: u32,
    }

    impl Student {
        /// 创建新的 Student 实例
        ///
        /// # 示例
        ///
        /// ```
        /// use comparisons_demo::advanced::Student;
        ///
        /// let student = Student::new("Alice", 95, 20);
        /// assert_eq!(student.name, "Alice");
        /// assert_eq!(student.grade, 95);
        /// ```
        pub fn new(name: &str, grade: u32, age: u32) -> Self {
            Student {
                name: name.to_string(),
                grade,
                age,
            }
        }
    }

    // 实现多字段比较：先按成绩，再按年龄
    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Student {
        fn cmp(&self, other: &Self) -> Ordering {
            // 先按成绩升序，再按年龄升序
            match self.grade.cmp(&other.grade) {
                Ordering::Equal => self.age.cmp(&other.age),
                other_order => other_order,
            }
        }
    }

    /// 使用优先队列找到前 N 个最大值
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::advanced::top_n_largest;
    ///
    /// let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    /// let top_3 = top_n_largest(&numbers, 3);
    /// assert_eq!(top_3, vec![9, 6, 5]);
    /// ```
    pub fn top_n_largest<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
        let mut heap = BinaryHeap::new();
        for item in items {
            heap.push(item.clone());
        }
        
        let mut result = Vec::new();
        for _ in 0..n.min(heap.len()) {
            if let Some(item) = heap.pop() {
                result.push(item);
            }
        }
        result
    }

    /// 使用优先队列找到前 N 个最小值
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::advanced::top_n_smallest;
    ///
    /// let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    /// let top_3 = top_n_smallest(&numbers, 3);
    /// assert_eq!(top_3, vec![1, 1, 2]);
    /// ```
    pub fn top_n_smallest<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
        let mut heap: BinaryHeap<Reverse<T>> = BinaryHeap::new();
        for item in items {
            heap.push(Reverse(item.clone()));
        }
        
        let mut result = Vec::new();
        for _ in 0..n.min(heap.len()) {
            if let Some(Reverse(item)) = heap.pop() {
                result.push(item);
            }
        }
        result
    }

    /// 比较两个向量是否在排序后相等
    ///
    /// # 示例
    ///
    /// ```
    /// use comparisons_demo::advanced::are_anagrams;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![3, 1, 2];
    /// assert!(are_anagrams(&vec1, &vec2));
    ///
    /// let vec3 = vec![1, 2, 3];
    /// let vec4 = vec![1, 2, 4];
    /// assert!(!are_anagrams(&vec3, &vec4));
    /// ```
    pub fn are_anagrams<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut a_sorted = a.to_vec();
        let mut b_sorted = b.to_vec();
        a_sorted.sort();
        b_sorted.sort();
        
        a_sorted == b_sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_basic_comparisons() {
        assert_eq!(basic::compare_numbers(5, 3), Ordering::Greater);
        assert_eq!(basic::compare_numbers(2, 7), Ordering::Less);
        assert_eq!(basic::compare_numbers(4, 4), Ordering::Equal);
    }

    #[test]
    fn test_string_comparisons() {
        assert_eq!(
            basic::compare_strings_ignore_case("Hello", "hello"),
            Ordering::Equal
        );
        assert_eq!(
            basic::compare_strings_ignore_case("apple", "Banana"),
            Ordering::Less
        );
    }

    #[test]
    fn test_find_extremes() {
        let numbers = vec![3, 7, 2, 9, 1];
        assert_eq!(basic::find_max(&numbers), Some(&9));
        assert_eq!(basic::find_min(&numbers), Some(&1));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(basic::find_max(&empty), None);
        assert_eq!(basic::find_min(&empty), None);
    }

    #[test]
    fn test_is_sorted() {
        assert!(basic::is_sorted(&[1, 2, 3, 4, 5]));
        assert!(!basic::is_sorted(&[1, 3, 2, 4, 5]));
        assert!(basic::is_sorted(&[5]));
        assert!(basic::is_sorted(&[] as &[i32]));
    }

    #[test]
    fn test_custom_person_comparison() {
        let person1 = custom::Person::new("Alice", 30);
        let person2 = custom::Person::new("Bob", 25);
        let person3 = custom::Person::new("Charlie", 30);
        
        assert!(person1 > person2);
        assert!(person1 == person3);
        assert!(person2 < person1);
    }

    #[test]
    fn test_product_comparison() {
        let product1 = custom::Product::new("Laptop", 999.99, 4.5);
        let product2 = custom::Product::new("Mouse", 29.99, 4.2);
        
        assert_eq!(custom::compare_by_price(&product1, &product2), Ordering::Greater);
        assert_eq!(custom::compare_by_name(&product1, &product2), Ordering::Less);
        assert!(product1 > product2); // 按评分比较
    }

    #[test]
    fn test_sorting_algorithms() {
        let numbers = vec![64, 34, 25, 12, 22, 11, 90];
        let expected = vec![11, 12, 22, 25, 34, 64, 90];
        
        let mut bubble_test = numbers.clone();
        algorithms::bubble_sort(&mut bubble_test);
        assert_eq!(bubble_test, expected);
        
        let mut selection_test = numbers.clone();
        algorithms::selection_sort(&mut selection_test);
        assert_eq!(selection_test, expected);
    }

    #[test]
    fn test_binary_search() {
        let numbers = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(algorithms::binary_search(&numbers, &7), Some(3));
        assert_eq!(algorithms::binary_search(&numbers, &4), None);
        assert_eq!(algorithms::binary_search(&numbers, &1), Some(0));
        assert_eq!(algorithms::binary_search(&numbers, &13), Some(6));
    }

    #[test]
    fn test_student_comparison() {
        let student1 = advanced::Student::new("Alice", 95, 20);
        let student2 = advanced::Student::new("Bob", 90, 19);
        let student3 = advanced::Student::new("Charlie", 95, 21);
        
        assert!(student1 > student2); // 更高的成绩
        assert!(student1 < student3); // 相同成绩，更年轻
    }

    #[test]
    fn test_top_n_functions() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        
        let top_3_largest = advanced::top_n_largest(&numbers, 3);
        assert_eq!(top_3_largest, vec![9, 6, 5]);
        
        let top_3_smallest = advanced::top_n_smallest(&numbers, 3);
        assert_eq!(top_3_smallest, vec![1, 1, 2]);
    }

    #[test]
    fn test_anagrams() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![3, 1, 2];
        assert!(advanced::are_anagrams(&vec1, &vec2));
        
        let vec3 = vec![1, 2, 3];
        let vec4 = vec![1, 2, 4];
        assert!(!advanced::are_anagrams(&vec3, &vec4));
    }
}
