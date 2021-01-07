use crate::Field;
use bitvec::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CapabilityInfo {
    bits: BitArray<LocalBits, [u8; 2]>,
}

impl CapabilityInfo {
    pub const NAME: &'static str = "Capability Information";
    pub const LENGTH: usize = 2;

    pub fn new(capability_info: [u8; 2]) -> CapabilityInfo {
        CapabilityInfo {
            bits: BitArray::new(capability_info),
        }
    }

    pub fn ess(&self) -> bool {
        self.bits[0]
    }

    pub fn ibss(&self) -> bool {
        self.bits[1]
    }

    pub fn cf_pollable(&self) -> bool {
        self.bits[2]
    }

    pub fn cf_poll_request(&self) -> bool {
        self.bits[3]
    }

    pub fn privacy(&self) -> bool {
        self.bits[4]
    }

    pub fn short_preamble(&self) -> bool {
        self.bits[5]
    }

    pub fn spectrum_management(&self) -> bool {
        self.bits[8]
    }

    pub fn qos(&self) -> bool {
        self.bits[9]
    }

    pub fn short_slot_time(&self) -> bool {
        self.bits[10]
    }

    pub fn apsd(&self) -> bool {
        self.bits[11]
    }

    pub fn radio_measurement(&self) -> bool {
        self.bits[12]
    }

    pub fn delayed_block_ack(&self) -> bool {
        self.bits[14]
    }

    pub fn immediate_block_ack(&self) -> bool {
        self.bits[15]
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field {
                title: "ESS".to_string(),
                value: self.ess().to_string(),
                subfields: None,
            },
            Field {
                title: "IBSS".to_string(),
                value: self.ibss().to_string(),
                subfields: None,
            },
            Field {
                title: "CF Pollable".to_string(),
                value: self.cf_pollable().to_string(),
                subfields: None,
            },
            Field {
                title: "CF-Poll Request".to_string(),
                value: self.cf_poll_request().to_string(),
                subfields: None,
            },
            Field {
                title: "Privacy".to_string(),
                value: self.privacy().to_string(),
                subfields: None,
            },
            Field {
                title: "Short Preamble".to_string(),
                value: self.short_preamble().to_string(),
                subfields: None,
            },
            Field {
                title: "Spectrum Management".to_string(),
                value: self.spectrum_management().to_string(),
                subfields: None,
            },
            Field {
                title: "QoS".to_string(),
                value: self.qos().to_string(),
                subfields: None,
            },
            Field {
                title: "Short Slot Time".to_string(),
                value: self.short_slot_time().to_string(),
                subfields: None,
            },
            Field {
                title: "APSD".to_string(),
                value: self.apsd().to_string(),
                subfields: None,
            },
            Field {
                title: "Radio Measurement".to_string(),
                value: self.radio_measurement().to_string(),
                subfields: None,
            },
            Field {
                title: "Delayed Block Ack".to_string(),
                value: self.delayed_block_ack().to_string(),
                subfields: None,
            },
            Field {
                title: "Immediate Block Ack".to_string(),
                value: self.immediate_block_ack().to_string(),
                subfields: None,
            },
        ]
    }
}

impl Display for CapabilityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut capabilities = Vec::new();

        if self.ess() {
            capabilities.push("ESS");
        }

        if self.ibss() {
            capabilities.push("IBSS");
        }

        if self.cf_pollable() {
            capabilities.push("CF Pollable");
        }

        if self.cf_poll_request() {
            capabilities.push("CF-Poll Request");
        }

        if self.privacy() {
            capabilities.push("Privacy");
        }

        if self.short_preamble() {
            capabilities.push("Short Preamble");
        }

        if self.spectrum_management() {
            capabilities.push("Spectrum Management");
        }

        if self.qos() {
            capabilities.push("QoS");
        }

        if self.short_slot_time() {
            capabilities.push("Short Slot Time");
        }

        if self.apsd() {
            capabilities.push("APSD");
        }

        if self.radio_measurement() {
            capabilities.push("Radio Measurement");
        }

        if self.delayed_block_ack() {
            capabilities.push("Delayed Block Ack");
        }

        if self.immediate_block_ack() {
            capabilities.push("Immediate Block Ack");
        }

        write!(f, "Capability Info:\r\n\t{}", capabilities.join("\r\n\t"))
    }
}
