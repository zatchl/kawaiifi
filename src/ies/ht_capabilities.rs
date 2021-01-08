use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct HtCapabilities {
    bits: BitVec<Lsb0, u8>,
}

impl HtCapabilities {
    pub const NAME: &'static str = "HT Capabilities";
    pub const ID: u8 = 45;
    pub const LENGTH: usize = 26;

    pub fn new(bytes: Vec<u8>) -> Result<HtCapabilities, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(HtCapabilities {
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

impl InformationElement for HtCapabilities {
    fn name(&self) -> &'static str {
        HtCapabilities::NAME
    }

    fn id(&self) -> u8 {
        HtCapabilities::ID
    }

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
