use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsParameterSet {
    bytes: [u8; 1],
}

impl DsParameterSet {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: [u8; 1]) -> DsParameterSet {
        DsParameterSet { bytes }
    }

    pub fn channel_number(&self) -> u8 {
        self.bytes[0]
    }
}

impl InformationElement for DsParameterSet {
    const NAME: &'static str = "DS Parameter Set";
    const ID: u8 = 3;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new("Current Channel", self.channel_number())]
    }
}

impl_display_for_ie!(DsParameterSet);
