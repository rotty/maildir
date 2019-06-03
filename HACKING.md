# Hacking on the maildir code

This is a collection of notes intended to make it easier to get into
the `maildir` crate's codebase.

## Testdata format

Due to restrictions Cargo imposes upon filenames included in packages,
Maildir test data is stored using percent-encoded filenames. See the
[`maildir-testdata` README] for details. This crate also provides a
handy tool for debugging the included test data by running arbitrary
programs against the test data in its Maildir-compatible form. For
example, to run the `explain` example against the included testdata,
run:

```sh
cargo build --all-targets
./target/debug/with-maildir-testdata ./target/debug/examples/explain testdata/maildir1
```

Note you cannot use `cargo run` here, as `with-maildir-testdata` will
start the command with the working directory set the copy of the test
data.

[`maildir-testdata` README]: https://github.com/staktrace/maildir/tree/master/maildir-testdata/README.md
