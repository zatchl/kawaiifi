use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorSpecific {
    bytes: Vec<u8>,
}

impl VendorSpecific {
    pub fn new(bytes: Vec<u8>) -> VendorSpecific {
        VendorSpecific { bytes }
    }
}

impl InformationElement for VendorSpecific {
    const NAME: &'static str = "Vendor Specific";
    const ID: u8 = 221;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(VendorSpecific);
