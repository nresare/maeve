mod io;

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
    //start: usize,
}

#[cfg(test)]
impl Parser {
    fn new(reader: Box<dyn Read>) -> Self {
        Parser {
            reader,
            buf: BytesMut::zeroed(BUF_SIZE),
        }
    }

    fn get_lines(&mut self) -> Result<Vec<String>> {
        let count = read_handling_short(&mut self.reader, self.buf.as_mut())?;
        let mut result: Vec<String> = Vec::new();
        let mut start = 0;
        for (i, c) in (self.buf[0..count]).iter().enumerate() {
            if *c == 0x0a {
                result.push(String::from_utf8(self.buf[start..i].to_vec())?);
                start = i + 1;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> Result<()> {
        let mut parser = Parser::new(Box::new(b"foo\nbar\nbaz\n".as_ref()));
        let result = parser.get_lines()?;
        assert_eq!(vec!["foo", "bar", "baz"], result);
        Ok(())
    }

    // #[test]
    // fn test_lines_from_file() -> Result<()> {
    //     let file = std::fs::File::open("kafka")?;
    //     let mut parser = Parser::new(Box::new(file));
    //     loop {
    //         let lines = parser.get_lines()?;
    //         if lines.len() == 0 {
    //             break;
    //         }
    //         println!("{:?}", lines.len());
    //     }
    //     Ok(())
    // }
}
