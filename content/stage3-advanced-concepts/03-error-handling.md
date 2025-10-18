# 错误处理

Rust 将错误分为两大类：可恢复（recoverable）和不可恢复（unrecoverable）的错误。

-   **可恢复错误**通常表示向用户报告问题并重试操作是合理的情况。例如，文件未找到的错误。
-   **不可恢复错误**是 bug，比如尝试访问超出数组末尾的位置。

Rust 没有异常。相反，它有 `Result<T, E>` 类型用于可恢复的错误，以及 `panic!` 宏，在程序遇到不可恢复的错误时停止执行。

## `panic!`

当 `panic!` 宏执行时，你的程序将打印一个失败消息，展开并清理栈，然后退出。另一种选择是立即中止，这会不清理就退出程序。然后，操作系统需要清理程序使用的内存。

## `Result`

`Result` 枚举定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` 和 `E` 是泛型类型参数。`T` 代表在成功情况下将在 `Ok` 变体中返回的值的类型，而 `E` 代表在失败情况下将在 `Err` 变体中返回的错误的类型。

## `?` 运算符

`?` 运算符可以用来简化错误处理。如果 `Result` 的值是 `Ok`，`?` 运算符将从 `Ok` 中取出内部值并继续执行。如果值是 `Err`，`?` 运算符将从函数中返回 `Err` 值。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```