use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Gitea configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiteaConfig {
    /// Gitea server URL
    pub base_url: String,
    /// Gitea API token
    pub token: String,
    /// Default organization/user
    #[serde(default)]
    pub default_owner: Option<String>,
}

impl Default for GiteaConfig {
    fn default() -> Self {
        Self {
            base_url: "https://gitea.com".to_string(),
            token: String::new(),
            default_owner: None,
        }
    }
}

/// Shared Gitea state
#[derive(Debug, Clone)]
pub struct GiteaState {
    pub config: Arc<RwLock<GiteaConfig>>,
}

impl GiteaState {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(GiteaConfig::default())),
        }
    }
}

impl Default for GiteaState {
    fn default() -> Self {
        Self::new()
    }
}

/// Gitea repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
    pub default_branch: String,
}

/// Gitea user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

/// Gitea issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Vec<Label>,
    pub assignee: Option<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
}

/// Gitea label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

/// Gitea pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub head: PRBranchInfo,
    pub base: PRBranchInfo,
    pub merged: Option<bool>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRBranchInfo {
    pub label: String,
    pub ref_name: String,
    pub sha: String,
    pub repo: Option<Repository>,
}

/// Create issue request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    pub body: Option<String>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
}
