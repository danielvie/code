import re
import subprocess
from enum import Enum

from pydantic import BaseModel, Field, field_validator
from pydantic_ai import Agent
from pydantic_ai.models.openai import OpenAIChatModel
from pydantic_ai.providers.openai import OpenAIProvider


# ── 1. Data Model ──────────────────────────────────────────────
class LanguageType(str, Enum):
    SYSTEMS = "systems"
    HIGH_LEVEL = "high_level"


class UserContext(BaseModel):
    name: str
    age: int
    location: str
    primary_lang: str
    lang_category: LanguageType
    zig_version: str = Field(
        description="The actual version number of Zig installed (e.g., '0.12.0')."
    )

    @field_validator("zig_version")
    @classmethod
    def check_not_tool_name(cls, v: str) -> str:
        # Catch empty strings or single-character hallucinations
        if not re.match(r"^\d+\.\d+\.\d+", v):
            raise ValueError(
                f"zig_version must be a real version number like '0.14.0', "
                f"got '{v}'. You MUST call the `get_zig_version` tool first "
                f"to retrieve the actual installed version, then put that "
                f"version string here."
            )
        return v


# ── 2. Agent & Tool ────────────────────────────────────────────
provider = OpenAIProvider(base_url="http://127.0.0.1:8033/v1", api_key="not-needed")
# provider = OpenAIProvider(base_url="http://127.0.0.1:11434/v1", api_key="not-needed")
# model = OpenAIChatModel("llama-3.1-8b-instruct", provider=provider)
model = OpenAIChatModel(
    "hf.co/bartowski/Nanbeige_Nanbeige4-3B-Thinking-2511-GGUF:Q8_0", provider=provider
)

agent = Agent(
    model=model,
    output_type=UserContext,
    retries=3,
    system_prompt=(
        "You are a system verification agent. "
        "Your goal is to extract user information AND verify the local Zig version. "
        "You MUST call the `get_zig_version` tool to check the installed version. "
        "Do NOT guess the Zig version."
    ),
)


@agent.tool_plain
def get_zig_version() -> str:
    """Check the system for the installed Zig version."""
    try:
        print("log: calling get_zig_version tool...")
        result = subprocess.run(["zig", "version"], capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "not installed"


# ── 3. Execution ───────────────────────────────────────────────
def main():
    user_input = "Lucas is 39, from São José dos Campos. He's a developer working with Zig."

    print("--- Running Pydantic-AI Agent ---")

    try:
        result = agent.run_sync(user_input)

        user_context = result.output

        print("\n--- Final Structured Output ---")
        print(f"  Name:        {user_context.name}")
        print(f"  Age:         {user_context.age}")
        print(f"  Location:    {user_context.location}")
        print(f"  Zig Version: {user_context.zig_version}")

        if user_context.zig_version == "0.14.0":
            print("✨ Environment matches your preferred Zig version!")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
