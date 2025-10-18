//! 高级错误处理演示
//!
//! 本示例代码旨在演示 Rust 的高级错误处理技术，包括：
//! 1.  **自定义错误类型**：如何创建自己的错误类型以更好地表达错误情况。
//! 2.  **`?` 运算符**：如何使用 `?` 运算符来简化错误传播。
//! 3.  **`main` 函数返回 `Result`**：如何让 `main` 函数返回一个 `Result`，以便将错误传递给调用者。

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

// 1. 自定义错误类型
// 创建一个枚举来表示可能发生的不同错误。
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

// 为 AppError 实现 Display trait，用于用户友好的错误信息。
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

// 为 AppError 实现 Error trait，以支持错误链和源错误。
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
        }
    }
}

// 实现 From<io::Error> for AppError，以便 `?` 运算符可以自动转换错误类型。
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

// 实现 From<std::num::ParseIntError> for AppError。
impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

// 2. 使用 `?` 运算符进行错误传播
// 这个函数读取文件内容并将其解析为数字。
fn read_number_from_file() -> Result<i32, AppError> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

// 3. `main` 函数返回 `Result`
// 这允许我们从 `main` 函数中传播错误，而不是在其中处理它们。
fn main() -> Result<(), Box<dyn Error>> {
    println!("Rust 高级错误处理演示\n");

    // 创建一个用于测试的文件
    std::fs::write("number.txt", "123")?;

    match read_number_from_file() {
        Ok(number) => println!("Number from file: {}", number),
        Err(e) => {
            eprintln!("Error reading number: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
        }
    }

    // 删除测试文件
    std::fs::remove_file("number.txt")?;

    println!("\n演示完成。");

    Ok(())
}