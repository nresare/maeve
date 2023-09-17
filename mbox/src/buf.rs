use crate::io::read_handling_short;
use bytes::BytesMut;
use std::io::Read;

/// our special that keeps track of where we are
pub struct Buf {
    inner: BytesMut,
    start: usize,
    end: usize,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Success,
    EndOfFile,
}

impl Buf {
    /// Constructs and returns a new Buf instance of the specified size
    pub fn new(size: usize) -> Self {
        Buf {
            inner: BytesMut::zeroed(size),
            start: 0,
            end: 0,
        }
    }

    /// Move any remaining data to the beginning of the buffer and fills the
    /// rest with the data obtain by calling read() on reader one or more times.
    pub fn fill(&mut self, reader: impl Read) -> Result<Status, std::io::Error> {
        if self.available() > 0 && self.start > 0 {
            move_data_to_beginning(self);
        }

        let target = &mut self.inner[self.end..];
        match read_handling_short(reader, target)? {
            0 => Ok(Status::EndOfFile),
            count => {
                self.end += count;
                Ok(Status::Success)
            }
        }
    }

    fn available(&self) -> usize {
        self.end - self.start
    }

    /// returns a slice of byte referencing into buf all bytes that has been
    /// read but not consumed.
    pub fn peek(&self) -> &[u8] {
        &self.inner[self.start..self.end]
    }

    /// mark count bytes as consumed.
    pub fn consume(&mut self, count: usize) {
        assert!(count <= self.end);
        self.start += count;
    }
}

fn move_data_to_beginning(buf: &mut Buf) {
    let copy_count = buf.end - buf.start;
    if copy_count <= buf.start {
        // The memory to be moved is not overlapping, e.g. the length of the
        // data to be moved is shorter than the position of the data from
        // the beginning of the buffer
        let source = &buf.inner.clone()[buf.start..buf.end];
        buf.inner[..source.len()].copy_from_slice(source);
    } else {
        // The source and target is overlapping, let's iterate over it
        for i in 0..copy_count {
            buf.inner[i] = buf.inner[i + buf.start]
        }
    }
    buf.start = 0;
    buf.end = copy_count;
}

#[cfg(test)]
mod test {
    use crate::buf::{move_data_to_beginning, Buf, Status};

    #[test]
    fn test_fill() {
        // More data in than the buffer size
        do_fill_once(b"abcd", 3, Status::Success);
        // Same amount of data
        do_fill_once(b"abc", 3, Status::Success);
        // less data than buffer size
        do_fill_once(b"abc", 4, Status::Success);
    }

    #[test]
    fn test_peek() {
        let r = r!(b"abc");
        let mut buf = Buf::new(3);
        assert_eq!(Status::Success, buf.fill(r).unwrap());
        assert_eq!(b"abc", buf.peek());
        buf.consume(1);
        assert_eq!(b"bc", buf.peek());
    }

    #[test]
    fn test_move_data_to_the_beginning_optimised() {
        let r = r!(b"abcdefgh");
        let mut buf = Buf::new(8);
        buf.fill(r).unwrap();
        buf.consume(4);
        move_data_to_beginning(&mut buf);
        assert_eq!(0, buf.start);
        assert_eq!(4, buf.end);
        assert_eq!(b"efgh", buf.peek());
    }

    #[test]
    fn test_move_data_to_the_beginning_overlapping() {
        let r = r!(b"abcdefgh");
        let mut buf = Buf::new(8);
        buf.fill(r).unwrap();
        buf.consume(1);
        move_data_to_beginning(&mut buf);
        assert_eq!(0, buf.start);
        assert_eq!(7, buf.end);
        assert_eq!(b"bcdefgh", buf.peek());
    }

    #[test]
    fn test_fill_twice() {
        let mut r = r!(b"abcdefgh");
        let mut buf = Buf::new(6);
        buf.fill(&mut r).unwrap();
        buf.consume(4);
        buf.fill(&mut r).unwrap();
        assert_eq!(0, buf.start);
        assert_eq!(4, buf.end);
        assert_eq!(b"efgh", buf.peek());
    }

    fn do_fill_once(data: &'static [u8], buf_size: usize, expected: Status) {
        let r = r!(data);
        let mut buf = Buf::new(buf_size);
        assert_eq!(expected, buf.fill(r).unwrap());
    }
}
