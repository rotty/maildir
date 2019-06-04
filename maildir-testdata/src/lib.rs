use std::{fs, io, os::unix::ffi::OsStrExt, path::Path};

use percent_encoding::percent_decode;
use tempfile::tempdir;
use walkdir::WalkDir;

// `cargo package` doesn't package files with certain characters, such as
// colons, in the name, so we percent-decode the file names when copying the
// data for the tests.
/// Runs the given function on a temporary testdata copy.
pub fn with_testdata<T, F>(
    testdata: impl AsRef<Path>,
    relocate: impl AsRef<Path>,
    func: F,
) -> io::Result<T>
where
    F: FnOnce(&Path) -> T,
{
    let relocate: &Path = relocate.as_ref();
    let tmp_dir = tempdir()?;
    let tmp_path = tmp_dir.path();
    fs::create_dir_all(tmp_path.join(relocate))?;
    for entry in WalkDir::new(testdata.as_ref()) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(testdata.as_ref()).unwrap();
        if relative.parent().is_none() {
            continue;
        }
        let decoded = percent_decode(relative.as_os_str().as_bytes())
            .decode_utf8()
            .unwrap();
        let relocated = relocate.join(decoded.as_ref());
        if entry.path().is_dir() {
            fs::create_dir(tmp_path.join(relocated))?;
        } else {
            fs::copy(entry.path(), tmp_path.join(relocated))?;
        }
    }
    Ok(func(tmp_path))
}
