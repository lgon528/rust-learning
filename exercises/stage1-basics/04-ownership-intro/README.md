# 练习 4：所有权介绍

## 目标

练习 Rust 的所有权概念，包括所有权转移、克隆和借用。

## 任务

1.  打开 `exercises/stage1-basics/04-ownership-intro/exercise/src/main.rs` 文件。
2.  修改 `main` 函数，使其包含以下内容：
    *   一个名为 `s1` 的 `String`，并将其初始化为 `"hello"`。
    *   一个名为 `s2` 的新 `String`，通过克隆 `s1` 创建。
    *   一个名为 `takes_ownership` 的函数，它接收一个 `String` 并打印它。
    *   一个名为 `makes_copy` 的函数，它接收一个 `i32` 并打印它。
3.  在 `main` 函数中，调用 `takes_ownership` 并传入 `s1`。然后，尝试再次打印 `s1`，并观察会发生什么。
4.  在 `main` 函数中，调用 `makes_copy` 并传入一个整数。然后，再次打印该整数，并观察会发生什么。
5.  使用 `cargo run -p exercise` 运行项目，并验证输出是否正确。

## 验收标准

-   `cargo run -p exercise` 命令执行成功，没有任何错误。
-   控制台输出 `s1` 的值（在 `takes_ownership` 函数中）、整数的值（在 `makes_copy` 函数中）以及再次打印的整数的值。