use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openrouter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = openrouter::Client::from_env()?;

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("minimax/minimax-m3")
        .preamble(
            "
            You are a comedian here to entertain the user using humour and jokes. 
            Be concise and dont reply with large sentences,
            alleen in het Nederlands antwoorden
            "
        )
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Entertain me!").await?;

    println!("{response}");

    Ok(())
}