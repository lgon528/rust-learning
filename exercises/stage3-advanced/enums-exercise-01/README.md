# 练习 2：枚举

## 目标

练习定义和使用枚举。

## 任务

1.  打开 `exercises/stage3-advanced/enums-exercise-01/src/main.rs` 文件。
2.  定义一个名为 `Message` 的枚举，它具有四个变体：
    *   `Quit`
    *   `Move { x: i32, y: i32 }`
    *   `Write(String)`
    *   `ChangeColor(i32, i32, i32)`
3.  在 `main` 函数中，创建一个 `Message::Write` 变体的实例，并将其打印出来。

## 验收标准

-   `cargo run -p enums-exercise-01` 命令执行成功，没有任何错误。
-   控制台输出 `Message::Write` 变体的内容。