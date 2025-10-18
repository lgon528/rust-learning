# 练习 1：所有权

## 目标

通过一个更复杂的示例来练习 Rust 的所有权和借用规则。

## 任务

1.  打开 `exercises/stage2-ownership/ownership-exercise-01/src/main.rs` 文件。
2.  创建一个名为 `inspect` 的函数，该函数接收一个对 `String` 的引用，并打印出该字符串及其长度。
3.  创建一个名为 `change` 的函数，该函数接收一个对 `String` 的可变引用，并在其末尾附加 `" world"`。
4.  创建一个名为 `eat` 的函数，该函数接收一个 `String` 并返回 `true`。
5.  在 `main` 函数中：
    *   创建一个可变的 `String`。
    *   调用 `inspect` 函数，并传入对该字符串的引用。
    *   调用 `change` 函数，并传入对该字符串的可变引用。
    *   调用 `inspect` 函数，并再次传入对该字符串的引用。
    *   调用 `eat` 函数，并传入该字符串。
    *   尝试再次调用 `inspect` 函数，并观察会发生什么。

## 验收标准

-   `cargo run -p ownership-exercise-01` 命令执行成功，没有任何错误。
-   控制台输出显示字符串在每个步骤中的状态。
-   对 `eat` 的调用之后，对 `inspect` 的调用将导致编译错误。