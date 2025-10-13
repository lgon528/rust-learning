# 模块系统基础

## 学习目标

通过本节学习，你将掌握：

- 理解 Rust 模块系统的基本概念
- 掌握模块的定义和使用方法
- 学会使用 `mod`、`use`、`pub` 关键字
- 理解模块的可见性规则
- 掌握文件系统与模块的对应关系

## 模块系统概述

### 什么是模块

Rust 的模块系统用于组织代码，提供以下功能：

- **命名空间管理**：避免命名冲突
- **封装控制**：控制代码的可见性
- **代码组织**：将相关功能组织在一起
- **重用机制**：便于代码复用和维护

### 模块系统的层次结构

```
crate (包根)
├── main.rs 或 lib.rs
├── mod module_a
│   ├── pub fn function_a()
│   └── mod sub_module
│       └── pub struct Data
└── mod module_b
    └── pub enum Status
```

## 模块定义

### 内联模块定义

```rust
// 在同一文件中定义模块
mod network {
    pub fn connect() {
        println!("连接到网络");
    }
    
    fn internal_setup() {
        println!("内部设置");
    }
    
    pub mod client {
        pub fn send_request() {
            println!("发送请求");
        }
    }
}

fn main() {
    network::connect();
    network::client::send_request();
    // network::internal_setup(); // 错误：私有函数不可访问
}
```

### 文件模块定义

**项目结构：**
```
src/
├── main.rs
├── network.rs
└── network/
    ├── mod.rs
    └── client.rs
```

**main.rs：**
```rust
mod network;  // 声明模块

fn main() {
    network::connect();
    network::client::send_request();
}
```

**network.rs：**
```rust
pub mod client;  // 声明子模块

pub fn connect() {
    println!("连接到网络");
}

fn internal_setup() {
    println!("内部设置");
}
```

**network/client.rs：**
```rust
pub fn send_request() {
    println!("发送请求");
}

pub fn receive_response() {
    println!("接收响应");
}
```

## 可见性控制

### pub 关键字

```rust
mod restaurant {
    pub mod front_of_house {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}  // 私有函数
        
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,      // 公有字段
            seasonal_fruit: String, // 私有字段
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("桃子"),
                }
            }
        }
        
        pub enum Appetizer {
            Soup,    // 枚举变体默认公有
            Salad,
        }
    }
}

fn main() {
    // 绝对路径
    crate::restaurant::front_of_house::add_to_waitlist();
    
    // 相对路径
    restaurant::front_of_house::hosting::add_to_waitlist();
    
    // 使用结构体
    let mut meal = restaurant::back_of_house::Breakfast::summer("黑麦");
    meal.toast = String::from("小麦");
    // meal.seasonal_fruit = String::from("蓝莓"); // 错误：私有字段
    
    // 使用枚举
    let order1 = restaurant::back_of_house::Appetizer::Soup;
    let order2 = restaurant::back_of_house::Appetizer::Salad;
}
```

### pub(crate) 和其他可见性修饰符

```rust
mod outer_mod {
    pub(crate) fn crate_visible() {}     // 整个 crate 可见
    pub(super) fn parent_visible() {}    // 父模块可见
    pub(self) fn module_visible() {}     // 当前模块可见（等同于私有）
    
    mod inner_mod {
        pub(in crate::outer_mod) fn restricted_visible() {} // 指定路径可见
        
        pub fn test() {
            super::parent_visible();  // 可以访问
            crate::outer_mod::crate_visible();  // 可以访问
        }
    }
}
```

## use 关键字

### 基本使用

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 引入函数的父模块（推荐）
use crate::front_of_house::hosting;

// 引入结构体、枚举、其他项（推荐）
use std::collections::HashMap;

fn main() {
    hosting::add_to_waitlist();
    
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

### use 的高级用法

```rust
// 重命名
use std::fmt::Result;
use std::io::Result as IoResult;

// 重新导出
pub use crate::front_of_house::hosting;

// 嵌套路径
use std::{
    cmp::Ordering,
    io::{self, Write},
};

// 通配符导入（谨慎使用）
use std::collections::*;

// 条件编译的导入
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;
```

## 模块路径

### 绝对路径和相对路径

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        pub fn seat_at_table() {
            // 相对路径
            add_to_waitlist();
            
            // 绝对路径
            crate::front_of_house::hosting::add_to_waitlist();
        }
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        
        // 使用 super 访问父模块
        super::front_of_house::hosting::add_to_waitlist();
    }
    
    fn cook_order() {}
}
```

### super 和 self 关键字

```rust
mod parent {
    fn parent_function() {
        println!("父模块函数");
    }
    
    mod child {
        pub fn child_function() {
            // 使用 super 访问父模块
            super::parent_function();
            
            // 使用 self 明确当前模块（可选）
            self::local_function();
        }
        
        fn local_function() {
            println!("子模块函数");
        }
    }
}
```

## 实际应用示例

### 库的模块组织

```rust
// lib.rs
pub mod config {
    pub struct Config {
        pub host: String,
        pub port: u16,
    }
    
    impl Config {
        pub fn new(host: String, port: u16) -> Config {
            Config { host, port }
        }
    }
}

pub mod database {
    use crate::config::Config;
    
    pub struct Connection {
        config: Config,
    }
    
    impl Connection {
        pub fn new(config: Config) -> Connection {
            Connection { config }
        }
        
        pub fn connect(&self) -> Result<(), String> {
            println!("连接到 {}:{}", self.config.host, self.config.port);
            Ok(())
        }
    }
}

pub mod api {
    use crate::database::Connection;
    
    pub fn start_server(conn: Connection) {
        match conn.connect() {
            Ok(_) => println!("服务器启动成功"),
            Err(e) => println!("服务器启动失败: {}", e),
        }
    }
}

// 重新导出常用项
pub use config::Config;
pub use database::Connection;
```

### 使用库

```rust
// main.rs
use my_library::{Config, Connection, api};

fn main() {
    let config = Config::new("localhost".to_string(), 8080);
    let connection = Connection::new(config);
    api::start_server(connection);
}
```

## 最佳实践

### 模块组织原则

1. **按功能分组**：将相关功能放在同一模块
2. **控制可见性**：只暴露必要的接口
3. **避免深层嵌套**：保持模块层次简单
4. **使用重新导出**：简化用户的导入路径

### 命名约定

```rust
// 模块名使用 snake_case
mod user_management {
    // 公有接口
    pub struct User {
        pub name: String,
        id: u64,  // 私有字段
    }
    
    // 构造函数
    impl User {
        pub fn new(name: String) -> User {
            User {
                name,
                id: generate_id(),
            }
        }
    }
    
    // 私有辅助函数
    fn generate_id() -> u64 {
        // 生成唯一 ID
        42
    }
}
```

### 文件组织策略

```
src/
├── lib.rs          # 库根，重新导出主要接口
├── config.rs       # 配置相关
├── database/       # 数据库模块
│   ├── mod.rs      # 模块声明
│   ├── connection.rs
│   └── query.rs
├── api/            # API 模块
│   ├── mod.rs
│   ├── handlers.rs
│   └── middleware.rs
└── utils.rs        # 工具函数
```

## 常见错误

### 1. 忘记声明模块

```rust
// 错误：忘记在 main.rs 中声明模块
// mod network;  // 需要这行

fn main() {
    network::connect();  // 错误：找不到 network
}
```

### 2. 可见性问题

```rust
mod outer {
    fn private_function() {}  // 私有函数
    
    mod inner {
        pub fn public_function() {
            super::private_function();  // 正确：可以访问父模块私有项
        }
    }
}

fn main() {
    // outer::private_function();     // 错误：私有函数不可访问
    outer::inner::public_function();  // 正确
}
```

### 3. 循环依赖

```rust
// 避免循环依赖
// mod a { use crate::b; }
// mod b { use crate::a; }  // 错误：循环依赖

// 解决方案：提取公共部分到第三个模块
mod common {
    pub struct SharedData;
}

mod a {
    use crate::common::SharedData;
}

mod b {
    use crate::common::SharedData;
}
```

## 学习检查清单

- [ ] 理解模块的基本概念和作用
- [ ] 掌握 `mod` 关键字的使用
- [ ] 理解 `pub` 可见性控制
- [ ] 掌握 `use` 关键字的各种用法
- [ ] 理解绝对路径和相对路径
- [ ] 掌握 `super` 和 `self` 的使用
- [ ] 理解文件系统与模块的对应关系
- [ ] 能够设计合理的模块结构
- [ ] 掌握重新导出的使用场景
- [ ] 了解模块组织的最佳实践

## 扩展阅读

- [Rust Book - Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust Reference - Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [Rust API Guidelines - Module organization](https://rust-lang.github.io/api-guidelines/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)