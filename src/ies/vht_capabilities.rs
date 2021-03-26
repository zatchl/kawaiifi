use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VhtCapabilities {
    bits: BitVec<Lsb0, u8>,
}

impl VhtCapabilities {
    pub const LENGTH: usize = 12;

    pub fn new(bytes: Vec<u8>) -> Result<VhtCapabilities, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(VhtCapabilities {
                bits: BitVec::from_vec(bytes),
            })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for VhtCapabilities {
    fn name(&self) -> &'static str {
        VhtCapabilities::NAME
    }

    fn id(&self) -> u8 {
        VhtCapabilities::ID
    }

impl InformationElement for VhtCapabilities {
    const NAME: &'static str = "VHT Capabilities";
    const ID: u8 = 191;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(VhtCapabilities);
    }
}
