use super::{Field, IeError, InformationElement};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlappingBssScanParams {
    bytes: [u8; Self::LENGTH],
}

impl OverlappingBssScanParams {
    pub const LENGTH: usize = 14;

    pub fn new(bytes: Vec<u8>) -> Result<OverlappingBssScanParams, IeError> {
        let bytes: [u8; Self::LENGTH] =
            bytes
                .try_into()
                .map_err(|ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: Self::NAME,
                    expected_length: Self::LENGTH,
                    actual_length: ie_data.len(),
                })?;
        Ok(OverlappingBssScanParams::from(bytes))
    }

    pub fn obss_scan_passive_dwell_tu(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[0..2].try_into().unwrap_or_default())
    }

    pub fn obss_scan_active_dwell_tu(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[2..4].try_into().unwrap_or_default())
    }

    pub fn bss_channel_width_trigger_scan_interval_secs(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[4..6].try_into().unwrap_or_default())
    }

    pub fn obss_scan_passive_total_per_channel_tu(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[6..8].try_into().unwrap_or_default())
    }

    pub fn obss_scan_active_total_per_channel_tu(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[8..10].try_into().unwrap_or_default())
    }

    pub fn bss_width_channel_transition_delay_factor(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[10..12].try_into().unwrap_or_default())
    }

    pub fn obss_scan_activity_threshold(&self) -> u16 {
        u16::from_ne_bytes(self.bytes[12..14].try_into().unwrap_or_default())
    }
}

impl InformationElement for OverlappingBssScanParams {
    const NAME: &'static str = "Overlapping BSS Scan Parameters";
    const ID: u8 = 74;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new(
                "OBSS Scan Passive Dwell",
                format!("{} TU", self.obss_scan_passive_dwell_tu()),
            ),
            Field::new(
                "OBSS Scan Active Dwell",
                format!("{} TU", self.obss_scan_active_dwell_tu()),
            ),
            Field::new(
                "BSS Channel Width Trigger Scan Interval",
                format!(
                    "{} seconds",
                    self.bss_channel_width_trigger_scan_interval_secs()
                ),
            ),
            Field::new(
                "OBSS Scan Passive Total Per Channel",
                format!("{} TU", self.obss_scan_passive_total_per_channel_tu()),
            ),
            Field::new(
                "OBSS Scan Active Total Per Channel",
                format!("{} TU", self.obss_scan_active_total_per_channel_tu()),
            ),
            Field::new(
                "BSS Width Channel Transition Delay Factor",
                self.bss_width_channel_transition_delay_factor().to_string(),
            ),
            Field::new(
                "OBSS Scan Activity Threshold",
                self.obss_scan_activity_threshold().to_string(),
            ),
        ]
    }
}

impl From<[u8; Self::LENGTH]> for OverlappingBssScanParams {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        OverlappingBssScanParams { bytes }
    }
}

impl_display_for_ie!(OverlappingBssScanParams);
