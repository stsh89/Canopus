mod commands;
mod resources;

use commands::{CommandError, Commands};
use resources::Resources;
use std::io::{Stdin, Stdout};

const BASE_URL: &str = "canopus://127.0.0.1";

fn main() -> Result<(), ShellError> {
    let mut exit = false;
    let mut shell = Shell::start();

    shell.print_welcome_message()?;

    while !exit {
        let input = shell.read()?;
        exit = shell.evaluate(input.trim())?;
    }

    Ok(())
}

struct Shell {
    stdout: Stdout,
    stdin: Stdin,
    resource: Option<Resources>,
}

enum ShellInput {
    Query(String),
    Command(Commands),
}

#[derive(Debug, thiserror::Error)]
enum ShellError {
    #[error(transparent)]
    Command(#[from] CommandError),

    #[error(r#"Unexpected error occured. Try to start shell again"#)]
    Unexpected(#[source] eyre::Error),
}

impl Shell {
    fn evaluate(&mut self, input: &str) -> Result<bool, ShellError> {
        match input.parse::<ShellInput>() {
            Ok(input) => match input {
                ShellInput::Command(command) => match command {
                    Commands::Quit => return Ok(true),
                    Commands::Help => print_help(&mut self.stdout)?,
                },
                ShellInput::Query(_query) => {}
            },
            Err(err) => match err {
                ShellError::Command(command_error) => print_error(&mut self.stdout, command_error)?,
                _ => return Err(err),
            },
        }

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

    fn print_welcome_message(&mut self) -> Result<(), ShellError> {
        let message = r#"Enter ".help" for usage hints."#;

        print_message(&mut self.stdout, message)
    }

    fn start() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            resource: None,
        }
    }

    fn read(&mut self) -> Result<String, ShellError> {
        use std::io::Write;

        self.stdout
            .write_all(self.prompt().as_bytes())
            .map_err(|err| ShellError::Unexpected(eyre::Error::new(err)))?;

        self.stdout
            .flush()
            .map_err(|err| ShellError::Unexpected(eyre::Error::new(err)))?;

        let mut buffer = String::new();

        if let Err(err) = self.stdin.read_line(&mut buffer) {
            return Err(ShellError::Unexpected(eyre::Error::new(err)));
        };

        Ok(buffer)
    }
}

fn print_error(stdout: &mut Stdout, error: impl std::error::Error) -> Result<(), ShellError> {
    print_message(stdout, error)?;

    Ok(())
}

fn print_help(stdout: &mut Stdout) -> Result<(), ShellError> {
    let message = commands::help_message();

    print_message(stdout, &message)?;

    Ok(())
}

fn print_message(stdout: &mut Stdout, message: impl std::fmt::Display) -> Result<(), ShellError> {
    use std::io::Write;

    let message = format!("\n{}\n\n", message);

    stdout
        .write_all(message.as_bytes())
        .map_err(|err| ShellError::Unexpected(eyre::Error::new(err)))?;

    Ok(())
}

impl std::str::FromStr for ShellInput {
    type Err = ShellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(".") {
            let command = s.parse()?;

            return Ok(Self::Command(command));
        }

        Ok(Self::Query(s.to_string()))
    }
}
