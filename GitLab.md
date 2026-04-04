# 🧭 一、目标（明确边界）

## ✅ 要实现

* 支持 GitLab（和 GitHub 并存）
* 用户可以切换 provider
* 同样功能：

    * 仓库列表
    * commit 列表
    * commit diff

---

## ❌ 不做（这阶段）

* 不做 GitLab OAuth
* 不做 group / org 深度功能
* 不做 MR（Merge Request）

👉 先对齐 GitHub MVP 功能

---

# 🧱 二、核心改造思路（关键）

你现在的问题本质是：

> ❌ 现在写死 GitHub
> ✅ 要抽象成“平台无关”

---

## 👉 引入 Provider 抽象层（核心设计）

新增一个概念：

```rust
enum Provider {
    GitHub,
    GitLab,
}
```

---

## 👉 AppState 升级

```rust
pub struct AppState {
    pub provider: Provider,
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
}
```

---

# 📦 三、目录改造（最小侵入）

在你原结构基础上改：

```text
services/
  github.rs
  gitlab.rs   ← 新增

clients/
  github_client.rs
  gitlab_client.rs ← 新增

models/
  repo.rs   ← 通用（不变）
  commit.rs ← 通用（不变）

providers/   ← 新增（关键）
  mod.rs
  provider_trait.rs
```

---

# 🧠 四、统一接口（最重要）

## 定义 Provider Trait（核心）

```rust
#[async_trait::async_trait]
pub trait GitProvider {
    async fn get_repos(&self) -> Result<Vec<Repo>, AppError>;
    
    async fn get_repo_detail(
        &self,
        owner: String,
        repo: String,
    ) -> Result<Repo, AppError>;

    async fn get_commits(
        &self,
        owner: String,
        repo: String,
    ) -> Result<Vec<Commit>, AppError>;

    async fn get_commit_detail(
        &self,
        owner: String,
        repo: String,
        sha: String,
    ) -> Result<CommitDetail, AppError>;
}
```

👉 **关键点：**

* 前端完全不用知道 GitHub / GitLab
* Rust 内部自动分发

---

# 🔌 五、Provider 实现

---

## ✅ GitHub 实现（你已有）

```rust
pub struct GitHubProvider {
    token: String,
}
```

实现 trait：

```rust
impl GitProvider for GitHubProvider {
    async fn get_repos(...) {
        github_client::fetch_repos(...)
    }
}
```

---

## ✅ GitLab 实现（新增）

```rust
pub struct GitLabProvider {
    token: String,
}
```

---

# 🌐 六、GitLab API（重点差异）

---

## 🔑 认证方式（和 GitHub 不同）

```text
PRIVATE-TOKEN: <your_token>
```

👉 ❗不是 Bearer

---

## ✅ 1. 获取仓库

```http
GET https://gitlab.com/api/v4/projects?membership=true
```

---

## ✅ 2. 获取 commit 列表

```http
GET /projects/{id}/repository/commits
```

---

## ✅ 3. commit detail

```http
GET /projects/{id}/repository/commits/{sha}
```

---

## ❗关键差异：project_id

GitLab 不用：

```text
owner/repo
```

而是：

```text
project_id
```

👉 解决方案：

### ✅ Repo 结构扩展

```rust
pub struct Repo {
    pub id: u64,          // GitHub: repo_id
    pub name: String,
    pub full_name: String,
    pub provider_id: String, // GitLab project_id（关键）
}
```

---

# 🔄 七、统一数据模型（必须做）

---

## GitHub → Repo

```rust
Repo {
  id: github.id,
  name: github.name,
  full_name: github.full_name,
  provider_id: github.full_name, // 用 owner/repo
}
```

---

## GitLab → Repo

```rust
Repo {
  id: gitlab.id,
  name: gitlab.name,
  full_name: gitlab.path_with_namespace,
  provider_id: gitlab.id.to_string(), // 核心
}
```

---

# 🔁 八、Service 层改造（核心）

---

## ❌ 原来（错误方式）

```rust
github_service::get_repos()
```

---

## ✅ 改成（动态分发）

```rust
pub async fn get_repos(state: &AppState) -> Result<Vec<Repo>, AppError> {
    let provider = build_provider(state)?;
    provider.get_repos().await
}
```

---

## build_provider（关键函数）

```rust
fn build_provider(state: &AppState) -> Result<Box<dyn GitProvider>, AppError> {
    match state.provider {
        Provider::GitHub => {
            let token = state.github_token.clone().ok_or(AppError::Unauthorized)?;
            Ok(Box::new(GitHubProvider { token }))
        }
        Provider::GitLab => {
            let token = state.gitlab_token.clone().ok_or(AppError::Unauthorized)?;
            Ok(Box::new(GitLabProvider { token }))
        }
    }
}
```

---

# 🧩 九、commands 层（基本不变）

你原来的：

```rust
#[tauri::command]
async fn get_repos(...)
```

👉 **不用改接口**

只改内部调用：

```rust
repo_service::get_repos(state)
```

---

# 🖥️ 十、前端改造（很少）

---

## ✅ 增加 provider 切换

```ts
type Provider = "github" | "gitlab";
```

---

## 新接口：

```ts
invoke("set_provider", { provider: "gitlab" })
invoke("set_gitlab_token", { token })
```

---

## UI（最简单）

* 一个下拉框：

    * GitHub
    * GitLab

---

# ⚠️ 十一、关键坑（已经帮你踩完）

---

## ❗ 1. GitLab 项目 ID 问题

必须用：

```text
project_id
```

不能用 repo name

---

## ❗ 2. URL 编码

GitLab API：

```text
/projects/:id
```

如果用 path：

👉 必须 URL encode

---

## ❗ 3. commit diff 字段不同

GitHub：

```json
files[].patch
```

GitLab：

```json
diffs[].diff
```

👉 必须统一：

```rust
diff: String
```

---

## ❗ 4. 分页（暂时忽略）

GitLab 默认分页

👉 MVP：

```text
?per_page=50
```

---

## ❗ 5. Token 权限

GitLab Token 需要：

```text
read_api
```

---

# 🚀 十二、完成标准（GitLab 支持成功）

✔ 能切换 GitHub / GitLab
✔ GitLab repo 能加载
✔ commit 能展示
✔ diff 能显示

---

# 🧠 十三、最终架构（你现在拥有）

你现在已经升级为：

```text
        UI
         ↓
    commands
         ↓
     services
         ↓
   Provider Trait
     ↓       ↓
 GitHub   GitLab
```

👉 这是**工业级结构**

---

# ✅ 最终总结

这套方案：

✔ 不破坏你现有 GitHub 代码
✔ 最小改动支持 GitLab
✔ 统一接口（前端无感知）
✔ 可扩展（未来加 Bitbucket）

