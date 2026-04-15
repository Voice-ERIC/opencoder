use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    AppConfig, AppState, ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
    DecomposeTaskRequest, DecomposeTaskResponse, Prompt, SubTask, Task, TaskStatus,
};

/// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

/// Get current API configuration
pub async fn get_config(
    State(state): State<Arc<AppState>>,
) -> Json<AppConfig> {
    let config = state.config.read().await;
    // Return config without exposing the API key fully
    let mut safe_config = config.clone();
    if !safe_config.api_key.is_empty() {
        // Mask the API key for security
        let len = safe_config.api_key.len();
        if len > 8 {
            safe_config.api_key = format!("{}...{}", 
                &safe_config.api_key[..4], 
                &safe_config.api_key[len-4..]
            );
        } else {
            safe_config.api_key = "****".to_string();
        }
    }
    Json(safe_config)
}

/// Save API configuration
pub async fn save_config(
    State(state): State<Arc<AppState>>,
    Json(config): Json<AppConfig>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut current_config = state.config.write().await;
    *current_config = config;
    
    // Try to save to .env file
    if let Err(e) = crate::config::save_to_env_file(&current_config) {
        tracing::warn!("Failed to save config to .env file: {}", e);
    }
    
    Ok(Json(json!({ "message": "Configuration saved successfully" })))
}

/// Handle chat completions (OpenAI-compatible API)
pub async fn chat_completions(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, String)> {
    let config = state.config.read().await;
    
    // Build the actual request to send to the API
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let api_url = format!("{}/chat/completions", config.api_endpoint.trim_end_matches('/'));
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", config.api_key)
            .parse()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    
    if let Some(org_id) = &config.organization_id {
        headers.insert(
            "OpenAI-Organization",
            org_id.parse().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        );
    }
    
    // Prepare the request body (OpenAI format)
    let request_body = serde_json::json!({
        "model": request.model,
        "messages": request.messages,
        "temperature": request.temperature.unwrap_or(0.7),
        "max_tokens": request.max_tokens.unwrap_or(1024),
        "stream": request.stream.unwrap_or(false),
    });
    
    let response = client
        .post(&api_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("API returned error: {}", error_text),
        ));
    }
    
    let completion_response: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    Ok(Json(completion_response))
}

/// Decompose a task into subtasks using AI
pub async fn decompose_task(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DecomposeTaskRequest>,
) -> Result<Json<DecomposeTaskResponse>, (StatusCode, String)> {
    let config = state.config.read().await;
    
    // Build a prompt for task decomposition
    let system_message = "You are an expert task planner. Break down complex tasks into clear, actionable subtasks. Each subtask should be specific, measurable, and achievable.".to_string();
    
    let user_message = format!(
        "Please break down the following task into manageable subtasks:\n\nTask: {}\n\nContext: {}\n\nPlease provide {} subtasks maximum. For each subtask, include a title and description.",
        request.task,
        request.context.unwrap_or_else(|| "No additional context provided.".to_string()),
        request.max_subtasks.unwrap_or(5)
    );
    
    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_message,
            name: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: user_message,
            name: None,
        },
    ];
    
    let chat_request = ChatCompletionRequest {
        model: config.model_name.clone(),
        messages,
        temperature: Some(0.7),
        max_tokens: Some(2048),
        stream: Some(false),
        system_prompt: None,
    };
    
    drop(config); // Release the lock before making the API call
    
    // Call the chat completion API
    let client = reqwest::Client::new();
    let config = state.config.read().await;
    let api_url = format!("{}/chat/completions", config.api_endpoint.trim_end_matches('/'));
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", config.api_key).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    
    let request_body = serde_json::json!({
        "model": config.model_name,
        "messages": chat_request.messages,
        "temperature": 0.7,
        "max_tokens": 2048,
    });
    
    let response = client
        .post(&api_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("API returned error: {}", error_text),
        ));
    }
    
    let completion: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    // Parse the AI response to extract subtasks
    // In a real implementation, you'd parse the structured response
    // Here we create a simple decomposition
    let ai_response = completion
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "Could not generate subtasks.".to_string());
    
    let mut subtasks = Vec::new();
    let lines: Vec<&str> = ai_response.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with(char::is_numeric) || line.trim().starts_with('-') || line.trim().starts_with('*') {
            subtasks.push(SubTask {
                id: uuid::Uuid::new_v4().to_string(),
                title: format!("Subtask {}", i + 1),
                description: line.trim().trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == '*').trim().to_string(),
                status: TaskStatus::Pending,
                result: None,
            });
        }
    }
    
    // If no subtasks were parsed, create a default one
    if subtasks.is_empty() {
        subtasks.push(SubTask {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Main Task".to_string(),
            description: ai_response,
            status: TaskStatus::Pending,
            result: None,
        });
    }
    
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        title: request.task.clone(),
        description: request.context.unwrap_or_default(),
        subtasks,
        status: TaskStatus::Pending,
        created_at: chrono::Utc::now(),
    };
    
    Ok(Json(DecomposeTaskResponse {
        task,
        suggestions: vec![
            "Review each subtask for clarity".to_string(),
            "Assign priorities to subtasks".to_string(),
            "Estimate time for each subtask".to_string(),
        ],
    }))
}

/// List all prompts
pub async fn list_prompts(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Prompt>> {
    let prompts = state.prompts.read().await;
    Json(prompts.clone())
}

/// Create a new prompt
pub async fn create_prompt(
    State(state): State<Arc<AppState>>,
    Json(prompt_data): Json<serde_json::Value>,
) -> Result<Json<Prompt>, StatusCode> {
    let name = prompt_data["name"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_string();
    
    let content = prompt_data["content"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_string();
    
    let system_message = prompt_data["system_message"]
        .as_str()
        .map(|s| s.to_string());
    
    let mut prompt = Prompt::new(name, content, system_message);
    
    // Extract variables from content (simple pattern: {{variable}})
    let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
    for cap in re.captures_iter(&prompt.content) {
        if let Some(var_name) = cap.get(1) {
            prompt.variables.push(var_name.as_str().to_string());
        }
    }
    
    let mut prompts = state.prompts.write().await;
    prompts.push(prompt.clone());
    
    Ok(Json(prompt))
}

/// Get a specific prompt by ID
pub async fn get_prompt(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Prompt>, StatusCode> {
    let prompts = state.prompts.read().await;
    
    prompts
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Update a prompt
pub async fn update_prompt(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(prompt_data): Json<serde_json::Value>,
) -> Result<Json<Prompt>, StatusCode> {
    let mut prompts = state.prompts.write().await;
    
    let prompt = prompts
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if let Some(name) = prompt_data["name"].as_str() {
        prompt.name = name.to_string();
    }
    
    if let Some(content) = prompt_data["content"].as_str() {
        prompt.content = content.to_string();
        
        // Update variables
        prompt.variables.clear();
        let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
        for cap in re.captures_iter(&prompt.content) {
            if let Some(var_name) = cap.get(1) {
                prompt.variables.push(var_name.as_str().to_string());
            }
        }
    }
    
    if let Some(system_message) = prompt_data["system_message"].as_str() {
        prompt.system_message = Some(system_message.to_string());
    }
    
    prompt.updated_at = chrono::Utc::now();
    
    Ok(Json(prompt.clone()))
}

/// Delete a prompt
pub async fn delete_prompt(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut prompts = state.prompts.write().await;
    
    let initial_len = prompts.len();
    prompts.retain(|p| p.id != id);
    
    if prompts.len() < initial_len {
        Ok(Json(json!({ "message": "Prompt deleted successfully" })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
