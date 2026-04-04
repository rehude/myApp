pub mod provider_trait;
pub mod github_provider;
pub mod gitlab_provider;

pub use provider_trait::GitProvider;
pub use github_provider::GitHubProvider;
pub use gitlab_provider::GitLabProvider;
