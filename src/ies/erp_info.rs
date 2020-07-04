use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct ErpInfo {
    bytes: Vec<u8>,
}

impl ErpInfo {
    pub const ID: u8 = 42;
    pub const NAME: &'static str = "ERP Info";
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<ErpInfo, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(ErpInfo { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }
}

impl InformationElement for ErpInfo {
    fn name(&self) -> &'static str {
        ErpInfo::NAME
    }

    fn id(&self) -> u8 {
        ErpInfo::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for ErpInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes)
    }
}
