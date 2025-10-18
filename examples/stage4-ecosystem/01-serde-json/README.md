# 示例 1：使用 Serde 进行 JSON 序列化和反序列化

本示例旨在演示如何使用 `serde` 和 `serde_json` 库在 Rust 中进行 JSON 序列化和反序列化。

## 核心概念

### 1. `serde`

`serde` 是一个用于在 Rust 数据结构和各种序列化格式之间进行转换的框架。它通过 `Serialize` 和 `Deserialize` trait 来实现这一功能。

### 2. `serde_json`

`serde_json` 是一个用于在 Rust 数据结构和 JSON 格式之间进行转换的库。它实现了 `serde` 的 `Serializer` 和 `Deserializer` trait。

### 3. `#[derive(Serialize, Deserialize)]`

这是一个派生宏，它可以自动为你的数据结构实现 `Serialize` 和 `Deserialize` trait。这使得在你的数据结构和 JSON 格式之间进行转换变得非常容易。

## 如何运行

你可以使用以下命令来运行本示例：

```bash
cargo run -p serde-json-demo
```

该命令将编译并执行 `src/main.rs` 文件，你将在终端看到关于 JSON 序列化和反序列化的演示输出。