use crate::Ie;
use enumflags2::BitFlags;
use std::{convert::From, ops::Deref};

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum SecurityProtocol {
    WEP = 1 << 0,
    WPA = 1 << 1,
    WPA2 = 1 << 2,
    WPA3 = 1 << 3,
}

// Use the Newtype pattern to create a type alias (SecurityProtocols) and implement the From trait
#[derive(Debug, Copy, Clone)]
pub struct SecurityProtocols(BitFlags<SecurityProtocol>);

impl Deref for SecurityProtocols {
    type Target = BitFlags<SecurityProtocol>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[Ie]> for SecurityProtocols {
    fn from(_: &[Ie]) -> Self {
        SecurityProtocols(BitFlags::empty())
    }
}
