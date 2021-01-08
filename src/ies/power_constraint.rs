use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct PowerConstraint {
    bytes: [u8; 1],
}

impl PowerConstraint {
    pub const NAME: &'static str = "Power Constraint";
    pub const ID: u8 = 32;
    pub const LENGTH: usize = 1;

    pub fn new(bytes: [u8; 1]) -> PowerConstraint {
        PowerConstraint { bytes }
    }

    pub fn power_constraint_db(&self) -> u8 {
        self.bytes[0]
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

    fn information_fields(&self) -> Vec<Field> {
        vec![Field {
            title: "Local Power Constraint".to_string(),
            value: format!("{} dB", self.power_constraint_db()),
            subfields: None,
        }]
    }
}
