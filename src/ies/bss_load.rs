use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BssLoad {
    bits: BitVec<LocalBits, u8>,
}

impl BssLoad {
    pub const LENGTH: usize = 5;

    pub fn new(bytes: Vec<u8>) -> Result<BssLoad, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(BssLoad {
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

    pub fn station_count(&self) -> u16 {
        self.bits[0..16].load::<u16>()
    }

    pub fn channel_utilization(&self) -> u8 {
        self.bits.as_raw_slice()[2]
    }

    pub fn available_admission_capacity(&self) -> u16 {
        self.bits[24..40].load::<u16>()
    }
}

impl InformationElement for BssLoad {
    const NAME: &'static str = "Bss Load";
    const ID: u8 = 11;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new("Station Count", self.station_count()),
            Field::new(
                "Channel Utilization",
                format!("{}%", self.channel_utilization()),
            ),
            Field::new(
                "Available Admission Capacity",
                format!(
                    "{} μs/s",
                    u32::from(self.available_admission_capacity()) * 32
                ),
            ),
        ]
    }
}

impl_display_for_ie!(BssLoad);
