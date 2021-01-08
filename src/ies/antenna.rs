use super::{Field, IeError, InformationElement};

#[derive(Debug, Clone)]
pub struct Antenna {
    bytes: Vec<u8>,
}

impl Antenna {
    pub const NAME: &'static str = "Antenna";
    pub const ID: u8 = 64;
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
    fn name(&self) -> &'static str {
        Antenna::NAME
    }

    fn id(&self) -> u8 {
        Antenna::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field {
            title: "Antenna ID".to_string(),
            value: self.antenna_id().to_string(),
            subfields: None,
        }]
    }
}
