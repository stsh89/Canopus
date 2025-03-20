use canopus_client::{Client, remarks};
use canopus_definitions::ApplicationResult;
use clap::{Parser, Subcommand, command};
use eyre::WrapErr;

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
    List {
        #[arg(short, long)]
        page_token: Option<String>,
    },
}

impl App {
    fn initialize() -> eyre::Result<Self> {
        dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

        let cli = Cli::try_parse().map_err(|err| err.exit()).unwrap();
        let client = Client::from_env().wrap_err_with(|| "Failed to initialize Canopus client")?;

        Ok(Self { cli, client })
    }

    async fn execute(self) -> ApplicationResult<()> {
        let Self {
            cli: Cli { command },
            client,
        } = self;

        match command {
            Commands::List { page_token } => {
                let page = remarks::index(&client, page_token).await?;

                println!("{}", page);
            }
        };

        Ok(())
    }
}
