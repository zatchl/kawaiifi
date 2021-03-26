use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wpa {
    bytes: Vec<u8>,
}

impl Wpa {
    pub const OUI: [u8; 4] = [0x00, 0x50, 0xF2, 0x01];

    pub fn new(bytes: Vec<u8>) -> Wpa {
        Wpa { bytes }
    }
}

impl InformationElement for Wpa {
    const NAME: &'static str = "WPA";
    const ID: u8 = 221;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(Wpa);
