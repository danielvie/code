from pathlib import Path

from mcp.server.fastmcp import FastMCP

mcp = FastMCP("step8-example")


@mcp.tool()
def list_project_files() -> str:
    """List files in the current project directory."""
    paths = sorted(
        str(path)
        for path in Path(".").iterdir()
        if path.name not in {".venv", "__pycache__"}
    )
    return "\n".join(paths)


if __name__ == "__main__":
    mcp.run()
