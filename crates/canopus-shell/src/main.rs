mod commands;
mod resources;

use canopus_cli::CliState;
use canopus_definitions::{ApplicationError, Result};
use commands::Commands;
use resources::Resources;
use std::io::{Stdin, Stdout};

const BASE_URL: &str = "canopus://127.0.0.1";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().map_err(Into::<eyre::Error>::into)?;

    let mut exit = false;
    let mut shell = Shell::start()?;

    shell.print_welcome_message()?;

    while !exit {
        let input = shell.read()?;

        match shell.evaluate(input.trim()).await {
            Ok(is_exit) => exit = is_exit,
            Err(err) => print_error(&mut shell.stdout, err)?,
        }
    }

    Ok(())
}

struct Shell {
    state: CliState,
    stdout: Stdout,
    stdin: Stdin,
    resource: Option<Resources>,
}

enum ShellInput {
    Query(String),
    Command(Commands),
}

impl Shell {
    async fn evaluate(&mut self, input: &str) -> Result<bool> {
        match input.parse::<ShellInput>()? {
            ShellInput::Command(command) => match command {
                Commands::Quit => return Ok(true),
                Commands::Help => print_help(&mut self.stdout)?,
            },
            ShellInput::Query(query) => {
                let mut args: Vec<&str> = query.split_whitespace().collect();
                args.insert(0, "");

                canopus_cli::run_from(&self.state, args).await?
            }
        };

        Ok(false)
    }

    fn prompt(&self) -> String {
        let url = if let Some(resource) = &self.resource {
            format!("{}/{}", BASE_URL, resource)
        } else {
            BASE_URL.to_string()
        };

        format!("{}\n>>> ", url)
    }

    fn print_welcome_message(&mut self) -> Result<()> {
        let message = r#"Enter ".help" for usage hints."#;

        print_message(&mut self.stdout, message)
    }

    fn start() -> Result<Self> {
        Ok(Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            resource: None,
            state: CliState::new()?,
        })
    }

    fn read(&mut self) -> Result<String> {
        use std::io::Write;

        self.stdout
            .write_all(self.prompt().as_bytes())
            .map_err(Into::<eyre::Error>::into)?;

        self.stdout.flush().map_err(Into::<eyre::Error>::into)?;

        let mut buffer = String::new();

        if let Err(err) = self.stdin.read_line(&mut buffer) {
            return Err(eyre::Error::from(err).into());
        };

        Ok(buffer)
    }
}

fn print_error(stdout: &mut Stdout, error: impl std::error::Error) -> Result<()> {
    print_message(stdout, error)?;

    Ok(())
}

fn print_help(stdout: &mut Stdout) -> Result<()> {
    let message = commands::help_message();

    print_message(stdout, &message)?;

    Ok(())
}

fn print_message(stdout: &mut Stdout, message: impl std::fmt::Display) -> Result<()> {
    use std::io::Write;

    let message = format!("\n{}\n\n", message);

    stdout
        .write_all(message.as_bytes())
        .map_err(Into::<eyre::Error>::into)?;

    Ok(())
}

impl std::str::FromStr for ShellInput {
    type Err = ApplicationError;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(".") {
            let command = s.parse().map_err(Into::<eyre::Error>::into)?;

            return Ok(Self::Command(command));
        }

        Ok(Self::Query(s.to_string()))
    }
}
