
```markdown
# OpenRouter Rust SDK 🦀

[![Crates.io](https://img.shields.io/crates/v/openrouter-sdk?style=for-the-badge&logo=rust)](https://crates.io/crates/openrouter-sdk)
[![Docs.rs](https://img.shields.io/docsrs/openrouter-sdk?style=for-the-badge&logo=docs.rs)](https://docs.rs/openrouter-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)

Type-safe Rust client for [OpenRouter.ai](https://openrouter.ai) with full LLM API support.

```bash
cargo add openrouter-sdk
```

## 🚀 Quickstart: Build a Chatbot in 5 Minutes

```rust
use openrouter_sdk::{OpenRouterClient, ChatRequest, Message, Content};
use std::io::{self, Write};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    let client = OpenRouterClient::new(
        std::env::var("OPENROUTER_API_KEY")?
    );

    println!("Chatbot ready! Type 'quit' to exit\n");
    
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: Content::Text("You're a helpful Rust assistant".to_string()),
    }];

    loop {
        print!("You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        messages.push(Message {
            role: "user".to_string(),
            content: Content::Text(input.to_string()),
        });

        let response = rt.block_on(client.chat(ChatRequest {
            model: "mistralai/mistral-7b-instruct".to_string(),
            messages: messages.clone(),
            temperature: Some(0.7),
            ..Default::default()
        }))?;

        if let Some(choice) = response.choices.first() {
            println!("Assistant: {}\n", choice.message.content);
            messages.push(choice.message.clone());
        }
    }
    Ok(())
}
```

### ▶️ Run it:
```bash
export OPENROUTER_API_KEY="your_api_key"
cargo run 
```

## 🔍 Key Features

- **Full Model Support**: All OpenRouter models (Mistral, Claude, GPT, etc.)
- **Conversation Memory**: Maintains chat history automatically
- **Multi-Modal Ready**: Handles text+image prompts
- **Production-Grade**: Timeouts, retries, and connection pooling

## 📚 More Examples

### Streaming Responses
```rust
let mut stream = client.chat_stream(request).await?;
while let Some(chunk) = stream.next().await {
    print!("{}", chunk?.text());
}
```

### Image Analysis
```rust
Content::Multi(vec![
    ContentPart::text("What's in this image?"),
    ContentPart::image("https://example.com/cat.jpg")
])
```

## 📦 Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
openrouter-sdk = "0.1.3"
tokio = { version = "1.0", features = ["full"] }
anywhow="1.0"
```

## 🤝 Contributing
PRs welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

💬 **Pro Tip**: The example above works as-is! Try modifying:
- `model` to switch LLMs
- `temperature` for creativity control
- System prompt for different personalities
```
