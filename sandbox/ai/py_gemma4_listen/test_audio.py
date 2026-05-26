import os
import sys
import base64
import warnings
from colorama import init, Fore, Style

warnings.filterwarnings("ignore", module="langgraph.*")
warnings.filterwarnings("ignore", module="langchain.*")
warnings.filterwarnings("ignore", category=DeprecationWarning)

# LangChain Imports
from langchain_ollama import ChatOllama
from langchain_core.messages import HumanMessage, SystemMessage
from langchain_core.tools import tool
from langgraph.prebuilt import create_react_agent

# Set local model environment
os.environ["OLLAMA_API_BASE"] = "http://localhost:11434"

init(autoreset=True)

FILE_NAME = "sample.wav"
MODEL_NAME = "gemma4:e4b"

print(f"{Fore.CYAN}--- LangChain + Ollama Pipeline Validation Test ---{Style.RESET_ALL}")
print(f"Model: {MODEL_NAME}")
print(f"Target File: {FILE_NAME}\n")

@tool
def execute_terminal_command(command: str) -> str:
    """Executes a formal terminal command (PowerShell)."""
    print(f"{Fore.BLUE}Executing Mock (PowerShell): {Style.BRIGHT}{command}{Style.RESET_ALL}")
    return f"Output: Mock execution successful."

try:
    with open(FILE_NAME, "rb") as f:
        audio_bytes = f.read()

    print(f"{Fore.YELLOW}Loaded {FILE_NAME}. Size: {len(audio_bytes)} bytes. Dispatching to LangChain...{Style.RESET_ALL}")

    llm = ChatOllama(model=MODEL_NAME, base_url="http://localhost:11434")

    # 1. Transcriber Agent - No tools, strict transcription
    print(f"\n{Fore.GREEN}Starting Phase 1 (Transcription)...{Style.RESET_ALL}")
    
    audio_b64 = base64.b64encode(audio_bytes).decode("utf-8")
    
    prompt = "Transcribe the attached audio exactly. Output only the transcript, no conversational text."
    transcribe_message = HumanMessage(
        content=[
            {"type": "text", "text": prompt},
            {"type": "image_url", "image_url": f"data:image/wav;base64,{audio_b64}"}
        ]
    )

    transcription_res = llm.invoke([transcribe_message])
    transcription = transcription_res.content.strip()
    
    print(f"{Fore.YELLOW}Transcribed: '{transcription}'{Style.RESET_ALL}")
    
    if not transcription or "you have not provided" in transcription.lower():
        print(f"{Fore.RED}Transcription Phase failed!{Style.RESET_ALL}")
        sys.exit(1)

    # 2. Command Agent - Receives pure text
    print(f"\n{Fore.GREEN}Starting Phase 2 (Execution)...{Style.RESET_ALL}")
    
    system_instruction = (
        "You are a local PC automation assistant. "
        "Analyze the user's transcribed intent and execute the correct tool command.\n"
        "- If told to perform a PC task, use the execute_terminal_command tool."
    )
    
    agent = create_react_agent(llm, tools=[execute_terminal_command])
    
    res = agent.invoke({"messages": [("system", system_instruction), ("user", f"Task: {transcription}")]})
    
    for message in res["messages"]:
        if message.type == "ai":
            if message.tool_calls:
                print(f"{Fore.LIGHTBLACK_EX}[Thought]: Using tool {message.tool_calls[0]['name']}{Style.RESET_ALL}")
            if message.content:
                print(f"{Fore.MAGENTA}Response: {message.content}{Style.RESET_ALL}")
        elif message.type == "tool":
            print(f"{Fore.BLUE}Tool Output: {message.content}{Style.RESET_ALL}")

except FileNotFoundError:
    print(f"{Fore.RED}Error: '{FILE_NAME}' not found. Please run 'uv run python record_sample.py' first.{Style.RESET_ALL}")
except Exception as e:
    import traceback
    traceback.print_exc()
    print(f"\n{Fore.RED}Pipeline Processing Error: {e}{Style.RESET_ALL}")

print(f"{Fore.CYAN}--- Validation Test Complete ---{Style.RESET_ALL}")
