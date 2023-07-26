use super::error::ErrorResponse;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as HttpClient, ClientBuilder, Response,
};
use serde::Serialize;
use std::env;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub api_key: String,
    pub organization: Option<String>,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        let api_key = match env::var("API_KEY") {
            Ok(val) => val,
            _ => return Err(anyhow::anyhow!("API_KEY must be set.")),
        };
        let organization = env::var("ORGANIZATION").ok();
        Ok(Self {
            api_key,
            organization,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    client: HttpClient,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", config.api_key.as_str()).as_str())?,
        );
        if let Some(org) = config.organization {
            headers.append(
                "OpenAI-Organization",
                HeaderValue::from_str(format!("{}", org.as_str()).as_str())?,
            );
        };
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self { client })
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        let res = self.client.get(url).send().await?;
        if !res.status().is_success() {
            let error: ErrorResponse = serde_json::from_str(res.text().await?.as_str())?;
            return Err(anyhow::anyhow!(format!(
                "{}: {}",
                error.error.code, error.error.message
            )));
        }
        Ok(res)
    }

    pub async fn post<T: Serialize>(&self, url: &str, body: T) -> Result<Response> {
        let res = self.client.post(url).json(&body).send().await?;
        if !res.status().is_success() {
            let error: ErrorResponse = serde_json::from_str(res.text().await?.as_str())?;
            return Err(anyhow::anyhow!(format!(
                "{}: {}",
                error.error.code, error.error.message
            )));
        }
        Ok(res)
    }
}
