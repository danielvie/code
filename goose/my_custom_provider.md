# Adding a Custom API Provider to Goose

This guide details how to integrate an LLM Provider that implements its own custom API into the Goose backend.

## Understanding Custom APIs

### What is a Custom API?
In the LLM ecosystem, OpenAI defined a robust and widely adopted API specification (e.g., `/v1/chat/completions`). Because of its popularity, many other systems—like vLLM, Ollama, and Groq—implement "OpenAI-compatible" API layers, allowing you to use standard clients just overriding the base URL. 

A **Custom API** is an API that deviates from this standard specification. Instead of the typical standard schemas, they might:
- Have differing schema payloads (e.g., nesting prompts in different structures, separate endpoints for tool calling).
- Use proprietary authentication mechanisms (custom headers, cryptographic signatures, or short-lived enterprise SSO tokens).
- Stream responses using deeply different Server-Sent Event (SSE) formats, or alternate transport formats entirely.

### Example: Custom API vs. OpenAI API
If you are passing a system prompt and a user prompt:

**OpenAI API Request Payload:**
```json
{
  "model": "gpt-4",
  "messages": [
    { "role": "system", "content": "You are a helpful assistant" },
    { "role": "user", "content": "Hello!" }
  ],
  "temperature": 0.7
}
```

**Custom API (E.g. AcmeCorp Internal LLM) Request Payload:**
```json
{
  "engine_id": "acme-v2",
  "inference_parameters": {
    "temp": 0.7
  },
  "prompt_context": "You are a helpful assistant",
  "query": "Hello!"
}
```

Because of these structural differences, a Custom API requires a custom Rust map layer (provider implementation) in Goose to parse Goose’s internal Message structures (`RMCP`) into the Custom API’s required schema, and subsequently translate the Custom API's response back into Goose's formats.

---

## Structure and Responsibilities of Interest

If you want to add a custom backend, you should be familiar with the following files inside `crates/goose/src/providers/`:

- **`base.rs`**: The core contract. Contains the `Provider` traits and `ProviderDef` traits. Any custom provider must implement these structs. It defines what a `Message` is, the `ProviderUsage` tally, and the `MessageStream`.
- **`api_client.rs`**: Provides an `ApiClient` that wraps standard HTTP request execution (using `reqwest`), automatic retries, header management, and SSE stream parsing mechanisms.
- **`<your_custom_provider>.rs`**: (The file you will create). It will hold the struct and implementations mapping Goose formats -> Custom API formats.
- **`mod.rs`**: Where module definitions are exported.
- **`init.rs`**: Contains the registry where your custom provider will be initialized and activated for Goose to recognize it.

---

## What to Look For and How to Get It

When exploring the implementation for your Custom API, you'll need the following information:

1. **Authentication specifics** 
   - *Tip:* Does it use a `Bearer <token>` header, a custom `x-api-key: <token>` header, or mTLS? Look at `api_client`'s `AuthMethod` enum for available options or inject custom headers manually.
2. **Endpoint URLs**
   - *Tip:* Pinpoint exactly what endpoint handles chat inferences (and specific endpoints for generation vs. embeddings vs. models limits).
3. **Payload Structure**
   - *Tip:* Inspect existing code where you call this API (Python, cURL, or Postman) to note exactly how text inputs and function/tool configurations are placed in the request body.
4. **Streaming Response Delimiters** 
   - *Tip:* Make a simple `curl` request to the streaming endpoint in your terminal. Observe how chunks yield. Are they `data: { ... }` blocks? Look for which JSON keys hold the incremental `text` or `tool_call` tokens natively.

---

## Implementing the Custom Provider

### 1. Create your Provider Code
Create `crates/goose/src/providers/my_custom.rs`. You'll define your struct containing state, and map responses with `ProviderDef` and `Provider`.

Here is a simplified example illustrating a mapping of Goose requests to a custom target:

```rust
use super::base::{Provider, ProviderDef, ProviderMetadata, MessageStream, Usage, ProviderUsage};
use super::api_client::{ApiClient, AuthMethod};
use crate::model::ModelConfig;
use super::errors::ProviderError;
use rmcp::model::{Message, Tool};
use async_trait::async_trait;
use serde_json::json;
use anyhow::Result;

pub struct MyCustomProvider {
    api_client: ApiClient,
    model: ModelConfig,
}

impl ProviderDef for MyCustomProvider {
    type Provider = Self;

    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "my_custom",
            "My Custom Provider",
            "An internal corporate LLM wrapper",
            "acme-v1",
            vec!["acme-v1", "acme-v2"],
            "https://internal.acme.corp/docs",
            vec![super::base::ConfigKey::new("ACME_API_KEY", true, true, None, true)],
        )
    }

    fn from_env(
        model: ModelConfig,
        _extensions: Vec<crate::config::ExtensionConfig>,
    ) -> futures::future::BoxFuture<'static, Result<Self::Provider>> {
        Box::pin(async move {
            let api_key = std::env::var("ACME_API_KEY")
                .unwrap_or_else(|_| "dummy_key".to_string());
            
            let mut api_client = ApiClient::new(
                "https://api.internal.acme.corp",
                AuthMethod::Bearer(api_key),
            )?;
            
            Ok(Self { api_client, model })
        })
    }
}

#[async_trait]
impl Provider for MyCustomProvider {
    fn get_name(&self) -> &str { "my_custom" }
    
    fn get_model_config(&self) -> ModelConfig { self.model.clone() }

    async fn stream(
        &self,
        _model_config: &ModelConfig,
        _session_id: &str,
        system: &str,
        messages: &[Message],
        _tools: &[Tool],
    ) -> Result<MessageStream, ProviderError> {
        // 1. Transform Goose `messages` to Custom Input Format
        let query_text = messages.last()
            .and_then(|m| m.content.first())
            .map(|c| c.as_text().unwrap_or_default())
            .unwrap_or_default()
            .to_string();

        let payload = json!({
            "engine_id": self.model.model_name,
            "prompt_context": system,
            "query": query_text
        });

        // 2. Transmit to API
        let response = self.api_client
            .post("/v1/generate")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ProviderError::ApiError(e.to_string()))?;

        // 3. Transform API Response back to Goose's MessageStream
        // Note: For SSE endpoints, look into using `post_stream` via `ApiClient`.
        // This is a simplified static string fetch.
        let text = response.text().await.unwrap_or_default();
        let message = Message::assistant().with_text(&text);
        let usage = ProviderUsage::new(self.model.model_name.clone(), Usage::default());
        
        Ok(super::base::stream_from_single_message(message, usage))
    }
}
```

### 2. Register the Provider
You must register your file with the module system.

**In `crates/goose/src/providers/mod.rs`:**
```rust
pub mod my_custom;
```

**In `crates/goose/src/providers/init.rs`:**
Inside the `init_registry` logic, add your provider registration to the block so Goose knows it exists:
```rust
async fn init_registry() -> RwLock<ProviderRegistry> {
    let mut registry = ProviderRegistry::new().with_providers(|registry| {
        // ... other providers
        registry.register::<MyCustomProvider>(false); // Register yours!
    });
    // ...
}
```

---

## Validation: Intermediary Tests

To validate that your path is correct without spinning up the entire GUI, you can do some intermediary sanity tests.

### 1. Cargo Checks
Ensure your module is properly hooked up and rust trait boundaries are respected:
```bash
cargo check -p goose
```

### 2. Unit Testing in Rust
Add an inline module at the bottom of `my_custom.rs` to validate the payload parsing structures locally.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_custom_provider_metadata() {
        let metadata = MyCustomProvider::metadata();
        assert_eq!(metadata.name, "my_custom");
    }
}
```
Run it via:
```bash
cargo test -p goose -- my_custom
```

### 3. CLI Trial run
Set your newly required keys as environment variables and target your provider through the Goose CLI. This lets you skip the GUI and run a fast verification pass of the complete event pipeline:

```bash
ACME_API_KEY="my-test-key" cargo run --bin goose -- --provider my_custom --model acme-v1 -m "Can you hear me?"
```
If it prints out your API response successfully, your integration works and is ready to be utilized fully within Goose!
