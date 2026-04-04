# 开发任务分解计划

## Context

产品文档定义了一个 Tauri 桌面应用，用于查看 GitHub 仓库和提交记录（只读）。当前项目为空，需要从零开始搭建完整技术栈。

---

## 阶段一：项目初始化

### 1.1 创建 Tauri + React 项目

- [ ] 初始化 Tauri 项目 `npm create tauri-app@latest`
- [ ] 选择 React + TypeScript 模板
- [ ] 安装前端依赖（zustand, react-router-dom）

### 1.2 配置 Rust 依赖

- [ ] 在 `src-tauri/Cargo.toml` 添加依赖：
  ```toml
  tauri = { version = "1", features = ["api-all"] }
  reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  tokio = { version = "1", features = ["full"] }
  thiserror = "1"
  ```

### 1.3 验证项目结构

- [ ] 确认 `src-tauri/src` 目录结构
- [ ] 确认前端 `src` 目录结构
- [ ] 运行 `cargo build` 和 `npm run dev` 验证能启动

---

## 阶段二：Rust 后端核心

### 2.1 状态管理

- [ ] 创建 `src-tauri/src/state.rs`
  ```rust
  pub struct AppState {
      pub github_token: Option<String>,
  }
  ```

### 2.2 错误定义

- [ ] 创建 `src-tauri/src/error.rs`
  ```rust
  #[derive(thiserror::Error, Debug)]
  pub enum AppError {
      #[error("Network error")]
      Network,
      #[error("Unauthorized")]
      Unauthorized,
      #[error("Not logged in")]
      NotLoggedIn,
      #[error("Unknown error")]
      Unknown,
  }
  ```

### 2.3 数据模型

- [ ] 创建 `src-tauri/src/models/repo.rs` - Repo 结构体
- [ ] 创建 `src-tauri/src/models/commit.rs` - Commit 结构体
- [ ] 创建 `src-tauri/src/models/mod.rs` - 模块导出

### 2.4 HTTP Client

- [ ] 创建 `src-tauri/src/clients/http_client.rs`
  ```rust
  fn build_client(token: &str) -> reqwest::Client
  ```

### 2.5 GitHub Client

- [ ] 创建 `src-tauri/src/clients/github_client.rs`
  - `get_repos(client, token) -> Vec<Repo>`
  - `get_repo_detail(client, token, owner, repo) -> RepoDetail`
  - `get_commits(client, token, owner, repo) -> Vec<CommitSummary>`
  - `get_commit_detail(client, token, owner, repo, sha) -> CommitDetail>`

### 2.6 Services（业务逻辑）

- [ ] 创建 `src-tauri/src/services/auth_service.rs` - token 管理
- [ ] 创建 `src-tauri/src/services/repo_service.rs` - 仓库业务逻辑
- [ ] 创建 `src-tauri/src/services/commit_service.rs` - 提交业务逻辑
- [ ] 创建 `src-tauri/src/services/mod.rs` - 模块导出

### 2.7 Tauri Commands（API 入口）

- [ ] 创建 `src-tauri/src/commands/mod.rs` - 模块导出
- [ ] 实现 `set_token(state, token) -> ApiResponse<()>`
- [ ] 实现 `get_repos(state) -> ApiResponse<Vec<Repo>>`
- [ ] 实现 `get_repo_detail(state, owner, repo) -> ApiResponse<RepoDetail>`
- [ ] 实现 `get_commits(state, owner, repo) -> ApiResponse<Vec<CommitSummary>>`
- [ ] 实现 `get_commit_detail(state, owner, repo, sha) -> ApiResponse<CommitDetail>>`

### 2.8 主入口配置

- [ ] 在 `src-tauri/src/lib.rs` 注册所有 commands
- [ ] 配置 `AppState` 和 `make_app`

---

## 阶段三：React 前端

### 3.1 页面组件

- [ ] 创建 `src/pages/LoginPage.tsx` - Token 输入页
- [ ] 创建 `src/pages/RepoListPage.tsx` - 仓库列表页
- [ ] 创建 `src/pages/RepoDetailPage.tsx` - 仓库详情页
- [ ] 创建 `src/pages/CommitListPage.tsx` - 提交列表页
- [ ] 创建 `src/pages/CommitDetailPage.tsx` - Commit 详情（含 diff）

### 3.2 路由配置

- [ ] 配置 React Router：
  ```
  /              → LoginPage
  /repos         → RepoListPage
  /repo/:owner/:repo → RepoDetailPage
  /repo/:owner/:repo/commits → CommitListPage
  /commit/:owner/:repo/:sha  → CommitDetailPage
  ```

### 3.3 状态管理 (Zustand)

- [ ] 创建 `src/store/useAppStore.ts`
  ```ts
  {
    token: string | null,
    repos: Repo[],
    selectedRepo: Repo | null,
    setToken,
    setRepos,
    setSelectedRepo
  }
  ```

### 3.4 API 调用封装

- [ ] 创建 `src/api/tauri.ts` - invoke 封装
- [ ] 统一错误处理和 ApiResponse 类型

### 3.5 样式

- [ ] 添加基础 CSS 样式（简洁即可）

---

## 阶段四：联调与测试

### 4.1 完整流程测试

- [ ] 输入 Token 登录
- [ ] 获取并显示仓库列表
- [ ] 点击仓库查看详情
- [ ] 查看提交列表
- [ ] 查看单个 commit 的 diff

### 4.2 错误处理测试

- [ ] 未登录时访问其他页面
- [ ] API 请求失败提示

---

## 关键文件路径

```
src-tauri/
├── Cargo.toml
├── src/
│   ├── lib.rs              # 主入口，注册 commands
│   ├── main.rs             # 程序入口
│   ├── state.rs            # AppState
│   ├── error.rs            # AppError
│   ├── models/
│   │   ├── mod.rs
│   │   ├── repo.rs
│   │   └── commit.rs
│   ├── clients/
│   │   ├── mod.rs
│   │   ├── http_client.rs
│   │   └── github_client.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── repo_service.rs
│   │   └── commit_service.rs
│   └── commands/
│       └── mod.rs          # 所有 tauri commands

src/
├── App.tsx
├── main.tsx
├── pages/
│   ├── LoginPage.tsx
│   ├── RepoListPage.tsx
│   ├── RepoDetailPage.tsx
│   ├── CommitListPage.tsx
│   └── CommitDetailPage.tsx
├── store/
│   └── useAppStore.ts
├── api/
│   └── tauri.ts
└── types/
    └── index.ts
```

---

## 验证标准

运行 `npm run tauri dev` 后：
1. 能看到登录页面
2. 输入 GitHub Token 后能显示仓库列表
3. 点击仓库能看到详情
4. 能看到提交列表
5. 点击提交能看到 diff 内容
