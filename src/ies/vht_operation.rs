use super::{Field, IeError, InformationElement};
use crate::{ChannelWidth, ChannelWidths};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VhtOperation {
    bits: BitVec<Lsb0, u8>,
}

impl VhtOperation {
    pub const LENGTH: usize = 5;

    pub fn new(bytes: Vec<u8>) -> Result<VhtOperation, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(VhtOperation {
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

    }


impl InformationElement for VhtOperation {
    const NAME: &'static str = "VHT Operation";
    const ID: u8 = 192;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(VhtOperation);
    }
}
