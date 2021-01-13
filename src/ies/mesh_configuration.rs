use super::{Field, IeError, InformationElement};
use bitvec::prelude::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ActivePathSelectionProtocol {
    Hybrid = 1,
    VendorSpecific = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ActivePathSelectionMetric {
    AirtimeLink = 1,
    VendorSpecific = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum CongestionControlMode {
    NotActivated = 0,
    CongestionControlSignaling = 1,
    VendorSpecific = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SynchronizationMethod {
    NeighborOffset = 1,
    VendorSpecific = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AuthenticationProtocol {
    NotRequired = 0,
    Sae = 1,
    Ieee8021X = 2,
    VendorSpecific = 255,
}

#[derive(Debug, Clone)]
pub struct MeshConfiguration {
    bits: BitVec<LocalBits, u8>,
}

impl MeshConfiguration {
    pub const NAME: &'static str = "Mesh Configuration";
    pub const ID: u8 = 113;
    pub const MIN_LENGTH: usize = 7;

    pub fn new(bytes: Vec<u8>) -> Result<MeshConfiguration, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(MeshConfiguration {
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

    pub fn active_path_selection_protocol(&self) -> ActivePathSelectionProtocol {
        ActivePathSelectionProtocol::try_from(self.bits.as_raw_slice()[0])
            .unwrap_or(ActivePathSelectionProtocol::Hybrid)
    }

    pub fn active_path_selection_metric(&self) -> ActivePathSelectionMetric {
        ActivePathSelectionMetric::try_from(self.bits.as_raw_slice()[1])
            .unwrap_or(ActivePathSelectionMetric::AirtimeLink)
    }

    pub fn congestion_control_mode(&self) -> CongestionControlMode {
        CongestionControlMode::try_from(self.bits.as_raw_slice()[2])
            .unwrap_or(CongestionControlMode::NotActivated)
    }

    pub fn synchronization_method(&self) -> SynchronizationMethod {
        SynchronizationMethod::try_from(self.bits.as_raw_slice()[3])
            .unwrap_or(SynchronizationMethod::NeighborOffset)
    }

    pub fn authentication_protocol(&self) -> AuthenticationProtocol {
        AuthenticationProtocol::try_from(self.bits.as_raw_slice()[4])
            .unwrap_or(AuthenticationProtocol::NotRequired)
    }

    // Mesh Formation Info

    pub fn connected_to_mesh_gate(&self) -> bool {
        self.bits[40]
    }

    pub fn number_of_peerings(&self) -> u8 {
        self.bits[41..=46].load()
    }

    pub fn connected_to_as(&self) -> bool {
        self.bits[47]
    }

    // Mesh Capability

    pub fn accepting_additional_mesh_peerings(&self) -> bool {
        self.bits[48]
    }

    pub fn mcca_supported(&self) -> bool {
        self.bits[49]
    }

    pub fn mcca_enabled(&self) -> bool {
        self.bits[50]
    }

    pub fn forwarding(&self) -> bool {
        self.bits[51]
    }

    pub fn mbca_enabled(&self) -> bool {
        self.bits[52]
    }

    pub fn tbtt_adjusting(&self) -> bool {
        self.bits[53]
    }

    pub fn mesh_power_save_level(&self) -> bool {
        self.bits[54]
    }
}

impl InformationElement for MeshConfiguration {
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
                title: "Active Path Selection Protocol".to_string(),
                value: format!("{:?}", self.active_path_selection_protocol()),
                subfields: None,
            },
            Field {
                title: "Active Path Selection Metric".to_string(),
                value: format!("{:?}", self.active_path_selection_metric()),
                subfields: None,
            },
            Field {
                title: "Congestion Control Mode".to_string(),
                value: format!("{:?}", self.congestion_control_mode()),
                subfields: None,
            },
            Field {
                title: "Synchronization Method".to_string(),
                value: format!("{:?}", self.synchronization_method()),
                subfields: None,
            },
            Field {
                title: "Authentication Protocol".to_string(),
                value: format!("{:?}", self.authentication_protocol()),
                subfields: None,
            },
            Field {
                title: "Mesh Formation Info".to_string(),
                value: format!("{:#04x}", self.bits.as_raw_slice()[5]),
                subfields: Some(vec![
                    Field {
                        title: "Connected to Mesh Gate".to_string(),
                        value: self.connected_to_mesh_gate().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "Number of Peerings".to_string(),
                        value: self.number_of_peerings().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "Connected to AS".to_string(),
                        value: self.connected_to_as().to_string(),
                        subfields: None,
                    },
                ]),
            },
            Field {
                title: "Mesh Capability".to_string(),
                value: format!("{:#04x}", self.bits.as_raw_slice()[6]),
                subfields: Some(vec![
                    Field {
                        title: "Accepting Additional Mesh Peerings".to_string(),
                        value: self.accepting_additional_mesh_peerings().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "MCCA Supported".to_string(),
                        value: self.mcca_supported().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "MCCA Enabled".to_string(),
                        value: self.mcca_enabled().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "Forwarding".to_string(),
                        value: self.forwarding().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "MBCA Enabled".to_string(),
                        value: self.mbca_enabled().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "TBTT Adjusting".to_string(),
                        value: self.tbtt_adjusting().to_string(),
                        subfields: None,
                    },
                    Field {
                        title: "Mesh Power Save Level".to_string(),
                        value: self.mesh_power_save_level().to_string(),
                        subfields: None,
                    },
                ]),
            },
        ]
    }
}
