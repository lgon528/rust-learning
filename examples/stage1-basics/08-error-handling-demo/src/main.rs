//! Rust 错误处理示例
//! 
//! 本示例展示了 Rust 中的各种错误处理技术：
//! - panic! 和不可恢复错误
//! - Result 类型和可恢复错误
//! - 自定义错误类型
//! - 错误传播和 ? 操作符
//! - 高级错误处理技巧

use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;

// ============================================================================
// 1. 自定义错误类型
// ============================================================================

/// 自定义错误枚举
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    InvalidInput(String),
    NotFound(String),
}

// 实现 Display trait
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO 错误: {}", err),
            MyError::Parse(err) => write!(f, "解析错误: {}", err),
            MyError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
            MyError::NotFound(msg) => write!(f, "未找到: {}", msg),
        }
    }
}

// 实现 Error trait
impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(err) => Some(err),
            MyError::Parse(err) => Some(err),
            _ => None,
        }
    }
}

// 实现 From trait 用于错误转换
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// ============================================================================
// 2. 文件操作示例
// ============================================================================

/// 读取文件内容
fn read_file_content(filename: &str) -> Result<String, MyError> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// 写入文件内容
fn write_file_content(filename: &str, content: &str) -> Result<(), MyError> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 处理配置文件
fn process_config_file(filename: &str) -> Result<HashMap<String, i32>, MyError> {
    let content = read_file_content(filename)?;
    let mut config = HashMap::new();
    
    for line in content.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(MyError::InvalidInput(
                format!("无效的配置行: {}", line)
            ));
        }
        
        let key = parts[0].trim().to_string();
        let value: i32 = parts[1].trim().parse()?;
        config.insert(key, value);
    }
    
    Ok(config)
}

// ============================================================================
// 3. 数学运算示例
// ============================================================================

/// 安全除法
fn safe_divide(a: f64, b: f64) -> Result<f64, MyError> {
    if b == 0.0 {
        Err(MyError::InvalidInput("除数不能为零".to_string()))
    } else {
        Ok(a / b)
    }
}

/// 计算平方根
fn safe_sqrt(x: f64) -> Result<f64, MyError> {
    if x < 0.0 {
        Err(MyError::InvalidInput("不能计算负数的平方根".to_string()))
    } else {
        Ok(x.sqrt())
    }
}

/// 复合数学运算
fn complex_calculation(a: f64, b: f64, c: f64) -> Result<f64, MyError> {
    let division_result = safe_divide(a, b)?;
    let sqrt_result = safe_sqrt(division_result + c)?;
    Ok(sqrt_result)
}

// ============================================================================
// 4. 用户管理系统示例
// ============================================================================

#[derive(Debug, Clone)]
struct User {
    _id: u32,
    _name: String,
    _email: String,
    _age: u8,
}

struct UserManager {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserManager {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// 添加用户
    fn add_user(&mut self, name: String, email: String, age: u8) -> Result<u32, MyError> {
        if name.trim().is_empty() {
            return Err(MyError::InvalidInput("用户名不能为空".to_string()));
        }
        
        if !email.contains('@') {
            return Err(MyError::InvalidInput("无效的邮箱格式".to_string()));
        }
        
        if age > 120 {
            return Err(MyError::InvalidInput("年龄不能超过120岁".to_string()));
        }
        
        let user = User {
            _id: self.next_id,
            _name: name,
            _email: email,
            _age: age,
        };
        
        self.users.insert(self.next_id, user);
        let user_id = self.next_id;
        self.next_id += 1;
        
        Ok(user_id)
    }
    
    /// 获取用户
    fn get_user(&self, id: u32) -> Result<&User, MyError> {
        self.users.get(&id)
            .ok_or_else(|| MyError::NotFound(format!("用户ID {} 不存在", id)))
    }
    
    /// 更新用户邮箱
    fn _update_email(&mut self, id: u32, new_email: String) -> Result<(), MyError> {
        if !new_email.contains('@') {
            return Err(MyError::InvalidInput("无效的邮箱格式".to_string()));
        }
        
        let user = self.users.get_mut(&id)
            .ok_or_else(|| MyError::NotFound(format!("用户ID {} 不存在", id)))?;
        
        user._email = new_email;
        Ok(())
    }
    
    /// 删除用户
    fn _delete_user(&mut self, id: u32) -> Result<User, MyError> {
        self.users.remove(&id)
            .ok_or_else(|| MyError::NotFound(format!("用户ID {} 不存在", id)))
    }
}

// ============================================================================
// 5. panic! 示例
// ============================================================================

/// 演示 panic! 的使用
fn demonstrate_panic() {
    println!("\n=== Panic 示例 ===");
    
    // 条件性 panic
    let should_panic = false;
    if should_panic {
        panic!("这是一个演示性的 panic!");
    }
    
    // 使用 assert! 宏
    let x = 5;
    assert!(x > 0, "x 必须大于 0，但得到了 {}", x);
    
    // 使用 assert_eq! 宏
    let expected = 10;
    let actual = 5 + 5;
    assert_eq!(expected, actual, "期望 {} 但得到了 {}", expected, actual);
    
    println!("所有断言都通过了！");
}

/// 演示可能导致 panic 的操作
fn risky_operations() {
    println!("\n=== 风险操作示例 ===");
    
    // 安全的数组访问
    let arr = [1, 2, 3, 4, 5];
    let index = 2;
    
    if index < arr.len() {
        println!("arr[{}] = {}", index, arr[index]);
    } else {
        println!("索引 {} 超出数组范围", index);
    }
    
    // 使用 get 方法进行安全访问
    match arr.get(10) {
        Some(value) => println!("找到值: {}", value),
        None => println!("索引超出范围"),
    }
}

// ============================================================================
// 6. 主函数和示例运行
// ============================================================================

fn main() {
    println!("Rust 错误处理示例程序");
    println!("======================\n");
    
    // 1. 演示 panic 和断言
    demonstrate_panic();
    risky_operations();
    
    // 2. 演示数学运算错误处理
    println!("\n=== 数学运算错误处理 ===");
    
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match complex_calculation(16.0, 4.0, 9.0) {
        Ok(result) => println!("复合计算结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // 3. 演示用户管理系统
    println!("\n=== 用户管理系统 ===");
    
    let mut user_manager = UserManager::new();
    
    // 添加用户
    match user_manager.add_user(
        "张三".to_string(),
        "zhangsan@example.com".to_string(),
        25
    ) {
        Ok(id) => println!("成功添加用户，ID: {}", id),
        Err(e) => println!("添加用户失败: {}", e),
    }
    
    // 添加无效用户
    match user_manager.add_user(
        "".to_string(),
        "invalid-email".to_string(),
        150
    ) {
        Ok(id) => println!("成功添加用户，ID: {}", id),
        Err(e) => println!("添加用户失败: {}", e),
    }
    
    // 获取用户
    match user_manager.get_user(1) {
        Ok(user) => println!("找到用户: {:?}", user),
        Err(e) => println!("获取用户失败: {}", e),
    }
    
    // 获取不存在的用户
    match user_manager.get_user(999) {
        Ok(user) => println!("找到用户: {:?}", user),
        Err(e) => println!("获取用户失败: {}", e),
    }
    
    // 4. 演示文件操作（创建临时文件）
    println!("\n=== 文件操作示例 ===");
    
    let temp_file = "temp_config.txt";
    let config_content = "# 配置文件\nmax_connections=100\ntimeout=30\ndebug=1";
    
    // 写入文件
    match write_file_content(temp_file, config_content) {
        Ok(()) => println!("成功写入配置文件"),
        Err(e) => println!("写入文件失败: {}", e),
    }
    
    // 读取并解析配置文件
    match process_config_file(temp_file) {
        Ok(config) => {
            println!("成功解析配置文件:");
            for (key, value) in config {
                println!("  {} = {}", key, value);
            }
        },
        Err(e) => println!("处理配置文件失败: {}", e),
    }
    
    // 清理临时文件
    if let Err(e) = std::fs::remove_file(temp_file) {
        println!("清理临时文件失败: {}", e);
    }
    
    // 5. 演示错误链
    println!("\n=== 错误链示例 ===");
    
    match process_config_file("nonexistent.txt") {
        Ok(_) => println!("成功处理文件"),
        Err(e) => {
            println!("错误: {}", e);
            
            // 打印错误链
            let mut source = e.source();
            while let Some(err) = source {
                println!("  原因: {}", err);
                source = err.source();
            }
        }
    }
    
    println!("\n程序执行完成！");
}

// ============================================================================
// 7. 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_divide() {
        assert!(safe_divide(10.0, 2.0).is_ok());
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(safe_divide(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_safe_sqrt() {
        assert!(safe_sqrt(9.0).is_ok());
        assert_eq!(safe_sqrt(9.0).unwrap(), 3.0);
        assert!(safe_sqrt(-1.0).is_err());
    }
    
    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new();
        
        // 测试添加有效用户
        let id = manager.add_user(
            "测试用户".to_string(),
            "test@example.com".to_string(),
            25
        ).unwrap();
        assert_eq!(id, 1);
        
        // 测试获取用户
        let user = manager.get_user(id).unwrap();
        assert_eq!(user._name, "测试用户");
        
        // 测试添加无效用户
        assert!(manager.add_user(
            "".to_string(),
            "invalid".to_string(),
            200
        ).is_err());
    }
    
    #[test]
    fn test_error_conversion() {
        let parse_error: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let my_error: MyError = parse_error.into();
        
        match my_error {
            MyError::Parse(_) => {}, // 期望的结果
            _ => panic!("错误转换失败"),
        }
    }
    
    #[test]
    #[should_panic(expected = "测试 panic")]
    fn test_panic() {
        panic!("测试 panic");
    }
    
    #[test]
    fn test_complex_calculation() {
        // 测试正常情况
        let result = complex_calculation(16.0, 4.0, 9.0).unwrap();
        assert_eq!(result, 3.605551275463989); // sqrt(16/4 + 9) = sqrt(4 + 9) = sqrt(13)
        
        // 测试除零错误
        assert!(complex_calculation(16.0, 0.0, 9.0).is_err());
        
        // 测试负数平方根错误
        assert!(complex_calculation(1.0, 4.0, -10.0).is_err());
    }
}
