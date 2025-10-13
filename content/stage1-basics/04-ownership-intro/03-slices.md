# 4.3 切片类型

切片（Slices）是Rust中一种特殊的引用类型，它允许你引用集合中连续的元素序列，而不需要拥有整个集合。切片提供了一种安全、高效的方式来处理数组、向量、字符串等数据结构的部分内容。理解切片对于编写高效的Rust代码至关重要。

## 🎯 学习目标

- 理解切片的概念和用途
- 掌握字符串切片（&str）的使用
- 学会创建和使用数组切片
- 理解切片的内存表示
- 掌握切片的索引和范围语法
- 学会在函数中使用切片参数
- 了解切片的安全性保证
- 掌握切片的常用方法和操作

## 🔍 什么是切片？

切片是对连续内存区域的引用，它包含指向数据的指针和长度信息。切片不拥有数据，只是借用数据的一部分。

### 切片的基本概念

```rust
fn main() {
    println!("=== 切片基础概念 ===");
    
    // 数组切片
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 创建切片：引用数组的一部分
    let slice = &array[2..7];  // 包含索引2到6的元素
    
    println!("原始数组: {:?}", array);
    println!("切片 [2..7]: {:?}", slice);
    println!("切片长度: {}", slice.len());
    
    // 向量切片
    let vector = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let vec_slice = &vector[1..4];
    
    println!("\n向量: {:?}", vector);
    println!("向量切片 [1..4]: {:?}", vec_slice);
    
    // 字符串切片
    let string = String::from("Hello, Rust Programming!");
    let str_slice = &string[0..5];
    
    println!("\n字符串: {}", string);
    println!("字符串切片 [0..5]: {}", str_slice);
    
    // 演示切片的引用特性
    demonstrate_slice_reference_nature();
    
    // 演示切片的内存表示
    demonstrate_slice_memory_layout();
}

fn demonstrate_slice_reference_nature() {
    println!("\n=== 切片的引用特性 ===");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    // 切片不拥有数据
    let slice1 = &data[0..3];
    let slice2 = &data[3..6];
    let slice3 = &data[6..];
    
    println!("原始数据: {:?}", data);
    println!("切片1 [0..3]: {:?}", slice1);
    println!("切片2 [3..6]: {:?}", slice2);
    println!("切片3 [6..]: {:?}", slice3);
    
    // 多个切片可以同时存在（都是不可变引用）
    println!("所有切片同时有效");
    
    // 原始数据仍然可用
    println!("原始数据仍可访问: {:?}", data);
    
    // 切片可以重叠
    let overlapping1 = &data[2..5];
    let overlapping2 = &data[4..7];
    
    println!("重叠切片1 [2..5]: {:?}", overlapping1);
    println!("重叠切片2 [4..7]: {:?}", overlapping2);
}

fn demonstrate_slice_memory_layout() {
    println!("\n=== 切片的内存布局 ===");
    
    let array = [10, 20, 30, 40, 50];
    let slice = &array[1..4];
    
    println!("数组: {:?}", array);
    println!("切片: {:?}", slice);
    
    // 切片包含两个部分：指针和长度
    println!("数组地址: {:p}", &array);
    println!("切片指向的地址: {:p}", slice.as_ptr());
    println!("切片长度: {}", slice.len());
    
    // 切片的大小是固定的（指针 + 长度）
    println!("切片类型大小: {} 字节", std::mem::size_of::<&[i32]>());
    println!("数组类型大小: {} 字节", std::mem::size_of::<[i32; 5]>());
    
    // 演示不同长度切片的类型
    let slice1: &[i32] = &array[0..2];
    let slice2: &[i32] = &array[0..5];
    
    println!("不同长度的切片具有相同类型: &[i32]");
    println!("slice1长度: {}, slice2长度: {}", slice1.len(), slice2.len());
}
```

## 📝 字符串切片

字符串切片（&str）是最常用的切片类型，它表示对字符串数据的借用。

### 字符串切片的创建和使用

```rust
fn main() {
    println!("=== 字符串切片演示 ===");
    
    // 字符串字面量就是切片
    let literal: &str = "Hello, World!";
    println!("字符串字面量: {}", literal);
    
    // 从String创建切片
    let owned_string = String::from("Rust Programming Language");
    let full_slice = &owned_string[..];
    let partial_slice = &owned_string[0..4];
    let end_slice = &owned_string[5..];
    
    println!("\n拥有的字符串: {}", owned_string);
    println!("完整切片: {}", full_slice);
    println!("部分切片 [0..4]: {}", partial_slice);
    println!("结尾切片 [5..]: {}", end_slice);
    
    // 字符串切片的范围语法
    demonstrate_string_slice_ranges(&owned_string);
    
    // 字符串切片的UTF-8安全性
    demonstrate_utf8_safety();
    
    // 字符串切片的常用操作
    demonstrate_string_slice_operations();
}

fn demonstrate_string_slice_ranges(s: &String) {
    println!("\n=== 字符串切片范围语法 ===");
    
    let len = s.len();
    println!("字符串: {} (长度: {})", s, len);
    
    // 各种范围语法
    let full = &s[..];           // 完整切片
    let from_start = &s[..10];   // 从开始到索引10
    let to_end = &s[5..];        // 从索引5到结尾
    let middle = &s[5..15];      // 中间部分
    
    println!("完整切片 [..]: {}", full);
    println!("从开始 [..10]: {}", from_start);
    println!("到结尾 [5..]: {}", to_end);
    println!("中间部分 [5..15]: {}", middle);
    
    // 使用变量作为索引
    let start = 0;
    let end = 4;
    let variable_range = &s[start..end];
    println!("变量范围 [{}..{}]: {}", start, end, variable_range);
    
    // 单词边界切片
    demonstrate_word_boundary_slicing(s);
}

fn demonstrate_word_boundary_slicing(s: &str) {
    println!("\n--- 单词边界切片 ---");
    
    // 安全的单词提取
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("分割的单词: {:?}", words);
    
    // 提取第一个单词
    if let Some(first_word) = words.first() {
        println!("第一个单词: {}", first_word);
    }
    
    // 提取最后一个单词
    if let Some(last_word) = words.last() {
        println!("最后一个单词: {}", last_word);
    }
    
    // 自定义单词提取函数
    let first = get_first_word(s);
    println!("自定义提取的第一个单词: {}", first);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn demonstrate_utf8_safety() {
    println!("\n=== UTF-8安全性 ===");
    
    let chinese = "你好，世界！Rust编程";
    println!("中文字符串: {}", chinese);
    println!("字节长度: {}", chinese.len());
    println!("字符数量: {}", chinese.chars().count());
    
    // ❌ 危险：可能在字符中间切割
    // let invalid_slice = &chinese[0..2];  // 可能panic
    
    // ✅ 安全：使用字符边界
    let char_indices: Vec<(usize, char)> = chinese.char_indices().collect();
    println!("字符索引: {:?}", &char_indices[0..5]);
    
    // 安全的字符串切片
    if let Some((pos, _)) = chinese.char_indices().nth(3) {
        let safe_slice = &chinese[0..pos];
        println!("安全切片（前3个字符）: {}", safe_slice);
    }
    
    // 使用字符迭代器
    let first_three_chars: String = chinese.chars().take(3).collect();
    println!("前3个字符: {}", first_three_chars);
    
    // 演示多字节字符
    demonstrate_multibyte_characters();
}

fn demonstrate_multibyte_characters() {
    println!("\n--- 多字节字符处理 ---");
    
    let emoji_string = "🦀 Rust 🚀 Programming 💻";
    println!("表情符号字符串: {}", emoji_string);
    println!("字节长度: {}", emoji_string.len());
    println!("字符数量: {}", emoji_string.chars().count());
    
    // 遍历字符
    for (i, ch) in emoji_string.chars().enumerate() {
        println!("字符 {}: {} (Unicode: U+{:04X})", i, ch, ch as u32);
    }
    
    // 安全的字符串操作
    let chars: Vec<char> = emoji_string.chars().collect();
    if chars.len() >= 3 {
        let substring: String = chars[0..3].iter().collect();
        println!("前3个字符组成的字符串: {}", substring);
    }
}

fn demonstrate_string_slice_operations() {
    println!("\n=== 字符串切片操作 ===");
    
    let text = "  Hello, Rust Programming!  ";
    println!("原始文本: '{}'", text);
    
    // 常用字符串切片方法
    println!("长度: {}", text.len());
    println!("是否为空: {}", text.is_empty());
    println!("去除空白: '{}'", text.trim());
    println!("转小写: {}", text.to_lowercase());
    println!("转大写: {}", text.to_uppercase());
    
    // 字符串搜索
    println!("\n--- 字符串搜索 ---");
    println!("包含'Rust': {}", text.contains("Rust"));
    println!("以'Hello'开始: {}", text.trim().starts_with("Hello"));
    println!("以'!'结束: {}", text.trim().ends_with("!"));
    
    if let Some(pos) = text.find("Rust") {
        println!("'Rust'的位置: {}", pos);
        let before_rust = &text[..pos];
        let after_rust = &text[pos + 4..];
        println!("'Rust'之前: '{}'", before_rust.trim());
        println!("'Rust'之后: '{}'", after_rust.trim());
    }
    
    // 字符串分割
    println!("\n--- 字符串分割 ---");
    let sentence = "apple,banana,cherry,date";
    let fruits: Vec<&str> = sentence.split(',').collect();
    println!("水果列表: {:?}", fruits);
    
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("单词列表: {:?}", words);
    
    // 字符串替换（返回新的String）
    let replaced = text.replace("Rust", "🦀");
    println!("替换后: {}", replaced);
}
```

## 📊 数组和向量切片

数组和向量切片允许你处理数值数据的子集，这在数据处理和算法实现中非常有用。

### 数组切片的创建和操作

```rust
fn main() {
    println!("=== 数组和向量切片 ===");
    
    // 数组切片
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 不同的切片创建方式
    let first_half = &numbers[..5];
    let second_half = &numbers[5..];
    let middle = &numbers[2..8];
    let every_other = &numbers[1..9];
    
    println!("原始数组: {:?}", numbers);
    println!("前半部分: {:?}", first_half);
    println!("后半部分: {:?}", second_half);
    println!("中间部分: {:?}", middle);
    
    // 向量切片
    let mut vector = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    
    demonstrate_vector_slices(&vector);
    demonstrate_mutable_slices(&mut vector);
    demonstrate_slice_operations();
    demonstrate_slice_patterns();
}

fn demonstrate_vector_slices(vec: &Vec<i32>) {
    println!("\n=== 向量切片演示 ===");
    
    println!("向量: {:?}", vec);
    
    // 各种切片操作
    let slice1 = &vec[0..3];
    let slice2 = &vec[vec.len()-3..];
    let slice3 = &vec[2..vec.len()-2];
    
    println!("前3个元素: {:?}", slice1);
    println!("后3个元素: {:?}", slice2);
    println!("去掉首尾2个元素: {:?}", slice3);
    
    // 使用get方法安全访问
    match vec.get(0..3) {
        Some(slice) => println!("安全获取前3个: {:?}", slice),
        None => println!("索引超出范围"),
    }
    
    // 条件切片
    let mid = vec.len() / 2;
    let left_half = &vec[..mid];
    let right_half = &vec[mid..];
    
    println!("左半部分: {:?}", left_half);
    println!("右半部分: {:?}", right_half);
}

fn demonstrate_mutable_slices(vec: &mut Vec<i32>) {
    println!("\n=== 可变切片演示 ===");
    
    println!("修改前: {:?}", vec);
    
    // 创建可变切片
    let mutable_slice = &mut vec[2..7];
    
    // 修改切片中的元素
    for item in mutable_slice.iter_mut() {
        *item *= 2;
    }
    
    println!("修改中间5个元素后: {:?}", vec);
    
    // 使用索引修改
    if let Some(slice) = vec.get_mut(0..3) {
        slice[0] = 999;
        slice[1] = 888;
        slice[2] = 777;
    }
    
    println!("修改前3个元素后: {:?}", vec);
    
    // 切片排序
    let sort_slice = &mut vec[3..8];
    sort_slice.sort();
    
    println!("部分排序后: {:?}", vec);
    
    // 切片反转
    let reverse_slice = &mut vec[1..6];
    reverse_slice.reverse();
    
    println!("部分反转后: {:?}", vec);
}

fn demonstrate_slice_operations() {
    println!("\n=== 切片操作演示 ===");
    
    let data = vec![1, 5, 3, 9, 2, 8, 4, 7, 6, 10];
    let slice = &data[2..8];
    
    println!("数据: {:?}", data);
    println!("切片: {:?}", slice);
    
    // 切片统计操作
    println!("切片长度: {}", slice.len());
    println!("是否为空: {}", slice.is_empty());
    println!("第一个元素: {:?}", slice.first());
    println!("最后一个元素: {:?}", slice.last());
    
    // 切片搜索
    println!("\n--- 切片搜索 ---");
    println!("包含5: {}", slice.contains(&5));
    println!("包含100: {}", slice.contains(&100));
    
    if let Some(pos) = slice.iter().position(|&x| x == 9) {
        println!("元素9的位置: {}", pos);
    }
    
    // 切片迭代
    println!("\n--- 切片迭代 ---");
    print!("正向迭代: ");
    for &item in slice {
        print!("{} ", item);
    }
    println!();
    
    print!("反向迭代: ");
    for &item in slice.iter().rev() {
        print!("{} ", item);
    }
    println!();
    
    // 切片转换
    println!("\n--- 切片转换 ---");
    let doubled: Vec<i32> = slice.iter().map(|&x| x * 2).collect();
    println!("加倍: {:?}", doubled);
    
    let filtered: Vec<&i32> = slice.iter().filter(|&&x| x > 5).collect();
    println!("大于5的元素: {:?}", filtered);
    
    let sum: i32 = slice.iter().sum();
    println!("总和: {}", sum);
    
    let max = slice.iter().max();
    println!("最大值: {:?}", max);
}

fn demonstrate_slice_patterns() {
    println!("\n=== 切片模式匹配 ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用切片进行模式匹配
    match numbers.as_slice() {
        [] => println!("空切片"),
        [x] => println!("单元素切片: {}", x),
        [x, y] => println!("双元素切片: {}, {}", x, y),
        [first, .., last] => println!("多元素切片，首: {}, 尾: {}", first, last),
    }
    
    // 处理不同长度的切片
    process_slice(&[]);
    process_slice(&[42]);
    process_slice(&[1, 2]);
    process_slice(&[1, 2, 3, 4, 5]);
    
    // 切片分组处理
    let large_data = (1..=20).collect::<Vec<i32>>();
    process_in_chunks(&large_data, 3);
}

fn process_slice(slice: &[i32]) {
    match slice {
        [] => println!("处理空切片"),
        [single] => println!("处理单元素: {}", single),
        [first, second] => println!("处理双元素: {} 和 {}", first, second),
        [first, middle @ .., last] => {
            println!("处理多元素: 首={}, 中间有{}个, 尾={}", 
                    first, middle.len(), last);
        }
    }
}

fn process_in_chunks(data: &[i32], chunk_size: usize) {
    println!("\n--- 分块处理 ---");
    println!("数据: {:?}", data);
    println!("块大小: {}", chunk_size);
    
    for (i, chunk) in data.chunks(chunk_size).enumerate() {
        println!("块 {}: {:?}", i, chunk);
    }
    
    // 精确分块（最后一块可能不足）
    println!("\n--- 精确分块 ---");
    for (i, chunk) in data.chunks_exact(chunk_size).enumerate() {
        println!("精确块 {}: {:?}", i, chunk);
    }
    
    // 剩余部分
    let remainder = data.chunks_exact(chunk_size).remainder();
    if !remainder.is_empty() {
        println!("剩余部分: {:?}", remainder);
    }
}
```

## 🔧 切片在函数中的使用

切片作为函数参数提供了灵活性和效率，允许函数接受不同类型的集合。

### 函数参数中的切片

```rust
fn main() {
    println!("=== 函数中的切片使用 ===");
    
    // 准备测试数据
    let array = [1, 2, 3, 4, 5];
    let vector = vec![10, 20, 30, 40, 50];
    let slice = &vector[1..4];
    
    // 同一个函数可以处理不同类型的数据
    println!("数组求和: {}", sum_slice(&array));
    println!("向量求和: {}", sum_slice(&vector));
    println!("切片求和: {}", sum_slice(slice));
    
    // 字符串处理函数
    let owned_string = String::from("Hello, World!");
    let string_literal = "Rust Programming";
    
    println!("\n字符串长度统计:");
    println!("拥有字符串: {}", count_chars(&owned_string));
    println!("字符串字面量: {}", count_chars(string_literal));
    println!("字符串切片: {}", count_chars(&owned_string[0..5]));
    
    // 演示不同的切片函数模式
    demonstrate_slice_function_patterns();
    
    // 演示切片的借用检查
    demonstrate_slice_borrowing();
}

// 通用的切片求和函数
fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// 字符串切片字符计数
fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn demonstrate_slice_function_patterns() {
    println!("\n=== 切片函数模式 ===");
    
    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];
    
    // 只读操作
    println!("最大值: {:?}", find_max(&numbers));
    println!("平均值: {:.2}", calculate_average(&numbers));
    println!("大于5的数量: {}", count_greater_than(&numbers, 5));
    
    // 返回切片的函数
    if let Some(subslice) = find_subslice(&numbers, &[3, 9, 2]) {
        println!("找到子切片: {:?}", subslice);
    }
    
    // 可变操作
    let mut mutable_numbers = numbers.clone();
    double_elements(&mut mutable_numbers[2..7]);
    println!("部分加倍后: {:?}", mutable_numbers);
    
    // 条件处理
    let result = process_if_valid(&numbers);
    println!("处理结果: {:?}", result);
    
    // 多切片操作
    let other_numbers = vec![2, 4, 6, 8, 10];
    let merged = merge_sorted_slices(&numbers[..3], &other_numbers[..3]);
    println!("合并结果: {:?}", merged);
}

// 查找最大值
fn find_max(slice: &[i32]) -> Option<i32> {
    slice.iter().max().copied()
}

// 计算平均值
fn calculate_average(slice: &[i32]) -> f64 {
    if slice.is_empty() {
        0.0
    } else {
        slice.iter().sum::<i32>() as f64 / slice.len() as f64
    }
}

// 计数大于指定值的元素
fn count_greater_than(slice: &[i32], threshold: i32) -> usize {
    slice.iter().filter(|&&x| x > threshold).count()
}

// 查找子切片
fn find_subslice<'a>(haystack: &'a [i32], needle: &[i32]) -> Option<&'a [i32]> {
    haystack.windows(needle.len())
        .find(|window| *window == needle)
}

// 可变切片操作
fn double_elements(slice: &mut [i32]) {
    for element in slice {
        *element *= 2;
    }
}

// 条件处理
fn process_if_valid(slice: &[i32]) -> Option<Vec<i32>> {
    if slice.len() >= 3 {
        Some(slice.iter().map(|&x| x * x).collect())
    } else {
        None
    }
}

// 合并两个已排序的切片
fn merge_sorted_slices(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_current = left_iter.next();
    let mut right_current = right_iter.next();
    
    loop {
        match (left_current, right_current) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push(*l);
                    left_current = left_iter.next();
                } else {
                    result.push(*r);
                    right_current = right_iter.next();
                }
            }
            (Some(l), None) => {
                result.push(*l);
                result.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                result.push(*r);
                result.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }
    
    result
}

fn demonstrate_slice_borrowing() {
    println!("\n=== 切片借用演示 ===");
    
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 多个不可变切片可以同时存在
    {
        let slice1 = &data[0..3];
        let slice2 = &data[3..6];
        let slice3 = &data[6..];
        
        println!("同时存在的不可变切片:");
        println!("切片1: {:?}", slice1);
        println!("切片2: {:?}", slice2);
        println!("切片3: {:?}", slice3);
    }
    
    // 可变切片是独占的
    {
        let mutable_slice = &mut data[2..8];
        
        // 在可变切片存在时，不能创建其他引用
        // let another_slice = &data[0..2];  // 编译错误
        
        // 修改可变切片
        for item in mutable_slice {
            *item += 10;
        }
        
        println!("通过可变切片修改: {:?}", mutable_slice);
    }
    
    println!("修改后的完整数据: {:?}", data);
    
    // 演示切片生命周期
    demonstrate_slice_lifetimes(&data);
}

fn demonstrate_slice_lifetimes(data: &[i32]) {
    println!("\n--- 切片生命周期 ---");
    
    // 切片的生命周期不能超过原始数据
    let slice_ref;
    {
        let temp_data = vec![100, 200, 300];
        // slice_ref = &temp_data[..];  // 编译错误：悬垂引用
    }
    
    // 正确的做法：确保数据生命周期足够长
    slice_ref = &data[0..3];
    println!("有效的切片引用: {:?}", slice_ref);
    
    // 函数返回切片
    let first_half = get_first_half(data);
    println!("前半部分: {:?}", first_half);
    
    // 切片作为结构体字段
    let analyzer = SliceAnalyzer::new(data);
    analyzer.print_analysis();
}

// 返回切片的函数
fn get_first_half(slice: &[i32]) -> &[i32] {
    let mid = slice.len() / 2;
    &slice[..mid]
}

// 包含切片引用的结构体
struct SliceAnalyzer<'a> {
    data: &'a [i32],
}

impl<'a> SliceAnalyzer<'a> {
    fn new(data: &'a [i32]) -> Self {
        SliceAnalyzer { data }
    }
    
    fn print_analysis(&self) {
        println!("切片分析:");
        println!("  长度: {}", self.data.len());
        println!("  总和: {}", self.data.iter().sum::<i32>());
        println!("  平均值: {:.2}", 
                self.data.iter().sum::<i32>() as f64 / self.data.len() as f64);
        if let (Some(min), Some(max)) = (self.data.iter().min(), self.data.iter().max()) {
            println!("  范围: {} - {}", min, max);
        }
    }
}
```

## ⚠️ 切片的安全性和常见错误

### 索引越界和安全访问

```rust
fn main() {
    println!("=== 切片安全性演示 ===");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // ❌ 危险：可能panic的操作
    demonstrate_unsafe_operations(&data);
    
    // ✅ 安全：推荐的操作方式
    demonstrate_safe_operations(&data);
    
    // 字符串切片的UTF-8安全性
    demonstrate_string_safety();
    
    // 切片边界检查
    demonstrate_bounds_checking();
}

fn demonstrate_unsafe_operations(data: &[i32]) {
    println!("\n=== 可能不安全的操作 ===");
    
    println!("数据: {:?}", data);
    
    // 这些操作在索引有效时是安全的，但在运行时可能panic
    println!("第一个元素: {}", data[0]);  // 如果data为空会panic
    println!("最后一个元素: {}", data[data.len() - 1]);  // 如果data为空会panic
    
    // 切片操作也可能panic
    let slice = &data[1..4];  // 如果data长度小于4会panic
    println!("切片 [1..4]: {:?}", slice);
    
    // ❌ 这些会在运行时panic（已注释避免程序崩溃）
    // println!("{}", data[10]);  // 索引越界
    // let bad_slice = &data[10..15];  // 范围越界
}

fn demonstrate_safe_operations(data: &[i32]) {
    println!("\n=== 安全的操作方式 ===");
    
    println!("数据: {:?}", data);
    
    // 使用get方法安全访问
    match data.get(0) {
        Some(first) => println!("第一个元素: {}", first),
        None => println!("数据为空"),
    }
    
    match data.get(data.len().saturating_sub(1)) {
        Some(last) => println!("最后一个元素: {}", last),
        None => println!("数据为空"),
    }
    
    // 安全的切片获取
    match data.get(1..4) {
        Some(slice) => println!("安全切片 [1..4]: {:?}", slice),
        None => println!("切片范围无效"),
    }
    
    // 使用first()和last()方法
    if let Some(first) = data.first() {
        println!("使用first(): {}", first);
    }
    
    if let Some(last) = data.last() {
        println!("使用last(): {}", last);
    }
    
    // 安全的索引检查
    let index = 10;
    if index < data.len() {
        println!("索引{}的值: {}", index, data[index]);
    } else {
        println!("索引{}超出范围（长度: {}）", index, data.len());
    }
    
    // 使用迭代器避免索引
    for (i, &value) in data.iter().enumerate() {
        if i >= 3 { break; }  // 只处理前3个
        println!("位置{}: {}", i, value);
    }
}

fn demonstrate_string_safety() {
    println!("\n=== 字符串切片安全性 ===");
    
    let text = "Hello, 世界! 🦀";
    println!("文本: {}", text);
    println!("字节长度: {}", text.len());
    println!("字符数量: {}", text.chars().count());
    
    // ❌ 危险：可能在字符边界中间切割
    // let bad_slice = &text[0..8];  // 可能在多字节字符中间切割
    
    // ✅ 安全：使用字符边界
    let char_boundary_indices: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    println!("字符边界索引: {:?}", char_boundary_indices);
    
    // 安全的字符串切片
    if char_boundary_indices.len() > 7 {
        let safe_slice = &text[char_boundary_indices[0]..char_boundary_indices[7]];
        println!("安全切片（前7个字符）: {}", safe_slice);
    }
    
    // 使用字符迭代器
    let first_5_chars: String = text.chars().take(5).collect();
    println!("前5个字符: {}", first_5_chars);
    
    // 检查是否在字符边界
    for i in 0..=text.len() {
        if text.is_char_boundary(i) {
            let slice = &text[..i];
            println!("边界{}: '{}'", i, slice);
        } else {
            println!("位置{}不是字符边界", i);
        }
    }
}

fn demonstrate_bounds_checking() {
    println!("\n=== 边界检查演示 ===");
    
    let numbers = vec![10, 20, 30, 40, 50];
    
    // 创建安全的切片函数
    fn safe_slice(data: &[i32], start: usize, end: usize) -> Option<&[i32]> {
        if start <= end && end <= data.len() {
            Some(&data[start..end])
        } else {
            None
        }
    }
    
    // 测试各种边界情况
    let test_cases = [
        (0, 3),   // 正常情况
        (2, 5),   // 到末尾
        (0, 10),  // 超出范围
        (3, 2),   // 起始大于结束
        (5, 5),   // 空切片
    ];
    
    for (start, end) in test_cases {
        match safe_slice(&numbers, start, end) {
            Some(slice) => println!("切片[{}..{}]: {:?}", start, end, slice),
            None => println!("切片[{}..{}]: 无效范围", start, end),
        }
    }
    
    // 动态边界检查
    fn get_middle_slice(data: &[i32]) -> &[i32] {
        let len = data.len();
        if len <= 2 {
            data  // 返回全部
        } else {
            let start = len / 4;
            let end = len - len / 4;
            &data[start..end]
        }
    }
    
    let middle = get_middle_slice(&numbers);
    println!("中间部分: {:?}", middle);
    
    // 测试空数据
    let empty: Vec<i32> = vec![];
    let empty_middle = get_middle_slice(&empty);
    println!("空数据的中间部分: {:?}", empty_middle);
}
```

## ✅ 学习检查清单

- [ ] 理解切片的基本概念和用途
- [ ] 掌握字符串切片（&str）的创建和使用
- [ ] 学会创建和操作数组、向量切片
- [ ] 理解切片的内存表示（指针+长度）
- [ ] 掌握切片的范围语法（..、..=、start..、..end）
- [ ] 学会在函数中使用切片参数
- [ ] 理解切片的借用规则和生命周期
- [ ] 掌握切片的安全访问方法（get、first、last）
- [ ] 了解UTF-8字符串切片的安全性考虑
- [ ] 能够避免常见的切片相关错误

## 📖 扩展阅读

- [Rust官方文档 - 切片类型](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust Reference - 切片类型](https://doc.rust-lang.org/reference/types/slice.html)
- [Rust by Example - 切片](https://doc.rust-lang.org/rust-by-example/primitives/array.html)
- [字符串和切片的深入理解](https://doc.rust-lang.org/std/primitive.str.html)

---

**下一节预告：** 在下一节中，我们将学习生命周期基础，了解Rust如何确保引用的有效性和安全性。