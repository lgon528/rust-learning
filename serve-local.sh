#!/bin/bash

# 本地开发服务器启动脚本
# 自动使用本地URL覆盖GitHub Pages的site_url

set -e

echo "🚀 启动Rust Learning本地开发服务器..."

# 激活虚拟环境
if [ -d "venv" ]; then
    echo "📦 激活Python虚拟环境..."
    source venv/bin/activate
else
    echo "❌ 虚拟环境不存在，请先运行:"
    echo "   python3 -m venv venv"
    echo "   source venv/bin/activate"
    echo "   pip install -r requirements.txt"
    exit 1
fi

# 检查依赖
echo "🔍 检查依赖..."
if ! pip list | grep -q mkdocs; then
    echo "📦 安装MkDocs依赖..."
    pip install -r requirements.txt
fi

# 启动本地服务器，使用dev_addr和自定义site_url
echo "🌐 启动MkDocs服务器..."
echo "📍 本地访问地址: http://127.0.0.1:8002/rust-learning/"
echo "⚡ 实时重载已启用"
echo "🛑 按 Ctrl+C 停止服务器"
echo ""

# 使用环境变量覆盖site_url，并指定自定义端口
SITE_URL_OVERRIDE="http://127.0.0.1:8002/rust-learning/" mkdocs serve \
    --dev-addr=127.0.0.1:8002 \
    --livereload
