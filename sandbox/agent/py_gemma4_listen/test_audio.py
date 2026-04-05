import ollama
from colorama import init, Fore, Style

init(autoreset=True)

FILE_NAME = "sample.wav"
MODEL_NAME = "gemma4:e4b"

print(f"{Fore.CYAN}--- Raw Ollama Audio Test ---{Style.RESET_ALL}")
print(f"Model: {MODEL_NAME}")
print(f"Target File: {FILE_NAME}\n")

try:
    # 1. Read raw bytes
    with open(FILE_NAME, "rb") as f:
        audio_bytes = f.read()

    print(f"{Fore.YELLOW}Payload ready. Sending {len(audio_bytes)} bytes to raw Ollama API...{Style.RESET_ALL}")

    # 2. Bypass LiteLLM & ADK to hit Ollama natively
    # Using the standard Ollama python library conventions.
    response = ollama.chat(
        model=MODEL_NAME,
        messages=[
            {
                'role': 'user',
                'content': 'Please transcribe this audio exactly.',
                'images': [audio_bytes]
            }
        ]
    )

    # 3. Print Output
    print(f"\n{Fore.GREEN}SUCCESS! Model reached and responded:{Style.RESET_ALL}")
    print(f"{Fore.MAGENTA}Response: {response['message']['content']}{Style.RESET_ALL}")
    
except FileNotFoundError:
    print(f"{Fore.RED}Error: '{FILE_NAME}' not found. Did you run 'uv run python record_sample.py' first?{Style.RESET_ALL}")
except Exception as e:
    print(f"\n{Fore.RED}Model connection/processing error: {e}{Style.RESET_ALL}")
