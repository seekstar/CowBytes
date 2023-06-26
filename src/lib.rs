use std::{ops::Deref, borrow::Cow};
use serde::{Serialize, Deserialize, de::Visitor};

mod tests;

#[derive(Debug, Eq)]
pub enum CowBytes<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}
impl<'a> CowBytes<'a> {
    pub fn is_borrowed(&self) -> bool {
        match self {
            CowBytes::Borrowed(_) => true,
            CowBytes::Owned(_) => false,
        }
    }
    pub fn is_owned(&self) -> bool {
        match self {
            CowBytes::Borrowed(_) => false,
            CowBytes::Owned(_) => true,
        }
    }
}
impl<'a> Deref for CowBytes<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => v.as_slice(),
        }
    }
}
impl<'a> PartialEq for CowBytes<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}
impl<'a> From<&'a [u8]> for CowBytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Borrowed(value)
    }
}
impl<'a> From<&'a Vec<u8>> for CowBytes<'a> {
    fn from(value: &'a Vec<u8>) -> Self {
        value.as_slice().into()
    }
}
impl<'a> From<Cow<'a, [u8]>> for CowBytes<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        match value {
            Cow::Borrowed(v) => Self::Borrowed(v),
            Cow::Owned(v) => Self::Owned(v),
        }
    }
}
impl<'a> Into<Cow<'a, [u8]>> for CowBytes<'a> {
    fn into(self) -> Cow<'a, [u8]> {
        match self {
            Self::Borrowed(v) => Cow::Borrowed(v),
            Self::Owned(v) => Cow::Owned(v),
        }
    }
}

struct CowBytesVisitor<'a>(&'a ());
impl<'a> CowBytesVisitor<'a> {
    fn new() -> Self {
        Self(&())
    }
}
impl<'de, 'a> Visitor<'de> for CowBytesVisitor<'a> {
    type Value = CowBytes<'a>;
    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a byte array")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::Owned(v.as_bytes().to_owned()))
    }
    // The default implementation of visit_borrowed_str forwards to visit_str.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::Owned(v.into_bytes()))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::Owned(v.to_vec()))
    }
    // The default implementation of visit_borrowed_bytes forwards to visit_bytes.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::Owned(v))
    }
}

impl<'a> Serialize for CowBytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.deref())
    }
}
impl<'a, 'de> Deserialize<'de> for CowBytes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(CowBytesVisitor::<'a>::new())
    }
}
