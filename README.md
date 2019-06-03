# maildir

[![Build Status](https://travis-ci.org/staktrace/maildir.svg?branch=master)](https://travis-ci.org/staktrace/maildir)
[![Crate](https://img.shields.io/crates/v/maildir.svg)](https://crates.io/crates/maildir)

A simple library to deal with Maildir folders.

The Maildir folder layout is specified in ["Using maildir format"] by
djb. The [maildir(5)] qmail man page is also relevant.

There exists an extension of the Maildir format, [Maildir++] which
allows for a single level of sub-folders. In this layout, all
subfolders must start with a dot, which means they are ignored by
regular Maildir clients. The dovecot IMAP server adds another option,
the ["fs" layout].

This crate currently only supports the regular Maildir format.

["Using maildir format"]: http://cr.yp.to/proto/maildir.html
[maildir(5)]: http://www.qmail.org/qmail-manual-html/man5/maildir.html
[Maildir++]: http://www.courier-mta.org/imap/README.maildirquota.html
["fs" layout]: https://wiki.dovecot.org/MailboxFormat/Maildir

### Note for Windows Users

This crate is probably not for you. The Maildir format requires colons
to be allowed in filenames, and is thus inheritently incompatible with
NTFS, the Windows default file system.

## API

The primary entry point for this crate is the Maildir structure, which
can be created from a path, like so:

```rust
    let maildir = Maildir::from("path/to/maildir");
```

The Maildir structure then has functions that can be used to access
and modify mail files.

## Documentation

See the rustdoc at
[http://staktrace.github.io/maildir/target/doc/maildir/](http://staktrace.github.io/maildir/target/doc/maildir/).

## Other notes

This is written by a newbie Rust programmer, so code may be
non-idiomatic or suboptimal. Pull requests are welcome!
