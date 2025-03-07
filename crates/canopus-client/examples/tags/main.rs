use canopus_client::Client;
use clap::{Parser, Subcommand, command};
use uuid::Uuid;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        id: Uuid,
    },

    List {
        #[arg(long)]
        page_token: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = Client::new()?;

    let result = match cli.command {
        Commands::Get { id } => get(client, id).await,
        Commands::List { page_token } => list(client, page_token).await,
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("\x1b[91m{}\x1b[0m", err),
    }

    Ok(())
}

async fn get(client: Client, id: Uuid) -> canopus_client::Result<String> {
    let tag = client.get_tag(id).await?;

    Ok(format!("{}", tag))
}

async fn list(client: Client, page_token: Option<String>) -> canopus_client::Result<String> {
    let page = client.list_tags(page_token).await?;

    let mut buffer = page
        .items
        .into_iter()
        .take(3)
        .map(|tag| format!("{}", tag))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(next_page_token) = page.next_page_token {
        buffer.push_str(&format!("\nNext page token: {}", next_page_token));
    }

    Ok(buffer)
}
