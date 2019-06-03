use maildir::MailEntry;
use maildir::Maildir;
use std::io;

fn list_mail(mail: MailEntry) {
    println!("Path:         {}", mail.path().display());
    println!("ID:           {}", mail.id());
    println!("Flags:        {}", mail.flags());
    println!("is_draft:     {}", mail.is_draft());
    println!("is_flagged:   {}", mail.is_flagged());
    println!("is_passed:    {}", mail.is_passed());
    println!("is_replied:   {}", mail.is_replied());
    println!("is_seen:      {}", mail.is_seen());
    println!("is_trashed:   {}", mail.is_trashed());
}

fn run<T>(args: impl IntoIterator<Item = T>) -> Result<(), io::Error>
where
    T: Into<Maildir>,
{
    args.into_iter().map(Into::into).try_for_each(|mdir| {
        mdir.list_new()
            .chain(mdir.list_cur())
            .map(|r| r.map(list_mail))
            .collect::<Result<_, _>>()
    })
}

fn main() {
    // not sure whether this is actually fast or something, but we don't care here, do we?
    let rc = match run(std::env::args().skip(1)) {
        Err(e) => {
            eprintln!("Error: {:?}", e);
            1
        }
        Ok(_) => 0,
    };
    std::process::exit(rc);
}
