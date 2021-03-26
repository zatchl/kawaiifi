use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rsn {
    bytes: Vec<u8>,
}

impl Rsn {

    pub fn new(bytes: Vec<u8>) -> Rsn {
        Rsn { bytes }
    }
}

impl InformationElement for Rsn {
    fn name(&self) -> &'static str {
        Rsn::NAME
    }

    fn id(&self) -> u8 {
        Rsn::ID
    }
}

impl InformationElement for Rsn {
    const NAME: &'static str = "RSN";
    const ID: u8 = 48;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        Vec::new()
    }
}

impl_display_for_ie!(Rsn);
    }
}
