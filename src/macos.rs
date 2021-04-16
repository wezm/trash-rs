use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Error;

pub fn is_implemented() -> bool {
    true
}

pub fn remove_all_canonicalized(full_paths: Vec<PathBuf>) -> Result<(), Error> {
    let full_paths = full_paths
        .into_iter()
        .map(|p| p.to_str().ok_or(Error::Unknown).map(|s| s.to_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    // AppleScript command to move files (or directories) to Trash looks like
    //   osascript -e 'tell application "Finder" to delete { POSIX file "file1", POSIX "file2" }'
    // The `-e` flag is used to execute only one line of AppleScript.
    let mut command = Command::new("osascript");
    let posix_files = full_paths
        .into_iter()
        .map(|p| format!("POSIX file \"{}\"", p))
        .collect::<Vec<String>>()
        .join(", ");
    let script = format!("tell application \"Finder\" to delete {{ {} }}", posix_files);

    let argv: Vec<OsString> = vec!["-e".into(), script.into()];
    command.args(argv);

    // Execute command
    let result = command.output().map_err(|e| Error::Remove { code: e.raw_os_error() })?;

    if !result.status.success() {
        return Err(Error::Remove { code: result.status.code() });
    }

    Ok(())
}

pub fn remove_all<I, T>(paths: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    let full_paths = paths
        .into_iter()
        // Convert paths into canonical, absolute forms and collect errors
        .map(|path| {
            path.as_ref().canonicalize().map_err(|e| {
                Error::new(
                    ErrorKind::CanonicalizePath {
                        original: path.as_ref().into(),
                    },
                    Box::new(e),
                )
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    remove_all_canonicalized(full_paths)
}

pub fn remove<T: AsRef<Path>>(path: T) -> Result<(), Error> {
    remove_all(&[path])
}

pub fn list() -> Result<Vec<TrashItem>, Error> {
    unimplemented!();
}

pub fn purge_all<I>(_items: I) -> Result<(), Error>
where
    I: IntoIterator<Item = TrashItem>,
{
    unimplemented!();
}

pub fn restore_all<I>(_items: I) -> Result<(), Error> {
    unimplemented!();
}
