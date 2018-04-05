
use std::str;

const SMALL_STRING_BUF: usize = 16;

/// Small string data storage.
///
/// Encapsulates a 16 bytes buffer allowing storage of small strings without an allocation.
#[derive(Debug, Clone, Copy)]
pub struct SmallString {
    length: usize,
    bytes: [u8; SMALL_STRING_BUF],
}

impl SmallString {

    /// Try to construct a small string.
    ///
    /// Returns `None` if the string slice is too large for the buffer.
    pub fn try_from(value: &str) -> Option<SmallString> {
        let value_bytes = value.as_bytes();
        if value_bytes.len() <= SMALL_STRING_BUF {
            let mut bytes = [0; SMALL_STRING_BUF];
            bytes[..value_bytes.len()].copy_from_slice(value_bytes);
            Some(SmallString {
                length: value_bytes.len(),
                bytes,
            })
        } else {
            None
        }
    }

    /// Get a reference to the stored string slice.
    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.bytes[..self.length])
            .expect("valid stored utf8 verified during small string creation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {

        let s = SmallString::try_from("1234567890123456").expect("full string stored");
        assert_eq!(s.as_str(), "1234567890123456");

        let s = SmallString::try_from("").expect("empty string stored");
        assert_eq!(s.as_str(), "");

        let s = SmallString::try_from("12345678901234567");
        assert!(s.is_none());
    }
}
