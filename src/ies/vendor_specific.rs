use super::{Display, InformationElement};

#[derive(Debug)]
pub struct VendorSpecific {
    bytes: Vec<u8>,
}

impl VendorSpecific {
    pub const ID: u8 = 221;
    pub const NAME: &'static str = "Vendor Specific";

    pub fn new(bytes: Vec<u8>) -> VendorSpecific {
        VendorSpecific { bytes }
    }
}

impl InformationElement for VendorSpecific {
    fn name(&self) -> &'static str {
        VendorSpecific::NAME
    }

    fn id(&self) -> u8 {
        VendorSpecific::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for VendorSpecific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
