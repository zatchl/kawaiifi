use super::{Field, IeError, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApChannelReport {
    bytes: Vec<u8>,
}

impl ApChannelReport {
    const MIN_LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<ApChannelReport, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(ApChannelReport { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::MIN_LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn operating_class(&self) -> u8 {
        self.bytes[0]
    }

    pub fn channel_list(&self) -> &[u8] {
        if self.bytes.len() == 1 {
            &[]
        } else {
            &self.bytes[1..]
        }
    }
}

impl InformationElement for ApChannelReport {
    const NAME: &'static str = "AP Channel Report";
    const ID: u8 = 51;

    fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new("Operating Class", self.operating_class()),
            Field::new("Channel List", format!("{:?}", self.channel_list())),
        ]
    }
}

impl_display_for_ie!(ApChannelReport);
