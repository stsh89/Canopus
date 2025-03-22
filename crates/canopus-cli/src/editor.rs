use crate::SUBSYSTEM_NAME;
use canopus_definitions::{ApplicationError, ApplicationResult};

const EDIT_CONTENT_FILE_NAME: &str = "EDIT_CONTENT";

pub fn edit(contents: &str) -> ApplicationResult<String> {
    set_file_contents(contents).map_err(|err| {
        ApplicationError::from_eyre(SUBSYSTEM_NAME, "unable to prepare file to edit", err)
    })?;

    start()
}

pub fn open() -> ApplicationResult<String> {
    clear_flie_contents().map_err(|err| {
        ApplicationError::from_eyre(SUBSYSTEM_NAME, "unable to prepare file to edit", err)
    })?;

    start()
}

fn start() -> ApplicationResult<String> {
    let status = std::process::Command::new("hx")
        .arg(EDIT_CONTENT_FILE_NAME)
        .status()
        .map_err(|err| {
            ApplicationError::from_eyre(
                SUBSYSTEM_NAME,
                "was not able to start text editor",
                eyre::Report::from(err),
            )
        })?;

    if !status.success() {
        return Err(ApplicationError::Internal {
            subsystem: SUBSYSTEM_NAME.to_string(),
            description: "received unexpected status from editor".to_string(),
            details: format!("{:}", status),
        });
    }

    let content = std::fs::read_to_string(EDIT_CONTENT_FILE_NAME).map_err(|err| {
        ApplicationError::from_eyre(
            SUBSYSTEM_NAME,
            "was not able to read edited content",
            eyre::Report::from(err),
        )
    })?;

    Ok(content.trim().to_string())
}

fn set_file_contents(contents: &str) -> eyre::Result<()> {
    std::fs::write(EDIT_CONTENT_FILE_NAME, contents)?;

    Ok(())
}

fn clear_flie_contents() -> eyre::Result<()> {
    std::fs::write(EDIT_CONTENT_FILE_NAME, "")?;

    Ok(())
}
