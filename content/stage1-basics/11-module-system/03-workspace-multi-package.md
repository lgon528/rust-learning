# 工作空间和多包项目

## 学习目标

通过本节学习，你将掌握：

- 理解工作空间的概念和优势
- 掌握工作空间的创建和配置
- 学会管理多包项目的依赖关系
- 掌握工作空间的构建和测试
- 理解包之间的通信和集成
- 学会工作空间的最佳实践

## 工作空间概述

### 什么是工作空间

工作空间（Workspace）是 Cargo 提供的一种管理多个相关包的机制，允许你：

- **统一管理**：在一个根目录下管理多个包
- **共享依赖**：所有包共享同一个 `Cargo.lock` 文件
- **统一构建**：可以一次性构建所有包
- **依赖优化**：避免重复编译相同的依赖
- **版本一致性**：确保所有包使用一致的依赖版本

### 工作空间的优势

1. **减少编译时间**：共享构建缓存和依赖
2. **简化依赖管理**：统一的版本控制
3. **便于开发**：相关包的集中开发
4. **CI/CD 友好**：统一的测试和部署流程
5. **代码重用**：包之间可以轻松共享代码

## 创建工作空间

### 基本工作空间结构

```
my-workspace/
├── Cargo.toml              # 工作空间配置
├── Cargo.lock              # 统一的依赖锁定
├── README.md
├── .gitignore
├── app/                    # 应用程序包
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── core/                   # 核心库包
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── utils/                  # 工具库包
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── target/                 # 共享构建目录
    ├── debug/
    └── release/
```

### 创建工作空间

```bash
# 1. 创建工作空间目录
mkdir my-workspace
cd my-workspace

# 2. 创建工作空间配置文件
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "app",
    "core",
    "utils",
]
EOF

# 3. 创建各个包
cargo new app
cargo new core --lib
cargo new utils --lib
```

## 工作空间配置

### 基本配置

```toml
# 根目录 Cargo.toml
[workspace]
# 指定工作空间成员
members = [
    "app",
    "core",
    "utils",
    "tools/*",           # 通配符匹配
]

# 排除某些目录
exclude = [
    "old-projects/*",
    "experiments",
]

# 默认成员（cargo 命令的默认目标）
default-members = ["app"]

# 解析器版本
resolver = "2"
```

### 共享依赖配置

```toml
# 工作空间级别的依赖
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
clap = "4.0"
tracing = "0.1"
tracing-subscriber = "0.3"

# 工作空间级别的元数据
[workspace.metadata]
authors = ["Team <team@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/team/my-workspace"

# 工作空间级别的配置
[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
```

### 成员包配置

```toml
# app/Cargo.toml
[package]
name = "app"
version.workspace = true      # 继承工作空间版本
edition.workspace = true      # 继承工作空间版本
authors.workspace = true      # 继承工作空间作者
license.workspace = true      # 继承工作空间许可证

[dependencies]
# 工作空间内部依赖
core = { path = "../core" }
utils = { path = "../utils" }

# 使用工作空间共享依赖
serde.workspace = true
tokio = { workspace = true, features = ["macros"] }
anyhow.workspace = true
clap.workspace = true

# 包特有的依赖
config = "0.13"
```

```toml
# core/Cargo.toml
[package]
name = "core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Core business logic"

[dependencies]
utils = { path = "../utils" }
serde.workspace = true
anyhow.workspace = true
tracing.workspace = true

# 可选特性
[features]
default = ["std"]
std = []
async = ["tokio"]

[dependencies.tokio]
workspace = true
optional = true
features = ["rt", "sync"]
```

## 包间依赖管理

### 内部依赖

```toml
# 路径依赖
[dependencies]
core = { path = "../core" }
utils = { path = "../utils", features = ["extra"] }

# 可选的内部依赖
optional-lib = { path = "../optional-lib", optional = true }

# 开发依赖
[dev-dependencies]
test-utils = { path = "../test-utils" }
```

### 版本管理策略

```toml
# 策略 1: 统一版本
[workspace.package]
version = "0.1.0"

# 所有包使用相同版本
[package]
version.workspace = true

# 策略 2: 独立版本
[package]
name = "stable-lib"
version = "1.0.0"        # 稳定库使用独立版本

[package]
name = "experimental-lib"
version = "0.1.0"        # 实验性库使用独立版本
```

### 特性传播

```toml
# core/Cargo.toml
[features]
default = ["std"]
std = []
async = ["tokio"]
serde_support = ["serde"]

# app/Cargo.toml
[dependencies]
core = { path = "../core", features = ["async", "serde_support"] }

# 条件特性
[features]
full = ["core/async", "core/serde_support"]
```

## 实际项目示例

### Web 应用工作空间

```toml
# Cargo.toml (根目录)
[workspace]
members = [
    "web-server",           # Web 服务器
    "api-client",           # API 客户端
    "shared-models",        # 共享数据模型
    "database",             # 数据库层
    "auth",                 # 认证模块
    "utils",                # 工具库
    "cli-tools/*",          # CLI 工具集
]

default-members = ["web-server"]
resolver = "2"

[workspace.dependencies]
# Web 框架
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = "0.5"

# 数据库
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 配置
config = "0.14"

# 测试
tokio-test = "0.4"

[workspace.metadata]
authors = ["Web Team <web-team@company.com>"]
license = "MIT"
repository = "https://github.com/company/web-app"
```

### 共享模型包

```rust
// shared-models/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
```

```toml
# shared-models/Cargo.toml
[package]
name = "shared-models"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Shared data models for the web application"

[dependencies]
serde.workspace = true
chrono = { version = "0.4", features = ["serde"] }
```

### 数据库层

```rust
// database/src/lib.rs
use anyhow::Result;
use shared_models::{User, CreateUserRequest};
use sqlx::{PgPool, Row};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
    
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let row = sqlx::query(
            "INSERT INTO users (username, email, password_hash) 
             VALUES ($1, $2, $3) 
             RETURNING id, username, email, created_at"
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(hash_password(&request.password)?)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })
    }
    
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, username, email, created_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            created_at: r.get("created_at"),
        }))
    }
}

fn hash_password(password: &str) -> Result<String> {
    // 实际项目中应该使用 bcrypt 或 argon2
    Ok(format!("hashed_{}", password))
}
```

```toml
# database/Cargo.toml
[package]
name = "database"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Database access layer"

[dependencies]
shared-models = { path = "../shared-models" }
sqlx.workspace = true
anyhow.workspace = true
tracing.workspace = true
```

### Web 服务器

```rust
// web-server/src/main.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use database::Database;
use shared_models::{ApiResponse, CreateUserRequest, User};
use std::sync::Arc;
use tracing::{info, error};

type AppState = Arc<Database>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/myapp".to_string());
    
    let db = Arc::new(Database::new(&database_url).await?);
    
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/health", get(health_check))
        .with_state(db);
    
    info!("Starting server on http://0.0.0.0:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn create_user(
    State(db): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    match db.create_user(request).await {
        Ok(user) => {
            info!("Created user: {}", user.username);
            Ok(Json(ApiResponse::success(user)))
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn get_user(
    State(db): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    match db.get_user_by_id(id).await {
        Ok(Some(user)) => Ok(Json(ApiResponse::success(user))),
        Ok(None) => Ok(Json(ApiResponse::error("User not found".to_string()))),
        Err(e) => {
            error!("Failed to get user: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn health_check() -> &'static str {
    "OK"
}
```

```toml
# web-server/Cargo.toml
[package]
name = "web-server"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Web server application"

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
# 工作空间内部依赖
shared-models = { path = "../shared-models" }
database = { path = "../database" }

# 外部依赖
axum.workspace = true
tokio.workspace = true
tower.workspace = true
tower-http.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[dev-dependencies]
tokio-test.workspace = true
```

## 工作空间命令

### 构建和运行

```bash
# 构建所有包
cargo build
cargo build --release

# 构建特定包
cargo build -p web-server
cargo build -p database

# 运行特定包
cargo run -p web-server
cargo run -p cli-tools --bin migrate

# 检查所有包
cargo check
cargo check -p shared-models
```

### 测试

```bash
# 测试所有包
cargo test

# 测试特定包
cargo test -p database
cargo test -p shared-models

# 运行集成测试
cargo test --test integration

# 并行测试控制
cargo test -- --test-threads=1
```

### 依赖管理

```bash
# 查看工作空间依赖树
cargo tree

# 查看特定包的依赖
cargo tree -p web-server

# 更新依赖
cargo update
cargo update -p serde

# 检查重复依赖
cargo tree --duplicates
```

### 发布管理

```bash
# 打包特定包
cargo package -p shared-models

# 发布特定包
cargo publish -p shared-models

# 批量发布（需要脚本）
for pkg in shared-models database; do
    cargo publish -p $pkg
    sleep 10  # 等待 crates.io 索引更新
done
```

## 高级配置

### 条件编译

```toml
# 根据目标平台包含不同的包
[workspace]
members = [
    "core",
    "cli",
]

# 仅在 Unix 系统上包含
[target.'cfg(unix)'.workspace]
members = ["unix-specific"]

# 仅在 Windows 系统上包含
[target.'cfg(windows)'.workspace]
members = ["windows-specific"]
```

### 自定义构建脚本

```toml
# 工作空间级别的构建依赖
[workspace.build-dependencies]
cc = "1.0"
pkg-config = "0.3"
```

### 补丁管理

```toml
# 工作空间级别的补丁
[patch.crates-io]
# 使用本地版本替换 crates.io 版本
my-dependency = { path = "../my-dependency" }

# 使用 Git 版本替换
problematic-crate = { git = "https://github.com/user/fixed-version" }
```

## 最佳实践

### 项目组织

1. **逻辑分层**：按功能层次组织包
   ```
   workspace/
   ├── core/           # 核心业务逻辑
   ├── api/            # API 层
   ├── web/            # Web 界面
   ├── cli/            # 命令行工具
   ├── shared/         # 共享组件
   └── tests/          # 集成测试
   ```

2. **依赖方向**：保持清晰的依赖方向
   ```
   cli ──→ core ──→ shared
   api ──→ core ──→ shared
   web ──→ api
   ```

3. **版本策略**：
   - 公共库使用独立版本
   - 应用程序可以共享版本
   - 实验性包使用 0.x 版本

### 依赖管理

```toml
# 好的做法：使用工作空间依赖
[workspace.dependencies]
common-dep = "1.0"

[dependencies]
common-dep.workspace = true

# 避免：在每个包中重复定义
# [dependencies]
# common-dep = "1.0"  # 不好
```

### 特性设计

```toml
# 核心库的特性设计
[features]
default = ["std"]
std = []                    # 标准库支持
alloc = []                  # 仅 alloc 支持
no-std = []                 # no_std 支持
async = ["tokio"]           # 异步支持
serde = ["dep:serde"]       # 序列化支持

# 应用程序的特性设计
[features]
default = ["full"]
full = ["core/async", "core/serde", "database"]
minimal = ["core/std"]
```

### 测试策略

```rust
// tests/integration_test.rs
// 工作空间级别的集成测试
use database::Database;
use shared_models::CreateUserRequest;

#[tokio::test]
async fn test_user_creation_flow() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    let user = db.create_user(request).await.unwrap();
    assert_eq!(user.username, "testuser");
    
    let retrieved = db.get_user_by_id(user.id).await.unwrap();
    assert!(retrieved.is_some());
}
```

### CI/CD 配置

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      # 检查格式
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      # 检查代码质量
      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      # 运行测试
      - name: Run tests
        run: cargo test --all-features
      
      # 检查文档
      - name: Check docs
        run: cargo doc --all-features --no-deps
```

## 常见问题和解决方案

### 1. 循环依赖

```toml
# 问题：A 依赖 B，B 依赖 A
# 解决方案：提取共享代码到新包

# 重构前
package-a → package-b
package-b → package-a  # 循环依赖

# 重构后
package-a → shared
package-b → shared
```

### 2. 版本冲突

```bash
# 查看冲突
cargo tree --duplicates

# 使用统一版本
[workspace.dependencies]
problematic-dep = "1.0"

# 或使用 patch
[patch.crates-io]
problematic-dep = { git = "https://github.com/user/fixed" }
```

### 3. 构建时间优化

```toml
# 使用更少的代码生成单元
[profile.dev]
codegen-units = 1

# 启用增量编译
[profile.dev]
incremental = true

# 优化依赖编译
[profile.dev.package."*"]
opt-level = 1
```

## 学习检查清单

- [ ] 理解工作空间的概念和优势
- [ ] 掌握工作空间的创建和配置
- [ ] 熟练使用工作空间级别的依赖管理
- [ ] 理解包间依赖关系的设计
- [ ] 掌握工作空间的构建和测试命令
- [ ] 了解版本管理策略
- [ ] 能够设计合理的项目结构
- [ ] 掌握特性传播和条件编译
- [ ] 了解工作空间的最佳实践
- [ ] 能够解决常见的依赖问题

## 扩展阅读

- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo Workspace Best Practices](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)