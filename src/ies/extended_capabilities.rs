use super::{Field, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct ExtendedCapabilities {
    bits: BitVec<Lsb0, u8>,
}

impl ExtendedCapabilities {
    pub const NAME: &'static str = "Extended Capabilities";
    pub const ID: u8 = 127;

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
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
