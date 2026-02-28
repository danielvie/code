get the manifest of the model
```powershell
curl -I https://registry.ollama.ai/v2/library/<model>/manifests/<tag>
e.g: curl -I https://registry.ollama.ai/v2/library/qwen3.5/manifests/27b
```

download model directly
```powershell
https://registry.ollama.ai/v2/library/[NOME_DO_MODELO]/blobs/sha256:[DIGEST]
```

create model
```powershell
ollama create my-model -f Modelfile
```

Modelfile
```powershell
FROM ./modelo.gguf
```

proxy
set the following env vars:

```powershell
$env:HTTP_PROXY = "http://your-proxy:port"
$env:HTTPS_PROXY = "https://your-proxy:port"
$env:NO_PROXY = "127.0.0.1,localhost"
```

