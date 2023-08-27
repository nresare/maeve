use std::io::Read;
use anyhow::Result;

const BUF_SIZE: usize = 8192;

struct Parser {
    reader: Box<dyn Read>,
    buf: Option<[u8; BUF_SIZE]>,
    start: usize,
}

impl Parser {
    fn new(reader: Box<dyn Read>) -> Self {
        Parser{reader, buf: None, start: 0}
    }

    fn get_lines(&mut self) -> Result<Vec<String>> {
        let buf = self.buf.get_or_insert_with(|| [0u8; BUF_SIZE]);
        let count = self.reader.read(&mut buf[self.start..])?;
        let mut result: Vec<String> = Vec::new();
        let mut start = 0;
        for (i, c) in (buf[0..count]).iter().enumerate() {
            if *c == 0x0a {
                result.push(String::from_utf8(buf[start..i].to_vec())?);
                start = i + 1;
            }
        }
        if start < count && start > 0 {
            let to_copy = (&buf[start..count]).clone();
            buf.copy_from_slice(to_copy);
            self.start = to_copy.len();
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
    fn test_lines_from_file() -> Result<()> {
        let file = std::fs::File::open("/Users/nresare/slask/vox")?;
        let mut parser = Parser::new(Box::new(file));
        loop {
            let lines = parser.get_lines()?;
            if lines.len() == 0 {
                break;
            }
            println!("{:?}", lines.len());
        }
        Ok(())
    }
}
