import random

from mcp.server.fastmcp import FastMCP

mcp = FastMCP("random-number")

@mcp.tool()
def random_number() -> str:
    """Generate a random integer from 0 to 1000 and return it with an MCP usage marker."""

    value = random.randint(0, 1000)
    return f"RANDOM_MCP_USED: {value}"


@mcp.tool()
def random_number_between(min_value: int, max_value: int) -> str:
    """Generate a random integer between the provided minimum and maximum values."""

    value = random.randint(min_value, max_value)
    return f"RANDOM_BETWEEN_MCP_USED: {value}"


@mcp.tool()
def random_float() -> str:
    """Generate a random floating-point number from 0.0 to 1.0."""

    value = random.random()
    return f"RANDOM_FLOAT_MCP_USED: {value}"


@mcp.tool()
def random_choice(options: list[str]) -> str:
    """Choose one random string from the provided options."""

    value = random.choice(options)
    return f"RANDOM_CHOICE_MCP_USED: {value}"


@mcp.tool()
def coin_flip() -> str:
    """Flip a coin and return either heads or tails."""

    value = random.choice(["heads", "tails"])
    return f"COIN_FLIP_MCP_USED: {value}"


@mcp.tool()
def roll_die(sides: int = 6) -> str:
    """Roll one die with the provided number of sides."""

    value = random.randint(1, sides)
    return f"ROLL_DIE_MCP_USED: {value}"


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
