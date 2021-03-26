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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshConfiguration {
    bits: BitVec<LocalBits, u8>,
}

impl MeshConfiguration {
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
    const NAME: &'static str = "Mesh Configuration";
    const ID: u8 = 113;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::new(
                "Active Path Selection Protocol",
                format!("{:?}", self.active_path_selection_protocol()),
            ),
            Field::new(
                "Active Path Selection Metric",
                format!("{:?}", self.active_path_selection_metric()),
            ),
            Field::new(
                "Congestion Control Mode",
                format!("{:?}", self.congestion_control_mode()),
            ),
            Field::new(
                "Synchronization Method",
                format!("{:?}", self.synchronization_method()),
            ),
            Field::new(
                "Authentication Protocol",
                format!("{:?}", self.authentication_protocol()),
            ),
            Field::with_subfields(
                "Mesh Formation Info",
                format!("{:#04x}", self.bits.as_raw_slice()[5]),
                vec![
                    Field::new("Connected to Mesh Gate", self.connected_to_mesh_gate()),
                    Field::new("Number of Peerings", self.number_of_peerings()),
                    Field::new("Connected to AS", self.connected_to_as()),
                ],
            ),
            Field::with_subfields(
                "Mesh Capability",
                format!("{:#04x}", self.bits.as_raw_slice()[6]),
                vec![
                    Field::new(
                        "Accepting Additional Mesh Peerings",
                        self.accepting_additional_mesh_peerings(),
                    ),
                    Field::new("MCCA Supported", self.mcca_supported()),
                    Field::new("MCCA Enabled", self.mcca_enabled()),
                    Field::new("Forwarding", self.forwarding()),
                    Field::new("MBCA Enabled", self.mbca_enabled()),
                    Field::new("TBTT Adjusting", self.tbtt_adjusting()),
                    Field::new("Mesh Power Save Level", self.mesh_power_save_level()),
                ],
            ),
        ]
    }
}

impl_display_for_ie!(MeshConfiguration);
