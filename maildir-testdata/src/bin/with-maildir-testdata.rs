use std::{
    env,
    ffi::{OsStr, OsString},
    io,
    path::Path,
    process::{self, Command, ExitStatus},
};

fn run(cwd: &Path, argv: &[OsString]) -> Result<ExitStatus, io::Error> {
    // We need an absolute path, as the exec system call issued by `spawn()'
    // takes place in the new working directory.
    let path: &Path = argv[0].as_ref();
    let absolute = if path.is_absolute() {
        path.into()
    } else {
        env::current_dir()?.join(path)
    };
    let cmd: &OsStr = absolute.as_ref();
    Command::new(cmd)
        .args(&argv[1..])
        .current_dir(cwd)
        .spawn()?
        .wait()
}

fn getenv(name: &str, default: &str) -> OsString {
    env::var_os(name).unwrap_or(OsString::from(default))
}

fn main() {
    let argv: Vec<_> = env::args_os().skip(1).collect();
    let dir = getenv("MAILDIR_TESTDATA", "testdata");
    let relocate = getenv("MAILDIR_TESTDATA_RELOCATE", "testdata");
    let rc = if argv.len() > 0 {
        match maildir_testdata::with_testdata(dir, relocate, |tmp_dir| run(tmp_dir, &argv)) {
            Ok(Ok(rc)) => rc.code().unwrap_or(127),
            Ok(Err(e)) => {
                eprintln!("Error running command: {}", e);
                1
            }
            Err(e) => {
                eprintln!("Error creating test data: {}", e);
                1
            }
        }
    } else {
        eprintln!("Usage: with-maildir-testdata CMD...");
        1
    };
    process::exit(rc);
}
