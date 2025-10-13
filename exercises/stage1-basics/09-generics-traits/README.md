# 练习：泛型和 Trait

这个练习将帮助你理解 Rust 的泛型和 trait，它们是实现代码重用的关键。

1.  **泛型**：学习如何使用泛型创建可用于多种数据类型的函数、结构体和枚举。
2.  **Trait**：学习如何定义共享行为的 trait。
3.  **生命周期**：将生命周期与泛型和 trait 结合使用。

## 任务

- [ ] 创建一个新项目 `generics-traits-practice`。
- [ ] 编写一个泛型函数，接收一个任意类型的切片，并返回最大的元素。
- [ ] 定义一个 `Summary` trait，包含一个 `summarize` 方法。
- [ ] 为 `Tweet` 和 `NewsArticle` 结构体实现 `Summary` trait。
- [ ] 编写一个函数，接收一个实现了 `Summary` trait 的项，并调用其 `summarize` 方法。