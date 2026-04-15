use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::models::AppState;
use crate::services::{
    CreateIssueRequest, GiteaConfig, Issue, PullRequest, Repository,
};

/// List repositories for the authenticated user or organization
pub async fn list_repos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Repository>>, (StatusCode, String)> {
    let gitea_config = state.config.read().await;
    
    // In a real implementation, you'd have a separate Gitea state
    // For now, we'll use a placeholder
    let base_url = "https://gitea.com";
    let token = "";
    
    let client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/user/repos", base_url);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("token {}", token).parse().unwrap(),
    );
    
    let response = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call Gitea API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Gitea returned error: {}", error_text),
        ));
    }
    
    let repos: Vec<Repository> = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    Ok(Json(repos))
}

/// List issues for a repository
pub async fn list_issues(
    State(state): State<Arc<AppState>>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<Json<Vec<Issue>>, (StatusCode, String)> {
    let config = state.config.read().await;
    
    // Get Gitea config from environment or use defaults
    let base_url = std::env::var("GITEA_URL").unwrap_or_else(|_| "https://gitea.com".to_string());
    let token = std::env::var("GITEA_TOKEN").unwrap_or_default();
    
    let client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/repos/{}/{}/issues", base_url, owner, repo);
    
    let mut headers = reqwest::header::HeaderMap::new();
    if !token.is_empty() {
        headers.insert(
            "Authorization",
            format!("token {}", token).parse().unwrap(),
        );
    }
    
    let response = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call Gitea API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Gitea returned error: {}", error_text),
        ));
    }
    
    let issues: Vec<Issue> = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    Ok(Json(issues))
}

/// Create a new issue
pub async fn create_issue(
    State(state): State<Arc<AppState>>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<CreateIssueRequest>,
) -> Result<Json<Issue>, (StatusCode, String)> {
    let config = state.config.read().await;
    
    let base_url = std::env::var("GITEA_URL").unwrap_or_else(|_| "https://gitea.com".to_string());
    let token = std::env::var("GITEA_TOKEN").unwrap_or_default();
    
    if token.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Gitea token is required".to_string(),
        ));
    }
    
    let client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/repos/{}/{}/issues", base_url, owner, repo);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("token {}", token).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    
    let body = serde_json::json!({
        "title": request.title,
        "body": request.body,
        "labels": request.labels,
        "assignees": request.assignees,
    });
    
    let response = client
        .post(&api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call Gitea API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Gitea returned error: {}", error_text),
        ));
    }
    
    let issue: Issue = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    Ok(Json(issue))
}

/// List pull requests for a repository
pub async fn list_pulls(
    State(state): State<Arc<AppState>>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<Json<Vec<PullRequest>>, (StatusCode, String)> {
    let config = state.config.read().await;
    
    let base_url = std::env::var("GITEA_URL").unwrap_or_else(|_| "https://gitea.com".to_string());
    let token = std::env::var("GITEA_TOKEN").unwrap_or_default();
    
    let client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/repos/{}/{}/pulls", base_url, owner, repo);
    
    let mut headers = reqwest::header::HeaderMap::new();
    if !token.is_empty() {
        headers.insert(
            "Authorization",
            format!("token {}", token).parse().unwrap(),
        );
    }
    
    let response = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to call Gitea API: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Gitea returned error: {}", error_text),
        ));
    }
    
    let pulls: Vec<PullRequest> = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Failed to parse response: {}", e)))?;
    
    Ok(Json(pulls))
}

/// Get Gitea configuration
pub async fn get_config(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let base_url = std::env::var("GITEA_URL").unwrap_or_else(|_| "https://gitea.com".to_string());
    let default_owner = std::env::var("GITEA_DEFAULT_OWNER").ok();
    
    // Don't expose the token
    let token_set = !std::env::var("GITEA_TOKEN").unwrap_or_default().is_empty();
    
    Json(json!({
        "base_url": base_url,
        "token_set": token_set,
        "default_owner": default_owner,
    }))
}

/// Save Gitea configuration
pub async fn save_config(
    State(state): State<Arc<AppState>>,
    Json(config): Json<GiteaConfig>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // In a real implementation, you'd save this to a persistent store
    // For now, we'll just log it
    tracing::info!("Saving Gitea config: base_url={}", config.base_url);
    
    // Try to save to .env file
    if let Ok(mut env_content) = std::fs::read_to_string(".env") {
        // Update existing values
        if env_content.contains("GITEA_URL=") {
            env_content = regex::Regex::new(r"GITEA_URL=.*\n")
                .unwrap()
                .replace(&env_content, &format!("GITEA_URL={}\n", config.base_url))
                .to_string();
        } else {
            env_content.push_str(&format!("GITEA_URL={}\n", config.base_url));
        }
        
        if let Err(e) = std::fs::write(".env", env_content) {
            tracing::warn!("Failed to save Gitea config to .env: {}", e);
        }
    }
    
    Ok(Json(json!({ "message": "Gitea configuration saved successfully" })))
}
