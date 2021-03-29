use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwentyFortyBssCoexistence {
    bits: BitVec<LocalBits, u8>,
}

impl TwentyFortyBssCoexistence {
    const MIN_LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<TwentyFortyBssCoexistence, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(TwentyFortyBssCoexistence {
                bits: BitVec::from_vec(bytes),
            })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::MIN_LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn information_request(&self) -> bool {
        self.bits[0]
    }

    pub fn forty_mhz_intolerant(&self) -> bool {
        self.bits[1]
    }

    pub fn twenty_mhz_bss_width_request(&self) -> bool {
        self.bits[2]
    }

    pub fn obss_scanning_exemption_request(&self) -> bool {
        self.bits[3]
    }

    pub fn obss_scanning_exemption_grant(&self) -> bool {
        self.bits[4]
    }
}

impl InformationElement for TwentyFortyBssCoexistence {
    const NAME: &'static str = "20/40 BSS Coexistence";
    const ID: u8 = 72;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new("Information Request", self.information_request()),
            Field::new("Forty MHz Intolerant", self.forty_mhz_intolerant()),
            Field::new(
                "20 MHz BSS Width Request",
                self.twenty_mhz_bss_width_request(),
            ),
            Field::new(
                "OBSS Scanning Exemption Request",
                self.obss_scanning_exemption_request(),
            ),
            Field::new(
                "OBSS Scanning Exemption Grant",
                self.obss_scanning_exemption_grant(),
            ),
        ]
    }
}

impl_display_for_ie!(TwentyFortyBssCoexistence);
