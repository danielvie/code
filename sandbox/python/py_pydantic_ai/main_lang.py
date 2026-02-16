import shutil
import subprocess

# 1. Updated Import based on your error message
from langchain.agents import create_agent
from langchain.agents.middleware.types import _InputAgentState
from langchain_core.tools import tool
from langchain_openai import ChatOpenAI
from pydantic import BaseModel, Field, SecretStr


# 2. Setup Data Models
class UserContext(BaseModel):
    name: str
    age: int
    location: str
    zig_version: str = Field(description="The verified Zig version.")


# 3. Define Tool
@tool
def get_zig_version() -> str:
    """
    Checks the system for the installed Zig version.
    Returns the version string or 'not instlled'
    """
    zig_path = shutil.which("zig")
    if not zig_path:
        return "Zig is not installed or not in the system PATH"

    try:
        result = subprocess.run(
            [zig_path, "version"], capture_output=True, text=True, check=True, timeout=5
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError, FileNotFoundError:
        return "not installed"


# 4. Execution
def main():
    # Setup LLM
    llm = ChatOpenAI(
        base_url="http://127.0.0.1:8033/v1",
        api_key=SecretStr("not-needed"),
        model="llama-3.1-8b-instruct",
        temperature=0,
    )

    tools = [get_zig_version]

    agent_runner = create_agent(llm, tools)

    user_input = (
        "Lucas is 39, from São José dos Campos. He's a developer working with Zig."
    )
    print("--- Running LangChain Agent ---")

    try:
        # Invoke the agent
        # inputs = {"messages": [("user", user_input)]}
        inputs: _InputAgentState = {
            "messages": [{"role": "user", "content": user_input}]
        }
        result = agent_runner.invoke(inputs)

        print("my results:")
        print(result)

        # Extract response (structure depends on if it returns a dict or message list)
        if isinstance(result, dict) and "messages" in result:
            print(f"\nFinal Answer: {result['messages'][-1].content}")
        elif isinstance(result, dict) and "output" in result:
            print(f"\nFinal Answer: {result['output']}")
        else:
            print(f"\nFinal Answer: {result}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
