use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct HtOperation {
    bytes: Vec<u8>,
}

impl HtOperation {
    pub const ID: u8 = 61;
    pub const NAME: &'static str = "HT Operation";
    pub const LENGTH: usize = 22;

    pub fn new(bytes: Vec<u8>) -> Result<HtOperation, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(HtOperation { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for HtOperation {
    fn name(&self) -> &'static str {
        HtOperation::NAME
    }

    fn id(&self) -> u8 {
        HtOperation::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for HtOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes())
    }
}
