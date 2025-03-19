use canopus_client::{Client, tags};
use canopus_definitions::ApplicationResult;
use clap::{Parser, Subcommand, command};
use eyre::WrapErr;
use uuid::Uuid;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    App::initialize()?.execute().await?;

    Ok(())
}

struct App {
    cli: Cli,
    client: Client,
}

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

impl App {
    fn initialize() -> eyre::Result<Self> {
        dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

        let cli = Cli::parse();
        let client = Client::from_env().wrap_err_with(|| "Failed to initialize Canopus client")?;

        Ok(Self { cli, client })
    }

    async fn execute(self) -> ApplicationResult<()> {
        let Self {
            cli: Cli { command },
            client,
        } = self;

        match command {
            Commands::Show { id } => {
                let tag = tags::show(&client, id).await?;

                println!("{}", tag);
            }
            Commands::List { page_token } => {
                let page = tags::index(&client, page_token).await?;

                println!("{}", page);
            }
        };

        Ok(())
    }
}
