# 1.1 Rust安装和配置

本节将指导你完成Rust开发环境的安装和配置，确保你能够顺利开始Rust学习之旅。

## 🎯 学习目标

- 理解Rust工具链的组成
- 掌握Rust的安装方法
- 配置开发环境
- 验证安装是否成功

## 📋 前置要求

- 操作系统：Windows 10+、macOS 10.12+、或现代Linux发行版
- 网络连接（用于下载工具链）
- 基本的命令行操作能力

## 🛠️ Rust工具链介绍

### 核心组件

Rust工具链包含以下核心组件：

| 组件 | 功能 | 说明 |
|------|------|------|
| `rustc` | Rust编译器 | 将Rust代码编译为可执行文件 |
| `cargo` | 包管理器和构建工具 | 管理依赖、构建项目、运行测试 |
| `rustup` | 工具链管理器 | 管理Rust版本和组件 |
| `rustfmt` | 代码格式化工具 | 自动格式化Rust代码 |
| `clippy` | 代码检查工具 | 提供代码质量建议 |

### 与其他语言对比

| 特性 | Rust | C/C++ | Golang |
|------|------|-------|--------|
| 包管理 | Cargo | 无官方工具 | go mod |
| 构建系统 | Cargo | Make/CMake | go build |
| 代码格式化 | rustfmt | clang-format | gofmt |
| 静态分析 | clippy | 第三方工具 | go vet |

## 🚀 安装步骤

### 方法1：使用rustup（推荐）

`rustup`是Rust官方推荐的安装工具，能够管理多个Rust版本。

#### macOS和Linux

```bash
# 下载并运行安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重新加载环境变量
source ~/.bashrc
# 或者
source ~/.zshrc
```

#### Windows

1. 访问 [rustup.rs](https://rustup.rs/)
2. 下载 `rustup-init.exe`
3. 运行安装程序，按提示操作

#### 安装选项说明

安装过程中会出现以下选项：

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

**推荐选择1（默认安装）**，这将安装：
- 最新稳定版Rust
- 标准库
- Cargo包管理器
- 基本开发工具

### 方法2：包管理器安装

#### macOS (Homebrew)

```bash
brew install rust
```

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install rustc cargo
```

#### CentOS/RHEL/Fedora

```bash
# CentOS/RHEL
sudo yum install rust cargo

# Fedora
sudo dnf install rust cargo
```

**注意**：包管理器安装的版本可能不是最新的，推荐使用rustup。

## ✅ 验证安装

### 检查版本信息

```bash
# 检查Rust编译器版本
rustc --version
# 输出示例：rustc 1.75.0 (82e1608df 2023-12-21)

# 检查Cargo版本
cargo --version
# 输出示例：cargo 1.75.0 (1d8b05cdd 2023-11-20)

# 检查rustup版本
rustup --version
# 输出示例：rustup 1.26.0 (5af9b9484 2023-04-05)
```

### 检查已安装组件

```bash
# 查看已安装的工具链
rustup show

# 查看已安装的组件
rustup component list --installed
```

预期输出：
```
Default host: x86_64-apple-darwin
rustup home:  /Users/username/.rustup

stable-x86_64-apple-darwin (default)
rustc 1.75.0 (82e1608df 2023-12-21)
```

## 🔧 环境配置

### 配置环境变量

确保以下环境变量已正确设置：

```bash
# 查看Rust相关环境变量
echo $CARGO_HOME    # 通常是 ~/.cargo
echo $RUSTUP_HOME   # 通常是 ~/.rustup
echo $PATH          # 应包含 ~/.cargo/bin
```

如果环境变量未设置，手动添加到shell配置文件：

```bash
# 对于bash用户，编辑 ~/.bashrc
# 对于zsh用户，编辑 ~/.zshrc

export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"
export PATH="$CARGO_HOME/bin:$PATH"
```

### 配置Cargo镜像（可选）

如果网络访问crates.io较慢，可以配置国内镜像：

```bash
# 创建Cargo配置目录
mkdir -p ~/.cargo

# 创建配置文件
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

[net]
git-fetch-with-cli = true
EOF
```

## 🎨 IDE和编辑器配置

### VS Code（推荐）

1. 安装VS Code
2. 安装rust-analyzer扩展
3. 安装CodeLLDB扩展（用于调试）

**推荐扩展列表**：
- `rust-lang.rust-analyzer` - Rust语言服务
- `vadimcn.vscode-lldb` - 调试支持
- `serayuzgur.crates` - Cargo.toml依赖管理
- `dustypomerleau.rust-syntax` - 语法高亮增强

### IntelliJ IDEA / CLion

1. 安装Rust插件
2. 配置Rust工具链路径
3. 启用Cargo支持

### Vim/Neovim

```vim
" 使用vim-plug管理插件
Plug 'rust-lang/rust.vim'
Plug 'neoclide/coc.nvim', {'branch': 'release'}

" 安装coc-rust-analyzer
:CocInstall coc-rust-analyzer
```

### Emacs

```elisp
;; 使用use-package管理
(use-package rust-mode
  :ensure t)

(use-package lsp-mode
  :ensure t
  :hook (rust-mode . lsp))
```

## 🔍 故障排除

### 常见问题

#### 问题1：命令未找到

```bash
$ rustc --version
bash: rustc: command not found
```

**解决方案**：
1. 检查PATH环境变量
2. 重新加载shell配置
3. 重新安装rustup

#### 问题2：网络连接问题

```
error: could not download file from 'https://...'
```

**解决方案**：
1. 检查网络连接
2. 配置代理（如果需要）
3. 使用离线安装包

#### 问题3：权限问题

```
permission denied
```

**解决方案**：
1. 不要使用sudo安装rustup
2. 检查用户目录权限
3. 使用用户级安装

### 诊断命令

```bash
# 检查rustup配置
rustup show

# 检查环境变量
env | grep -i rust

# 检查PATH
echo $PATH | tr ':' '\n' | grep cargo

# 测试网络连接
curl -I https://forge.rust-lang.org/
```

## 📚 工具链管理

### 版本管理

```bash
# 安装特定版本
rustup install 1.74.0

# 设置默认版本
rustup default stable

# 切换版本
rustup default 1.74.0

# 更新到最新版本
rustup update
```

### 组件管理

```bash
# 安装额外组件
rustup component add rustfmt
rustup component add clippy
rustup component add rust-src

# 查看可用组件
rustup component list

# 移除组件
rustup component remove rust-docs
```

### 目标平台管理

```bash
# 添加交叉编译目标
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-apple-darwin

# 查看已安装目标
rustup target list --installed

# 查看所有可用目标
rustup target list
```

## 🎯 最佳实践

### 1. 使用稳定版本

```bash
# 推荐使用stable版本进行学习
rustup default stable
```

### 2. 定期更新

```bash
# 每月更新一次工具链
rustup update
```

### 3. 项目级工具链

```bash
# 为特定项目设置工具链版本
echo "1.74.0" > rust-toolchain.toml
```

### 4. 备份配置

```bash
# 备份Cargo配置
cp ~/.cargo/config.toml ~/.cargo/config.toml.backup
```

## 🧪 验证练习

完成以下练习来验证你的安装：

### 练习1：版本检查

```bash
# 运行以下命令并记录输出
rustc --version
cargo --version
rustup --version
```

### 练习2：创建测试项目

```bash
# 创建新项目
cargo new hello_rust
cd hello_rust

# 构建项目
cargo build

# 运行项目
cargo run
```

预期输出：
```
Hello, world!
```

### 练习3：工具验证

```bash
# 格式化代码
cargo fmt

# 运行代码检查
cargo clippy

# 运行测试
cargo test
```

## 📖 延伸阅读

- [Rust官方安装指南](https://forge.rust-lang.org/infra/channel-layout.html)
- [rustup文档](https://rust-lang.github.io/rustup/)
- [Cargo文档](https://doc.rust-lang.org/cargo/)

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 成功安装Rust工具链
- [ ] 验证rustc、cargo、rustup版本
- [ ] 配置开发环境（IDE/编辑器）
- [ ] 创建并运行第一个Rust项目
- [ ] 理解工具链的基本管理命令
- [ ] 解决常见的安装问题

---

**恭喜！** 🎉 你已经成功配置了Rust开发环境。

[← 返回目录](../README.md) | [下一节：开发工具选择 →](./02-dev-tools.md)