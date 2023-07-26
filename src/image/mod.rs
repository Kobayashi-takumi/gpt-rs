pub mod model;

use super::client::Client;
use super::entry_point::{EntryPoint, Function, Version};
use anyhow::Result;
use model::{CreateImageRequest, CreateImageResponse, CreateImageResponseDto, Url, B64};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreateImageBuilder {
    version: Option<Version>,
    request: Option<CreateImageRequest>,
}

impl CreateImageBuilder {
    pub fn version(mut self, value: Version) -> Self {
        self.version = Some(value);
        self
    }
    pub fn request(mut self, value: CreateImageRequest) -> Self {
        self.request = Some(value);
        self
    }
    pub fn build(&self) -> Result<CreateImage> {
        let version = self.version.unwrap_or(Default::default());
        let entry_point = EntryPoint::default()
            .set_version(version)
            .set_function(Function::CreateImage);
        let request = match &self.request {
            Some(val) => val.clone(),
            _ => return Err(anyhow::anyhow!("Request must be set.")),
        };
        Ok(CreateImage {
            entry_point,
            request,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateImage {
    entry_point: EntryPoint,
    request: CreateImageRequest,
}

impl CreateImage {
    pub fn new(request: CreateImageRequest) -> Self {
        let entry_point = EntryPoint::default().set_function(Function::CreateImage);
        Self {
            entry_point,
            request,
        }
    }
    pub fn builder() -> CreateImageBuilder {
        Default::default()
    }
    pub async fn execute(&self, client: &Client) -> Result<CreateImageResponse> {
        let res = client
            .post(&self.entry_point.path(), self.request.clone())
            .await?;
        let res = res.text().await?;
        let res = match &self.request.response_format {
            Some(val) if val == &model::Format::B64_Json => {
                let res: CreateImageResponseDto<B64> = serde_json::from_str(res.as_str())?;
                res.into()
            }
            _ => {
                let res: CreateImageResponseDto<Url> = serde_json::from_str(res.as_str())?;
                res.into()
            }
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::super::client::Config;
    use super::*;
    #[test]
    fn builder() -> Result<()> {
        let builder = CreateImageBuilder::default();
        assert!(builder.clone().build().is_err());
        let builder = builder.request("".into());
        assert!(builder.build().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn post() -> Result<()> {
        let client = Client::new(Config::from_env()?)?;
        // Json
        let res = CreateImage::builder()
            .request(CreateImageRequest {
                prompt: "doc".to_string(),
                n: 1,
                size: Default::default(),
                response_format: Some(model::Format::B64_Json),
                user: None,
            })
            .build()?
            .execute(&client)
            .await;
        assert!(res.is_ok());
        // URL
        let res = CreateImage::builder()
            .request("doc".into())
            .build()?
            .execute(&client)
            .await;
        assert!(res.is_ok());
        Ok(())
    }
}
