import re
import shutil
import subprocess
from typing import Final

from langchain.agents import create_agent
from langchain_core.messages import HumanMessage
from langchain_core.tools import tool
from langchain_openai import ChatOpenAI
from pydantic import BaseModel, Field, SecretStr, field_validator

ZIG_SEMVER_RE: Final[re.Pattern[str]] = re.compile(r"^\d+\.\d+\.\d+$")
DEFAULT_BASE_URL: Final[str] = "http://127.0.0.1:8033/v1"
DEFAULT_MODEL: Final[str] = (
    "hf.co/bartowski/Nanbeige_Nanbeige4-3B-Thinking-2511-GGUF:Q8_0"
)


class UserContext(BaseModel):
    """Structured user context with a verified local Zig version."""

    name: str
    age: int
    location: str
    zig_version: str = Field(description="The verified Zig version.")

    @field_validator("zig_version")
    @classmethod
    def must_be_real_version(cls, v: str) -> str:
        # Enforce a strict semver-like version number (e.g. 0.14.0, 1.2.3).
        if not ZIG_SEMVER_RE.match(v):
            raise ValueError(
                "zig_version must be a real version number like '0.14.0'. "
                "You MUST call the `get_zig_version` tool first and use its result."
            )
        return v


@tool
def get_zig_version() -> str:
    """Return the installed Zig version string, or a sentinel if not available.

    Notes:
      - This tool returns a plain version string (e.g. "0.14.0") on success.
      - It returns "not installed" if Zig is missing or cannot be executed.
    """
    if not shutil.which("zig"):
        return "not installed"

    try:
        result = subprocess.run(
            ["zig", "version"],
            capture_output=True,
            text=True,
            check=True,
        )
        version = (result.stdout or "").strip()
        return version or "not installed"
    except (subprocess.CalledProcessError, FileNotFoundError, OSError):
        return "not installed"


def build_llm() -> ChatOpenAI:
    return ChatOpenAI(
        base_url=DEFAULT_BASE_URL,
        api_key=SecretStr("not-needed"),
        model=DEFAULT_MODEL,
        temperature=0,
    )


def build_agent(llm: ChatOpenAI):
    system_prompt = (
        "You are a system verification agent. "
        "Your goal is to extract user information AND verify the local Zig version. "
        "You MUST call the `get_zig_version` tool to check the installed version. "
        "Do NOT guess the Zig version."
    )
    return create_agent(
        model=llm,
        tools=[get_zig_version],
        system_prompt=system_prompt,
        response_format=UserContext,
    )


def main() -> None:
    user_input = (
        "Lucas is 39, from São José dos Campos. He's a developer working with Zig."
    )

    llm = build_llm()
    agent = build_agent(llm)

    print("--- Running LangChain Agent ---")

    result = agent.invoke({"messages": [HumanMessage(content=user_input)]})
    user_context: UserContext = result["structured_response"]

    print("\n--- Final Structured Output ---")
    print(f"  Name:        {user_context.name}")
    print(f"  Age:         {user_context.age}")
    print(f"  Location:    {user_context.location}")
    print(f"  Zig Version: {user_context.zig_version}")


if __name__ == "__main__":
    main()
