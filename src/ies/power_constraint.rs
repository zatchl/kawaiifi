use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct PowerConstraint {
    bytes: Vec<u8>,
}

impl PowerConstraint {
    pub const ID: u8 = 32;
    pub const NAME: &'static str = "Power Constraint";
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<PowerConstraint, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(PowerConstraint { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn power_constraint_db(&self) -> u8 {
        self.bytes.get(0).unwrap_or(&0).clone()
    }
}

impl InformationElement for PowerConstraint {
    fn name(&self) -> &'static str {
        PowerConstraint::NAME
    }

    fn id(&self) -> u8 {
        PowerConstraint::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for PowerConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {} dB",
            PowerConstraint::NAME,
            self.power_constraint_db()
        )
    }
}
