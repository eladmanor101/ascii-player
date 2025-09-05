use std::io;

pub fn err(msg: &str) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, msg))
}