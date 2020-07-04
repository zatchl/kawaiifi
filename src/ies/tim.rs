use super::{Display, InformationElement};

#[derive(Debug)]
pub struct Tim {
    bytes: Vec<u8>,
}

impl Tim {
    pub const ID: u8 = 5;
    pub const NAME: &'static str = "TIM";

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
}

impl Display for Tim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
