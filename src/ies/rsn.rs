use super::{Display, InformationElement};

#[derive(Debug)]
pub struct Rsn {
    bytes: Vec<u8>,
}

impl Rsn {
    pub const ID: u8 = 48;
    pub const NAME: &'static str = "RSN";

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

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for Rsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
