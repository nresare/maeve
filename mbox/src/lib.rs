#[macro_use]
mod macros;
mod io;
pub mod buf;

use anyhow::Result;
use std::io::Read;
use crate::buf::{Buf, Status};

pub struct Parser {
    reader: Box<dyn Read>,
    buf: Buf,
}

impl Parser {
    pub fn new(reader: Box<dyn Read>) -> Self {
        Parser::with_buf_size(reader, BUF_SIZE)
    }

    pub fn with_buf_size(reader: Box<dyn Read>, buf_size: usize) -> Self {
        Parser {
            reader,
            buf: Buf::new(buf_size),
        }
    }

    pub fn get_lines(&mut self) -> Result<Vec<String>> {
        let mut lines: Vec<String> = Vec::new();
        loop {
            let result = self.buf.fill(&mut self.reader)?;
            if result == Status::EndOfFile {
                break;
            }
            handle_filled(&mut lines, &mut self.buf)?;
        }
        // handle the last piece of data as a special case as
        // there might not be a newline around.
        let slice = self.buf.peek();
        if slice.len() > 0 {
            lines.push(String::from_utf8(slice.to_vec())?);
        }
        Ok(lines)
    }
}

const BUF_SIZE: usize = 8192;


fn handle_filled(lines: &mut Vec<String>, buf: &mut Buf) -> Result<()> {
    loop {
        let slice = buf.peek();
        match find_newline(slice) {
            None => {
                return Ok(())
            }
            Some(pos) => {
                lines.push(String::from_utf8(slice[0..pos].to_vec())?);
                buf.consume(pos + 1)?;
            }
        }
    }
}

fn find_newline(slice: &[u8]) -> Option<usize> {
    for (i, c) in slice.iter().enumerate() {
        if *c == 0x0a {
            return Some(i)
        }
    }
    return None
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
        assert_eq!(vec!["foo", "bar", "baz"], result);
        Ok(())
    }

    #[test]
    fn test_lines_from_file() -> Result<()> {
        let file = std::fs::File::open("kafka")?;
        let mut parser = Parser::new(Box::new(file));
        loop {
            let lines = parser.get_lines()?;
            if lines.len() == 0 {
                return Ok(());
            }
            for line in lines {
                print!("{}\n", line)
            }
        }
    }
}
