use super::{Field, IeError, InformationElement};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsParameterSet {
    bytes: [u8; 1],
}

impl DsParameterSet {
    pub const LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<Self, IeError> {
        let bytes: [u8; Self::LENGTH] =
            bytes
                .try_into()
                .map_err(|ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: Self::NAME,
                    expected_length: Self::LENGTH,
                    actual_length: ie_data.len(),
                })?;
        Ok(DsParameterSet::from(bytes))
    }

    pub fn channel_number(&self) -> u8 {
        self.bytes[0]
    }
}

impl InformationElement for DsParameterSet {
    const NAME: &'static str = "DS Parameter Set";
    const ID: u8 = 3;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new("Current Channel", self.channel_number())]
    }
}

impl From<[u8; Self::LENGTH]> for DsParameterSet {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        DsParameterSet { bytes }
    }
}

impl_display_for_ie!(DsParameterSet);
