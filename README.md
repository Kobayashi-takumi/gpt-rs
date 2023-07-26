# gpt

This crate provides a simple way to interact with the OpenAI API from Rust.

# Example

This asynchronous example uses Tokio and enables some optional features, so your Cargo.toml could look like this:

```toml
[dependencies]
gpt = "0.1"
tokio = { version = "1", features = ["full"] }
```

And then the code:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Config {
        api_key:"<Your API_KEY>".to_string(),
        organization: Some("<Your ORGANIZATION>"),
    })?;
    let res = Chat::builder()
        .config(Default::default())
        .request(vec!["hi".into()])
        .build()?
        .execute(&client)
        .await?;
}
```

## [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Config {
        api_key:"<Your API_KEY>".to_string(),
        organization: Some("<Your ORGANIZATION>".to_string()),
    })?;
    let res = Chat::builder()
        .config(Default::default())
        .request(vec!["hi".into()])
        .build()?
        .execute(&client)
        .await?;
    Ok(())
}
```

## [Create image](https://platform.openai.com/docs/api-reference/images/create)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Config {
        api_key:"<Your API_KEY>".to_string(),
        organization: Some("<Your ORGANIZATION>"),
    })?;
    let res = CreateImage::builder()
        .request("doc".into())
        .build()?
        .execute(&client)
        .await;
    Ok(())
}
```
