use canopus_definitions::{ApplicationError, ApplicationResult};

const EDIT_CONTENT_FILE_NAME: &str = "EDIT_CONTENT";

pub fn edit(contents: &str) -> ApplicationResult<String> {
    set_file_contents(contents).map_err(from_fs_err)?;

    start()
}

pub fn open() -> ApplicationResult<String> {
    clear_flie_contents().map_err(from_fs_err)?;

    start()
}

fn start() -> ApplicationResult<String> {
    let status = std::process::Command::new("hx")
        .arg(EDIT_CONTENT_FILE_NAME)
        .status()
        .map_err(|err| {
            ApplicationError::internal(
                "failed to start helix editor, check if it is installed",
                err,
            )
        })?;

    if !status.success() {
        return Err(ApplicationError::msg(
            "helix editor exited with unsuccessful status code",
        ));
    }

    let content = std::fs::read_to_string(EDIT_CONTENT_FILE_NAME).map_err(from_fs_err)?;

    Ok(content.trim().to_string())
}

fn set_file_contents(contents: &str) -> std::io::Result<()> {
    std::fs::write(EDIT_CONTENT_FILE_NAME, contents)
}

fn clear_flie_contents() -> std::io::Result<()> {
    std::fs::write(EDIT_CONTENT_FILE_NAME, "")
}

fn from_fs_err(err: std::io::Error) -> ApplicationError {
    ApplicationError::internal("editor failed to make file IO operation", err)
}
