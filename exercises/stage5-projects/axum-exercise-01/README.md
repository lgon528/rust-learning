# Axum 练习

## 目标

修改 `src/main.rs` 文件，以创建一个具有以下路由的 Web 服务器：

- `GET /`：返回一个简单的“Hello, World!”消息。
- `GET /json`：返回一个 JSON 对象。
- `POST /echo`：将请求正文回显为响应。

## 提示

- 使用 `axum::response::Json` 来处理 JSON 响应。
- 使用 `axum::extract::Json` 来提取 JSON 请求正文。