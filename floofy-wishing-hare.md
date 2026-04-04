# Git Repo Viewer - 开发任务分解计划

## 项目概述

Tauri 桌面应用，支持 GitHub 和 GitLab 仓库查看（只读）

---

## 当前状态

✅ GitHub + GitLab 双平台支持
✅ 仓库列表、详情、提交列表、commit diff 均已实现

---

## 项目结构

```
src-tauri/
├── src/
│   ├── main.rs              # 程序入口
│   ├── lib.rs              # 模块入口，注册 commands
│   ├── state.rs            # AppState (Provider, tokens, gitlab_url)
│   ├── error.rs            # AppError 定义
│   ├── models/             # 数据模型
│   │   ├── repo.rs         # Repo, RepoDetail, Owner
│   │   └── commit.rs       # CommitSummary, CommitDetail, CommitFile
│   ├── providers/          # 核心抽象层
│   │   ├── provider_trait.rs   # GitProvider trait
│   │   ├── github_provider.rs  # GitHub 实现
│   │   └── gitlab_provider.rs # GitLab 实现
│   ├── services/           # 业务逻辑
│   │   ├── auth_service.rs    # token/provider 管理
│   │   ├── repo_service.rs    # 仓库业务
│   │   └── commit_service.rs   # 提交业务
│   └── commands/           # Tauri commands
│       └── mod.rs          # set_provider, set_github_token, set_gitlab_token, set_gitlab_url, get_repos, get_repo_detail, get_commits, get_commit_detail
└── tauri.conf.json

src/                       # React 前端
├── App.tsx                # 路由配置
├── main.tsx
├── pages/
│   ├── LoginPage.tsx      # 登录页，支持选择 GitHub/GitLab，输入 URL 和 Token
│   ├── RepoListPage.tsx   # 仓库列表
│   ├── CommitListPage.tsx # 提交列表（包含 repo 详情）
│   └── CommitDetailPage.tsx # Commit 详情，包含 diff
├── store/
│   └── useAppStore.ts     # Zustand store
└── api/
    └── tauri.ts          # Tauri invoke 封装

Cargo.toml 依赖:
- tauri = { version = "1", features = ["shell-open"] }
- reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
- serde = { version = "1", features = ["derive"] }
- serde_json = "1"
- tokio = { version = "1", features = ["full"] }
- thiserror = "1"
- async-trait = "0.1"
- percent-encoding = "2"
```

---

## 架构设计

```
        UI (React)
           ↓
    commands (Tauri invoke)
           ↓
     services (业务逻辑)
           ↓
   GitProvider Trait (抽象层)
      ↓           ↓
GitHubProvider  GitLabProvider
```

---

## 关键实现细节

### Provider 抽象

```rust
#[async_trait]
pub trait GitProvider {
    async fn get_repos(&self) -> Result<Vec<Repo>, AppError>;
    async fn get_repo_detail(&self, owner: String, repo: String) -> Result<RepoDetail, AppError>;
    async fn get_commits(&self, owner: String, repo: String) -> Result<Vec<CommitSummary>, AppError>;
    async fn get_commit_detail(&self, owner: String, repo: String, sha: String) -> Result<CommitDetail, AppError>;
}
```

### GitHub API 格式
- `owner/repo` 格式
- Bearer token 认证

### GitLab API 格式
- Numeric project ID
- `PRIVATE-TOKEN` header 认证
- 单独 endpoint 获取 diff: `/projects/{id}/repository/commits/{sha}/diff`

### GitLab URL 支持
- 支持私有 GitLab 实例（如 `https://gitlab.51tyty.com`）
- Token 需要 `read_api` 权限

---

## 运行命令

```bash
# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

---

## 待优化项（可选）

1. 移除调试用的 `eprintln!` 日志
2. 添加 loading 状态优化
3. GitLab diff 添加 additions/deletions 统计
4. 错误处理优化（区分 401/403/404/500 等）

---

## 最近修复记录

1. GitLab commit diff 使用 `/repository/commits/{sha}/diff` endpoint（原本的 endpoint 不返回 diff）
2. GitLab 使用 `visibility` 字段代替 `private`
3. GitHub/GitLab 共存：通过 `full_name` (owner/repo) 传递给 GitHub，numeric ID 传递给 GitLab
4. GitLab URL 编码 project ID（包含特殊字符时需要）
