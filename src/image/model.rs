use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateImageRequest {
    pub prompt: String,
    pub n: i64,
    pub size: Size,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl From<&str> for CreateImageRequest {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
impl From<String> for CreateImageRequest {
    fn from(value: String) -> Self {
        Self {
            prompt: value,
            n: 1,
            size: Default::default(),
            response_format: None,
            user: None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "256x256")]
    Small,
    #[serde(rename = "512x512")]
    Medium,
    #[default]
    #[serde(rename = "1024x1024")]
    Large,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Format {
    #[default]
    Url,
    B64_Json,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum CreateImageResponse {
    Url { created: u64, data: Vec<Url> },
    B64_Json { created: u64, data: Vec<B64> },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateImageResponseDto<T> {
    pub created: u64,
    pub data: Vec<T>,
}

impl From<CreateImageResponseDto<Url>> for CreateImageResponse {
    fn from(value: CreateImageResponseDto<Url>) -> Self {
        Self::Url {
            created: value.created,
            data: value.data,
        }
    }
}
impl From<CreateImageResponseDto<B64>> for CreateImageResponse {
    fn from(value: CreateImageResponseDto<B64>) -> Self {
        Self::B64_Json {
            created: value.created,
            data: value.data,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct B64 {
    pub b64_json: String,
}
