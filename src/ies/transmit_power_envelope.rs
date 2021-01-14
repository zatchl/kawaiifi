use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitInterpretation {
    EIRP,
    Unknown(u8),
}

impl UnitInterpretation {
    pub fn value(&self) -> u8 {
        match self {
            UnitInterpretation::EIRP => 0,
            UnitInterpretation::Unknown(value) => *value,
        }
    }
}

impl Display for UnitInterpretation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitInterpretation::EIRP => write!(f, "EIRP"),
            UnitInterpretation::Unknown(value) => write!(f, "Unknown ({})", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransmitPowerEnvelope {
    bits: BitVec<LocalBits, u8>,
}

impl TransmitPowerEnvelope {
    pub const NAME: &'static str = "Transmit Power Envelope";
    pub const ID: u8 = 195;
    pub const MIN_LENGTH: usize = 2;

    pub fn new(bytes: Vec<u8>) -> Result<TransmitPowerEnvelope, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(TransmitPowerEnvelope {
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

    pub fn local_maximum_transmit_power_count(&self) -> u8 {
        self.bits[0..=2].load()
    }

    pub fn local_maximum_transmit_power_unit_interpretation(&self) -> UnitInterpretation {
        match self.bits[3..=5].load() {
            0 => UnitInterpretation::EIRP,
            value => UnitInterpretation::Unknown(value),
        }
    }

    pub fn local_maximum_transmit_power_twenty_mhz_dbm(&self) -> i8 {
        self.bits.as_raw_slice()[1] as i8
    }

    pub fn local_maximum_transmit_power_forty_mhz_dbm(&self) -> Option<i8> {
        self.bits.as_raw_slice().get(2).map(|byte| *byte as i8)
    }

    pub fn local_maximum_transmit_power_eighty_mhz_dbm(&self) -> Option<i8> {
        self.bits.as_raw_slice().get(3).map(|byte| *byte as i8)
    }

    pub fn local_maximum_transmit_power_one_sixty_mhz_dbm(&self) -> Option<i8> {
        self.bits.as_raw_slice().get(4).map(|byte| *byte as i8)
    }
}

impl InformationElement for TransmitPowerEnvelope {
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
        let mut information_fields = vec![
            Field::new(
                "Transmit Power Information",
                format!("{:#04x}", self.bits.as_raw_slice()[0]),
                Some(vec![
                    Field::new(
                        "Local Maximum Transmit Power Count",
                        self.local_maximum_transmit_power_count(),
                        None,
                    ),
                    Field::new(
                        "Local Maximum Transmit Power Unit Interpretation",
                        self.local_maximum_transmit_power_unit_interpretation(),
                        None,
                    ),
                ]),
            ),
            Field::new(
                "Local Maximum Transmit Power For 20 MHz",
                format!("{} dBm", self.local_maximum_transmit_power_twenty_mhz_dbm()),
                None,
            ),
        ];

        if let Some(max_tx_forty_mhz) = self.local_maximum_transmit_power_forty_mhz_dbm() {
            information_fields.push(Field::new(
                "Local Maximum Transmit Power For 40 MHz",
                format!("{} dBm", max_tx_forty_mhz),
                None,
            ))
        }

        if let Some(max_tx_eighty_mhz) = self.local_maximum_transmit_power_eighty_mhz_dbm() {
            information_fields.push(Field::new(
                "Local Maximum Transmit Power For 80 MHz",
                format!("{} dBm", max_tx_eighty_mhz),
                None,
            ))
        }

        if let Some(max_tx_one_sixty_mhz) = self.local_maximum_transmit_power_one_sixty_mhz_dbm() {
            information_fields.push(Field::new(
                "Local Maximum Transmit Power For 160/80+80 MHz",
                format!("{} dBm", max_tx_one_sixty_mhz),
                None,
            ))
        }

        information_fields
    }
}
