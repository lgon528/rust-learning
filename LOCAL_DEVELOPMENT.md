# 本地开发指南

本文档说明如何在本地运行和预览Rust Learning文档网站。

## 🚀 快速开始

### 方式一：使用本地配置文件（推荐）

```bash
# 启动本地开发服务器
source venv/bin/activate
mkdocs serve -f mkdocs.local.yml --dev-addr=127.0.0.1:8002 --livereload
```

### 方式二：使用便捷脚本

```bash
# 使用我们提供的脚本
./serve-local.sh
```

### 方式三：使用环境变量

```bash
# 手动设置环境变量
source venv/bin/activate
SITE_URL_OVERRIDE="http://127.0.0.1:8002/rust-learning/" mkdocs serve --dev-addr=127.0.0.1:8002
```

## 🌐 访问地址

本地开发服务器启动后，访问：
- **主要地址**: http://127.0.0.1:8002/rust-learning/
- **特性**: 实时重载、代码高亮、中文搜索

## 📁 配置文件说明

### mkdocs.yml
- **用途**: GitHub Pages部署的主配置
- **site_url**: https://lgon528.github.io/rust-learning/
- **环境**: 生产环境

### mkdocs.local.yml
- **用途**: 本地开发专用配置
- **site_url**: http://127.0.0.1:8002/rust-learning/
- **特性**: 继承主配置，覆盖关键设置
- **环境**: 开发环境

## 🔄 工作流程

### 1. 本地开发
```bash
# 启动本地服务器
mkdocs serve -f mkdocs.local.yml --dev-addr=127.0.0.1:8002 --livereload

# 修改文件，实时预览
# 编辑 content/**/*.md 文件
```

### 2. 测试构建
```bash
# 测试生产环境构建
mkdocs build -f mkdocs.yml

# 检查生成的文件
ls -la site/
```

### 3. GitHub部署
```bash
# 推送到main分支，自动触发GitHub Actions
git add .
git commit -m "Update documentation"
git push origin main
```

## 🛠️ 故障排除

### 虚拟环境问题
```bash
# 重新创建虚拟环境
rm -rf venv
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### 端口占用
```bash
# 使用不同端口
mkdocs serve -f mkdocs.local.yml --dev-addr=127.0.0.1:8008

# 或者查看占用端口的进程
lsof -i :8002
```

### 配置错误
```bash
# 验证配置文件
mkdocs config --config-file mkdocs.local.yml

# 检查文件路径
mkdocs serve --config-file mkdocs.local.yml --verbose
```

## 📝 开发建议

1. **使用本地配置文件**: 始终使用 `mkdocs.local.yml` 进行本地开发
2. **实时预览**: 启用 `--livereload` 选项
3. **定期测试构建**: 推送前运行 `mkdocs build -f mkdocs.yml` 测试
4. **检查链接**: 使用 `mkdocs build --strict` 检查链接完整性
5. **保持同步**: 确保 `mkdocs.local.yml` 与 `mkdocs.yml` 保持适当的同步

## 🔗 相关链接

- [MkDocs官方文档](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
- [GitHub Actions部署配置](.github/workflows/gh-pages.yml)
