use super::{Field, IeError, InformationElement};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerConstraint {
    bytes: [u8; Self::LENGTH],
}

impl PowerConstraint {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<PowerConstraint, IeError> {
        let bytes: [u8; Self::LENGTH] =
            bytes
                .try_into()
                .map_err(|ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: Self::NAME,
                    expected_length: Self::LENGTH,
                    actual_length: ie_data.len(),
                })?;
        Ok(PowerConstraint::from(bytes))
    }

    pub fn power_constraint_db(&self) -> u8 {
        self.bytes[0]
    }
}

impl InformationElement for PowerConstraint {
    const NAME: &'static str = "Power Constraint";
    const ID: u8 = 32;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new(
            "Local Power Constraint",
            format!("{} dB", self.power_constraint_db()),
        )]
    }
}

impl From<[u8; Self::LENGTH]> for PowerConstraint {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        PowerConstraint { bytes }
    }
}

impl_display_for_ie!(PowerConstraint);
