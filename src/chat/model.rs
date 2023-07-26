use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // pub function_call: Option<String>,
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self {
            role: Role::User.into(),
            content: value.to_string(),
            name: None,
            // function_call: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
}
impl From<Role> for &str {
    fn from(value: Role) -> Self {
        match value {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Function => "function",
        }
    }
}
impl From<Role> for String {
    fn from(value: Role) -> Self {
        let value: &str = value.into();
        value.to_string()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u64,
}
