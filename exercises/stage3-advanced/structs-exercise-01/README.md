# 练习 1：结构体

## 目标

练习定义和使用结构体。

## 任务

1.  打开 `exercises/stage3-advanced/structs-exercise-01/src/main.rs` 文件。
2.  定义一个名为 `Rectangle` 的结构体，它具有 `width` 和 `height` 两个字段，类型均为 `u32`。
3.  为 `Rectangle` 结构体实现一个名为 `area` 的方法，该方法返回矩形的面积。
4.  在 `main` 函数中，创建一个 `Rectangle` 实例，并调用 `area` 方法打印出其面积。

## 验收标准

-   `cargo run -p structs-exercise-01` 命令执行成功，没有任何错误。
-   控制台输出矩形的面积。