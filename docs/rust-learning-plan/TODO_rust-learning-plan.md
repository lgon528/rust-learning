# 待办事项：rust-learning-plan

## 1. 数据库配置

- **描述**：在 `examples/stage5-projects/01-axum-web-app` 项目中，数据库连接字符串是硬编码的。为了提高安全性和灵活性，应将其移至 `.env` 文件中。
- **操作指引**：
    1. 在 `examples/stage5-projects/01-axum-web-app` 目录下创建一个 `.env` 文件。
    2. 在 `.env` 文件中添加 `DATABASE_URL=postgres://user:password@localhost/database`。
    3. 修改 `src/main.rs` 文件，使用 `dotenv` crate 从 `.env` 文件中加载数据库连接字符串。

## 2. 错误处理

- **描述**：在 `examples/stage5-projects/02-system-programming-cli` 项目中，错误处理可以进一步改进，以提供更友好的错误消息。
- **操作指引**：
    1. 使用 `anyhow` 或 `thiserror` crate 来增强错误处理。
    2. 为不同的错误类型提供更具体的错误消息。

## 3. 区块链增强

- **描述**：`examples/stage5-projects/03-blockchain-demo` 是一个非常基础的实现。可以添加更多功能以使其更完整。
- **操作指引**：
    1. 实现一个工作量证明（Proof of Work）算法。
    2. 添加一个P2P网络，以在多个节点之间同步区块链。
    3. 实现一个交易系统。