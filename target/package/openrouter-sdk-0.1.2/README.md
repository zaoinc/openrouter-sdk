

```markdown
# OpenRouter Rust SDK 🦀

[![Crates.io](https://img.shields.io/crates/v/openrouter-sdk?style=for-the-badge&logo=rust)](https://crates.io/crates/openrouter-sdk)
[![Docs.rs](https://img.shields.io/docsrs/openrouter-sdk?style=for-the-badge&logo=docs.rs)](https://docs.rs/openrouter-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/zaoinc/openrouter-sdk?style=for-the-badge&logo=github)](https://github.com/zaoinc/openrouter-sdk)

The **official Rust interface** for [OpenRouter.ai](https://openrouter.ai), providing:

⚡ **Blazing fast** LLM inference with Rust's zero-cost abstractions  
🔒 **Type-safe** interactions with all OpenRouter models  
🌐 **Multi-modal** support (text + images)  
📡 **Streaming** responses for real-time applications  

```bash
cargo add openrouter-sdk
```

## Quick Start

1. Get your [OpenRouter API key](https://openrouter.ai/keys)
2. Add the dependency:

```toml
[dependencies]
openrouter-sdk = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

3. Basic usage:

```rust
use openrouter_sdk::{OpenRouterClient, ChatRequest, Message, Content};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenRouterClient::new(std::env::var("OPENROUTER_API_KEY")?);
    
    let response = client.chat(ChatRequest {
        model: "mistralai/mistral-7b-instruct".into(),
        messages: vec![
            Message {
                role: "user".into(),
                content: Content::Text("Explain Rust traits".into())
            }
        ],
        ..Default::default()
    }).await?;

    println!("Response: {}", response.choices[0].message.content);
    Ok(())
}
```

## Key Features

### 🔄 Model Agnostic
```rust
// Works with any OpenRouter model
let models = vec![
    "anthropic/claude-3-opus",
    "google/gemini-pro",
    "openai/gpt-4-turbo"
];
```

### 🖼️ Multi-Modal Support
```rust
Content::Multi(vec![
    ContentPart::text("Describe this image"),
    ContentPart::image("https://example.com/cat.jpg")
])
```

### 🌊 Streaming Responses
```rust
let mut stream = client.chat_stream(request).await?;
while let Some(chunk) = stream.next().await {
    print!("{}", chunk?.text());
}
```

## Advanced Usage

### Custom Configuration
```rust
let client = OpenRouterClient::new("your-key")
    .with_base_url("https://custom.endpoint") // For proxies
    .with_timeout(Duration::from_secs(30));
```

### Error Handling
```rust
match client.chat(request).await {
    Ok(res) => /* success */,
    Err(OpenRouterError::RateLimited(retry_after)) => {
        eprintln!("Rate limited. Try after: {:?}", retry_after);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

|

## Contributing

We welcome contributions! Please see:
- [GitHub Issues](https://github.com/zaoinc/openrouter-sdk/issues)
- [Contributing Guide](CONTRIBUTING.md)

## License

MIT © [ZAO Inc.](https://github.com/zaoinc)
```