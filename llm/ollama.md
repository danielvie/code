# useful commands

*create new model in ollama*
```powershell
ollama create modelname:tag -f ./Modelfile
```

*run nvidia-smi frequently*
```powershell
while($true) { Clear-Host; nvidia-smi | select -first 12; Sleep 1 }
```

**example Modelfile**
```dockerfile
FROM ./Qwen2.5-Coder-14B-Instruct-Q5_K_M.gguf

TEMPLATE """
<|im_start|>system
{{ .System }}<|im_end|>
<|im_start|>user
{{ .Prompt }}<|im_end|>
<|im_start|>assistant
{{ .Response }}<|im_end|>
"""

# stop at the end of the assistent instruction
PARAMETER stop "<|im_end|>"
PARAMETER stop "<|im_start|>"

# 
PARAMETER temperature 0.7
PARAMETER num_ctx 16384

SYSTEM """
You are a helpful assistant that answers questions based on the provided context.
If you don't know the answer, say you don't know.
Always be concise and to the point.
"""

```
