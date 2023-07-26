pub mod config;
pub mod model;

use super::client::Client;
use super::entry_point::{EntryPoint, Function, Version};
use anyhow::Result;
use config::ChatConfig;
use model::{CompletionRequest, CompletionResponse, Message};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ChatBuilder {
    version: Option<Version>,
    config: Option<ChatConfig>,
    request: Option<Vec<Message>>,
}

impl ChatBuilder {
    pub fn version(mut self, value: Version) -> Self {
        self.version = Some(value);
        self
    }
    pub fn config(mut self, value: ChatConfig) -> Self {
        self.config = Some(value);
        self
    }
    pub fn request(mut self, value: Vec<Message>) -> Self {
        self.request = Some(value);
        self
    }
    pub fn build(&self) -> Result<Chat> {
        let version = self.version.unwrap_or(Default::default());
        let config = match self.config.clone() {
            Some(val) => val,
            _ => return Err(anyhow::anyhow!("Config must be set.")),
        };
        let entry_point = EntryPoint::default()
            .set_version(version)
            .set_function(Function::CreateChatCompletion);
        let request = match &self.request {
            Some(val) => val.clone(),
            _ => return Err(anyhow::anyhow!("Request must be set.")),
        };
        Ok(Chat {
            entry_point,
            config,
            request,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chat {
    entry_point: EntryPoint,
    config: ChatConfig,
    request: Vec<Message>,
}

impl Chat {
    pub fn new(config: ChatConfig, request: Vec<Message>) -> Self {
        Self {
            entry_point: Default::default(),
            config,
            request,
        }
    }
    pub fn builder() -> ChatBuilder {
        Default::default()
    }
    async fn execute(&self, client: &Client) -> Result<CompletionResponse> {
        let request = CompletionRequest {
            model: self.config.model.as_ref().to_string(),
            temperature: self.config.temperature,
            messages: self.request.clone(),
        };
        let res = client.post(&self.entry_point.path(), request).await?;
        let res: CompletionResponse = serde_json::from_str(res.text().await?.as_str())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::super::client::Config;
    use super::*;
    #[test]
    fn builder() -> Result<()> {
        let builder = ChatBuilder::default();
        assert!(builder.clone().build().is_err());
        let builder = builder.config(ChatConfig::default()).request(vec![]);
        assert!(builder.build().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn post() -> Result<()> {
        let client = Client::new(Config::from_env()?)?;
        let res = Chat::builder()
            .config(Default::default())
            .request(vec!["hi".into()])
            .build()?
            .execute(&client)
            .await;
        assert!(res.is_ok());
        Ok(())
    }
}
