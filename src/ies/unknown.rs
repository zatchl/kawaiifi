use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown {
    bytes: Vec<u8>,
    id: u8,
    id_ext: Option<u8>,
}

impl Unknown {
    pub fn new(bytes: Vec<u8>, id: u8, id_ext: Option<u8>) -> Unknown {
        Unknown { bytes, id, id_ext }
    }
}

impl InformationElement for Unknown {
    const NAME: &'static str = "Unknown";
    const ID: u8 = u8::MAX;

    fn id(&self) -> u8 {
        self.id
    }

    fn id_ext(&self) -> Option<u8> {
        self.id_ext
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(Unknown);
