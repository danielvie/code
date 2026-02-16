

# server

```powershell
llama-server -hf unsloth/gemma-3-12b-it-GGUF:Q5_K_XL --port 8033 --chat-template chatml
```

llama-server -hf unsloth/gemma-3-12b-it-GGUF:Q5_K_XL --port 8033 --no-jinja

llama-server -hf unsloth/gemma-3-12b-it-GGUF:Q5_K_XL -c 8192 --temp 0.4 --port 8033 --chat-template chatml

llama-server -hf unsloth/gemma-3-12b-it-GGUF:Q5_K_XL -c 8192 --temp 0.4 --port 8033 --ctx-size 4096 --n-gpu-layers 99 --no-jinja


$env:localappdata/llama.cpp/unsloth_gemma-3-12b-it-GGUF_gemma-3-12b-it-UD-Q5_K_XL.gguf




curl http://127.0.0.1:8033/v1/chat/completions -H "Content-Type: application/json" -d '{
    "model": "gemma-3-12b-it",
    "messages": [{"role": "user", "content": "Say hello in JSON format"}]
  }'
