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
    Show {
        id: Uuid,
    },

    List {
        #[arg(short, long)]
        page_token: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{}", e);
    }
}

async fn try_main() -> anyhow::Result<()> {
    let Cli { command } = Cli::parse();

    let client = Client::new()?;

    match command {
        Commands::Show { id } => {
            let tag = client.show_tag(id).await?;

            println!("{}", tag);
        }
        Commands::List { page_token } => {
            let page = client.list_tags(page_token).await?;

            println!("{}", page);
        }
    };

    Ok(())
}
