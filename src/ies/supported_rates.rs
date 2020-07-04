use super::{Display, InformationElement};

#[derive(Debug)]
pub struct SupportedRates {
    bytes: Vec<u8>,
}

impl SupportedRates {
    pub const ID: u8 = 1;
    pub const NAME: &'static str = "Supported Rates";

    pub fn new(bytes: Vec<u8>) -> SupportedRates {
        SupportedRates { bytes }
    }

    pub fn basic_rates(&self) -> Vec<f64> {
        self.bytes
            .iter()
            .filter_map(|byte| {
                if byte & 1 == 1 {
                    Some((byte >> 1) as f64 / 2.0)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn all_rates(&self) -> Vec<f64> {
        self.bytes
            .iter()
            .map(|&byte| (byte >> 1) as f64 / 2.0)
            .collect()
    }
}

impl InformationElement for SupportedRates {
    fn name(&self) -> &'static str {
        SupportedRates::NAME
    }

    fn id(&self) -> u8 {
        SupportedRates::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for SupportedRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            SupportedRates::NAME,
            self.bytes
                .iter()
                .map(|&byte| {
                    let rate = (byte >> 1) as f64 / 2.0;
                    if byte & 1 == 1 {
                        format!("{:.1}*", rate)
                    } else {
                        format!("{:.1}", rate)
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
