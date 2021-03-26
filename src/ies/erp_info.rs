use super::{Field, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErpInfo {
    bits: BitArray<LocalBits, [u8; 1]>,
}

impl ErpInfo {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: [u8; 1]) -> ErpInfo {
        ErpInfo {
            bits: BitArray::new(bytes),
        }
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

impl_display_for_ie!(ErpInfo);
