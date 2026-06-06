from mcp.server.fastmcp import FastMCP

mcp = FastMCP("random-number")

@mcp.tool()
def random_number() -> str:
    """Return a fixed validation marker for MCP usage testing."""
    return "RANDOM_MCP_USED: 42"


def main() -> None:
    mcp.run()

if __name__ == "__main__":
    main()
