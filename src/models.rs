use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// OpenAI-compatible API endpoint
    pub api_endpoint: String,
    /// API key for authentication
    pub api_key: String,
    /// Model name to use
    pub model_name: String,
    /// Optional organization ID
    #[serde(default)]
    pub organization_id: Option<String>,
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    60
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model_name: "gpt-3.5-turbo".to_string(),
            organization_id: None,
            timeout_secs: 60,
        }
    }
}

/// Shared application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub prompts: Arc<RwLock<Vec<Prompt>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            prompts: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Prompt template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub content: String,
    pub system_message: Option<String>,
    pub variables: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Prompt {
    pub fn new(name: String, content: String, system_message: Option<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            content,
            system_message,
            variables: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Task decomposition structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub subtasks: Vec<SubTask>,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub result: Option<String>,
}

/// Chat message structures (OpenAI-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<UsageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Task decomposition request/response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeTaskRequest {
    pub task: String,
    pub context: Option<String>,
    pub max_subtasks: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeTaskResponse {
    pub task: Task,
    pub suggestions: Vec<String>,
}
