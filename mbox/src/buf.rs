use std::io::Read;
use bytes::BytesMut;
use anyhow::Result;

/// our special that keeps track of where we are
struct Buf {
    inner: BytesMut,
    start: usize,
}

enum Status {
    Success,
    EndOfFile
}

impl<'a> Buf {
    /// Constructs and returns a new Buf instance of the specified size
    fn new(size: usize) -> Self {
        Buf {
            inner: BytesMut::zeroed(size),
            start: 0,
        }
    }

    /// Move any remaining data to the beginning of the buffer and fills the
    /// rest with the data obtain by calling read() on reader one or more times.
    fn fill(mut self, _reader: impl Read) -> Result<Status> {
        todo!()
    }

    /// returns a slice of byte referencing into buf all bytes that has been
    /// read but not consumed.
    fn peek() -> &'a [u8] {
        todo!()
    }

    /// mark count bytes as consumed.
    fn consume(count: usize) -> Result<()> {
        todo!()
    }

}
#[cfg(test)]
mod test {

    #[test]
    fn test_roundtrip() {
        let ra = r!(b"abc");
    }
}
