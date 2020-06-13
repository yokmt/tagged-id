use core::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use thiserror::Error;
use uuid::Uuid;

#[cfg(feature = "diesel")]
mod diesel;
#[cfg(feature = "serde")]
mod serde;

pub struct TaggedId<T> {
    inner: Uuid,
    _phantom: PhantomData<T>,
}

impl<T> TaggedId<T> {
    pub fn new() -> Self {
        TaggedId::from_uuid(Uuid::new_v4())
    }

    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        Ok(TaggedId::from_uuid(Uuid::from_slice(b)?))
    }

    pub fn parse_str(s: &str) -> Result<Self, Error> {
        Ok(TaggedId::from_uuid(Uuid::parse_str(s)?))
    }

    fn from_uuid(id: Uuid) -> Self {
        Self {
            inner: id,
            _phantom: PhantomData,
        }
    }
}

impl<T> FromStr for TaggedId<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl<T> Copy for TaggedId<T> {}

impl<T> Clone for TaggedId<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for TaggedId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Uuid::fmt(&self.inner, f)
    }
}

impl<T> fmt::Debug for TaggedId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Uuid::fmt(&self.inner, f)
    }
}

impl<T> PartialEq for TaggedId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ne(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

#[derive(Debug, Error)]
#[error("TaggedId Error {0}")]
pub struct Error(#[from] uuid::Error);

#[cfg(test)]
mod tests {
    use super::*;

    struct MyTag;

    type MyId = TaggedId<MyTag>;

    #[test]
    fn parse_from_str() {
        assert!(MyId::parse_str("12345678-1111-1111-1111-111111111111").is_ok());
        assert!(MyId::parse_str("").is_err());
        assert!(MyId::parse_str("hoge").is_err());
        assert!(MyId::parse_str("12345678-1111-1111-1111-111111111111-a").is_err());
    }

    #[test]
    fn can_equal() {
        let id1 = MyId::parse_str("12345678-1111-1111-1111-111111111111").unwrap();
        let id2 = MyId::parse_str("12345678-1111-1111-1111-111111111111").unwrap();
        assert_eq!(id2, id1);

        let id3 = MyId::parse_str("12345678-1111-1111-1111-222222222222").unwrap();
        assert_ne!(id1, id3);
    }

    #[test]
    fn fmt_display() {
        let id = MyId::parse_str("12345678-1111-1111-1111-111111111111").unwrap();
        assert_eq!("12345678-1111-1111-1111-111111111111", format!("{}", id))
    }

    #[test]
    fn to_string() {
        let id = MyId::parse_str("12345678-1111-1111-1111-111111111111").unwrap();
        assert_eq!("12345678-1111-1111-1111-111111111111", id.to_string())
    }
}
