use canopus_cli::CliState;
use canopus_definitions::Result;
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
}

impl Shell {
    async fn evaluate(&mut self, input: &str) -> Result<bool> {
        if input == "quit" || input == "exit" {
            return Ok(true);
        }

        let mut args: Vec<&str> = input.split_whitespace().collect();
        args.insert(0, "");

        canopus_cli::run_from(&self.state, args).await?;

        Ok(false)
    }

    fn prompt(&self) -> String {
        format!("{}> ", BASE_URL)
    }

    fn print_welcome_message(&mut self) -> Result<()> {
        let message = r#"Enter "help" for usage hints. Enter "quit" or "exit" to close the shell."#;

        print_message(&mut self.stdout, message)?;

        println!();
        println!();

        Ok(())
    }

    fn start() -> Result<Self> {
        Ok(Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
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
    println!();

    Ok(())
}

fn print_message(stdout: &mut Stdout, message: impl std::fmt::Display) -> Result<()> {
    use std::io::Write;

    let message = format!("{}", message);

    stdout
        .write_all(message.as_bytes())
        .map_err(Into::<eyre::Error>::into)?;

    Ok(())
}
