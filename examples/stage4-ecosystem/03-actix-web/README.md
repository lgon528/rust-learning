# 示例 3：使用 Actix Web 构建 Web 服务器

本示例旨在演示如何使用 `actix-web` 框架在 Rust 中构建一个简单的 Web 服务器。

## 核心概念

### 1. `actix-web`

`actix-web` 是一个功能强大、实用且速度极快的 Web 框架，用于构建 Web 应用程序和微服务。它基于 `actix` actor 框架构建，并提供了许多开箱即用的功能，例如：

-   路由
-   中间件
-   静态文件服务
-   WebSocket 支持
-   SSL/TLS 支持

### 2. `HttpServer`

`HttpServer` 是 `actix-web` 中用于创建 HTTP 服务器的结构体。它负责接受传入的连接，并将它们分派给你的应用程序。

### 3. `App`

`App` 是 `actix-web` 中用于构建应用程序的结构体。你可以在 `App` 上注册路由、中间件和其他服务。

### 4. `#[get("/")]`

这是一个属性宏，用于将一个函数标记为 HTTP GET 请求的处理程序。`actix-web` 还提供了其他属性宏，例如 `#[post("/")]`、`#[put("/")]` 和 `#[delete("/")]`。

## 如何运行

你可以使用以下命令来运行本示例：

```bash
cargo run -p actix-web-demo
```

该命令将编译并执行 `src/main.rs` 文件，并启动一个在 `127.0.0.1:8080` 上监听的 Web 服务器。你可以使用 Web 浏览器或 `curl` 等工具来访问该服务器。