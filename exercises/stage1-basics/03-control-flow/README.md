# 练习 3：控制流

## 目标

练习 Rust 的控制流结构，包括 `if` 表达式和 `loop` 循环。

## 任务

1.  打开 `exercises/stage1-basics/03-control-flow/exercise/src/main.rs` 文件。
2.  修改 `main` 函数，使其包含以下内容：
    *   一个名为 `number` 的变量，并将其初始化为 `3`。
    *   一个 `if-else` 表达式，检查 `number` 是否小于 `5`。如果是，则打印 "condition was true"；否则，打印 "condition was false"。
    *   一个 `loop`，它将一个计数器从 `0` 增加到 `10`。当计数器达到 `10` 时，循环应使用 `break` 关键字返回计数器的两倍。
    *   在 `loop` 之后，添加对 `println!` 宏的调用，以打印 `loop` 的结果。
3.  使用 `cargo run -p exercise` 运行项目，并验证输出是否正确。

## 验收标准

-   `cargo run -p exercise` 命令执行成功，没有任何错误。
-   控制台输出 "condition was true" 和 `loop` 的结果（`20`）。