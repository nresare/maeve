#[macro_use]
mod macros;
mod io;
mod buf;

#[cfg(test)]
use crate::io::read_handling_short;
#[cfg(test)]
use anyhow::Result;
#[cfg(test)]
use bytes::BytesMut;
#[cfg(test)]
use std::io::Read;
#[cfg(test)]
const BUF_SIZE: usize = 8192;

#[cfg(test)]
struct Parser {
    reader: Box<dyn Read>,
    buf: BytesMut,
    start: usize,
    buf_size: usize,
}

#[cfg(test)]
impl Parser {
    fn new(reader: Box<dyn Read>) -> Self {
        Parser::with_buf_size(reader, BUF_SIZE)
    }

    fn with_buf_size(reader: Box<dyn Read>, buf_size: usize) -> Self {
        Parser {
            reader,
            buf: BytesMut::zeroed(buf_size),
            start: 0,
            buf_size,
        }
    }

    fn get_lines(&mut self) -> Result<Vec<String>> {
        let buf = &mut self.buf;
        let end = read_handling_short(&mut self.reader, &mut buf[self.start..])? + self.start;
        let mut result: Vec<String> = Vec::new();
        let mut next_start = 0;
        for (i, c) in (buf[0..end]).iter().enumerate() {
            if *c == 0x0a {
                result.push(String::from_utf8(buf[next_start..i].to_vec())?);
                next_start = i + 1;
            }
        }
        self.start = 0;
        if end > 0 && end < buf.len() {
            // our read didn't fill the buffer, so we probably hit EOF
            result.push(String::from_utf8(buf[next_start..end].to_vec())?);
        } else if next_start < end {
            // this will modify buf to point to the last part
            let _ = buf.split_to(next_start);
            let copied = buf.len();
            buf.resize(self.buf_size, 0);
            self.start = copied;

            println!("moving {} bytes to the start of the buffer", copied);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let mut parser = Parser::new(Box::new(b"foo\nbar\nbaz".as_ref()));
        let result = parser.get_lines()?;
        assert_eq!(vec!["foo", "bar", "baz"], result);
        Ok(())
    }

    #[test]
    fn test_read_with_small_buffer() -> Result<()> {
        let mut parser = Parser::with_buf_size(
            Box::new(b"foo\nbar\nbaz".as_ref()),
            5
        );
        let result = parser.get_lines()?;
        assert_eq!(vec!["foo"], result);
        let result = parser.get_lines()?;
        assert_eq!(vec!["bar"], result);
        let result = parser.get_lines()?;
        assert_eq!(vec!["baz"], result);
        let result = parser.get_lines()?;
        assert_eq!(0, result.len());
        Ok(())
    }

    #[test]
    fn test_lines_from_file() -> Result<()> {
        let file = std::fs::File::open("kafka")?;
        let mut parser = Parser::new(Box::new(file));
        loop {
            let lines = parser.get_lines()?;
            if lines.len() == 0 {
                break;
            }
            for line in lines {
                print!("{}\n", line)
            }
            //println!("{:?}", lines.len());
        }
        Ok(())
    }
}
