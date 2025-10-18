# 代码质量检查

我们使用 `clippy` 来确保代码质量。

## 如何使用

在项目根目录下运行以下命令：

```bash
cargo clippy --workspace -- -D warnings
```

这将检查工作区中的所有 crate，并将任何警告视为错误。