use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct Wpa {
    bytes: Vec<u8>,
}

impl Wpa {
    pub const NAME: &'static str = "WPA";
    pub const ID: u8 = 221;
    pub const OUI: [u8; 4] = [0x00, 0x50, 0xF2, 0x01];

    pub fn new(bytes: Vec<u8>) -> Result<Wpa, String> {
        Ok(Wpa { bytes })
    }
}

impl InformationElement for Wpa {
    fn name(&self) -> &'static str {
        Wpa::NAME
    }

    fn id(&self) -> u8 {
        Wpa::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
