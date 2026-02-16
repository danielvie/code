import subprocess
from enum import Enum

from pydantic import BaseModel, Field, field_validator
from pydantic_ai import Agent
from pydantic_ai.models.openai import OpenAIChatModel
from pydantic_ai.providers.openai import OpenAIProvider

# 1. Setup
provider = OpenAIProvider(base_url="http://127.0.0.1:8033/v1", api_key="not-needed")
model = OpenAIChatModel("llama-3.1-8b-instruct", provider=provider)


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
        if "get_zig_version" in v or "tool" in v.lower():
            raise ValueError(
                "The zig_version field is empty. You MUST call the 'get_zig_version' tool"
                "to find the version before returning the final JSON"
            )

        # Catch the model just repeating the tool name
        if "get_zig_version" in v.lower() or "tool" in v.lower():
            raise ValueError(
                "You put the tool name in the field! Call the tool first,"
                "then put the actual version number here"
            )
        return v


# 2. Define Agent & Tools
agent = Agent(
    model=model,
    output_type=UserContext,
    retries=3,  # This allows the model to try again if the validator fails
    system_prompt=(
        "You are a system verification agent."
        "Your ONLY goal is to fill out the UserContext schema."
        "CRITICAL: You are FORBIDDEN from gressing the zig_version"
        "You MUST call the tool `get_zig_version` to check the local version. "
        "If you do not have the tool output, you cannot finish the task."
    ),
)


@agent.tool_plain
def get_zig_version() -> str:
    """Check the system for the installed Zig version."""
    try:
        # Running 'zig version' command
        print("log: calling get_zig_version tool...")
        result = subprocess.run(
            ["zig", "version"], capture_output=True, text=True, check=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError, FileNotFoundError:
        return "not installed"


# 3. Execution
def main():
    user_input = (
        "Lucas is 39, from São José dos Campos. He's a developer working with Zig."
    )

    print("Agent is analyzing and checking system tools...")

    try:
        # We use run_sync. If the validator fails, pydantic-ai automatically
        # feeds the error back to the LLM so it can correct itself.
        result = agent.run_sync(user_input)

        print("\n--- Final Extraction with System Verification ---")
        data = result.output
        print(f"Name: {data.name}")
        print(f"Location: {data.location}")
        print(f"Verified Zig Version: {data.zig_version}")

        if data.zig_version == "0.14.0":
            print("✨ Environment matches your preferred Zig version!")

    except Exception as e:
        print(f"Failed to complete agent loop: {e}")


if __name__ == "__main__":
    main()
