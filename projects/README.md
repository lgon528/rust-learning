# 实战项目

本目录包含Rust学习计划第四阶段的三个实战项目，涵盖Web开发、系统编程和区块链三个应用领域。

## 项目概览

### 🌐 Web应用项目 (web-application)

**技术栈**: Axum + PostgreSQL + Tokio

**项目特点**:
- 现代异步Web框架
- RESTful API设计
- 数据库集成和ORM
- 用户认证和授权
- 中间件和错误处理

**学习目标**:
- 掌握Rust异步编程
- 理解Web服务架构
- 学习数据库操作
- 实践API设计

### ⚙️ 系统编程项目 (system-programming)

**技术栈**: 标准库 + libc + nix

**项目特点**:
- 底层系统调用
- 内存管理优化
- 并发和多线程
- 文件系统操作
- 网络编程

**学习目标**:
- 深入理解系统编程
- 掌握unsafe Rust
- 学习性能优化
- 实践并发编程

### ⛓️ 区块链项目 (blockchain)

**技术栈**: 密码学库 + 网络库 + 序列化

**项目特点**:
- 区块链核心概念
- 加密算法实现
- P2P网络通信
- 共识机制
- 智能合约基础

**学习目标**:
- 理解区块链原理
- 掌握密码学应用
- 学习分布式系统
- 实践网络编程

## 开发环境

### 前置要求

```bash
# 安装Rust工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装必要的系统依赖
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install postgresql postgresql-contrib libpq-dev

# 安装开发工具
cargo install cargo-watch cargo-audit cargo-tarpaulin
```

### 环境配置

```bash
# 复制环境配置文件
cp .env.example .env

# 编辑配置文件
vim .env

# 启动数据库服务
# macOS
brew services start postgresql

# Linux
sudo systemctl start postgresql
```

## 项目开发流程

### 1. 项目初始化

```bash
# 进入项目目录
cd projects/web-application

# 安装依赖
cargo build

# 运行数据库迁移
cargo run --bin migrate
```

### 2. 开发模式

```bash
# 启动开发服务器（自动重载）
cargo watch -x run

# 运行测试（监视模式）
cargo watch -x test
```

### 3. 代码质量检查

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 安全审计
cargo audit

# 测试覆盖率
cargo tarpaulin --out Html
```

### 4. 性能测试

```bash
# 运行基准测试
cargo bench

# 性能分析
cargo run --release --bin profiler
```

## 学习路径

### 建议顺序

1. **Web应用项目** (6-8周)
   - 从熟悉的Web开发入手
   - 学习Rust异步编程模型
   - 掌握现代Web开发实践

2. **系统编程项目** (4-6周)
   - 深入底层系统概念
   - 理解内存管理和性能优化
   - 掌握并发编程技巧

3. **区块链项目** (6-8周)
   - 应用前两个项目的知识
   - 学习分布式系统设计
   - 实践密码学和网络编程

### 学习重点

**第一阶段 - Web开发基础**:
- HTTP服务器实现
- 路由和中间件
- 数据库集成
- 错误处理

**第二阶段 - 系统编程深入**:
- 内存管理
- 文件I/O
- 网络编程
- 并发控制

**第三阶段 - 区块链应用**:
- 数据结构设计
- 加密算法
- 网络协议
- 共识机制

## 项目评估

### 评估维度

- **功能完整性** (30%): 实现所有要求的功能
- **代码质量** (25%): 遵循最佳实践和设计模式
- **性能表现** (20%): 满足性能要求和基准测试
- **测试覆盖** (15%): 完整的单元测试和集成测试
- **文档质量** (10%): 清晰的API文档和使用说明

### 交付标准

每个项目需要包含：

- ✅ 完整的功能实现
- ✅ 全面的测试套件
- ✅ 详细的API文档
- ✅ 部署和运维指南
- ✅ 性能基准报告

## 部署指南

### 本地部署

```bash
# 构建发布版本
cargo build --release

# 运行生产服务
./target/release/web-server
```

### Docker部署

```bash
# 构建Docker镜像
docker build -t rust-web-app .

# 运行容器
docker run -p 3000:3000 rust-web-app
```

### 云平台部署

详见各项目目录中的部署文档。

## 故障排除

### 常见问题

1. **编译错误**: 检查Rust版本和依赖版本
2. **数据库连接**: 确认数据库服务运行和配置正确
3. **端口冲突**: 修改配置文件中的端口设置
4. **权限问题**: 检查文件和目录权限

### 获取帮助

- 查看项目文档: `cargo doc --open`
- 运行诊断工具: `cargo run --bin diagnostic`
- 提交Issue: 在项目仓库中报告问题