use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tim {
    bytes: Vec<u8>,
}

impl Tim {
    pub fn new(bytes: Vec<u8>) -> Tim {
        Tim { bytes }
    }
}

impl InformationElement for Tim {
    const NAME: &'static str = "TIM";
    const ID: u8 = 5;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(Tim);
