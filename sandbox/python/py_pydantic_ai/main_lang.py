import shutil
import subprocess

from langchain.agents import create_agent
from langchain_core.tools import tool
from langchain_openai import ChatOpenAI
from pydantic import BaseModel, Field, SecretStr, field_validator


# ── 1. Data Model ──────────────────────────────────────────────
class UserContext(BaseModel):
    name: str
    age: int
    location: str
    zig_version: str = Field(description="The verified Zig version.")

    @field_validator("zig_version")
    @classmethod
    def must_be_real_version(cls, v: str) -> str:
        if "get_zig_version" in v.lower() or "tool" in v.lower():
            raise ValueError(
                "You put the tool name instead of the version! "
                "Call the get_zig_version tool first, then use the actual version number."
            )
        return v


# ── 2. Tool ────────────────────────────────────────────────────
@tool
def get_zig_version() -> str:
    """Check the system for the installed Zig version.
    Returns the version string or 'not installed'.
    """
    zig_path = shutil.which("zig")
    if not zig_path:
        return "Zig is not installed or not in the system PATH"

    try:
        result = subprocess.run(
            [zig_path, "version"],
            capture_output=True,
            text=True,
            check=True,
            timeout=5,
        )
        return result.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "not installed"


# ── 3. Execution ───────────────────────────────────────────────
def main():
    llm = ChatOpenAI(
        base_url="http://127.0.0.1:8033/v1",
        api_key=SecretStr("not-needed"),
        model="llama-3.1-8b-instruct",
        temperature=0,
    )

    agent = create_agent(
        model=llm,
        tools=[get_zig_version],
        system_prompt=(
            "You are a system verification agent. "
            "Your goal is to extract user information AND verify the local Zig version. "
            "You MUST call the `get_zig_version` tool to check the installed version. "
            "Do NOT guess the Zig version."
        ),
        response_format=UserContext,
    )

    user_input = (
        "Lucas is 39, from São José dos Campos. He's a developer working with Zig."
    )

    print("--- Running LangChain Agent ---")

    try:
        result = agent.invoke({"messages": [("user", user_input)]})

        # The structured response is in result["structured_response"]
        user_context: UserContext = result["structured_response"]

        print("\n--- Final Structured Output ---")
        print(f"  Name:        {user_context.name}")
        print(f"  Age:         {user_context.age}")
        print(f"  Location:    {user_context.location}")
        print(f"  Zig Version: {user_context.zig_version}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
