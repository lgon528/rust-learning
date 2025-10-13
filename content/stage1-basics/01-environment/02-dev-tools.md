# 1.2 开发工具选择

选择合适的开发工具能够显著提升Rust开发效率。本节将介绍主流的Rust开发工具，帮助你选择最适合的开发环境。

## 🎯 学习目标

- 了解主流Rust开发工具的特点
- 掌握开发工具的配置方法
- 理解不同工具的适用场景
- 配置高效的开发环境

## 🛠️ 开发工具概览

### 工具分类

| 类别 | 工具 | 特点 | 推荐指数 |
|------|------|------|----------|
| **IDE** | VS Code | 轻量、插件丰富 | ⭐⭐⭐⭐⭐ |
| **IDE** | IntelliJ IDEA/CLion | 功能强大、智能提示 | ⭐⭐⭐⭐ |
| **编辑器** | Vim/Neovim | 高度可定制、快捷 | ⭐⭐⭐⭐ |
| **编辑器** | Emacs | 强大扩展性 | ⭐⭐⭐ |
| **在线** | Rust Playground | 快速测试、分享 | ⭐⭐⭐⭐ |

## 🚀 VS Code配置（推荐）

### 为什么选择VS Code？

- ✅ **免费开源**：完全免费，社区活跃
- ✅ **轻量快速**：启动快，资源占用少
- ✅ **插件丰富**：Rust生态支持完善
- ✅ **跨平台**：Windows、macOS、Linux全支持
- ✅ **调试友好**：内置调试支持

### 必装插件

#### 1. rust-analyzer

**最重要的Rust插件**，提供：
- 语法高亮和错误检查
- 智能代码补全
- 代码导航和重构
- 内联类型提示

```bash
# 安装命令
code --install-extension rust-lang.rust-analyzer
```

#### 2. CodeLLDB

**调试支持插件**：
- 断点调试
- 变量查看
- 调用栈分析

```bash
code --install-extension vadimcn.vscode-lldb
```

#### 3. Crates

**依赖管理插件**：
- Cargo.toml智能提示
- 版本更新检查
- 依赖文档链接

```bash
code --install-extension serayuzgur.crates
```

### 推荐插件列表

```bash
# 一键安装所有推荐插件
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
code --install-extension tamasfe.even-better-toml
code --install-extension dustypomerleau.rust-syntax
code --install-extension swellaby.vscode-rust-test-adapter
```

### VS Code配置文件

创建 `.vscode/settings.json`：

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.completion.addCallParentheses": false,
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "files.watcherExclude": {
        "**/target/**": true
    }
}
```

### 调试配置

创建 `.vscode/launch.json`：

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'your_project'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=your_project",
                    "--package=your_project"
                ],
                "filter": {
                    "name": "your_project",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=your_project",
                    "--package=your_project"
                ],
                "filter": {
                    "name": "your_project",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## 🧠 IntelliJ IDEA/CLion配置

### 优势特点

- ✅ **智能重构**：强大的代码重构功能
- ✅ **项目管理**：优秀的大型项目支持
- ✅ **集成工具**：内置版本控制、数据库工具
- ✅ **调试器**：功能丰富的调试环境

### 安装Rust插件

1. 打开 `File` → `Settings` → `Plugins`
2. 搜索 "Rust"
3. 安装官方Rust插件
4. 重启IDE

### 配置Rust工具链

1. `File` → `Settings` → `Languages & Frameworks` → `Rust`
2. 设置工具链路径：
   - Toolchain location: `~/.cargo/bin`
   - Standard library: `$(rustc --print sysroot)/lib/rustlib/src/rust/library`

### 推荐设置

```
# 代码风格
Editor → Code Style → Rust
- Use tab character: false
- Tab size: 4
- Indent: 4

# 自动导入
Editor → General → Auto Import
- Add unambiguous imports on the fly: true
- Optimize imports on the fly: true

# 代码检查
Editor → Inspections → Rust
- 启用所有推荐检查
```

## ⚡ Vim/Neovim配置

### 适合人群

- 习惯命令行操作的开发者
- 追求极致效率的用户
- 服务器环境开发

### 基础配置

#### 使用vim-plug管理插件

```vim
" ~/.vimrc 或 ~/.config/nvim/init.vim

" 插件管理
call plug#begin('~/.vim/plugged')

" Rust支持
Plug 'rust-lang/rust.vim'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'dense-analysis/ale'

" 文件管理
Plug 'preservim/nerdtree'
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'

" 状态栏
Plug 'vim-airline/vim-airline'
Plug 'vim-airline/vim-airline-themes'

call plug#end()

" Rust配置
let g:rustfmt_autosave = 1
let g:rust_clip_command = 'pbcopy'

" ALE配置
let g:ale_linters = {'rust': ['analyzer']}
let g:ale_fixers = {'rust': ['rustfmt']}
let g:ale_fix_on_save = 1
```

#### CoC配置

```bash
# 安装rust-analyzer
:CocInstall coc-rust-analyzer

# CoC配置文件 ~/.config/nvim/coc-settings.json
{
    "rust-analyzer.server.path": "rust-analyzer",
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### 常用快捷键

```vim
" 代码导航
nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

" 代码操作
nmap <leader>rn <Plug>(coc-rename)
nmap <leader>f  <Plug>(coc-format-selected)

" 错误导航
nmap <silent> [g <Plug>(coc-diagnostic-prev)
nmap <silent> ]g <Plug>(coc-diagnostic-next)
```

## 🌐 在线开发环境

### Rust Playground

**官方在线编辑器**：[https://play.rust-lang.org/](https://play.rust-lang.org/)

**特点**：
- ✅ 无需安装，即开即用
- ✅ 支持多个Rust版本
- ✅ 可以分享代码片段
- ✅ 支持外部crate（有限）

**使用场景**：
- 快速测试代码片段
- 学习和实验新特性
- 分享代码示例
- 面试编程题

### GitHub Codespaces

**云端开发环境**：
- 完整的VS Code体验
- 预配置的Rust环境
- 与GitHub仓库集成

### Gitpod

**一键启动开发环境**：
- 基于Docker的环境
- 支持自定义配置
- 与Git仓库集成

## 🔧 开发工具对比

### 功能对比表

| 功能 | VS Code | IntelliJ | Vim/Neovim | Playground |
|------|---------|----------|------------|------------|
| **学习曲线** | 低 | 中 | 高 | 极低 |
| **启动速度** | 快 | 慢 | 极快 | 即时 |
| **内存占用** | 中 | 高 | 低 | 无 |
| **插件生态** | 丰富 | 丰富 | 丰富 | 无 |
| **调试功能** | 强 | 极强 | 中 | 无 |
| **重构支持** | 中 | 强 | 中 | 无 |
| **项目管理** | 中 | 强 | 弱 | 无 |
| **跨平台** | 是 | 是 | 是 | 是 |

### 选择建议

#### 初学者推荐：VS Code

**理由**：
- 学习成本低
- 社区支持好
- 插件丰富
- 免费开源

#### 专业开发：IntelliJ IDEA/CLion

**理由**：
- 功能最全面
- 大型项目支持好
- 重构功能强大
- 调试体验优秀

#### 高效开发：Vim/Neovim

**理由**：
- 操作效率极高
- 资源占用少
- 高度可定制
- 服务器友好

#### 快速测试：Rust Playground

**理由**：
- 无需安装
- 快速验证想法
- 方便分享
- 学习友好

## 🎨 主题和美化

### VS Code主题推荐

```bash
# 安装流行主题
code --install-extension zhuangtongfa.Material-theme
code --install-extension PKief.material-icon-theme
code --install-extension dracula-theme.theme-dracula
```

### 字体推荐

**编程字体特点**：
- 等宽字符
- 清晰的字符区分
- 支持连字符

**推荐字体**：
- [Fira Code](https://github.com/tonsky/FiraCode) - 支持连字符
- [JetBrains Mono](https://www.jetbrains.com/lp/mono/) - 专为开发设计
- [Source Code Pro](https://github.com/adobe-fonts/source-code-pro) - Adobe开源
- [Cascadia Code](https://github.com/microsoft/cascadia-code) - 微软开源

### 配置示例

```json
{
    "editor.fontFamily": "'Fira Code', 'JetBrains Mono', monospace",
    "editor.fontLigatures": true,
    "editor.fontSize": 14,
    "workbench.colorTheme": "Material Theme",
    "workbench.iconTheme": "material-icon-theme"
}
```

## 🛠️ 辅助工具

### 命令行工具

#### bat - 更好的cat

```bash
# 安装
cargo install bat

# 使用
bat src/main.rs  # 语法高亮显示文件
```

#### exa - 更好的ls

```bash
# 安装
cargo install exa

# 使用
exa -la  # 彩色文件列表
```

#### ripgrep - 更快的grep

```bash
# 安装
cargo install ripgrep

# 使用
rg "pattern" src/  # 快速搜索代码
```

### Git工具

#### gitui - 终端Git界面

```bash
# 安装
cargo install gitui

# 使用
gitui  # 启动Git TUI
```

#### delta - 更好的git diff

```bash
# 安装
cargo install git-delta

# 配置git使用delta
git config --global core.pager delta
git config --global interactive.diffFilter 'delta --color-only'
```

## 📊 性能监控工具

### cargo-watch - 自动重新编译

```bash
# 安装
cargo install cargo-watch

# 使用
cargo watch -x run      # 文件变化时自动运行
cargo watch -x test     # 文件变化时自动测试
cargo watch -x clippy   # 文件变化时自动检查
```

### cargo-expand - 宏展开

```bash
# 安装
cargo install cargo-expand

# 使用
cargo expand            # 展开所有宏
cargo expand main       # 展开main函数中的宏
```

### cargo-bloat - 分析二进制大小

```bash
# 安装
cargo install cargo-bloat

# 使用
cargo bloat --release   # 分析release版本的大小
```

## 🧪 实践练习

### 练习1：配置VS Code

1. 安装VS Code
2. 安装rust-analyzer插件
3. 创建测试项目并验证功能

```bash
# 创建项目
cargo new tool_test
cd tool_test

# 用VS Code打开
code .
```

### 练习2：调试配置

1. 在代码中设置断点
2. 启动调试会话
3. 查看变量值和调用栈

```rust
// src/main.rs
fn main() {
    let x = 42;
    let y = x * 2;  // 在此行设置断点
    println!("Result: {}", y);
}
```

### 练习3：工具链验证

```bash
# 验证所有工具正常工作
cargo fmt      # 格式化代码
cargo clippy   # 代码检查
cargo test     # 运行测试
cargo doc      # 生成文档
```

## 📚 推荐资源

### 官方文档
- [rust-analyzer用户手册](https://rust-analyzer.github.io/manual.html)
- [VS Code Rust扩展文档](https://code.visualstudio.com/docs/languages/rust)

### 社区资源
- [Awesome Rust Tools](https://github.com/rust-unofficial/awesome-rust#development-tools)
- [Rust开发工具对比](https://areweideyet.com/)

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 选择适合自己的开发工具
- [ ] 配置基本的开发环境
- [ ] 安装和配置必要的插件
- [ ] 设置代码格式化和检查
- [ ] 配置调试环境
- [ ] 了解常用的辅助工具
- [ ] 能够高效地编写和调试Rust代码

---

**工具配置完成！** 🛠️ 现在你拥有了一个高效的Rust开发环境。

[← 上一节：Rust安装和配置](./01-installation.md) | [下一节：第一个Rust程序 →](./03-hello-world.md)