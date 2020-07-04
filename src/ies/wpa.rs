use super::{Display, InformationElement};

#[derive(Debug)]
pub struct Wpa {
    bytes: Vec<u8>,
}

impl Wpa {
    pub const ID: u8 = 221;
    pub const NAME: &'static str = "WPA";
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
}

impl Display for Wpa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
