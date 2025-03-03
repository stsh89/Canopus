#[allow(dead_code)]
pub enum Resources {
    Tags,
    Remarks,
}

impl std::fmt::Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Resources::Tags => "tags",
            Resources::Remarks => "remarks",
        };

        f.write_str(text)
    }
}
