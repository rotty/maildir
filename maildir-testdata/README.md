# maildir-testdata

This is an internal crate, used by [`maildir`] to handle storing
Maildir test data in Cargo packages.

[`maildir`]: https://crates.io/crates/maildir

## Motivation

Cargo forbids, presumably for portability reasons, certain characters
to appear in the packages it produces. This poses a problem for test
data that is supposed to be in the Maildir format, as the forbidden
colon character (`:`) is part of the Maildir filename format.

Enter `maildir-testdata`, which allows for arbitrary filenames to be
reconstructed from files in Cargo package. The trick is simple: do not
operate directly on source files, but instead copy the source files,
renaming them in the process. The renaming is simply done by
[percent-decoding] the source file names to produce the destination
names. The destination is currently always resides under the temporary
directory as returned by `std::env::temp_dir()`.

This approach allows, given a filesystem which allows all
Maildir-relevant characters mounted at the destination, running tests
against Maildir test data which can be shipped directly in a Cargo
package without needing an extra layer, like an archive file. This
approach also, by fortunate side effect, ensures that testing code
using this mechism always operates on a copy of the data instead of
the original.

[percent-decoding]: https://en.wikipedia.org/wiki/Percent-encoding
