# System Programming CLI

一个功能全面的系统编程命令行工具，使用 Rust 开发。

## 🚀 功能特性

### 📁 文件操作
- ✅ **文件统计**: 行数、字数、字符数统计
- ✅ **文件搜索**: 基础文件查找
- ✅ **目录树**: 可视化目录结构
- ✅ **磁盘使用**: 基础磁盘使用分析

### 🖥️ 系统监控
- ✅ **系统信息**: 基础系统信息显示
- ✅ **进程监控**: 简化的进程列表
- ✅ **用户信息**: 当前用户信息显示

## 🛠️ 技术栈

### 核心依赖
- **clap 4.4**: 现代化命令行界面框架
- **walkdir 2.4**: 目录遍历

### 系统信息
- **标准库**: 使用 Rust 标准库实现跨平台兼容性

## 📦 安装和使用

### 构建项目
```bash
cd examples/stage5-projects/02-system-programming-cli
cargo build --release
```

### 使用示例

#### 文件操作
```bash
# 统计文件行数
./target/release/system-programming-cli file count file1.txt file2.txt

# 显示目录树
./target/release/system-programming-cli file tree /path/to/dir

# 磁盘使用分析
./target/release/system-programming-cli file du /path/to/dir
```

#### 系统监控
```bash
# 显示系统信息
./target/release/system-programming-cli config --show
```

## 🎨 项目架构

```
src/
├── main.rs              # 主程序入口
├── commands/            # 命令处理模块
│   ├── file_commands.rs
│   └── mod.rs
├── config/             # 配置管理模块
│   └── mod.rs
└── Cargo.toml          # 项目配置
```

## 🧪 开发状态

### ✅ 已完成功能
- 基础CLI框架搭建
- 文件操作命令
- 系统信息显示
- 配置管理基础

### 🚧 开发中功能
- 更高级的系统监控功能
- 详细的进度显示
- 更丰富的文件操作选项

## 📋 待办事项

### 短期目标
- [ ] 添加更多文件操作选项
- [ ] 增强系统监控功能
- [ ] 改进错误处理和用户体验

### 长期目标
- [ ] 添加图形化界面选项
- [ ] 支持插件系统
- [ ] 集成更多第三方工具

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](../../../../../LICENSE) 文件。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [clap 命令行解析库](https://clap.rs/)
- [walkdir 文件系统遍历库](https://github.com/BurntSushi/walkdir)

## 🆘 故障排除

### 常见问题

**Q: 编译错误**
A: 确保使用 Rust 1.70+ 版本，运行 `rustc --version` 检查

**Q: 权限被拒绝**
A: 确保有足够的文件系统权限，必要时使用 `sudo`

**Q: 功能不完整**
A: 当前为基础版本，更多功能正在开发中

---

**开发团队**: Rust Learning Project
**最后更新**: 2025-11-15
**版本**: 0.1.0 (基础版本)
