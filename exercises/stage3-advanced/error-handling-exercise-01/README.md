# 练习 3：错误处理

## 目标

练习使用 `Result` 和 `?` 运算符进行错误处理。

## 任务

1.  打开 `exercises/stage3-advanced/error-handling-exercise-01/src/main.rs` 文件。
2.  创建一个名为 `read_username_from_file` 的函数，该函数从名为 `hello.txt` 的文件中读取用户名。
3.  该函数应返回一个 `Result<String, io::Error>`。
4.  在 `main` 函数中，调用 `read_username_from_file` 函数并处理 `Result`：
    *   如果 `Ok`，则打印用户名。
    *   如果 `Err`，则打印错误。

## 验收标准

-   `cargo run -p error-handling-exercise-01` 命令执行成功，没有任何错误。
-   如果 `hello.txt` 文件存在并包含内容，则控制台输出用户名。
-   如果 `hello.txt` 文件不存在，则控制台输出错误信息。