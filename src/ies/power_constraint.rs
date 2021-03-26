use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerConstraint {
    bytes: [u8; 1],
}

impl PowerConstraint {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: [u8; 1]) -> PowerConstraint {
        PowerConstraint { bytes }
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

impl_display_for_ie!(PowerConstraint);
