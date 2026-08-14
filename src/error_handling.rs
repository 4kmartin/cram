pub(crate) fn print_io_error(err: &std::io::Error) {
    println!("{}", err);
}

pub(crate) fn build_io_error(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, msg)
}
