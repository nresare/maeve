#[cfg(test)]
macro_rules! r {
    ( $x:expr ) => {
        Box::new(::std::io::Cursor::new(&$x[..]))
    };
}
