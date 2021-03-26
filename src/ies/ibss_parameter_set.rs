use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbssParameterSet {
    bytes: [u8; Self::LENGTH],
}

impl IbssParameterSet {
    pub const LENGTH: usize = 2;

    pub fn new(bytes: [u8; Self::LENGTH]) -> IbssParameterSet {
        IbssParameterSet { bytes }
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

impl_display_for_ie!(IbssParameterSet);
