use anyhow::Result;
use std::io::Read;

/// A variant of Reader.read() that will handle short reads by repeating the read until
/// there is a zero read.
pub fn read_handling_short(mut reader: impl Read, buf: &mut [u8]) -> Result<usize> {
    let mut this_count = reader.read(buf)?;
    let mut total_count = this_count;
    while this_count > 0 && total_count < buf.len() {
        this_count = reader.read(&mut buf[total_count..])?;
        total_count += this_count;
    }
    Ok(total_count)
}

#[cfg(test)]
mod test {
    macro_rules! r {
        ( $x:expr ) => {
            Cursor::new(&$x[..])
        };
    }

    use crate::io::read_handling_short;
    use anyhow::Result;
    use std::io::{Cursor, Read};
    use std::ops::Deref;
    #[test]
    fn read_data_larger_than_buf() -> Result<()> {
        let mut reader = r!(b"abcdefghijklmnop");

        let mut buf = [0u8; 2];
        let count = read_handling_short(&mut reader, &mut buf[..])?;
        assert_eq!(b"ab"[..], buf[..2]);
        assert_eq!(2, count);

        let count = read_handling_short(&mut reader, &mut buf[..])?;
        assert_eq!(b"cd"[..], buf[..2]);
        assert_eq!(2, count);
        Ok(())
    }

    #[test]
    fn read_data_smaller_than_buf() -> Result<()> {
        let mut reader = r!(b"abcd");

        let mut buf = [0u8; 6];
        let count = read_handling_short(&mut reader, &mut buf)?;
        assert_eq!(4, count);
        Ok(())
    }

    struct ShortReader {
        to_write: Vec<Box<[u8]>>,
    }

    impl Read for ShortReader {
        fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
            match self.to_write.pop() {
                None => Ok(0),
                Some(data) => {
                    if buf.len() > data.len() {
                        buf = &mut buf[..data.len()];
                    }
                    buf.copy_from_slice(data.deref());
                    Ok(data.len())
                }
            }
        }
    }

    #[test]
    fn short_read() -> Result<()> {
        let mut data = vec![Box::from(&b"abc"[..]), Box::from(&b"def"[..])];
        data.reverse();
        let mut s = ShortReader { to_write: data };
        let mut buf = [0u8; 6];
        let count = read_handling_short(&mut s, buf.as_mut())?;
        assert_eq!(&b"abcdef"[..], buf.as_ref());
        assert_eq!(6, count);
        let count = read_handling_short(&mut s, buf.as_mut())?;
        assert_eq!(0, count);

        Ok(())
    }
}
