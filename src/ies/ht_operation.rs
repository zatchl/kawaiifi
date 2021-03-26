use super::{Field, IeError, InformationElement};
use crate::{ChannelWidth, ChannelWidths};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtOperation {
    bits: BitVec<Lsb0, u8>,
}

impl HtOperation {
    pub const LENGTH: usize = 22;

    pub fn new(bytes: Vec<u8>) -> Result<HtOperation, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(HtOperation {
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

impl InformationElement for HtOperation {
    fn name(&self) -> &'static str {
        HtOperation::NAME
    }

    fn id(&self) -> u8 {
        HtOperation::ID
    }

impl InformationElement for HtOperation {
    const NAME: &'static str = "HT Operation";
    const ID: u8 = 61;

    fn bytes(&self) -> &[u8] {
        &self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(HtOperation);

    }
}
