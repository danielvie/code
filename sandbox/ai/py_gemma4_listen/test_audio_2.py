import os
import sys
import base64
from colorama import init, Fore, Style
from pydantic import BaseModel, Field

# Google ADK Imports
from google.adk.agents.llm_agent import Agent
from google.adk import Runner
from google.adk.sessions.in_memory_session_service import InMemorySessionService
from google.genai import types
from google.adk.models.lite_llm import LiteLlm

# Set local model environment
os.environ["OLLAMA_API_BASE"] = "http://localhost:11434"

init(autoreset=True)

FILE_NAME = "sample.wav"
MODEL_NAME = "gemma4:e4b"

print(f"{Fore.CYAN}--- ADK + LiteLLM Pipeline Validation Test ---{Style.RESET_ALL}")
print(f"Model: {MODEL_NAME}")
print(f"Target File: {FILE_NAME}\n")

# Apply LiteLLM hot-patch for vision support just like in main.py
import litellm
litellm.model_cost[f"ollama_chat/{MODEL_NAME}"] = {"supports_vision": True, "supports_audio": True}

class TerminalCommand(BaseModel):
    command: str = Field(description="The formal terminal command to execute (PowerShell).")

def execute_terminal_command(params: TerminalCommand) -> str:
    print(f"{Fore.BLUE}Executing Mock (PowerShell): {Style.BRIGHT}{params.command}{Style.RESET_ALL}")
    return f"Output: Mock execution successful."

try:
    with open(FILE_NAME, "rb") as f:
        audio_bytes = f.read()

    print(f"{Fore.YELLOW}Loaded {FILE_NAME}. Size: {len(audio_bytes)} bytes. Dispatching to ADK Runner...{Style.RESET_ALL}")

    model = LiteLlm(model=f"ollama_chat/{MODEL_NAME}")
    agent = Agent(
        name="PCAssistant",
        model=model,
        instruction=(
            "You are a local PC automation assistant. "
            "Analyze the user's intent from the provided audio/text and execute the correct tool command.\n"
            "- If asked a question, use your knowledge.\n"
            "- If told to perform a PC task, use the execute_terminal_command tool.\n"
            "Respond naturally after executing tools."
        ),
        tools=[execute_terminal_command]
    )
    
    runner = Runner(
        app_name="TestApp",
        agent=agent,
        session_service=InMemorySessionService(),
        auto_create_session=True
    )

    prompt = "Transcribe this exactly."
    part_prompt = types.Part.from_text(text=prompt)
    part_audio = types.Part.from_bytes(data=audio_bytes, mime_type="image/wav")
    new_message = types.Content(role="user", parts=[part_prompt, part_audio])

    print(f"\n{Fore.GREEN}Starting ADK Evaluation Loop...{Style.RESET_ALL}")
    for event in runner.run(
        user_id="local_user",
        session_id="local_test_session",
        new_message=new_message
    ):
        if getattr(event, "content", None) and getattr(event.content, "parts", None):
            for part in event.content.parts:
                if getattr(part, "thought", False) and part.text:
                    print(f"{Fore.LIGHTBLACK_EX}[Thought]: {part.text.replace(chr(10), ' ')}{Style.RESET_ALL}")
                elif getattr(part, "text", None):
                    print(f"{Fore.MAGENTA}Response: {part.text}{Style.RESET_ALL}")

except FileNotFoundError:
    print(f"{Fore.RED}Error: '{FILE_NAME}' not found. Please run 'uv run python record_sample.py' first.{Style.RESET_ALL}")
except Exception as e:
    import traceback
    traceback.print_exc()
    print(f"\n{Fore.RED}ADK Pipeline Processing Error: {e}{Style.RESET_ALL}")

print(f"{Fore.CYAN}--- Validation Test Complete ---{Style.RESET_ALL}")
