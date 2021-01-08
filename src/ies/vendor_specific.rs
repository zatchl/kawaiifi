use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct VendorSpecific {
    bytes: Vec<u8>,
}

impl VendorSpecific {
    pub const NAME: &'static str = "Vendor Specific";
    pub const ID: u8 = 221;

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

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
