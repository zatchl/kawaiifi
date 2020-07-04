use super::{Display, InformationElement};

#[derive(Debug)]
pub struct ExtendedCapabilities {
    bytes: Vec<u8>,
}

impl ExtendedCapabilities {
    pub const ID: u8 = 127;
    pub const NAME: &'static str = "Extended Capabilities";

    pub fn new(bytes: Vec<u8>) -> ExtendedCapabilities {
        ExtendedCapabilities { bytes }
    }
}

impl InformationElement for ExtendedCapabilities {
    fn name(&self) -> &'static str {
        ExtendedCapabilities::NAME
    }

    fn id(&self) -> u8 {
        ExtendedCapabilities::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for ExtendedCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes)
    }
}
