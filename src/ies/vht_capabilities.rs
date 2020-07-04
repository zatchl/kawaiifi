use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct VhtCapabilities {
    bytes: Vec<u8>,
}

impl VhtCapabilities {
    pub const ID: u8 = 191;
    pub const NAME: &'static str = "VHT Capabilities";
    pub const LENGTH: usize = 12;

    pub fn new(bytes: Vec<u8>) -> Result<VhtCapabilities, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(VhtCapabilities { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for VhtCapabilities {
    fn name(&self) -> &'static str {
        VhtCapabilities::NAME
    }

    fn id(&self) -> u8 {
        VhtCapabilities::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for VhtCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
