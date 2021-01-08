use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct DsParameterSet {
    bytes: [u8; 1],
}

impl DsParameterSet {
    pub const NAME: &'static str = "DS Parameter Set";
    pub const ID: u8 = 3;
    pub const LENGTH: usize = 1;

    pub fn new(bytes: [u8; 1]) -> DsParameterSet {
        DsParameterSet { bytes }
    }

    pub fn channel_number(&self) -> u8 {
        self.bytes[0]
    }
}

impl InformationElement for DsParameterSet {
    fn name(&self) -> &'static str {
        DsParameterSet::NAME
    }

    fn id(&self) -> u8 {
        DsParameterSet::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field {
            title: "Current Channel".to_string(),
            value: self.channel_number().to_string(),
            subfields: None,
        }]
    }
}
