# Cargo 和 Crates.io

Cargo 是 Rust 的构建系统和包管理器。Cargo 负责管理 Rust 项目的许多任务，例如构建代码、下载依赖库以及构建这些库。

## Crates.io

Crates.io 是 Rust 社区的中央包注册中心，它托管了成千上万个第三方库（称为 “crates”）。你可以使用 Cargo 将这些库添加到你的项目中。

## `Cargo.toml`

`Cargo.toml` 文件是 Cargo 的配置文件。它包含了项目的元数据和依赖项。

```toml
[package]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"

[dependencies]
serde = "1.0"
```

## 常用命令

-   `cargo build`：构建项目。
-   `cargo run`：构建并运行项目。
-   `cargo test`：运行项目的测试。
-   `cargo doc`：为项目构建文档。
-   `cargo publish`：将库发布到 Crates.io。