use super::{Field, IeError, InformationElement};
use crate::ChannelNumber;
use std::convert::TryFrom;
use std::{fmt::Display, str};

pub enum Environment {
    Any,
    Outdoor,
    Indoor,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Any => write!(f, "Any"),
            Environment::Outdoor => write!(f, "Outdoor"),
            Environment::Indoor => write!(f, "Indoor"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct OperatingInfo {
    operating_extension_id: u8,
    operating_class: u8,
    coverage_class: u8,
}

impl OperatingInfo {
    pub fn operating_extension_id(&self) -> u8 {
        self.operating_extension_id
    }

    pub fn operating_class(&self) -> u8 {
        self.operating_class
    }

    pub fn coverage_class(&self) -> u8 {
        self.coverage_class
    }

    pub fn air_propagation_time_us(&self) -> Option<u16> {
        match self.coverage_class {
            0..=31 => Some(self.coverage_class as u16 * 3),
            _ => None,
        }
    }
}

impl Display for OperatingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operating Info:\r\n\tOperating Extension ID: {}\r\n\tOperating Class: {}\r\n\tCoverage Class: {}\r\n\t", self.operating_extension_id, self.operating_class,self.coverage_class)
    }
}

pub struct SubbandInfo {
    first_channel_number: ChannelNumber,
    number_of_channels: u8,
    max_transmit_power_level_dbm: i8,
    operating_info: Option<OperatingInfo>,
}

impl SubbandInfo {
    pub fn first_channel_number(&self) -> ChannelNumber {
        self.first_channel_number
    }

    pub fn number_of_channels(&self) -> u8 {
        self.number_of_channels
    }

    pub fn max_transmit_power_level_dbm(&self) -> i8 {
        self.max_transmit_power_level_dbm
    }

    pub fn operating_class(&self) -> Option<OperatingInfo> {
        self.operating_info
    }
}

impl Display for SubbandInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subband Info:\r\n\tFirst Channel Number: {}\r\n\tNumber of Channels: {}\r\n\tMaximum Transmit Power Level: {} dBm", self.first_channel_number, self.number_of_channels, self.max_transmit_power_level_dbm)
    }
}

#[derive(Debug, Clone)]
pub struct Country {
    bytes: Vec<u8>,
}

impl Country {
    pub const NAME: &'static str = "Country";
    pub const ID: u8 = 7;
    pub const MIN_LENGTH: usize = 6;

    pub fn new(bytes: Vec<u8>) -> Result<Country, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(Country { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::MIN_LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    // Country String

    pub fn country_string(&self) -> &str {
        str::from_utf8(&self.bytes[0..=2]).unwrap()
    }

    pub fn country_code(&self) -> &str {
        str::from_utf8(&self.bytes[0..=1]).unwrap()
    }

    pub fn environment(&self) -> Option<Environment> {
        match self.bytes[2] as char {
            ' ' => Some(Environment::Any),
            'O' => Some(Environment::Outdoor),
            'I' => Some(Environment::Indoor),
            _ => None,
        }
    }

    // Triplets

    pub fn subband_info(&self) -> Vec<SubbandInfo> {
        let mut subbands = Vec::new();
        let mut last_operating_info = None;

        for triplet in self.bytes[3..].chunks_exact(3) {
            if let Ok(channel_number) = ChannelNumber::try_from(triplet[0]) {
                subbands.push(SubbandInfo {
                    first_channel_number: channel_number,
                    number_of_channels: triplet[1],
                    max_transmit_power_level_dbm: triplet[2] as i8,
                    operating_info: last_operating_info,
                });
            } else if triplet[0] > 200 {
                last_operating_info = Some(OperatingInfo {
                    operating_extension_id: triplet[0],
                    operating_class: triplet[1],
                    coverage_class: triplet[2],
                });
            }
        }

        subbands
    }
}

impl InformationElement for Country {
    fn name(&self) -> &'static str {
        Country::NAME
    }

    fn id(&self) -> u8 {
        Country::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field {
                title: "Country Code".to_string(),
                value: self.country_code().to_string(),
                subfields: None,
            },
            Field {
                title: "Environment".to_string(),
                value: self.environment().unwrap_or(Environment::Any).to_string(),
                subfields: None,
            },
        ]
    }
}

// impl Display for Country {
// fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// write!(
// f,
// "{}: {} {}\r\n{}",
// Self::NAME,
// self.country_code(),
// self.environment().unwrap_or(Environment::Any),
// self.subband_info()
// .iter()
// .map(|subband_info| subband_info.to_string())
// .collect::<Vec<String>>()
// .join("\r\n")
// )
// }
// }
