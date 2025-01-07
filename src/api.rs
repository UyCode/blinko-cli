use crate::config::Config;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde_json::json;

pub struct NoteApi {
    client: reqwest::Client,
    base_url: String,
    config: Config,
}

impl NoteApi {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:1122/api/v1/note".to_string(),
            config,
        }
    }

    pub async fn create_note(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/upsert", self.base_url);
        let body = json!({
            "content": content,
            "type": 0
        });

        self.send_request("POST", &url, Some(body)).await
    }

    pub async fn update_note(&self, id: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/update/{}", self.base_url, id);
        let body = json!({
            "content": content,
            "type": 0
        });

        self.send_request("PUT", &url, Some(body)).await
    }

    pub async fn delete_note(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/delete/{}", self.base_url, id);
        self.send_request("DELETE", &url, None).await
    }

    async fn send_request(
        &self,
        method: &str,
        url: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.is_session_valid() {
            return Err("Not authenticated. Use 'note_cli -o login' to authenticate.".into());
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!(
                "next-auth.session-token={}",
                self.config.session_token
            ))?,
        );

        let mut request = self
            .client
            .request(method.parse()?, url)
            .headers(headers)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            println!("Operation successful!");
        } else {
            println!("Error: {}", response.status());
            println!("Response: {}", response.text().await?);
        }

        Ok(())
    }
} 