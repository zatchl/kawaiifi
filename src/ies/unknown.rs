use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct Unknown {
    bytes: Vec<u8>,
    id: u8,
    id_ext: Option<u8>,
}

impl Unknown {
    pub const NAME: &'static str = "Unknown";
    pub const ID: u8 = u8::MAX;

    pub fn new(bytes: Vec<u8>, id: u8, id_ext: Option<u8>) -> Unknown {
        Unknown { bytes, id, id_ext }
    }
}

impl InformationElement for Unknown {
    fn name(&self) -> &'static str {
        Unknown::NAME
    }

    fn id(&self) -> u8 {
        self.id
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
