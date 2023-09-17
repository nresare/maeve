#[macro_use]
mod macros;
mod buf;
mod io;
mod mbox;

use crate::buf::{Buf, Status};
use crate::mbox::find_from;
use std::io::Read;
use thiserror::Error;

pub struct Parser {
    reader: Box<dyn Read>,
    buf: Buf,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Expected 'From ' at beginning of input is missing")]
    MissingFirstFrom,
    #[error("Reading caused io::Error")]
    ReadError(#[from] std::io::Error),
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

    pub fn get_messages(&mut self) -> Result<Vec<String>, ParseError> {
        let mut message: Vec<String> = Vec::new();

        let mut first = true;

        while let Status::Success = self.buf.fill(&mut self.reader)? {
            if first {
                // The first message is a special case, as there is no double newline from the
                // previous message to match on
                if !self.buf.peek().starts_with(b"From ") {
                    return Err(ParseError::MissingFirstFrom);
                }
                first = false;
            }

            inner_get_message(&mut message, &mut self.buf);
        }

        message.push(make_message(self.buf.peek()));
        Ok(message)
    }
}

fn inner_get_message(messages: &mut Vec<String>, buf: &mut Buf) {
    let mut slice = buf.peek();

    while let Some(pos) = find_from(slice) {
        messages.push(make_message(&slice[0..pos]));

        // why + 2? Because the first part of the matching string, "\n\n" could be
        // thought of as in between the messages. Another side effect of this is that
        // find_from() now won't match at pos 0 the next time around.
        buf.consume(pos + 2);
        slice = buf.peek();
    }
}

fn make_message(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

const BUF_SIZE: usize = 8192;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn test_get_messages() {
        let mut parser = Parser::new(Box::new(r!("From abc")));
        let result = parser.get_messages().unwrap();
        assert_eq!(vec!["From abc"], result);

        let mut parser = Parser::new(Box::new(r!("From abc\n\nFrom cde")));
        let result = parser.get_messages().unwrap();
        assert_eq!(vec!["From abc", "From cde"], result);
    }

    #[test]
    fn test_get_messages_small_buffer() {
        let msg = "From abc\n\nFrom cde\n\nFrom xyz\n\nFrom zzt";
        let mut parser = Parser::with_buf_size(Box::new(r!(msg)), 15);
        let result = parser.get_messages().unwrap();
        assert_eq!(vec!["From abc", "From cde", "From xyz", "From zzt"], result);
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(ErrorKind::Other.into())
        }
    }

    #[test]
    fn test_get_messages_with_io_error() {
        let mut parser = Parser::new(Box::new(ErrorReader {}));
        let result = parser.get_messages();
        assert!(matches!(result, Err(ParseError::ReadError(i)) if i.kind() == ErrorKind::Other));
    }

    #[test]
    fn test_get_messages_without_leading_from() {
        let mut parser = Parser::new(Box::new(r!("No From in the beginning")));
        assert!(matches!(
            parser.get_messages(),
            Err(ParseError::MissingFirstFrom)
        ));
    }
}
