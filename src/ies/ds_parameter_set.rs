use super::{Display, IeError, InformationElement};

#[derive(Debug)]
pub struct DsParameterSet {
    bytes: Vec<u8>,
}

impl DsParameterSet {
    pub const ID: u8 = 3;
    pub const NAME: &'static str = "DS Parameter Set";
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<DsParameterSet, IeError> {
        if bytes.len() == Self::LENGTH {
            Ok(DsParameterSet { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn channel_number(&self) -> u8 {
        self.bytes.get(0).unwrap_or(&0).clone()
    }
}

impl InformationElement for DsParameterSet {
    fn name(&self) -> &'static str {
        DsParameterSet::NAME
    }

    fn id(&self) -> u8 {
        DsParameterSet::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for DsParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: Channel {}", Self::NAME, self.channel_number())
    }
}
