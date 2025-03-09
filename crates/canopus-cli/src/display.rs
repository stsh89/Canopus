use crate::Format as CliFormat;

pub struct Renderer {
    format: Format,
}

pub struct RenderOptions {
    pub format: Format,
}

pub enum Format {
    Text,
    Json,
}

impl Renderer {
    pub fn new(options: RenderOptions) -> Self {
        let RenderOptions { format } = options;

        Self { format }
    }

    pub fn render(&self, data: impl std::fmt::Display) {
        match self.format {
            Format::Text => println!("{}", data),
            Format::Json => todo!(),
        }
    }
}

impl From<CliFormat> for Format {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Text => Format::Text,
            CliFormat::Json => Format::Json,
        }
    }
}
