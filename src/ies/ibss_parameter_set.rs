use super::{Field, IeError, InformationElement};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbssParameterSet {
    bytes: [u8; Self::LENGTH],
}

impl IbssParameterSet {
    pub const LENGTH: usize = 2;

    pub fn new(bytes: Vec<u8>) -> Result<IbssParameterSet, IeError> {
        let bytes: [u8; Self::LENGTH] =
            bytes
                .try_into()
                .map_err(|ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: Self::NAME,
                    expected_length: Self::LENGTH,
                    actual_length: ie_data.len(),
                })?;
        Ok(IbssParameterSet::from(bytes))
    }

    pub fn atim_window_tu(&self) -> u16 {
        u16::from_ne_bytes(self.bytes)
    }
}

impl InformationElement for IbssParameterSet {
    const NAME: &'static str = "IBSS Parameter Set";
    const ID: u8 = 6;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new(
            "ATIM Window",
            format!("{} TU", self.atim_window_tu()),
        )]
    }
}

impl From<[u8; Self::LENGTH]> for IbssParameterSet {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        IbssParameterSet { bytes }
    }
}

impl_display_for_ie!(IbssParameterSet);
