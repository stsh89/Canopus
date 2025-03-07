pub enum Commands {
    Help,
    Quit,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(r#"Error: unkown command or invaild arguments: "{0}". Enter ".help" for help"#)]
    Parse(String),
}

impl std::str::FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = match s {
            ".help" => Self::Help,
            ".quit" => Self::Quit,
            _ => return Err(Error::Parse(s.to_string())),
        };

        Ok(command)
    }
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Commands::Help => ".help",
            Commands::Quit => ".quit",
        };

        f.write_str(text)
    }
}

pub fn help_message() -> String {
    let message = format!(
        r#"
{0: <5} Show help text
{1: <5} Stop interpreting input stream and exit
        "#,
        Commands::Help,
        Commands::Quit,
    );

    message.trim().to_string()
}
