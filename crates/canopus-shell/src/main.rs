use canopus_cli::{Cli, CliContext};
use eyre::WrapErr;

const BASE_URL: &str = "canopus://127.0.0.1";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().wrap_err_with(|| "Failed to load .env file")?;

    let app = CliContext::initialize().wrap_err_with(|| "Failed to initialize CLI application")?;

    println!(r#"Enter "help" for usage hints. Enter "quit" or "exit" to close the shell."#);
    println!();
    println!();

    loop {
        let input = read().wrap_err_with(|| "Failed to read user input")?;
        let input = input.trim();

        if input == "quit" || input == "exit" {
            return Ok(());
        }

        let mut args: Vec<&str> = input.split_whitespace().collect();
        args.insert(0, "");

        let cli = match Cli::new_with_args(&args) {
            Ok(cli) => cli,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        if let Err(err) = app.execute(cli).await {
            eprintln!("{}", err);
        }
    }
}

fn prompt() -> String {
    format!("{}> ", BASE_URL)
}

fn read() -> eyre::Result<String> {
    use std::io::Write;

    std::io::stdout().write_all(prompt().as_bytes())?;
    std::io::stdout().flush()?;

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
