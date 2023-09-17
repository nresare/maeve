const FROM_BYTES: &[u8; 7] = b"\n\nFrom ";
const MATCH_LEN: usize = FROM_BYTES.len();

/// Look for the byte sequence of the string "\n\nFrom " encoded in UTF-8.
pub fn find_from(slice: &[u8]) -> Option<usize> {
    let mut progress: usize = 0;
    for (i, c) in slice.iter().enumerate() {
        if *c == FROM_BYTES[progress] {
            progress += 1;
            if progress == MATCH_LEN {
                return Some(i + 1 - MATCH_LEN);
            }
        } else {
            progress = 0;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::mbox::find_from;

    #[test]
    fn test_no_match() {
        assert_eq!(None, find_from(b"abc"));
        assert_eq!(None, find_from(b"\n\nFrom"));
    }

    #[test]
    fn test_match() {
        assert_eq!(Some(1), find_from(b"X\n\nFrom "));
        assert_eq!(Some(0), find_from(b"\n\nFrom abc"))
    }
}
