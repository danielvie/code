
`.pi/agents/models.json`
```json
{
  "providers": {
    "my-local-provider": {
      "baseUrl": "http://127.0.0.1:3500/v1",
      "apiKey": "sk-not-required",
      "api": "openai-completions",
      "models": [
        {
          "id": "gpt-5.2",
          "name": "Local GPT 5.2",
          "contextWindow": 128000,
          "maxTokens": 4096,
          "input": ["text"]
        }
      ]
    }
  }
}
```

`.pi/agents/settings.json`
```json
{
    "lastChangelogVersion": "0.54.0",
    "defaultThinkingLevel": "off",
    "defaultProvider": "my-local-provider",
    "defaultModel": "gpt-5.2"
}
```
