use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16(u16);

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16(value)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16(*value as u16)
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        SaturatingU16(self.0.saturating_add(other.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;

    fn add(self, other: u16) -> u16 {
        self.0.saturating_add(other)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        SaturatingU16(self.0.saturating_add(other.0))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u16> for SaturatingU16 {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}
