//! Small bridges to the operating system.

use std::io;
use std::path::Path;
use std::process::Command;

/// Opens `folder` in the platform file manager without waiting for it.
///
/// # Errors
///
/// Returns an error when the folder cannot be created or the file manager
/// cannot be started.
pub fn open_folder(folder: &Path) -> io::Result<()> {
    std::fs::create_dir_all(folder)?;
    let program = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "explorer"
    } else {
        "xdg-open"
    };
    Command::new(program).arg(folder).spawn().map(drop)
}
