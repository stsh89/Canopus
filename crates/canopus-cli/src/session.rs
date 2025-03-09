use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};
use crate::Result;

#[derive(Default, Deserialize, Serialize)]
pub struct Session {
    remarks_pagination_token: Option<String>,

    tags_pagination_token: Option<String>,

    #[serde(skip)]
    is_changed: bool,
}

enum SessionAttribute {
    RemarksPaginationToken(Option<String>),
    TagsPaginationToken(Option<String>),
}

impl Session {
    fn change_attribute(&mut self, attribute: SessionAttribute) {
        self.is_changed = true;

        match attribute {
            SessionAttribute::RemarksPaginationToken(token) => {
                self.remarks_pagination_token = token;
            }
            SessionAttribute::TagsPaginationToken(token) => {
                self.tags_pagination_token = token;
            }
        }
    }

    pub fn clear_remarks_pagination_token(&mut self) {
        self.change_attribute(SessionAttribute::RemarksPaginationToken(None));
    }

    pub fn clear_tags_pagination_token(&mut self) {
        self.change_attribute(SessionAttribute::TagsPaginationToken(None));
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn start() -> Result<Session> {
        get_or_initialize_session()
    }

    pub fn remarks_pagination_token(&self) -> Option<&str> {
        self.remarks_pagination_token.as_deref()
    }

    pub fn tags_pagination_token(&self) -> Option<&str> {
        self.tags_pagination_token.as_deref()
    }

    pub fn reset(self) -> Result<Self> {
        let session = Self::default();

        session.save()?;

        Ok(session)
    }

    pub fn set_remarks_pagination_token(&mut self, token: String) {
        self.change_attribute(SessionAttribute::RemarksPaginationToken(Some(token)));
    }

    pub fn set_tags_pagination_token(&mut self, token: String) {
        self.change_attribute(SessionAttribute::TagsPaginationToken(Some(token)));
    }

    pub fn save(&self) -> Result<()> {
        write_session_file(self)
    }
}

pub fn get_or_initialize_session() -> Result<Session> {
    let session = get_session()?.unwrap_or_default();

    Ok(session)
}

fn get_session() -> Result<Option<Session>> {
    use std::io::Read;

    let mut file = read_session_file()?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.is_empty() {
        return Ok(None);
    }

    let session = serde_json::from_str(&content)?;

    Ok(session)
}

fn read_session_file() -> Result<File> {
    let mut options = OpenOptions::new();

    options.read(true);

    open_session_file(&mut options)
}

fn write_session_file(session: &Session) -> Result<()> {
    let mut options = OpenOptions::new();

    options.write(true);
    options.truncate(true);

    let file = open_session_file(&mut options)?;

    serde_json::to_writer(file, session)?;

    Ok(())
}

fn open_session_file(options: &mut OpenOptions) -> Result<File> {
    if !Path::new("session.json").exists() {
        File::create("session.json")?;
    }

    let file = options.open("session.json")?;

    Ok(file)
}
