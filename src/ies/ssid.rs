use super::{Display, InformationElement};

#[derive(Debug)]
pub struct Ssid {
    bytes: Vec<u8>,
}

impl Ssid {
    pub const ID: u8 = 0;
    pub const NAME: &'static str = "SSID";

    pub fn new(bytes: Vec<u8>) -> Ssid {
        Ssid { bytes }
    }
}

impl InformationElement for Ssid {
    fn name(&self) -> &'static str {
        Ssid::NAME
    }

    fn id(&self) -> u8 {
        Ssid::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for Ssid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            Ssid::NAME,
            String::from_utf8_lossy(&self.bytes)
        )
    }
}
