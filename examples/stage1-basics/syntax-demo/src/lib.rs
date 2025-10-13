//! # Rust 语法基础演示库
//!
//! 这个库演示了 Rust 语法基础的各种特性，包括：
//! - 变量和可变性
//! - 数据类型
//! - 函数定义和调用
//! - 注释和文档
//!
//! ## 使用示例
//!
//! ```rust
//! use syntax_demo::variables::demonstrate_variables;
//! use syntax_demo::functions::greet;
//!
//! // 演示变量使用
//! demonstrate_variables();
//!
//! // 演示函数调用
//! let message = greet("Rust");
//! println!("{}", message);
//! ```

/// 变量和可变性演示模块
pub mod variables {
    /// 演示变量的基本使用
    pub fn demonstrate_variables() {
        println!("=== 变量演示 ===");
        
        // 不可变变量
        let x = 5;
        println!("不可变变量 x = {}", x);
        
        // 可变变量
        let mut y = 10;
        println!("可变变量 y = {}", y);
        y = 15;
        println!("修改后 y = {}", y);
        
        // 变量遮蔽
        let z = 20;
        println!("第一个 z = {}", z);
        let z = z + 5;
        println!("遮蔽后 z = {}", z);
        let z = "现在是字符串";
        println!("再次遮蔽 z = {}", z);
    }
    
    /// 演示常量的使用
    pub fn demonstrate_constants() {
        const MAX_POINTS: u32 = 100_000;
        println!("常量 MAX_POINTS = {}", MAX_POINTS);
    }
}

/// 数据类型演示模块
pub mod data_types {
    /// 演示标量类型
    pub fn demonstrate_scalar_types() {
        println!("=== 标量类型演示 ===");
        
        // 整数类型
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        
        println!("十进制: {}", decimal);
        println!("十六进制: {}", hex);
        println!("八进制: {}", octal);
        println!("二进制: {}", binary);
        println!("字节: {}", byte);
        
        // 浮点类型
        let f1 = 2.0; // f64
        let f2: f32 = 3.0; // f32
        println!("f64: {}, f32: {}", f1, f2);
        
        // 布尔类型
        let t = true;
        let f: bool = false;
        println!("布尔值: {}, {}", t, f);
        
        // 字符类型
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("字符: {}, {}, {}", c, z, heart_eyed_cat);
    }
    
    /// 演示复合类型
    pub fn demonstrate_compound_types() {
        println!("=== 复合类型演示 ===");
        
        // 元组类型
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup; // 解构
        println!("元组解构: x={}, y={}, z={}", x, y, z);
        
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;
        println!("元组索引: {}, {}, {}", five_hundred, six_point_four, one);
        
        // 数组类型
        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June",
                     "July", "August", "September", "October", "November", "December"];
        
        println!("数组第一个元素: {}", a[0]);
        println!("月份数组长度: {}", months.len());
        
        // 指定类型和长度的数组
        let _a: [i32; 5] = [1, 2, 3, 4, 5];
        let a = [3; 5]; // [3, 3, 3, 3, 3]
        println!("重复元素数组: {:?}", a);
    }
    
    /// 演示类型转换
    pub fn demonstrate_type_conversion() {
        println!("=== 类型转换演示 ===");
        
        let x = 10u8;
        let y = x as u16;
        println!("u8 {} 转换为 u16 {}", x, y);
        
        let a = 3.14f64;
        let b = a as i32;
        println!("f64 {} 转换为 i32 {}", a, b);
    }
}

/// 函数演示模块
pub mod functions {
    /// 简单的问候函数
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    /// 演示函数的各种特性
    pub fn demonstrate_functions() {
        println!("=== 函数演示 ===");
        
        // 无参数函数
        say_hello();
        
        // 有参数函数
        let result = add(5, 3);
        println!("5 + 3 = {}", result);
        
        // 多个参数
        print_labeled_measurement(5, 'h');
        
        // 表达式和语句
        let y = {
            let x = 3;
            x + 1  // 表达式，没有分号
        };
        println!("表达式结果: {}", y);
        
        // 提前返回
        let result = early_return(10);
        println!("提前返回结果: {}", result);
    }
    
    /// 无参数函数
    fn say_hello() {
        println!("Hello, world!");
    }
    
    /// 有参数和返回值的函数
    fn add(x: i32, y: i32) -> i32 {
        x + y  // 表达式返回
    }
    
    /// 多个参数的函数
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {}{}", value, unit_label);
    }
    
    /// 演示提前返回
    fn early_return(x: i32) -> i32 {
        if x > 5 {
            return x * 2;  // 提前返回
        }
        x + 1
    }
    
    /// 演示函数指针
    pub fn demonstrate_function_pointers() {
        println!("=== 函数指针演示 ===");
        
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        
        let answer = do_twice(add_one, 5);
        println!("do_twice(add_one, 5) = {}", answer);
    }
}

/// 注释和文档演示模块
/// 
/// 这个模块演示了 Rust 中各种注释类型的使用
pub mod comments {
    /*!
     * 这是内部文档注释
     * 用于模块级别的文档
     */
    
    /// 这是一个文档注释
    /// 
    /// # 示例
    /// 
    /// ```
    /// use syntax_demo::comments::documented_function;
    /// let result = documented_function(42);
    /// assert_eq!(result, 84);
    /// ```
    /// 
    /// # 参数
    /// 
    /// * `x` - 输入的整数
    /// 
    /// # 返回值
    /// 
    /// 返回输入值的两倍
    pub fn documented_function(x: i32) -> i32 {
        // 这是行注释
        x * 2
    }
    
    /// 演示各种注释类型
    pub fn demonstrate_comments() {
        println!("=== 注释演示 ===");
        
        // 单行注释
        let x = 5; // 行尾注释
        
        /*
         * 多行注释
         * 可以跨越多行
         */
        let y = 10;
        
        println!("x = {}, y = {}", x, y);
    }
    
    /// 包含代码示例的文档
    /// 
    /// ```
    /// # use syntax_demo::comments::example_with_code;
    /// let result = example_with_code();
    /// println!("结果: {}", result);
    /// ```
    pub fn example_with_code() -> &'static str {
        "这是一个示例"
    }
}

/// 综合演示模块
pub mod comprehensive {
    use super::*;
    
    /// 综合演示所有语法特性
    pub fn run_all_demos() {
        println!("🦀 Rust 语法基础综合演示 🦀\n");
        
        // 变量演示
        variables::demonstrate_variables();
        println!();
        
        variables::demonstrate_constants();
        println!();
        
        // 数据类型演示
        data_types::demonstrate_scalar_types();
        println!();
        
        data_types::demonstrate_compound_types();
        println!();
        
        data_types::demonstrate_type_conversion();
        println!();
        
        // 函数演示
        functions::demonstrate_functions();
        println!();
        
        functions::demonstrate_function_pointers();
        println!();
        
        // 注释演示
        comments::demonstrate_comments();
        println!();
        
        println!("✅ 所有演示完成！");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greet_function() {
        let result = functions::greet("Rust");
        assert_eq!(result, "Hello, Rust!");
    }
    
    #[test]
    fn test_documented_function() {
        let result = comments::documented_function(21);
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_example_with_code() {
        let result = comments::example_with_code();
        assert_eq!(result, "这是一个示例");
    }
}
