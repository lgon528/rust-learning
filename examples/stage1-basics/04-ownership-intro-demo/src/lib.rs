//! # Ownership Introduction Demo - Rust 所有权入门演示
//!
//! 本库演示了 Rust 中所有权系统的基础概念，包括：
//! - 所有权规则和移动语义
//! - 借用和引用
//! - 可变性和不可变性
//! - 生命周期基础
//!
//! # 示例
//!
//! ```
//! use ownership_intro_demo::ownership::*;
//! use ownership_intro_demo::borrowing::*;
//!
//! // 所有权转移示例
//! let s1 = String::from("hello");
//! let s2 = take_ownership(s1);
//! // s1 在这里不再有效
//!
//! // 借用示例
//! let s = String::from("world");
//! let len = calculate_length(&s);
//! // s 在这里仍然有效
//! ```

/// 所有权基础概念模块
pub mod ownership {
    /// 演示所有权转移
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::ownership::take_ownership;
    /// 
    /// let s = String::from("hello");
    /// let result = take_ownership(s);
    /// // s 在这里不再有效，所有权已转移
    /// assert_eq!(result, "hello");
    /// ```
    pub fn take_ownership(s: String) -> String {
        println!("接收到字符串: {}", s);
        s // 返回所有权
    }
    
    /// 演示复制语义（Copy trait）
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::ownership::copy_value;
    /// 
    /// let x = 5;
    /// let result = copy_value(x);
    /// // x 在这里仍然有效，因为 i32 实现了 Copy
    /// assert_eq!(x, 5);
    /// assert_eq!(result, 10);
    /// ```
    pub fn copy_value(x: i32) -> i32 {
        println!("接收到数值: {}", x);
        x * 2
    }
    
    /// 演示克隆操作
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::ownership::clone_string;
    /// 
    /// let s1 = String::from("hello");
    /// let s2 = clone_string(&s1);
    /// // s1 仍然有效
    /// assert_eq!(s1, "hello");
    /// assert_eq!(s2, "hello");
    /// ```
    pub fn clone_string(s: &String) -> String {
        s.clone()
    }
    
    /// 演示所有权在函数间的传递
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::ownership::gives_ownership;
    /// 
    /// let s = gives_ownership();
    /// assert_eq!(s, "yours");
    /// ```
    pub fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string // 移动所有权给调用者
    }
    
    /// 演示获取并返回所有权
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::ownership::takes_and_gives_back;
    /// 
    /// let s1 = String::from("hello");
    /// let s2 = takes_and_gives_back(s1);
    /// // s1 不再有效，s2 现在拥有所有权
    /// assert_eq!(s2, "hello");
    /// ```
    pub fn takes_and_gives_back(a_string: String) -> String {
        a_string // 返回所有权
    }
}

/// 借用和引用模块
pub mod borrowing {
    /// 计算字符串长度（不可变借用）
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::borrowing::calculate_length;
    /// 
    /// let s = String::from("hello");
    /// let len = calculate_length(&s);
    /// // s 在这里仍然有效
    /// assert_eq!(len, 5);
    /// assert_eq!(s, "hello");
    /// ```
    pub fn calculate_length(s: &String) -> usize {
        s.len()
    }
    
    /// 尝试修改借用的值（编译错误示例）
    /// 注意：这个函数展示了为什么不能修改不可变引用
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::borrowing::cannot_change;
    /// 
    /// let s = String::from("hello");
    /// cannot_change(&s);
    /// assert_eq!(s, "hello"); // s 未被修改
    /// ```
    pub fn cannot_change(some_string: &String) {
        // some_string.push_str(", world"); // 这会导致编译错误
        println!("不能修改借用的字符串: {}", some_string);
    }
    
    /// 演示多个不可变引用
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::borrowing::multiple_immutable_refs;
    /// 
    /// let s = String::from("hello");
    /// let result = multiple_immutable_refs(&s, &s);
    /// assert_eq!(result, 10); // "hello" 长度是 5，5 + 5 = 10
    /// ```
    pub fn multiple_immutable_refs(s1: &String, s2: &String) -> usize {
        s1.len() + s2.len()
    }
    
    /// 演示字符串切片
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::borrowing::first_word;
    /// 
    /// let s = String::from("hello world");
    /// let word = first_word(&s);
    /// assert_eq!(word, "hello");
    /// 
    /// // 也可以直接使用字符串字面量
    /// let word2 = first_word("rust programming");
    /// assert_eq!(word2, "rust");
    /// ```
    pub fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
}

/// 可变性和可变借用模块
pub mod mutability {
    /// 修改字符串（可变借用）
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::mutability::change_string;
    /// 
    /// let mut s = String::from("hello");
    /// change_string(&mut s);
    /// assert_eq!(s, "hello, world!");
    /// ```
    pub fn change_string(some_string: &mut String) {
        some_string.push_str(", world!");
    }
    
    /// 演示可变引用的限制
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::mutability::mutable_reference_rules;
    /// 
    /// let mut s = String::from("hello");
    /// let result = mutable_reference_rules(&mut s);
    /// assert_eq!(result, "hello, modified!");
    /// ```
    pub fn mutable_reference_rules(s: &mut String) -> String {
        s.push_str(", modified!");
        s.clone()
    }
    
    /// 演示作用域和借用规则
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::mutability::scope_and_borrowing;
    /// 
    /// let mut s = String::from("hello");
    /// let result = scope_and_borrowing(&mut s);
    /// assert_eq!(result, 12); // "hello, world" 的长度
    /// ```
    pub fn scope_and_borrowing(s: &mut String) -> usize {
        {
            let r1 = &*s; // 不可变引用
            println!("r1: {}", r1);
        } // r1 在这里离开作用域
        
        s.push_str(", world"); // 现在可以创建可变引用
        s.len()
    }
    
    /// 演示悬垂引用的预防
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::mutability::no_dangling_reference;
    /// 
    /// let result = no_dangling_reference();
    /// assert_eq!(result, "hello");
    /// ```
    pub fn no_dangling_reference() -> String {
        let s = String::from("hello");
        s // 返回所有权而不是引用
    }
}

/// 生命周期基础模块
pub mod lifetimes {
    /// 演示生命周期注解
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::lifetimes::longest;
    /// 
    /// let string1 = String::from("abcd");
    /// let string2 = "xyz";
    /// let result = longest(string1.as_str(), string2);
    /// assert_eq!(result, "abcd");
    /// ```
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    /// 演示结构体中的生命周期
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::lifetimes::ImportantExcerpt;
    /// 
    /// let novel = String::from("Call me Ishmael. Some years ago...");
    /// let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    /// let excerpt = ImportantExcerpt {
    ///     part: first_sentence,
    /// };
    /// assert_eq!(excerpt.part, "Call me Ishmael");
    /// ```
    pub struct ImportantExcerpt<'a> {
        pub part: &'a str,
    }
    
    impl<'a> ImportantExcerpt<'a> {
        /// 获取摘录内容的长度
        /// 
        /// # 示例
        /// 
        /// ```
        /// use ownership_intro_demo::lifetimes::ImportantExcerpt;
        /// 
        /// let novel = String::from("Call me Ishmael.");
        /// let excerpt = ImportantExcerpt { part: &novel };
        /// assert_eq!(excerpt.level(), 16);
        /// ```
        pub fn level(&self) -> usize {
            self.part.len()
        }
        
        /// 返回摘录的第一个单词
        /// 
        /// # 示例
        /// 
        /// ```
        /// use ownership_intro_demo::lifetimes::ImportantExcerpt;
        /// 
        /// let text = "Hello world";
        /// let excerpt = ImportantExcerpt { part: text };
        /// assert_eq!(excerpt.first_word(), "Hello");
        /// ```
        pub fn first_word(&self) -> &str {
            let bytes = self.part.as_bytes();
            
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &self.part[0..i];
                }
            }
            
            self.part
        }
    }
    
    /// 演示静态生命周期
    /// 
    /// # 示例
    /// 
    /// ```
    /// use ownership_intro_demo::lifetimes::get_static_str;
    /// 
    /// let s = get_static_str();
    /// assert_eq!(s, "I have a static lifetime.");
    /// ```
    pub fn get_static_str() -> &'static str {
        "I have a static lifetime."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ownership() {
        // 测试所有权转移
        let s1 = String::from("hello");
        let s2 = ownership::take_ownership(s1);
        assert_eq!(s2, "hello");
        
        // 测试复制语义
        let x = 5;
        let y = ownership::copy_value(x);
        assert_eq!(x, 5); // x 仍然有效
        assert_eq!(y, 10);
        
        // 测试克隆
        let s1 = String::from("test");
        let s2 = ownership::clone_string(&s1);
        assert_eq!(s1, "test"); // s1 仍然有效
        assert_eq!(s2, "test");
    }
    
    #[test]
    fn test_borrowing() {
        // 测试不可变借用
        let s = String::from("hello");
        let len = borrowing::calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s 仍然有效
        
        // 测试字符串切片
        let s = String::from("hello world");
        let word = borrowing::first_word(&s);
        assert_eq!(word, "hello");
        
        // 测试字符串字面量
        let word = borrowing::first_word("rust programming");
        assert_eq!(word, "rust");
    }
    
    #[test]
    fn test_mutability() {
        // 测试可变借用
        let mut s = String::from("hello");
        mutability::change_string(&mut s);
        assert_eq!(s, "hello, world!");
        
        // 测试作用域和借用
        let mut s = String::from("hello");
        let len = mutability::scope_and_borrowing(&mut s);
        assert_eq!(len, 12);
        assert_eq!(s, "hello, world");
    }
    
    #[test]
    fn test_lifetimes() {
        // 测试生命周期函数
        let string1 = String::from("long string is long");
        let string2 = "xyz";
        let result = lifetimes::longest(string1.as_str(), string2);
        assert_eq!(result, "long string is long");
        
        // 测试结构体生命周期
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = lifetimes::ImportantExcerpt {
            part: first_sentence,
        };
        assert_eq!(excerpt.level(), 15);
        assert_eq!(excerpt.first_word(), "Call");
        
        // 测试静态生命周期
        let s = lifetimes::get_static_str();
        assert_eq!(s, "I have a static lifetime.");
    }
    
    #[test]
    fn test_comprehensive_example() {
        // 综合测试：演示所有权系统的各个方面
        let mut data = String::from("Rust");
        
        // 1. 不可变借用
        let len = borrowing::calculate_length(&data);
        assert_eq!(len, 4);
        
        // 2. 可变借用
        mutability::change_string(&mut data);
        assert_eq!(data, "Rust, world!");
        
        // 3. 字符串切片
        let first = borrowing::first_word(&data);
        assert_eq!(first, "Rust,");
        
        // 4. 所有权转移
        let data2 = ownership::takes_and_gives_back(data);
        // data 在这里不再有效
        assert_eq!(data2, "Rust, world!");
        
        // 5. 生命周期
        let result = lifetimes::longest(&data2, "short");
        assert_eq!(result, "Rust, world!");
    }
}
