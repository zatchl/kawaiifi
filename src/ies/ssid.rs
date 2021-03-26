use super::{Field, InformationElement};
use std::str;
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ssid {
    bytes: Vec<u8>,
}

impl Ssid {
    pub fn new(bytes: Vec<u8>) -> Ssid {
        Ssid { bytes }
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.bytes)
    }
}

impl InformationElement for Ssid {
    const NAME: &'static str = "SSID";
    const ID: u8 = 0;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new("SSID", self.as_str().unwrap_or_default())]
    }
}

impl_display_for_ie!(Ssid);
