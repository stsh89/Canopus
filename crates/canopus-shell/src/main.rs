mod commands;
mod resources;

use canopus_client::{Client, Error as ClientError};
use canopus_definitions::Tag;
use commands::{Commands, Error as CommandError};
use resources::Resources;
use std::io::{Stdin, Stdout};
use uuid::Uuid;

const BASE_URL: &str = "canopus://127.0.0.1";

#[tokio::main]
async fn main() -> Result<()> {
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

type Result<T> = std::result::Result<T, Error>;

struct Shell {
    client: Client,
    stdout: Stdout,
    stdin: Stdin,
    resource: Option<Resources>,
}

enum ShellInput {
    Query(String),
    Command(Commands),
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Client(ClientError),

    #[error(transparent)]
    Command(#[from] CommandError),

    #[error(r#"Internal shell error. Try to start shell again"#)]
    Internal(#[source] eyre::Error),
}

impl Shell {
    async fn evaluate(&mut self, input: &str) -> Result<bool> {
        match input.parse::<ShellInput>()? {
            ShellInput::Command(command) => match command {
                Commands::Quit => return Ok(true),
                Commands::Help => print_help(&mut self.stdout)?,
            },
            ShellInput::Query(query) => {
                let id: Uuid = query
                    .parse()
                    .map_err(|_err| Error::Internal(eyre::Error::msg("Invalid UUID")))?;

                let tag = self.get_tag(id).await?;

                print_tag(&mut self.stdout, tag)?
            }
        };

        Ok(false)
    }

    async fn get_tag(&self, id: Uuid) -> Result<Tag> {
        let tag = self.client.get_tag(id).await?;

        Ok(tag)
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
            client: Client::new()?,
        })
    }

    fn read(&mut self) -> Result<String> {
        use std::io::Write;

        self.stdout
            .write_all(self.prompt().as_bytes())
            .map_err(|err| Error::Internal(eyre::Error::new(err)))?;

        self.stdout
            .flush()
            .map_err(|err| Error::Internal(eyre::Error::new(err)))?;

        let mut buffer = String::new();

        if let Err(err) = self.stdin.read_line(&mut buffer) {
            return Err(Error::Internal(eyre::Error::new(err)));
        };

        Ok(buffer)
    }
}

fn print_error(stdout: &mut Stdout, error: impl std::error::Error) -> Result<()> {
    print_message(stdout, error)?;

    Ok(())
}

fn print_tag(stdout: &mut Stdout, tag: Tag) -> Result<()> {
    print_message(stdout, tag.title().as_str())?;

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
        .map_err(|err| Error::Internal(eyre::Error::new(err)))?;

    Ok(())
}

impl std::str::FromStr for ShellInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(".") {
            let command = s.parse()?;

            return Ok(Self::Command(command));
        }

        Ok(Self::Query(s.to_string()))
    }
}

impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        match value {
            ClientError::Connection => todo!(),
            ClientError::InvalidArgument { argument, reason } => todo!(),
            ClientError::NotFound { resource, id } => todo!(),
            ClientError::InternalServer => todo!(),
            ClientError::Internal(report) => todo!(),
            ClientError::Unimplemented => todo!(),
        }
    }
}
