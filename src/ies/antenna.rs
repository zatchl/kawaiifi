use super::{Field, IeError, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Antenna {
    bytes: Vec<u8>,
}

impl Antenna {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<Antenna, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(Antenna { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn antenna_id(&self) -> u8 {
        self.bytes[0]
    }
}

impl InformationElement for Antenna {
    const NAME: &'static str = "Antenna";
    const ID: u8 = 64;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new("Antenna ID", self.antenna_id())]
    }
}

impl_display_for_ie!(Antenna);
