use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct HtCapabilities {
    bytes: Vec<u8>,
}

impl HtCapabilities {
    pub const ID: u8 = 45;
    pub const NAME: &'static str = "HT Capabilities";
    pub const LENGTH: usize = 26;

    pub fn new(bytes: Vec<u8>) -> Result<HtCapabilities, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(HtCapabilities { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for HtCapabilities {
    fn name(&self) -> &'static str {
        HtCapabilities::NAME
    }

    fn id(&self) -> u8 {
        HtCapabilities::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for HtCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
