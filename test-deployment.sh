#!/bin/bash

# 部署测试脚本
# 验证本地和GitHub Pages配置

set -e

echo "🧪 开始部署测试..."

# 激活虚拟环境
if [ -d "venv" ]; then
    source venv/bin/activate
else
    echo "❌ 虚拟环境不存在"
    exit 1
fi

echo ""
echo "1️⃣ 测试本地配置构建..."
mkdocs build -f mkdocs.local.yml

echo ""
echo "2️⃣ 检查本地配置的site_url..."
if grep -q "http://127.0.0.1:8002/rust-learning/" site/index.html; then
    echo "✅ 本地site_url正确"
else
    echo "❌ 本地site_url错误"
    exit 1
fi

echo ""
echo "3️⃣ 测试生产配置构建..."
mkdocs build -f mkdocs.yml

echo ""
echo "4️⃣ 检查生产配置的site_url..."
if grep -q "https://lgon528.github.io/rust-learning/" site/index.html; then
    echo "✅ 生产site_url正确"
else
    echo "❌ 生产site_url错误"
    exit 1
fi

echo ""
echo "5️⃣ 启动本地开发服务器进行实时测试..."
echo "🌐 访问地址: http://127.0.0.1:8002/rust-learning/"
echo "⚡ 实时重载已启用"
echo "🛑 按 Ctrl+C 停止服务器"
echo ""

# 启动本地服务器
mkdocs serve -f mkdocs.local.yml --dev-addr=127.0.0.1:8002 --livereload
