use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Clone, PartialEq)]
pub enum AppendByteOrder {
    BigEndian,
    LittleEndian,
}

#[derive(Clone, PartialEq)]
pub struct Bytes {
    container: Vec<u8>,
    append_byte_order: AppendByteOrder,
}

pub trait AsBytes {
    fn into_bytes(self) -> Bytes;
    fn into_bytes_le(self) -> Bytes;
}

impl Mul<usize> for Bytes {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut container = vec![];
        for _ in 0..rhs {
            container.extend(self.container.iter());
        }

        Self {
            container,
            append_byte_order: self.append_byte_order,
        }
    }
}

impl Add<Bytes> for Bytes {
    type Output = Self;

    fn add(self, rhs: Bytes) -> Self::Output {
        let mut container = self.container;
        container.extend(rhs.container);
        Self {
            container,
            append_byte_order: self.append_byte_order,
        }
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, b) in self.container.iter().enumerate() {
            if i == self.container.len() - 1 {
                write!(f, "{:02x}", b)?;
                break;
            }

            write!(f, "{:02x} ", b)?;
        }
        Ok(())
    }
}

impl Bytes {
    pub fn empty() -> Self {
        Self {
            container: vec![],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }

    pub fn new(v: Vec<u8>) -> Self {
        Self {
            container: v,
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }

    pub fn reverse(self) -> Self {
        Self {
            container: self.container.into_iter().rev().collect(),
            append_byte_order: self.append_byte_order,
        }
    }

    pub fn combine(v: Vec<Bytes>, append_byte_order: AppendByteOrder) -> Self {
        let mut container = vec![];
        for bytes in v {
            container.extend(bytes.container);
        }

        Self {
            container,
            append_byte_order,
        }
    }

    pub fn set_append_byte_order(&mut self, append_byte_order: AppendByteOrder) {
        self.append_byte_order = append_byte_order;
    }

    pub fn add(&mut self, bytes: Bytes) -> usize {
        if self.append_byte_order != bytes.append_byte_order {
            self.container.extend(bytes.container.iter().rev());
        } else {
            self.container.extend(bytes.container);
        }

        self.len()
    }

    pub fn set_at(&mut self, start_idx: usize, bytes: Bytes) {
        for (i, b) in bytes.container.iter().enumerate() {
            self.container[start_idx + i] = *b;
        }
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.container.iter()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Self {
            container: v,
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(b: Bytes) -> Self {
        b.container
    }
}

impl From<u8> for Bytes {
    fn from(b: u8) -> Self {
        Self {
            container: vec![b],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<u16> for Bytes {
    fn from(b: u16) -> Self {
        Self {
            container: vec![(b >> 8) as u8, b as u8],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<u32> for Bytes {
    fn from(b: u32) -> Self {
        Self {
            container: vec![(b >> 24) as u8, (b >> 16) as u8, (b >> 8) as u8, b as u8],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<u64> for Bytes {
    fn from(b: u64) -> Self {
        Self {
            container: vec![
                (b >> 56) as u8,
                (b >> 48) as u8,
                (b >> 40) as u8,
                (b >> 32) as u8,
                (b >> 24) as u8,
                (b >> 16) as u8,
                (b >> 8) as u8,
                b as u8,
            ],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<i8> for Bytes {
    fn from(b: i8) -> Self {
        Self {
            container: vec![b as u8],
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl From<&str> for Bytes {
    fn from(s: &str) -> Self {
        Self {
            container: s.as_bytes().iter().copied().collect(),
            append_byte_order: AppendByteOrder::BigEndian,
        }
    }
}

impl AsBytes for Bytes {
    fn into_bytes(self) -> Bytes {
        self
    }

    fn into_bytes_le(mut self) -> Bytes {
        Self {
            container: self.container.drain(..).rev().collect(),
            append_byte_order: AppendByteOrder::LittleEndian,
        }
    }
}

impl AsBytes for u8 {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

impl AsBytes for u16 {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

impl AsBytes for u32 {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

impl AsBytes for u64 {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

impl AsBytes for i8 {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

impl AsBytes for &str {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_bytes_le(self) -> Bytes {
        Bytes::from(self).into_bytes_le()
    }
}

macro_rules! bytes {
    ($($byte:expr),+) => ({
        Bytes::new(vec![$($byte),+])
    });
}

pub(crate) use bytes;
