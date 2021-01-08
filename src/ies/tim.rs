use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct Tim {
    bytes: Vec<u8>,
}

impl Tim {
    pub const NAME: &'static str = "TIM";
    pub const ID: u8 = 5;

    pub fn new(bytes: Vec<u8>) -> Tim {
        Tim { bytes }
    }
}

impl InformationElement for Tim {
    fn name(&self) -> &'static str {
        Tim::NAME
    }

    fn id(&self) -> u8 {
        Tim::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}
