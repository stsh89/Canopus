use canopus_engine::{remarks::Remark, tags::Tag};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemarkPresenter {
    id: Uuid,
    essence: String,
    tags: Vec<TagPresenter>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemarkRowPresenter {
    id: Uuid,
    essence: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagPresenter {
    id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub fn write_remark(remark_presenter: RemarkPresenter, mut writer: impl Write) -> io::Result<()> {
    serde_json::to_writer_pretty(&mut writer, &remark_presenter)?;

    Ok(())
}

pub fn write_remarks_table(
    rows: Vec<RemarkRowPresenter>,
    mut writer: impl Write,
) -> io::Result<()> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(&mut writer);

    for row in rows {
        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn write_tag(tag_presenter: TagPresenter, mut writer: impl Write) -> io::Result<()> {
    serde_json::to_writer_pretty(&mut writer, &tag_presenter)?;

    Ok(())
}

impl From<Remark> for RemarkPresenter {
    fn from(value: Remark) -> Self {
        RemarkPresenter {
            id: value.id(),
            essence: value.essence().to_string(),
            tags: value.tags().iter().map(TagPresenter::from).collect(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<Remark> for RemarkRowPresenter {
    fn from(value: Remark) -> Self {
        RemarkRowPresenter {
            id: value.id(),
            essence: {
                let mut s = value.essence().to_string();
                if s.len() > 40 {
                    s.truncate(37);
                    s.push_str("...");
                }
                s
            },
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<&Tag> for TagPresenter {
    fn from(value: &Tag) -> Self {
        TagPresenter {
            id: value.id(),
            title: value.title().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<Tag> for TagPresenter {
    fn from(value: Tag) -> Self {
        TagPresenter::from(&value)
    }
}
