# 项目2：构建系统编程 CLI

本项目演示了如何构建一个简单的命令行界面（CLI）工具，用于计算文件中的行数。这是系统编程中的一个常见任务。

## 核心概念

- **命令行参数**：使用 `std::env::args` 来访问命令行参数。
- **文件 I/O**：使用 `std::fs::File` 和 `std::io::BufReader` 来读取文件。
- **错误处理**：使用 `Result` 和 `Box<dyn std::error::Error>` 来处理潜在的错误。

## 运行项目

要运行此项目，请导航到 `examples/stage5-projects/02-system-programming-cli` 目录并运行以下命令：

```bash
cargo run -- <文件路径>
```

例如，要计算 `Cargo.toml` 文件中的行数，请运行：

```bash
cargo run -- Cargo.toml
```