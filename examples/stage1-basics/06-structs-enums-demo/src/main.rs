//! 结构体和枚举综合示例
//! 
//! 本示例展示了 Rust 中结构体和枚举的各种用法，包括：
//! - 基本结构体定义和使用
//! - 方法和关联函数
//! - 枚举定义和模式匹配
//! - Option 和 Result 的使用
//! - 高级特性如生命周期和泛型

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 基本结构体定义
// ============================================================================

/// 用户信息结构体
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    username: String,
    email: String,
    active: bool,
}

impl User {
    /// 创建新用户（关联函数）
    fn new(id: u32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            active: true,
        }
    }
    
    /// 验证邮箱格式（关联函数）
    fn with_validated_email(id: u32, username: String, email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(User::new(id, username, email))
        } else {
            Err(format!("无效的邮箱格式: {}", email))
        }
    }
    
    /// 获取用户显示名称（方法）
    fn display_name(&self) -> &str {
        &self.username
    }
    
    /// 更新邮箱（可变方法）
    fn update_email(&mut self, new_email: String) -> Result<(), String> {
        if new_email.contains('@') {
            self.email = new_email;
            Ok(())
        } else {
            Err("无效的邮箱格式".to_string())
        }
    }
    
    /// 停用用户（可变方法）
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    /// 检查用户是否活跃（方法）
    fn is_active(&self) -> bool {
        self.active
    }
}

// ============================================================================
// 元组结构体和单元结构体
// ============================================================================

/// RGB 颜色（元组结构体）
#[derive(Debug, Clone, Copy, PartialEq)]
struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }
    
    fn _red(&self) -> u8 { self.0 }
    fn _green(&self) -> u8 { self.1 }
    fn _blue(&self) -> u8 { self.2 }
    
    fn as_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// 单元结构体示例
#[derive(Debug, Clone, Copy, PartialEq)]
struct AlwaysEqual;

// ============================================================================
// 枚举定义
// ============================================================================

/// 消息类型枚举
#[derive(Debug, Clone, PartialEq)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    /// 处理消息（枚举方法）
    fn process(&self) {
        match self {
            Message::Quit => println!("收到退出消息"),
            Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => println!("写入消息: {}", text),
            Message::ChangeColor(color) => println!("改变颜色为: {}", color.as_hex()),
        }
    }
    
    /// 检查是否为退出消息
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

/// 用户状态枚举
#[derive(Debug, Clone, PartialEq)]
enum UserStatus {
    Active,
    Inactive,
    Suspended { reason: String, until: Option<String> },
    Banned,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatus::Active => write!(f, "活跃"),
            UserStatus::Inactive => write!(f, "非活跃"),
            UserStatus::Suspended { reason, until } => {
                match until {
                    Some(date) => write!(f, "暂停至 {} (原因: {})", date, reason),
                    None => write!(f, "永久暂停 (原因: {})", reason),
                }
            }
            UserStatus::Banned => write!(f, "已封禁"),
        }
    }
}

// ============================================================================
// 错误处理枚举
// ============================================================================

/// 自定义错误类型
#[derive(Debug, Clone, PartialEq)]
enum UserError {
    NotFound(u32),
    InvalidEmail(String),
    AlreadyExists(String),
    _PermissionDenied,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::NotFound(id) => write!(f, "用户 {} 不存在", id),
            UserError::InvalidEmail(email) => write!(f, "无效的邮箱: {}", email),
            UserError::AlreadyExists(username) => write!(f, "用户名 {} 已存在", username),
            UserError::_PermissionDenied => write!(f, "权限不足"),
        }
    }
}

impl std::error::Error for UserError {}

// ============================================================================
// 用户管理系统
// ============================================================================

/// 用户管理器
#[derive(Debug)]
struct UserManager {
    users: HashMap<u32, User>,
    user_status: HashMap<u32, UserStatus>,
    next_id: u32,
}

impl UserManager {
    fn new() -> Self {
        UserManager {
            users: HashMap::new(),
            user_status: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// 创建新用户
    fn create_user(&mut self, username: String, email: String) -> Result<u32, UserError> {
        // 检查用户名是否已存在
        if self.users.values().any(|u| u.username == username) {
            return Err(UserError::AlreadyExists(username));
        }
        
        // 验证邮箱
        let user = User::with_validated_email(self.next_id, username, email)
            .map_err(|_| UserError::InvalidEmail("邮箱格式无效".to_string()))?;
        
        let user_id = self.next_id;
        self.users.insert(user_id, user);
        self.user_status.insert(user_id, UserStatus::Active);
        self.next_id += 1;
        
        Ok(user_id)
    }
    
    /// 获取用户
    fn _get_user(&self, id: u32) -> Result<&User, UserError> {
        self.users.get(&id).ok_or(UserError::NotFound(id))
    }
    
    /// 获取用户状态
    fn _get_user_status(&self, id: u32) -> Result<&UserStatus, UserError> {
        self.user_status.get(&id).ok_or(UserError::NotFound(id))
    }
    
    /// 更新用户状态
    fn update_user_status(&mut self, id: u32, status: UserStatus) -> Result<(), UserError> {
        if self.users.contains_key(&id) {
            self.user_status.insert(id, status);
            Ok(())
        } else {
            Err(UserError::NotFound(id))
        }
    }
    
    /// 列出所有活跃用户
    fn list_active_users(&self) -> Vec<&User> {
        self.users
            .iter()
            .filter_map(|(id, user)| {
                match self.user_status.get(id) {
                    Some(UserStatus::Active) => Some(user),
                    _ => None,
                }
            })
            .collect()
    }
    
    /// 获取用户统计信息
    fn get_statistics(&self) -> UserStatistics {
        let mut stats = UserStatistics::default();
        
        for status in self.user_status.values() {
            match status {
                UserStatus::Active => stats.active += 1,
                UserStatus::Inactive => stats.inactive += 1,
                UserStatus::Suspended { .. } => stats.suspended += 1,
                UserStatus::Banned => stats.banned += 1,
            }
        }
        
        stats.total = self.users.len();
        stats
    }
}

/// 用户统计信息
#[derive(Debug, Default)]
struct UserStatistics {
    total: usize,
    active: usize,
    inactive: usize,
    suspended: usize,
    banned: usize,
}

impl fmt::Display for UserStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "用户统计: 总计 {}, 活跃 {}, 非活跃 {}, 暂停 {}, 封禁 {}",
            self.total, self.active, self.inactive, self.suspended, self.banned
        )
    }
}

// ============================================================================
// 高级特性示例
// ============================================================================

/// 带生命周期的结构体
#[derive(Debug)]
struct UserProfile<'a> {
    user: &'a User,
    bio: Option<&'a str>,
    avatar_url: Option<&'a str>,
}

impl<'a> UserProfile<'a> {
    fn new(user: &'a User) -> Self {
        UserProfile {
            user,
            bio: None,
            avatar_url: None,
        }
    }
    
    fn with_bio(mut self, bio: &'a str) -> Self {
        self.bio = Some(bio);
        self
    }
    
    fn with_avatar(mut self, avatar_url: &'a str) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }
    
    fn display(&self) {
        println!("用户档案:");
        println!("  用户名: {}", self.user.username);
        println!("  邮箱: {}", self.user.email);
        
        if let Some(bio) = self.bio {
            println!("  简介: {}", bio);
        }
        
        if let Some(avatar) = self.avatar_url {
            println!("  头像: {}", avatar);
        }
    }
}

/// 泛型结构体示例
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn _is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: fmt::Display> Container<T> {
    fn display_all(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("  [{}]: {}", i, item);
        }
    }
}

// ============================================================================
// 主函数和示例
// ============================================================================

fn main() {
    println!("=== 结构体和枚举综合示例 ===");
    
    // 基本结构体使用
    println!("\n1. 基本结构体使用:");
    demonstrate_basic_structs();
    
    // 枚举和模式匹配
    println!("\n2. 枚举和模式匹配:");
    demonstrate_enums_and_matching();
    
    // 用户管理系统
    println!("\n3. 用户管理系统:");
    demonstrate_user_management();
    
    // Option 和 Result 使用
    println!("\n4. Option 和 Result 使用:");
    demonstrate_option_result();
    
    // 高级特性
    println!("\n5. 高级特性:");
    demonstrate_advanced_features();
}

fn demonstrate_basic_structs() {
    // 创建用户
    let mut user = User::new(1, "alice".to_string(), "alice@example.com".to_string());
    println!("创建用户: {:?}", user);
    
    // 使用方法
    println!("用户显示名: {}", user.display_name());
    println!("用户是否活跃: {}", user.is_active());
    
    // 更新邮箱
    match user.update_email("alice.new@example.com".to_string()) {
        Ok(()) => println!("邮箱更新成功"),
        Err(e) => println!("邮箱更新失败: {}", e),
    }
    
    // 停用用户
    user.deactivate();
    println!("停用后用户状态: 活跃={}", user.is_active());
    
    // 元组结构体
    let red = Color::new(255, 0, 0);
    println!("红色: {:?}, 十六进制: {}", red, red.as_hex());
    
    // 单元结构体
    let _always_equal = AlwaysEqual;
    println!("单元结构体: {:?}", _always_equal);
}

fn demonstrate_enums_and_matching() {
    let messages = vec![
        Message::Write("Hello, World!".to_string()),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(Color::new(0, 255, 0)),
        Message::Quit,
    ];
    
    for message in &messages {
        message.process();
        
        // 使用 if let 进行简单匹配
        if let Message::Write(text) = message {
            println!("  -> 这是一条文本消息，长度: {}", text.len());
        }
        
        // 使用 matches! 宏
        if message.is_quit() {
            println!("  -> 这是退出消息");
        }
    }
    
    // 用户状态枚举
    let statuses = vec![
        UserStatus::Active,
        UserStatus::Inactive,
        UserStatus::Suspended {
            reason: "违规行为".to_string(),
            until: Some("2024-12-31".to_string()),
        },
        UserStatus::Banned,
    ];
    
    for status in &statuses {
        println!("用户状态: {}", status);
    }
}

fn demonstrate_user_management() {
    let mut manager = UserManager::new();
    
    // 创建用户
    let user_ids = [
        manager.create_user("alice".to_string(), "alice@example.com".to_string()),
        manager.create_user("bob".to_string(), "bob@example.com".to_string()),
        manager.create_user("charlie".to_string(), "invalid-email".to_string()),
    ];
    
    for (i, result) in user_ids.iter().enumerate() {
        match result {
            Ok(id) => println!("用户 {} 创建成功，ID: {}", i + 1, id),
            Err(e) => println!("用户 {} 创建失败: {}", i + 1, e),
        }
    }
    
    // 更新用户状态
    if let Ok(user_id) = user_ids[1] {
        let _ = manager.update_user_status(
            user_id,
            UserStatus::Suspended {
                reason: "测试暂停".to_string(),
                until: Some("2024-06-01".to_string()),
            },
        );
    }
    
    // 显示统计信息
    println!("\n{}", manager.get_statistics());
    
    // 列出活跃用户
    println!("\n活跃用户:");
    for user in manager.list_active_users() {
        println!("  - {} ({})", user.username, user.email);
    }
}

fn demonstrate_option_result() {
    // Option 示例
    let numbers = [1, 2, 3, 4, 5];
    
    // 安全的索引访问
    match numbers.get(2) {
        Some(value) => println!("索引 2 的值: {}", value),
        None => println!("索引 2 不存在"),
    }
    
    // 使用 map 和 unwrap_or
    let doubled = numbers.get(10).map(|x| x * 2).unwrap_or(0);
    println!("索引 10 的值翻倍（默认0）: {}", doubled);
    
    // Result 示例
    let parse_results = vec!["42", "abc", "123"];
    
    for s in parse_results {
        match s.parse::<i32>() {
            Ok(num) => println!("解析 '{}' 成功: {}", s, num),
            Err(e) => println!("解析 '{}' 失败: {}", s, e),
        }
    }
    
    // 链式错误处理
    let result = "42"
        .parse::<i32>()
        .map(|x| x * 2)
        .map(|x| x + 10);
    
    println!("链式处理结果: {:?}", result);
}

fn demonstrate_advanced_features() {
    // 生命周期示例
    let user = User::new(1, "alice".to_string(), "alice@example.com".to_string());
    let bio = "Rust 开发者";
    let avatar = "https://example.com/avatar.jpg";
    
    let profile = UserProfile::new(&user)
        .with_bio(bio)
        .with_avatar(avatar);
    
    profile.display();
    
    // 泛型容器示例
    let mut string_container = Container::<String>::new();
    string_container.add("Hello".to_string());
    string_container.add("World".to_string());
    string_container.add("Rust".to_string());
    
    println!("\n字符串容器 (长度: {}):", string_container.len());
    string_container.display_all();
    
    let mut number_container = Container::<i32>::new();
    number_container.add(1);
    number_container.add(2);
    number_container.add(3);
    
    println!("\n数字容器 (长度: {}):", number_container.len());
    number_container.display_all();
    
    // 获取特定元素
    if let Some(first) = string_container.get(0) {
        println!("\n第一个字符串: {}", first);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "test".to_string(), "test@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "test");
        assert_eq!(user.email, "test@example.com");
        assert!(user.active);
    }
    
    #[test]
    fn test_user_validation() {
        let result = User::with_validated_email(
            1,
            "test".to_string(),
            "invalid-email".to_string(),
        );
        assert!(result.is_err());
        
        let result = User::with_validated_email(
            1,
            "test".to_string(),
            "test@example.com".to_string(),
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_color() {
        let red = Color::new(255, 0, 0);
        assert_eq!(red._red(), 255);
        assert_eq!(red._green(), 0);
        assert_eq!(red._blue(), 0);
        assert_eq!(red.as_hex(), "#FF0000");
    }
    
    #[test]
    fn test_message_processing() {
        let message = Message::Write("test".to_string());
        assert!(!message.is_quit());
        
        let quit_message = Message::Quit;
        assert!(quit_message.is_quit());
    }
    
    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new();
        
        let user_id = manager
            .create_user("test".to_string(), "test@example.com".to_string())
            .unwrap();
        
        assert!(manager._get_user(user_id).is_ok());
        assert!(manager._get_user(999).is_err());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.active, 1);
    }
    
    #[test]
    fn test_container() {
        let mut container = Container::<i32>::new();
        assert!(container._is_empty());
        
        container.add(42);
        assert_eq!(container.len(), 1);
        assert_eq!(container.get(0), Some(&42));
        assert_eq!(container.get(1), None);
    }
}
