//! # Rust 控制流演示
//!
//! 这个库演示了 Rust 中各种控制流结构的使用方法，包括条件语句、循环和模式匹配。
//!
//! ## 主要内容
//!
//! - `if` 表达式和条件分支
//! - `loop`、`while` 和 `for` 循环
//! - `match` 表达式和模式匹配
//! - `if let` 和 `while let` 语法糖
//! - 循环控制：`break` 和 `continue`
//!
//! ## 使用示例
//!
//! ```
//! use control_flow_demo::conditionals::check_number;
//! use control_flow_demo::loops::count_to_n;
//! use control_flow_demo::pattern_matching::describe_value;
//!
//! // 条件判断
//! let result = check_number(42);
//! println!("{}", result);
//!
//! // 循环计数
//! count_to_n(5);
//!
//! // 模式匹配
//! let description = describe_value(Some(100));
//! println!("{}", description);
//! ```

/// 条件语句演示模块
/// 
/// 演示 `if` 表达式的各种用法
pub mod conditionals {
    /// 检查数字的性质
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::conditionals::check_number;
    /// 
    /// assert_eq!(check_number(0), "零");
    /// assert_eq!(check_number(5), "正数");
    /// assert_eq!(check_number(-3), "负数");
    /// ```
    pub fn check_number(n: i32) -> &'static str {
        if n > 0 {
            "正数"
        } else if n < 0 {
            "负数"
        } else {
            "零"
        }
    }
    
    /// 使用 if 作为表达式
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::conditionals::get_absolute_value;
    /// 
    /// assert_eq!(get_absolute_value(-5), 5);
    /// assert_eq!(get_absolute_value(3), 3);
    /// ```
    pub fn get_absolute_value(n: i32) -> i32 {
        if n < 0 { -n } else { n }
    }
    
    /// 复杂条件判断
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::conditionals::categorize_age;
    /// 
    /// assert_eq!(categorize_age(5), "儿童");
    /// assert_eq!(categorize_age(15), "青少年");
    /// assert_eq!(categorize_age(25), "成年人");
    /// assert_eq!(categorize_age(70), "老年人");
    /// ```
    pub fn categorize_age(age: u32) -> &'static str {
        if age < 13 {
            "儿童"
        } else if age < 20 {
            "青少年"
        } else if age < 60 {
            "成年人"
        } else {
            "老年人"
        }
    }
    
    /// 演示条件语句的各种用法
    pub fn demonstrate_conditionals() {
        println!("=== 条件语句演示 ===");
        
        let number = 42;
        println!("数字 {} 是: {}", number, check_number(number));
        
        let negative = -15;
        println!("数字 {} 的绝对值是: {}", negative, get_absolute_value(negative));
        
        let age = 25;
        println!("年龄 {} 属于: {}", age, categorize_age(age));
        
        // 使用 if let 进行模式匹配
        let optional_value = Some(10);
        if let Some(value) = optional_value {
            println!("可选值包含: {}", value);
        } else {
            println!("可选值为空");
        }
    }
}

/// 循环演示模块
/// 
/// 演示各种循环结构的使用
pub mod loops {
    /// 使用 for 循环计数到 n
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::loops::count_to_n;
    /// 
    /// count_to_n(3); // 输出: 1, 2, 3
    /// ```
    pub fn count_to_n(n: u32) {
        println!("计数到 {}:", n);
        for i in 1..=n {
            println!("{}", i);
        }
    }
    
    /// 计算阶乘
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::loops::factorial;
    /// 
    /// assert_eq!(factorial(5), 120);
    /// assert_eq!(factorial(0), 1);
    /// ```
    pub fn factorial(n: u32) -> u64 {
        let mut result = 1;
        let mut i = 1;
        
        while i <= n {
            result *= i as u64;
            i += 1;
        }
        
        result
    }
    
    /// 使用 loop 和 break 查找第一个偶数
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::loops::find_first_even;
    /// 
    /// let numbers = vec![1, 3, 5, 8, 9, 12];
    /// assert_eq!(find_first_even(&numbers), Some(8));
    /// 
    /// let odd_numbers = vec![1, 3, 5, 7];
    /// assert_eq!(find_first_even(&odd_numbers), None);
    /// ```
    pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
        let mut index = 0;
        
        loop {
            if index >= numbers.len() {
                break None;
            }
            
            if numbers[index] % 2 == 0 {
                break Some(numbers[index]);
            }
            
            index += 1;
        }
    }
    
    /// 使用嵌套循环和标签
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::loops::find_pair_sum;
    /// 
    /// let numbers = vec![1, 2, 3, 4, 5];
    /// assert_eq!(find_pair_sum(&numbers, 7), Some((2, 5)));
    /// assert_eq!(find_pair_sum(&numbers, 10), None);
    /// ```
    pub fn find_pair_sum(numbers: &[i32], target: i32) -> Option<(i32, i32)> {
        for (i, &a) in numbers.iter().enumerate() {
            for &b in &numbers[i + 1..] {
                if a + b == target {
                    return Some((a, b));
                }
            }
        }
        None
    }
    
    /// 演示各种循环结构
    pub fn demonstrate_loops() {
        println!("=== 循环演示 ===");
        
        // for 循环
        println!("\n--- for 循环 ---");
        count_to_n(3);
        
        // while 循环
        println!("\n--- while 循环计算阶乘 ---");
        let fact = factorial(5);
        println!("5! = {}", fact);
        
        // loop 循环
        println!("\n--- loop 循环查找偶数 ---");
        let numbers = vec![1, 3, 5, 8, 9, 12];
        if let Some(even) = find_first_even(&numbers) {
            println!("找到第一个偶数: {}", even);
        }
        
        // 嵌套循环
        println!("\n--- 嵌套循环查找配对 ---");
        if let Some((a, b)) = find_pair_sum(&numbers, 9) {
            println!("找到配对: {} + {} = 9", a, b);
        }
        
        // while let 循环
        println!("\n--- while let 循环 ---");
        let mut stack = vec![1, 2, 3, 4, 5];
        while let Some(value) = stack.pop() {
            println!("弹出: {}", value);
        }
    }
}

/// 模式匹配演示模块
/// 
/// 演示 `match` 表达式和各种模式匹配技巧
pub mod pattern_matching {
    /// 定义一个枚举用于演示
    #[derive(Debug, Clone, PartialEq)]
    pub enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
        Hsl { h: u16, s: u8, l: u8 },
    }
    
    /// 描述颜色
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::pattern_matching::{Color, describe_color};
    /// 
    /// assert_eq!(describe_color(Color::Red), "纯红色");
    /// assert_eq!(describe_color(Color::Rgb(255, 0, 0)), "RGB红色");
    /// ```
    pub fn describe_color(color: Color) -> &'static str {
        match color {
            Color::Red => "纯红色",
            Color::Green => "纯绿色",
            Color::Blue => "纯蓝色",
            Color::Rgb(255, 0, 0) => "RGB红色",
            Color::Rgb(0, 255, 0) => "RGB绿色",
            Color::Rgb(0, 0, 255) => "RGB蓝色",
            Color::Rgb(_, _, _) => "其他RGB颜色",
            Color::Hsl { h: 0..=60, .. } => "暖色调HSL",
            Color::Hsl { h: 180..=300, .. } => "冷色调HSL",
            Color::Hsl { .. } => "其他HSL颜色",
        }
    }
    
    /// 描述 Option 值
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::pattern_matching::describe_value;
    /// 
    /// assert_eq!(describe_value(Some(42)), "有值: 42");
    /// assert_eq!(describe_value::<i32>(None), "无值");
    /// ```
    pub fn describe_value<T: std::fmt::Display>(value: Option<T>) -> String {
        match value {
            Some(v) => format!("有值: {}", v),
            None => "无值".to_string(),
        }
    }
    
    /// 处理 Result 类型
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::pattern_matching::handle_result;
    /// 
    /// let ok_result: Result<i32, &str> = Ok(42);
    /// assert_eq!(handle_result(ok_result), "成功: 42");
    /// 
    /// let err_result: Result<i32, &str> = Err("错误信息");
    /// assert_eq!(handle_result(err_result), "错误: 错误信息");
    /// ```
    pub fn handle_result<T: std::fmt::Display, E: std::fmt::Display>(result: Result<T, E>) -> String {
        match result {
            Ok(value) => format!("成功: {}", value),
            Err(error) => format!("错误: {}", error),
        }
    }
    
    /// 匹配数字范围
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::pattern_matching::classify_number;
    /// 
    /// assert_eq!(classify_number(5), "小数");
    /// assert_eq!(classify_number(50), "中数");
    /// assert_eq!(classify_number(150), "大数");
    /// ```
    pub fn classify_number(n: i32) -> &'static str {
        match n {
            i32::MIN..=-1 => "负数",
            0 => "零",
            1..=10 => "小数",
            11..=100 => "中数",
            101..=i32::MAX => "大数",
        }
    }
    
    /// 使用守卫条件的匹配
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::pattern_matching::describe_pair;
    /// 
    /// assert_eq!(describe_pair((2, 4)), "都是偶数");
    /// assert_eq!(describe_pair((1, 3)), "都是奇数");
    /// assert_eq!(describe_pair((1, 2)), "混合");
    /// ```
    pub fn describe_pair(pair: (i32, i32)) -> &'static str {
        match pair {
            (x, y) if x % 2 == 0 && y % 2 == 0 => "都是偶数",
            (x, y) if x % 2 == 1 && y % 2 == 1 => "都是奇数",
            _ => "混合",
        }
    }
    
    /// 演示模式匹配的各种用法
    pub fn demonstrate_pattern_matching() {
        println!("=== 模式匹配演示 ===");
        
        // 枚举匹配
        println!("\n--- 枚举匹配 ---");
        let colors = vec![
            Color::Red,
            Color::Rgb(255, 128, 0),
            Color::Hsl { h: 240, s: 100, l: 50 },
        ];
        
        for color in colors {
            println!("{:?} -> {}", color, describe_color(color.clone()));
        }
        
        // Option 匹配
        println!("\n--- Option 匹配 ---");
        let values = vec![Some(42), None, Some(0)];
        for value in values {
            println!("{:?} -> {}", value, describe_value(value));
        }
        
        // Result 匹配
        println!("\n--- Result 匹配 ---");
        let results: Vec<Result<i32, &str>> = vec![
            Ok(100),
            Err("计算错误"),
        ];
        for result in results {
            println!("{:?} -> {}", result, handle_result(result));
        }
        
        // 数字范围匹配
        println!("\n--- 数字范围匹配 ---");
        let numbers = vec![-5, 0, 5, 50, 150];
        for num in numbers {
            println!("{} -> {}", num, classify_number(num));
        }
        
        // 守卫条件匹配
        println!("\n--- 守卫条件匹配 ---");
        let pairs = vec![(2, 4), (1, 3), (1, 2)];
        for pair in pairs {
            println!("{:?} -> {}", pair, describe_pair(pair));
        }
    }
}

/// 高级控制流演示模块
/// 
/// 演示一些高级的控制流技巧
pub mod advanced {
    /// 使用闭包和迭代器的函数式风格
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::advanced::process_numbers;
    /// 
    /// let numbers = vec![1, 2, 3, 4, 5, 6];
    /// let result = process_numbers(&numbers);
    /// assert_eq!(result, vec![4, 16, 36]); // 偶数的平方: 2->4, 4->16, 6->36
    /// ```
    pub fn process_numbers(numbers: &[i32]) -> Vec<i32> {
        numbers
            .iter()
            .filter(|&&x| x % 2 == 0)  // 过滤偶数
            .map(|&x| x * x)           // 计算平方
            .collect()                 // 收集结果
    }
    
    /// 使用 ? 操作符进行错误传播
    /// 
    /// # 示例
    /// 
    /// ```
    /// use control_flow_demo::advanced::divide_and_add;
    /// 
    /// assert_eq!(divide_and_add(10, 2, 5), Ok(10));
    /// assert!(divide_and_add(10, 0, 5).is_err());
    /// ```
    pub fn divide_and_add(a: i32, b: i32, c: i32) -> Result<i32, String> {
        let result = divide(a, b)? + c;
        Ok(result)
    }
    
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("除零错误".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    /// 演示高级控制流
    pub fn demonstrate_advanced() {
        println!("=== 高级控制流演示 ===");
        
        // 函数式编程风格
        println!("\n--- 函数式风格 ---");
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let even_squares = process_numbers(&numbers);
        println!("偶数的平方: {:?}", even_squares);
        
        // 错误传播
        println!("\n--- 错误传播 ---");
        match divide_and_add(10, 2, 5) {
            Ok(result) => println!("计算结果: {}", result),
            Err(error) => println!("计算错误: {}", error),
        }
        
        match divide_and_add(10, 0, 5) {
            Ok(result) => println!("计算结果: {}", result),
            Err(error) => println!("计算错误: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conditionals() {
        assert_eq!(conditionals::check_number(5), "正数");
        assert_eq!(conditionals::check_number(-3), "负数");
        assert_eq!(conditionals::check_number(0), "零");
        
        assert_eq!(conditionals::get_absolute_value(-10), 10);
        assert_eq!(conditionals::get_absolute_value(7), 7);
        
        assert_eq!(conditionals::categorize_age(10), "儿童");
        assert_eq!(conditionals::categorize_age(16), "青少年");
        assert_eq!(conditionals::categorize_age(30), "成年人");
        assert_eq!(conditionals::categorize_age(65), "老年人");
    }
    
    #[test]
    fn test_loops() {
        assert_eq!(loops::factorial(5), 120);
        assert_eq!(loops::factorial(0), 1);
        assert_eq!(loops::factorial(1), 1);
        
        let numbers = vec![1, 3, 5, 8, 9];
        assert_eq!(loops::find_first_even(&numbers), Some(8));
        
        let odd_numbers = vec![1, 3, 5, 7];
        assert_eq!(loops::find_first_even(&odd_numbers), None);
        
        let test_numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(loops::find_pair_sum(&test_numbers, 7), Some((2, 5)));
        assert_eq!(loops::find_pair_sum(&test_numbers, 10), None);
    }
    
    #[test]
    fn test_pattern_matching() {
        use pattern_matching::Color;
        
        assert_eq!(pattern_matching::describe_color(Color::Red), "纯红色");
        assert_eq!(pattern_matching::describe_color(Color::Rgb(255, 0, 0)), "RGB红色");
        
        assert_eq!(pattern_matching::describe_value(Some(42)), "有值: 42");
        assert_eq!(pattern_matching::describe_value::<i32>(None), "无值");
        
        let ok_result: Result<i32, &str> = Ok(42);
        assert_eq!(pattern_matching::handle_result(ok_result), "成功: 42");
        
        assert_eq!(pattern_matching::classify_number(5), "小数");
        assert_eq!(pattern_matching::classify_number(50), "中数");
        assert_eq!(pattern_matching::classify_number(-10), "负数");
        
        assert_eq!(pattern_matching::describe_pair((2, 4)), "都是偶数");
        assert_eq!(pattern_matching::describe_pair((1, 3)), "都是奇数");
        assert_eq!(pattern_matching::describe_pair((1, 2)), "混合");
    }
    
    #[test]
    fn test_advanced() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result = advanced::process_numbers(&numbers);
        assert_eq!(result, vec![4, 16, 36]);
        
        assert_eq!(advanced::divide_and_add(10, 2, 5), Ok(10));
        assert!(advanced::divide_and_add(10, 0, 5).is_err());
    }
    
    #[test]
    fn test_comprehensive_example() {
        // 综合测试：使用多种控制流结构
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut result = Vec::new();
        
        for num in numbers {
            let processed = match num {
                n if n % 2 == 0 => {
                    // 偶数：计算平方
                    if n > 5 {
                        Some(n * n)
                    } else {
                        Some(n * 2)
                    }
                },
                n if n % 3 == 0 => {
                    // 奇数且能被3整除：计算立方
                    Some(n * n * n)
                },
                _ => None,
            };
            
            if let Some(value) = processed {
                result.push(value);
            }
        }
        
        // 验证结果
        // 1: 无处理, 2: 4(2*2), 3: 27(3^3), 4: 8(4*2), 5: 无处理, 6: 36(6^2), 7: 无处理, 8: 64(8^2), 9: 729(9^3), 10: 100(10^2)
        assert_eq!(result, vec![4, 27, 8, 36, 64, 729, 100]);
    }
}
