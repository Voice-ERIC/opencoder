use crate::models::AppConfig;
use std::env;

/// Load configuration from environment variables
pub fn load_from_env() -> AppConfig {
    AppConfig {
        api_endpoint: env::var("API_ENDPOINT")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
        api_key: env::var("API_KEY").unwrap_or_default(),
        model_name: env::var("MODEL_NAME")
            .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
        organization_id: env::var("ORGANIZATION_ID").ok(),
        timeout_secs: env::var("TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60),
    }
}

/// Save configuration to environment file
pub fn save_to_env_file(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(".env")?;
    writeln!(file, "API_ENDPOINT={}", config.api_endpoint)?;
    writeln!(file, "API_KEY={}", config.api_key)?;
    writeln!(file, "MODEL_NAME={}", config.model_name)?;
    if let Some(org_id) = &config.organization_id {
        writeln!(file, "ORGANIZATION_ID={}", org_id)?;
    }
    writeln!(file, "TIMEOUT_SECS={}", config.timeout_secs)?;
    
    Ok(())
}
