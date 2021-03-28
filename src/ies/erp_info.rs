use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErpInfo {
    bits: BitArray<LocalBits, [u8; 1]>,
}

impl ErpInfo {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<ErpInfo, IeError> {
        let bytes: [u8; Self::LENGTH] =
            bytes
                .try_into()
                .map_err(|ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: Self::NAME,
                    expected_length: Self::LENGTH,
                    actual_length: ie_data.len(),
                })?;
        Ok(ErpInfo::from(bytes))
    }

    pub fn non_erp_present(&self) -> bool {
        self.bits[0]
    }

    pub fn use_protection(&self) -> bool {
        self.bits[1]
    }

    pub fn barker_preamble_mode(&self) -> bool {
        self.bits[2]
    }
}

impl InformationElement for ErpInfo {
    const NAME: &'static str = "ERP Info";
    const ID: u8 = 42;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new("Non-ERP Present", self.non_erp_present()),
            Field::new("Use Protection", self.use_protection()),
            Field::new("Barker Preamble Mode", self.barker_preamble_mode()),
        ]
    }
}

impl From<[u8; Self::LENGTH]> for ErpInfo {
    fn from(bytes: [u8; 1]) -> Self {
        ErpInfo {
            bits: BitArray::new(bytes),
        }
    }
}

impl_display_for_ie!(ErpInfo);
