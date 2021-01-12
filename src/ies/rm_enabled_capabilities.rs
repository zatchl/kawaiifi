use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct RmEnabledCapabilities {
    bits: BitVec<LocalBits, u8>,
}

impl RmEnabledCapabilities {
    pub const NAME: &'static str = "RM Enabled Capabilities";
    pub const ID: u8 = 70;
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
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn id(&self) -> u8 {
        Self::ID
    }

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field {
                title: "Link Measurement Capability".to_string(),
                value: self.link_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Neighbor Report Capability".to_string(),
                value: self.neighbor_report_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Parallel Measurements Capability".to_string(),
                value: self.parallel_measurements_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Repeated Measurements Capability".to_string(),
                value: self.repeated_measurements_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Beacon Passive Measurement Capability".to_string(),
                value: self.beacon_passive_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Beacon Active Measurement Capability".to_string(),
                value: self.beacon_active_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Beacon Table Measurement Capability".to_string(),
                value: self.beacon_table_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Beacon Measurement Reporting Conditions Capability".to_string(),
                value: self
                    .beacon_measurement_reporting_conditions_capability()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "Frame Measurement Capability".to_string(),
                value: self.frame_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Channel Load Measurement Capability".to_string(),
                value: self.channel_load_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Noise Histogram Measurement Capability".to_string(),
                value: self.noise_histogram_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Statistics Measurement Capability".to_string(),
                value: self.statistics_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "LCI Measurement Capability".to_string(),
                value: self.lci_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "LCI Azimuth Capability".to_string(),
                value: self.lci_azimuth_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Transmit Stream/Category Measurement Capability".to_string(),
                value: self
                    .transmit_stream_category_measurement_capability()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "Triggered Transmit Stream/Category Measurement Capability".to_string(),
                value: self
                    .triggered_transmit_stream_category_measurement_capability()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "AP Channel Report Capability".to_string(),
                value: self.ap_channel_report_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "RM MIB Capability".to_string(),
                value: self.rm_mib_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Operating Channel Max Measurement Duration".to_string(),
                value: self
                    .operating_channel_max_measurement_duration()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "Nonoperating Channel Max Measurement Duration".to_string(),
                value: self
                    .nonoperating_channel_max_measurement_duration()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "Measurement Pilot Capability".to_string(),
                value: self.measurement_pilot_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Measurement Pilot Transmission Information Capability".to_string(),
                value: self
                    .measurement_pilot_transmission_information_capability()
                    .to_string(),
                subfields: None,
            },
            Field {
                title: "Neighbor Report TSF Offset Capability".to_string(),
                value: self.neighbor_report_tsf_offset_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "RCPI Measurement Capability".to_string(),
                value: self.rcpi_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "RSNI Measurement Capability".to_string(),
                value: self.rsni_measurement_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "BSS Average Access Delay Capability".to_string(),
                value: self.bss_average_access_delay_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "BSS Available Admission Capacity Capability".to_string(),
                value: self.bss_available_admission_capacity_capacity().to_string(),
                subfields: None,
            },
            Field {
                title: "Antenna Capability".to_string(),
                value: self.antenna_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "FTM Range Report Capability".to_string(),
                value: self.ftm_range_report_capability().to_string(),
                subfields: None,
            },
            Field {
                title: "Civic Location Measurement Capability".to_string(),
                value: self.civic_location_measurement_capability().to_string(),
                subfields: None,
            },
        ]
    }
}
