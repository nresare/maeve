use bytes::{Buf, Bytes, BytesMut};
use anyhow::Result;
use std::str::from_utf8;
fn main() -> Result<()> {
    println!("hello, world");
    let mut a = MyStruct::new();
    println!("Value of a.data: {}", from_utf8(&a.data)?);

    for i in &a {
        println!("Found {}", from_utf8(i)?);
    }

    println!("Second iterator");
    a.replace();
    for i in &a {
        println!("Found {}", from_utf8(i)?);
    }

    Ok(())
}

struct MyStruct {
    data: Bytes,
}

impl MyStruct {
    fn new() -> Self {
        let mut bs = BytesMut::zeroed(10);
        let mut n = b'a';
        for i in bs.iter_mut() {
            *i = n;
            n += 1;
        }
        MyStruct{data: bs.freeze()}
    }

    fn replace(&mut self) {
        let mut bs = BytesMut::zeroed(10);
        let mut n = b'b';
        for i in bs.iter_mut() {
            *i = n;
            n += 1;
        }
        self.data = bs.freeze()
    }

    fn iter(&self) -> Iter {
        Iter{inner: self, pos: 0}
    }

}

impl<'a> IntoIterator for &'a MyStruct {
    type Item = &'a [u8];
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct Iter<'a> {
    inner: &'a MyStruct,
    pos: usize,
}
impl <'a>Iterator for Iter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos + 1 < self.inner.data.len() {
            true => {
                let pos = self.pos;
                self.pos += 2;
                Some(&self.inner.data[pos..pos+2])
            },
            false => None,
        }
    }
}
