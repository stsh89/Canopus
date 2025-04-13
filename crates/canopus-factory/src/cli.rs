use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub database_url: String,

    #[arg(short, long)]
    pub port: u16,
}
