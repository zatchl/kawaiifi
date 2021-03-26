use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RmEnabledCapabilities {
    bits: BitVec<LocalBits, u8>,
}

impl RmEnabledCapabilities {
    pub const MIN_LENGTH: usize = 5;

    pub fn new(bytes: Vec<u8>) -> Result<RmEnabledCapabilities, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(RmEnabledCapabilities {
                bits: BitVec::from_vec(bytes),
            })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::MIN_LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn link_measurement_capability(&self) -> bool {
        self.bits[0]
    }

    pub fn neighbor_report_capability(&self) -> bool {
        self.bits[1]
    }

    pub fn parallel_measurements_capability(&self) -> bool {
        self.bits[2]
    }

    pub fn repeated_measurements_capability(&self) -> bool {
        self.bits[3]
    }

    pub fn beacon_passive_measurement_capability(&self) -> bool {
        self.bits[4]
    }

    pub fn beacon_active_measurement_capability(&self) -> bool {
        self.bits[5]
    }

    pub fn beacon_table_measurement_capability(&self) -> bool {
        self.bits[6]
    }

    pub fn beacon_measurement_reporting_conditions_capability(&self) -> bool {
        self.bits[7]
    }

    pub fn frame_measurement_capability(&self) -> bool {
        self.bits[8]
    }

    pub fn channel_load_measurement_capability(&self) -> bool {
        self.bits[9]
    }

    pub fn noise_histogram_measurement_capability(&self) -> bool {
        self.bits[10]
    }

    pub fn statistics_measurement_capability(&self) -> bool {
        self.bits[11]
    }

    pub fn lci_measurement_capability(&self) -> bool {
        self.bits[12]
    }

    pub fn lci_azimuth_capability(&self) -> bool {
        self.bits[13]
    }

    pub fn transmit_stream_category_measurement_capability(&self) -> bool {
        self.bits[14]
    }

    pub fn triggered_transmit_stream_category_measurement_capability(&self) -> bool {
        self.bits[15]
    }

    pub fn ap_channel_report_capability(&self) -> bool {
        self.bits[16]
    }

    pub fn rm_mib_capability(&self) -> bool {
        self.bits[17]
    }

    pub fn operating_channel_max_measurement_duration(&self) -> u8 {
        self.bits[18..=20].load()
    }

    pub fn nonoperating_channel_max_measurement_duration(&self) -> u8 {
        self.bits[21..=23].load()
    }

    pub fn measurement_pilot_capability(&self) -> u8 {
        self.bits[24..=26].load()
    }

    pub fn measurement_pilot_transmission_information_capability(&self) -> bool {
        self.bits[27]
    }

    pub fn neighbor_report_tsf_offset_capability(&self) -> bool {
        self.bits[28]
    }

    pub fn rcpi_measurement_capability(&self) -> bool {
        self.bits[29]
    }

    pub fn rsni_measurement_capability(&self) -> bool {
        self.bits[30]
    }

    pub fn bss_average_access_delay_capability(&self) -> bool {
        self.bits[31]
    }

    pub fn bss_available_admission_capacity_capacity(&self) -> bool {
        self.bits[32]
    }

    pub fn antenna_capability(&self) -> bool {
        self.bits[33]
    }

    pub fn ftm_range_report_capability(&self) -> bool {
        self.bits[34]
    }

    pub fn civic_location_measurement_capability(&self) -> bool {
        self.bits[35]
    }
}

impl InformationElement for RmEnabledCapabilities {
    const NAME: &'static str = "RM Enabled Capabilities";
    const ID: u8 = 70;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new(
                "Link Measurement Capability",
                self.link_measurement_capability(),
            ),
            Field::new(
                "Neighbor Report Capability",
                self.neighbor_report_capability(),
            ),
            Field::new(
                "Parallel Measurements Capability",
                self.parallel_measurements_capability(),
            ),
            Field::new(
                "Repeated Measurements Capability",
                self.repeated_measurements_capability(),
            ),
            Field::new(
                "Beacon Passive Measurement Capability",
                self.beacon_passive_measurement_capability(),
            ),
            Field::new(
                "Beacon Active Measurement Capability",
                self.beacon_active_measurement_capability(),
            ),
            Field::new(
                "Beacon Table Measurement Capability",
                self.beacon_table_measurement_capability(),
            ),
            Field::new(
                "Beacon Measurement Reporting Conditions Capability",
                self.beacon_measurement_reporting_conditions_capability(),
            ),
            Field::new(
                "Frame Measurement Capability",
                self.frame_measurement_capability(),
            ),
            Field::new(
                "Channel Load Measurement Capability",
                self.channel_load_measurement_capability(),
            ),
            Field::new(
                "Noise Histogram Measurement Capability",
                self.noise_histogram_measurement_capability(),
            ),
            Field::new(
                "Statistics Measurement Capability",
                self.statistics_measurement_capability(),
            ),
            Field::new(
                "LCI Measurement Capability",
                self.lci_measurement_capability(),
            ),
            Field::new("LCI Azimuth Capability", self.lci_azimuth_capability()),
            Field::new(
                "Transmit Stream/Category Measurement Capability",
                self.transmit_stream_category_measurement_capability(),
            ),
            Field::new(
                "Triggered Transmit Stream/Category Measurement Capability",
                self.triggered_transmit_stream_category_measurement_capability(),
            ),
            Field::new(
                "AP Channel Report Capability",
                self.ap_channel_report_capability(),
            ),
            Field::new("RM MIB Capability", self.rm_mib_capability()),
            Field::new(
                "Operating Channel Max Measurement Duration",
                self.operating_channel_max_measurement_duration(),
            ),
            Field::new(
                "Nonoperating Channel Max Measurement Duration",
                self.nonoperating_channel_max_measurement_duration(),
            ),
            Field::new(
                "Measurement Pilot Capability",
                self.measurement_pilot_capability(),
            ),
            Field::new(
                "Measurement Pilot Transmission Information Capability",
                self.measurement_pilot_transmission_information_capability(),
            ),
            Field::new(
                "Neighbor Report TSF Offset Capability",
                self.neighbor_report_tsf_offset_capability(),
            ),
            Field::new(
                "RCPI Measurement Capability",
                self.rcpi_measurement_capability(),
            ),
            Field::new(
                "RSNI Measurement Capability",
                self.rsni_measurement_capability(),
            ),
            Field::new(
                "BSS Average Access Delay Capability",
                self.bss_average_access_delay_capability(),
            ),
            Field::new(
                "BSS Available Admission Capacity Capability",
                self.bss_available_admission_capacity_capacity(),
            ),
            Field::new("Antenna Capability", self.antenna_capability()),
            Field::new(
                "FTM Range Report Capability",
                self.ftm_range_report_capability(),
            ),
            Field::new(
                "Civic Location Measurement Capability",
                self.civic_location_measurement_capability(),
            ),
        ]
    }
}

impl_display_for_ie!(RmEnabledCapabilities);
