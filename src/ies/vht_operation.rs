use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct VhtOperation {
    bytes: Vec<u8>,
}

impl VhtOperation {
    pub const ID: u8 = 192;
    pub const NAME: &'static str = "VHT Operation";
    pub const LENGTH: usize = 5;

    pub fn new(bytes: Vec<u8>) -> Result<VhtOperation, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(VhtOperation { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for VhtOperation {
    fn name(&self) -> &'static str {
        VhtOperation::NAME
    }

    fn id(&self) -> u8 {
        VhtOperation::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for VhtOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
