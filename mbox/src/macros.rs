macro_rules! r {
    ( $x:expr ) => {
        ::std::io::Cursor::new(&$x[..])
    };
}
