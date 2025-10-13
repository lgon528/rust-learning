# 3.2 循环语句

循环是程序中重复执行代码块的重要控制结构。Rust提供了三种主要的循环类型：`loop`、`while`和`for`，每种都有其特定的使用场景和优势。本节将详细介绍这些循环的使用方法、最佳实践以及如何选择合适的循环类型。

## 🎯 学习目标

- 掌握loop、while、for三种循环的语法和用法
- 理解循环控制语句（break、continue）
- 学会使用循环标签处理嵌套循环
- 掌握迭代器的基本使用
- 了解循环的性能考虑和最佳实践

## 🔄 loop循环

### 基础loop循环

```rust
fn main() {
    // 基本的无限循环
    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("计数: {}", counter);
        
        if counter == 5 {
            break;  // 退出循环
        }
    }
    
    println!("循环结束，最终计数: {}", counter);
    
    // loop作为表达式返回值
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // 返回值
        }
    };
    
    println!("loop返回的结果: {}", result);
    
    // 使用loop实现重试逻辑
    let mut attempts = 0;
    let max_attempts = 3;
    
    let success = loop {
        attempts += 1;
        println!("尝试第{}次...", attempts);
        
        // 模拟操作（这里简单用随机性代替）
        if attempts == 2 {  // 假设第2次成功
            break true;
        }
        
        if attempts >= max_attempts {
            break false;
        }
    };
    
    if success {
        println!("操作成功！");
    } else {
        println!("操作失败，已达到最大重试次数");
    }
}
```

### loop的高级用法

```rust
fn main() {
    // 使用loop实现状态机
    let mut state = State::Start;
    let mut step = 0;
    
    loop {
        step += 1;
        println!("步骤 {}: 当前状态 {:?}", step, state);
        
        state = match state {
            State::Start => {
                println!("  开始处理...");
                State::Processing
            },
            State::Processing => {
                println!("  正在处理...");
                if step > 3 {
                    State::Complete
                } else {
                    State::Processing
                }
            },
            State::Complete => {
                println!("  处理完成！");
                break;
            },
            State::Error => {
                println!("  发生错误！");
                break;
            }
        };
        
        // 防止无限循环的安全检查
        if step > 10 {
            println!("  达到最大步数，强制退出");
            break;
        }
    }
    
    // 使用loop处理用户输入（模拟）
    simulate_user_input_loop();
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    Processing,
    Complete,
    Error,
}

fn simulate_user_input_loop() {
    let simulated_inputs = vec!["help", "status", "process", "quit"];
    let mut input_index = 0;
    
    println!("\n=== 模拟用户输入循环 ===");
    
    loop {
        // 模拟获取用户输入
        let input = if input_index < simulated_inputs.len() {
            let cmd = simulated_inputs[input_index];
            input_index += 1;
            cmd
        } else {
            "quit"  // 模拟用户最终输入quit
        };
        
        println!("用户输入: {}", input);
        
        match input {
            "help" => {
                println!("  可用命令: help, status, process, quit");
            },
            "status" => {
                println!("  系统状态: 正常运行");
            },
            "process" => {
                println!("  开始处理任务...");
                // 这里可以调用其他函数
            },
            "quit" => {
                println!("  再见！");
                break;
            },
            _ => {
                println!("  未知命令: {}，输入 'help' 查看帮助", input);
            }
        }
    }
}
```

## 🔄 while循环

### 基础while循环

```rust
fn main() {
    // 基本while循环
    let mut number = 3;
    
    while number != 0 {
        println!("倒计时: {}!", number);
        number -= 1;
    }
    
    println!("发射！🚀");
    
    // while循环处理集合
    let mut stack = vec![1, 2, 3, 4, 5];
    
    println!("\n处理栈中的元素:");
    while let Some(value) = stack.pop() {
        println!("弹出: {}", value);
    }
    
    println!("栈已清空");
    
    // while循环进行搜索
    let numbers = vec![1, 3, 5, 7, 9, 2, 4, 6, 8];
    let target = 7;
    let mut index = 0;
    let mut found = false;
    
    while index < numbers.len() {
        if numbers[index] == target {
            println!("找到目标值 {} 在索引 {}", target, index);
            found = true;
            break;
        }
        index += 1;
    }
    
    if !found {
        println!("未找到目标值 {}", target);
    }
    
    // while循环实现简单的游戏逻辑
    play_guessing_game();
}

fn play_guessing_game() {
    println!("\n=== 猜数字游戏 ===");
    
    let secret_number = 42;  // 在实际游戏中，这应该是随机数
    let guesses = vec![30, 50, 40, 42];  // 模拟用户猜测
    let mut guess_index = 0;
    let mut attempts = 0;
    let max_attempts = 5;
    
    while attempts < max_attempts {
        attempts += 1;
        
        // 模拟获取用户猜测
        let guess = if guess_index < guesses.len() {
            let g = guesses[guess_index];
            guess_index += 1;
            g
        } else {
            break;  // 没有更多猜测了
        };
        
        println!("第{}次猜测: {}", attempts, guess);
        
        if guess == secret_number {
            println!("🎉 恭喜！你猜对了！");
            break;
        } else if guess < secret_number {
            println!("太小了！");
        } else {
            println!("太大了！");
        }
    }
    
    if attempts >= max_attempts {
        println!("游戏结束！答案是 {}", secret_number);
    }
}
```

### while let模式匹配

```rust
fn main() {
    // while let处理Option
    let mut optional_values = vec![Some(1), Some(2), Some(3), None, Some(4)];
    let mut index = 0;
    
    println!("处理可选值:");
    while let Some(value) = optional_values.get(index).and_then(|x| *x) {
        println!("处理值: {}", value);
        index += 1;
    }
    
    // while let处理Result
    let parse_attempts = vec!["42", "abc", "123", "xyz", "789"];
    let mut attempt_index = 0;
    
    println!("\n解析字符串为数字:");
    while attempt_index < parse_attempts.len() {
        let input = parse_attempts[attempt_index];
        attempt_index += 1;
        
        match input.parse::<i32>() {
            Ok(number) => {
                println!("成功解析 '{}' 为 {}", input, number);
            },
            Err(_) => {
                println!("无法解析 '{}' 为数字", input);
            }
        }
    }
    
    // while let处理迭代器
    let mut chars = "Hello".chars();
    
    println!("\n逐个处理字符:");
    while let Some(ch) = chars.next() {
        println!("字符: '{}' (ASCII: {})", ch, ch as u8);
    }
    
    // while let处理自定义枚举
    let mut message_queue = vec![
        Message::Text("Hello".to_string()),
        Message::Number(42),
        Message::Quit,
        Message::Text("World".to_string()),
    ];
    
    println!("\n处理消息队列:");
    while let Some(message) = message_queue.pop() {
        match message {
            Message::Text(text) => println!("收到文本消息: {}", text),
            Message::Number(num) => println!("收到数字消息: {}", num),
            Message::Quit => {
                println!("收到退出消息，停止处理");
                break;
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
    Quit,
}
```

## 🔄 for循环

### 基础for循环

```rust
fn main() {
    // 遍历范围
    println!("基本范围遍历:");
    for i in 1..6 {
        println!("数字: {}", i);
    }
    
    // 包含结束值的范围
    println!("\n包含结束值的范围:");
    for i in 1..=5 {
        println!("数字: {}", i);
    }
    
    // 遍历数组
    let array = [10, 20, 30, 40, 50];
    
    println!("\n遍历数组:");
    for element in array {
        println!("元素: {}", element);
    }
    
    // 遍历数组的引用（不获取所有权）
    println!("\n遍历数组引用:");
    for element in &array {
        println!("元素引用: {}", element);
    }
    
    // 遍历向量
    let vec = vec!["apple", "banana", "cherry", "date"];
    
    println!("\n遍历向量:");
    for fruit in &vec {
        println!("水果: {}", fruit);
    }
    
    // 带索引的遍历
    println!("\n带索引的遍历:");
    for (index, fruit) in vec.iter().enumerate() {
        println!("索引 {}: {}", index, fruit);
    }
    
    // 遍历字符串的字符
    let text = "Hello, 世界!";
    
    println!("\n遍历字符串字符:");
    for ch in text.chars() {
        println!("字符: '{}'", ch);
    }
    
    // 遍历字符串的字节
    println!("\n遍历字符串字节:");
    for byte in text.bytes() {
        println!("字节: {} ('{}')", byte, byte as char);
    }
}
```

### for循环的高级用法

```rust
use std::collections::HashMap;

fn main() {
    // 遍历HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    scores.insert("Diana", 98);
    
    println!("学生成绩:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    
    // 计算统计信息
    let mut total = 0;
    let mut count = 0;
    
    for score in scores.values() {
        total += score;
        count += 1;
    }
    
    let average = total as f64 / count as f64;
    println!("平均分: {:.2}", average);
    
    // 过滤和处理
    println!("\n高分学生 (>= 90):");
    for (name, score) in &scores {
        if *score >= 90 {
            println!("{}: {} ⭐", name, score);
        }
    }
    
    // 嵌套循环 - 乘法表
    println!("\n乘法表 (1-5):");
    for i in 1..=5 {
        for j in 1..=5 {
            print!("{:2} ", i * j);
        }
        println!();  // 换行
    }
    
    // 处理二维数组
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    
    println!("\n矩阵遍历:");
    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            println!("matrix[{}][{}] = {}", row_index, col_index, value);
        }
    }
    
    // 使用迭代器方法
    println!("\n使用迭代器方法:");
    let numbers: Vec<i32> = (1..=10).collect();
    
    // 过滤偶数并平方
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    
    println!("偶数的平方: {:?}", even_squares);
    
    // 查找第一个满足条件的元素
    let first_large = numbers
        .iter()
        .find(|&&x| x > 7);
    
    match first_large {
        Some(value) => println!("第一个大于7的数: {}", value),
        None => println!("没有找到大于7的数"),
    }
}
```

## 🏷️ 循环标签和控制

### break和continue

```rust
fn main() {
    // 基本的break和continue
    println!("跳过偶数，遇到8时停止:");
    for i in 1..=10 {
        if i % 2 == 0 {
            if i == 8 {
                break;  // 遇到8时完全停止
            }
            continue;  // 跳过偶数
        }
        println!("奇数: {}", i);
    }
    
    // 在while循环中使用break和continue
    println!("\n处理数字序列:");
    let numbers = vec![1, -2, 3, -4, 5, 0, 6, -7, 8];
    let mut index = 0;
    
    while index < numbers.len() {
        let num = numbers[index];
        index += 1;
        
        if num == 0 {
            println!("遇到零，停止处理");
            break;
        }
        
        if num < 0 {
            println!("跳过负数: {}", num);
            continue;
        }
        
        println!("处理正数: {}", num);
    }
    
    // loop中的break返回值
    println!("\n查找第一个完全平方数:");
    let mut n = 1;
    
    let perfect_square = loop {
        let square = n * n;
        
        if square > 50 {
            break square;  // 返回找到的平方数
        }
        
        n += 1;
    };
    
    println!("第一个大于50的完全平方数: {}", perfect_square);
}
```

### 循环标签

```rust
fn main() {
    // 嵌套循环中的标签使用
    println!("查找矩阵中的目标值:");
    
    let matrix = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];
    
    let target = 11;
    let mut found = false;
    
    'outer: for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            println!("检查位置 ({}, {}): {}", row_idx, col_idx, value);
            
            if value == target {
                println!("找到目标值 {} 在位置 ({}, {})", target, row_idx, col_idx);
                found = true;
                break 'outer;  // 跳出外层循环
            }
        }
    }
    
    if !found {
        println!("未找到目标值 {}", target);
    }
    
    // 复杂的嵌套循环控制
    println!("\n复杂的循环控制示例:");
    
    'main_loop: loop {
        println!("进入主循环");
        
        for i in 1..=3 {
            println!("  外层循环 i = {}", i);
            
            'inner: for j in 1..=3 {
                println!("    内层循环 j = {}", j);
                
                if i == 2 && j == 2 {
                    println!("    跳过内层循环的剩余部分");
                    continue 'inner;
                }
                
                if i == 3 && j == 1 {
                    println!("    退出主循环");
                    break 'main_loop;
                }
                
                println!("    处理 ({}, {})", i, j);
            }
        }
        
        println!("主循环的一次迭代完成");
        break;  // 防止无限循环
    }
    
    // 使用标签处理错误情况
    process_data_with_labels();
}

fn process_data_with_labels() {
    println!("\n使用标签处理数据:");
    
    let data_sets = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 0, 9, 10],  // 包含0，应该跳过整个数据集
        vec![11, 12, 13, 14, 15],
    ];
    
    'dataset: for (set_idx, dataset) in data_sets.iter().enumerate() {
        println!("处理数据集 {}: {:?}", set_idx, dataset);
        
        for &value in dataset {
            if value == 0 {
                println!("  发现无效值0，跳过整个数据集");
                continue 'dataset;
            }
            
            if value > 10 {
                println!("  处理大值: {}", value);
            } else {
                println!("  处理小值: {}", value);
            }
        }
        
        println!("  数据集 {} 处理完成", set_idx);
    }
}
```

## 🚀 迭代器基础

### 迭代器的创建和使用

```rust
fn main() {
    // 从集合创建迭代器
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - 创建不可变引用的迭代器
    println!("使用 iter():");
    for item in vec.iter() {
        println!("  {}", item);  // item 是 &i32
    }
    
    // into_iter() - 创建获取所有权的迭代器
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("\n使用 into_iter():");
    for item in vec2.into_iter() {
        println!("  {}", item);  // item 是 i32
    }
    // 注意：vec2 在这里已经被移动，不能再使用
    
    // iter_mut() - 创建可变引用的迭代器
    let mut vec3 = vec![1, 2, 3, 4, 5];
    println!("\n使用 iter_mut() 修改元素:");
    for item in vec3.iter_mut() {
        *item *= 2;  // item 是 &mut i32
    }
    println!("修改后的向量: {:?}", vec3);
    
    // 范围迭代器
    println!("\n范围迭代器:");
    for i in (0..5).rev() {  // 反向迭代
        println!("  倒计时: {}", i);
    }
    
    // 字符串迭代器
    let text = "Hello";
    println!("\n字符迭代器:");
    for (index, ch) in text.chars().enumerate() {
        println!("  字符 {}: '{}'", index, ch);
    }
    
    // 自定义步长
    println!("\n自定义步长 (每次增加2):");
    for i in (0..10).step_by(2) {
        println!("  {}", i);
    }
}
```

### 迭代器方法链

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作：过滤、映射、收集
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .map(|&x| x * x)           // 平方
        .collect();                // 收集结果
    
    println!("偶数的平方: {:?}", even_squares);
    
    // 查找操作
    let first_large = numbers
        .iter()
        .find(|&&x| x > 7);
    
    println!("第一个大于7的数: {:?}", first_large);
    
    // 聚合操作
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    
    println!("总和: {}", sum);
    println!("乘积: {}", product);
    println!("最大值: {:?}", max);
    println!("最小值: {:?}", min);
    
    // 条件检查
    let all_positive = numbers.iter().all(|&&x| x > 0);
    let any_even = numbers.iter().any(|&&x| x % 2 == 0);
    
    println!("所有数都是正数: {}", all_positive);
    println!("存在偶数: {}", any_even);
    
    // 复杂的数据处理
    let words = vec!["hello", "world", "rust", "programming", "language"];
    
    let long_words: Vec<String> = words
        .iter()
        .filter(|word| word.len() > 4)     // 长度大于4
        .map(|word| word.to_uppercase())   // 转大写
        .collect();
    
    println!("长单词(大写): {:?}", long_words);
    
    // 分组和统计
    let word_lengths: Vec<usize> = words
        .iter()
        .map(|word| word.len())
        .collect();
    
    println!("单词长度: {:?}", word_lengths);
    
    // 使用enumerate获取索引
    println!("\n带索引的单词:");
    for (index, word) in words.iter().enumerate() {
        println!("  {}: {}", index, word);
    }
}
```

## ⚡ 性能考虑和最佳实践

### 循环性能优化

```rust
fn main() {
    // 避免在循环中重复计算
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 不好的做法：每次都计算长度
    // for i in 0..data.len() {
    //     // 每次迭代都调用 data.len()
    // }
    
    // 好的做法：预先计算长度
    let len = data.len();
    for i in 0..len {
        println!("索引 {}: {}", i, data[i]);
    }
    
    // 更好的做法：使用迭代器
    for (index, value) in data.iter().enumerate() {
        println!("索引 {}: {}", index, value);
    }
    
    // 字符串处理的性能考虑
    let mut result = String::new();
    let words = vec!["hello", "world", "rust", "programming"];
    
    // 不好的做法：频繁的字符串连接
    // for word in &words {
    //     result = result + word + " ";  // 每次都创建新字符串
    // }
    
    // 好的做法：使用 push_str
    for word in &words {
        result.push_str(word);
        result.push(' ');
    }
    
    println!("连接结果: {}", result.trim());
    
    // 更好的做法：使用 join
    let joined = words.join(" ");
    println!("使用join: {}", joined);
    
    // 大数据集的处理
    process_large_dataset();
}

fn process_large_dataset() {
    println!("\n处理大数据集的最佳实践:");
    
    // 模拟大数据集
    let large_data: Vec<i32> = (1..=1000).collect();
    
    // 使用迭代器进行惰性计算
    let result: Vec<i32> = large_data
        .iter()
        .filter(|&&x| x % 2 == 0)      // 只处理偶数
        .take(10)                      // 只取前10个
        .map(|&x| x * x)               // 计算平方
        .collect();
    
    println!("前10个偶数的平方: {:?}", result);
    
    // 早期退出优化
    let target = 100;
    let mut found_index = None;
    
    for (index, &value) in large_data.iter().enumerate() {
        if value == target {
            found_index = Some(index);
            break;  // 找到后立即退出
        }
    }
    
    match found_index {
        Some(index) => println!("找到目标值 {} 在索引 {}", target, index),
        None => println!("未找到目标值 {}", target),
    }
    
    // 使用迭代器的 position 方法更简洁
    let index = large_data.iter().position(|&x| x == target);
    println!("使用position方法找到索引: {:?}", index);
}
```

### 选择合适的循环类型

```rust
fn main() {
    println!("循环类型选择指南:");
    
    // 1. 已知迭代次数 - 使用 for 循环
    println!("\n1. 已知迭代次数 - for循环:");
    for i in 1..=5 {
        println!("  第{}次迭代", i);
    }
    
    // 2. 遍历集合 - 使用 for 循环
    println!("\n2. 遍历集合 - for循环:");
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("  水果: {}", fruit);
    }
    
    // 3. 条件控制的循环 - 使用 while 循环
    println!("\n3. 条件控制 - while循环:");
    let mut count = 0;
    while count < 3 {
        count += 1;
        println!("  计数: {}", count);
    }
    
    // 4. 无限循环或复杂控制 - 使用 loop
    println!("\n4. 复杂控制 - loop循环:");
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        println!("  尝试第{}次", attempts);
        
        if attempts == 3 {
            break "成功";
        }
        
        if attempts > 5 {
            break "失败";
        }
    };
    println!("  结果: {}", result);
    
    // 5. 数据处理 - 使用迭代器
    println!("\n5. 数据处理 - 迭代器:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let processed: Vec<String> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| format!("偶数: {}", x))
        .collect();
    
    for item in processed {
        println!("  {}", item);
    }
}
```

## 🧪 实践练习

### 练习1：数字处理系统

```rust
// TODO: 实现一个数字处理系统
// 要求：
// 1. 生成1-100的数字序列
// 2. 找出所有质数
// 3. 计算质数的统计信息（总数、平均值、最大值、最小值）
// 4. 找出所有完全平方数
// 5. 找出既是质数又接近完全平方数的数字（差值小于5）

fn is_prime(n: u32) -> bool {
    // TODO: 实现质数判断
    todo!("实现质数判断逻辑")
}

fn is_perfect_square(n: u32) -> bool {
    // TODO: 实现完全平方数判断
    todo!("实现完全平方数判断逻辑")
}

fn find_primes_in_range(start: u32, end: u32) -> Vec<u32> {
    // TODO: 找出范围内的所有质数
    todo!("实现质数查找")
}

fn calculate_statistics(numbers: &[u32]) -> (usize, f64, u32, u32) {
    // TODO: 计算统计信息：(总数, 平均值, 最大值, 最小值)
    todo!("实现统计计算")
}

fn find_special_numbers(primes: &[u32], squares: &[u32]) -> Vec<u32> {
    // TODO: 找出既是质数又接近完全平方数的数字
    todo!("实现特殊数字查找")
}

fn main() {
    println!("=== 数字处理系统 ===");
    
    // 生成数字序列
    let numbers: Vec<u32> = (1..=100).collect();
    println!("处理范围: 1-100");
    
    // 找出质数
    let primes = find_primes_in_range(1, 100);
    println!("\n找到的质数: {:?}", primes);
    
    // 计算质数统计
    let (count, avg, max, min) = calculate_statistics(&primes);
    println!("\n质数统计:");
    println!("  总数: {}", count);
    println!("  平均值: {:.2}", avg);
    println!("  最大值: {}", max);
    println!("  最小值: {}", min);
    
    // 找出完全平方数
    let squares: Vec<u32> = numbers
        .iter()
        .filter(|&&n| is_perfect_square(n))
        .cloned()
        .collect();
    println!("\n完全平方数: {:?}", squares);
    
    // 找出特殊数字
    let special = find_special_numbers(&primes, &squares);
    println!("\n特殊数字（质数且接近完全平方数）: {:?}", special);
}
```

### 练习2：文本分析器

```rust
use std::collections::HashMap;

// TODO: 实现一个文本分析器
// 要求：
// 1. 统计单词频率
// 2. 找出最长和最短的单词
// 3. 统计字符频率
// 4. 找出回文单词
// 5. 生成文本摘要

#[derive(Debug)]
struct TextAnalysis {
    word_count: HashMap<String, usize>,
    char_count: HashMap<char, usize>,
    total_words: usize,
    total_chars: usize,
    longest_word: String,
    shortest_word: String,
    palindromes: Vec<String>,
}

impl TextAnalysis {
    fn new() -> Self {
        TextAnalysis {
            word_count: HashMap::new(),
            char_count: HashMap::new(),
            total_words: 0,
            total_chars: 0,
            longest_word: String::new(),
            shortest_word: String::new(),
            palindromes: Vec::new(),
        }
    }
    
    // TODO: 分析文本
    fn analyze(&mut self, text: &str) {
        todo!("实现文本分析逻辑")
    }
    
    // TODO: 获取最频繁的单词
    fn most_frequent_words(&self, n: usize) -> Vec<(&String, &usize)> {
        todo!("实现最频繁单词查找")
    }
    
    // TODO: 获取最频繁的字符
    fn most_frequent_chars(&self, n: usize) -> Vec<(&char, &usize)> {
        todo!("实现最频繁字符查找")
    }
    
    // TODO: 生成摘要
    fn generate_summary(&self) -> String {
        todo!("实现摘要生成")
    }
}

// TODO: 辅助函数
fn is_palindrome(word: &str) -> bool {
    todo!("实现回文检查")
}

fn clean_word(word: &str) -> String {
    // 移除标点符号并转换为小写
    todo!("实现单词清理")
}

fn main() {
    let sample_text = "
        Hello world! This is a sample text for analysis.
        The quick brown fox jumps over the lazy dog.
        A man a plan a canal Panama.
        Rust is a systems programming language.
        Programming in Rust is fun and safe.
        Hello again, world!
    ";
    
    println!("=== 文本分析器 ===");
    println!("原始文本:");
    println!("{}", sample_text);
    
    let mut analyzer = TextAnalysis::new();
    analyzer.analyze(sample_text);
    
    println!("\n=== 分析结果 ===");
    println!("总单词数: {}", analyzer.total_words);
    println!("总字符数: {}", analyzer.total_chars);
    println!("最长单词: {}", analyzer.longest_word);
    println!("最短单词: {}", analyzer.shortest_word);
    
    println!("\n最频繁的5个单词:");
    for (word, count) in analyzer.most_frequent_words(5) {
        println!("  {}: {}", word, count);
    }
    
    println!("\n最频繁的5个字符:");
    for (ch, count) in analyzer.most_frequent_chars(5) {
        println!("  '{}': {}", ch, count);
    }
    
    println!("\n回文单词: {:?}", analyzer.palindromes);
    
    println!("\n文本摘要:");
    println!("{}", analyzer.generate_summary());
}
```

### 练习3：游戏循环系统

```rust
// TODO: 实现一个简单的游戏循环系统
// 要求：
// 1. 玩家可以移动（上下左右）
// 2. 地图上有宝藏和陷阱
// 3. 玩家有生命值和分数
// 4. 游戏有胜利和失败条件
// 5. 实现游戏状态的保存和恢复

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Empty,
    Player,
    Treasure,
    Trap,
    Wall,
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Player {
    position: Position,
    health: i32,
    score: i32,
    treasures_collected: usize,
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<CellType>>,
    player: Player,
    game_over: bool,
    victory: bool,
    turn_count: usize,
}

impl Game {
    // TODO: 创建新游戏
    fn new(width: usize, height: usize) -> Self {
        todo!("实现游戏初始化")
    }
    
    // TODO: 移动玩家
    fn move_player(&mut self, direction: Direction) -> Result<(), String> {
        todo!("实现玩家移动逻辑")
    }
    
    // TODO: 检查游戏状态
    fn check_game_state(&mut self) {
        todo!("实现游戏状态检查")
    }
    
    // TODO: 显示地图
    fn display_map(&self) {
        todo!("实现地图显示")
    }
    
    // TODO: 显示游戏状态
    fn display_status(&self) {
        todo!("实现状态显示")
    }
    
    // TODO: 处理单元格事件
    fn handle_cell_event(&mut self, cell_type: CellType) {
        todo!("实现单元格事件处理")
    }
    
    // TODO: 保存游戏状态
    fn save_state(&self) -> GameState {
        todo!("实现状态保存")
    }
    
    // TODO: 恢复游戏状态
    fn restore_state(&mut self, state: GameState) {
        todo!("实现状态恢复")
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct GameState {
    player_position: Position,
    health: i32,
    score: i32,
    treasures_collected: usize,
    turn_count: usize,
}

fn main() {
    println!("=== 宝藏猎人游戏 ===");
    
    let mut game = Game::new(8, 6);
    let mut saved_states: Vec<GameState> = Vec::new();
    
    // 模拟游戏输入
    let moves = vec![
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Down,
        Direction::Right,
        Direction::Up,
        Direction::Right,
    ];
    
    println!("游戏开始！");
    game.display_map();
    game.display_status();
    
    for (turn, direction) in moves.iter().enumerate() {
        println!("\n=== 第{}回合 ===", turn + 1);
        println!("移动方向: {:?}", direction);
        
        // 保存状态（每3回合保存一次）
        if turn % 3 == 0 {
            saved_states.push(game.save_state());
            println!("游戏状态已保存");
        }
        
        match game.move_player(direction.clone()) {
            Ok(()) => {
                game.display_map();
                game.display_status();
                
                if game.game_over {
                    if game.victory {
                        println!("\n🎉 恭喜！你赢得了游戏！");
                    } else {
                        println!("\n💀 游戏结束！你失败了！");
                    }
                    break;
                }
            },
            Err(error) => {
                println!("移动失败: {}", error);
            }
        }
    }
    
    // 演示状态恢复
    if !saved_states.is_empty() {
        println!("\n=== 恢复到之前的状态 ===");
        game.restore_state(saved_states[0].clone());
        game.display_map();
        game.display_status();
    }
}
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 熟练使用loop、while、for三种循环
- [ ] 理解何时使用哪种循环类型
- [ ] 正确使用break和continue控制循环
- [ ] 掌握循环标签的使用方法
- [ ] 理解loop作为表达式的概念
- [ ] 使用while let进行模式匹配
- [ ] 掌握for循环遍历各种数据结构
- [ ] 了解迭代器的基本使用
- [ ] 理解循环的性能考虑
- [ ] 能够选择合适的循环类型解决问题

## 📚 延伸阅读

- [Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust Reference - Loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [Rust by Example - Loops](https://doc.rust-lang.org/rust-by-example/flow_control/loop.html)

---

**循环语句掌握完成！** 🎯 你现在能够熟练使用Rust的各种循环结构。

[← 上一节：条件语句](./01-conditionals.md) | [下一节：模式匹配 →](./03-pattern-matching.md)