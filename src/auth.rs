use crate::config::Config;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: String,
    #[serde(rename = "signinUrl")]
    pub signin_url: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CsrfResponse {
    #[serde(rename = "csrfToken")]
    pub csrf_token: String,
}

pub async fn authenticate(
    config: &mut Config,
    username: Option<String>,
    password: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let username = username.unwrap_or_else(|| "admin".to_string());
    let password = password.unwrap_or_else(|| "123456".to_string());

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    // Step 1: Get auth providers
    let providers_url = "http://localhost:1122/api/auth/providers";
    let _providers: HashMap<String, AuthProvider> = client
        .get(providers_url)
        .send()
        .await?
        .json()
        .await?;

    // Step 2: Get CSRF token
    let csrf_url = "http://localhost:1122/api/auth/csrf";
    let csrf_response: CsrfResponse = client
        .get(csrf_url)
        .send()
        .await?
        .json()
        .await?;

    // Step 3: Authenticate with credentials
    let auth_url = "http://localhost:1122/api/auth/callback/credentials";
    let params = [
        ("username", username.as_str()),
        ("password", password.as_str()),
        ("callbackUrl", "/"),
        ("redirect", "false"),
        ("csrfToken", &csrf_response.csrf_token),
        ("json", "true"),
    ];

    let response = client
        .post(auth_url)
        .form(&params)
        .send()
        .await?;

    // Try to find session token in all Set-Cookie headers
    for cookie_header in response.headers().get_all("set-cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(session_token) = extract_session_token(cookie_str) {
                config.session_token = session_token;
                config.save()?;
                return Ok(());
            }
        }
    }

    // Only print and return error if status is not successful
    if !response.status().is_success() {
        if let Ok(error_text) = response.text().await {
            eprintln!("Authentication error: {}", error_text);
        }
        return Err("Failed to authenticate".into());
    }

    Err("No session token found in response".into())
}

fn extract_session_token(cookie_str: &str) -> Option<String> {
    cookie_str
        .split(';')
        .find(|s| s.contains("next-auth.session-token="))
        .map(|s| s.trim().replace("next-auth.session-token=", ""))
} 