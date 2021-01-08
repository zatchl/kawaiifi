use super::{Field, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct ErpInfo {
    bits: BitArray<LocalBits, [u8; 1]>,
}

impl ErpInfo {
    pub const NAME: &'static str = "ERP Info";
    pub const ID: u8 = 42;
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
    fn name(&self) -> &'static str {
        ErpInfo::NAME
    }

    fn id(&self) -> u8 {
        ErpInfo::ID
    }

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field {
                title: "Non-ERP Present".to_string(),
                value: self.non_erp_present().to_string(),
                subfields: None,
            },
            Field {
                title: "Use Protection".to_string(),
                value: self.use_protection().to_string(),
                subfields: None,
            },
            Field {
                title: "Barker Preamble Mode".to_string(),
                value: self.barker_preamble_mode().to_string(),
                subfields: None,
            },
        ]
    }
}
