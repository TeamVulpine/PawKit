use std::io::{stdout, Write};

pub(crate) fn print(s: &str) {
    let mut stdout = stdout().lock();
    stdout.write(s.as_bytes()).unwrap();
    stdout.flush().unwrap();
}
