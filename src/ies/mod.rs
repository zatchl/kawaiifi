mod antenna;
mod bss_load;
mod country;
mod ds_parameter_set;
mod erp_info;
mod extended_capabilities;
mod ht_capabilities;
mod ht_operation;
mod ibss_parameter_set;
mod measurement_pilot_transmission;
mod mesh_id;
mod overlapping_bss_scan_params;
mod power_constraint;
mod rm_enabled_capabilities;
mod rsn;
mod ssid;
mod supported_rates;
mod tim;
mod transmit_power_envelope;
mod unknown;
mod vendor_specific;
mod vht_capabilities;
mod vht_operation;
mod wpa;

pub use antenna::Antenna;
pub use bss_load::BssLoad;
pub use country::Country;
pub use ds_parameter_set::DsParameterSet;
pub use enum_dispatch::enum_dispatch;
pub use erp_info::ErpInfo;
pub use extended_capabilities::ExtendedCapabilities;
pub use ht_capabilities::HtCapabilities;
pub use ht_operation::HtOperation;
pub use ibss_parameter_set::IbssParameterSet;
pub use measurement_pilot_transmission::MeasurementPilotTransmission;
pub use mesh_id::MeshId;
pub use overlapping_bss_scan_params::OverlappingBssScanParams;
pub use power_constraint::PowerConstraint;
pub use rm_enabled_capabilities::RmEnabledCapabilities;
pub use rsn::Rsn;
pub use ssid::Ssid;
pub use supported_rates::{DataRate, ExtendedSupportedRates, SupportedRates};
pub use tim::Tim;
pub use transmit_power_envelope::TransmitPowerEnvelope;
pub use unknown::Unknown;
pub use vendor_specific::VendorSpecific;
pub use vht_capabilities::VhtCapabilities;
pub use vht_operation::VhtOperation;
pub use wpa::Wpa;

use crate::Field;
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};
use std::{convert::TryInto, fmt::Display};
use thiserror::Error;

#[enum_dispatch]
pub trait InformationElement {
    fn name(&self) -> &'static str;
    fn id(&self) -> u8;
    fn id_ext(&self) -> Option<u8> {
        None
    }
    fn bytes(&self) -> &[u8];
    fn information_fields(&self) -> Vec<Field>;
}

pub const MAX_IE_ID: u8 = 255;

#[enum_dispatch(InformationElement)]
#[derive(Debug, Clone)]
pub enum Ie {
    Antenna,
    BssLoad,
    Country,
    DsParameterSet,
    ErpInfo,
    ExtendedCapabilities,
    ExtendedSupportedRates,
    HtCapabilities,
    HtOperation,
    IbssParameterSet,
    MeasurementPilotTransmission,
    MeshId,
    OverlappingBssScanParams,
    PowerConstraint,
    RmEnabledCapabilities,
    Rsn,
    Ssid,
    SupportedRates,
    Tim,
    TransmitPowerEnvelope,
    Unknown,
    VendorSpecific,
    VhtCapabilities,
    VhtOperation,
    Wpa,
}

impl Ie {
    fn new(ie_data: Vec<u8>, ie_id: u8, ie_id_ext: Option<u8>) -> Result<Ie, IeError> {
        Ok(match ie_id {
            Antenna::ID => Ie::from(Antenna::new(ie_data)?),
            BssLoad::ID => Ie::from(BssLoad::new(ie_data)?),
            Country::ID => Ie::from(Country::new(ie_data)?),
            DsParameterSet::ID => Ie::from(DsParameterSet::new(ie_data.try_into().map_err(
                |ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: DsParameterSet::NAME,
                    expected_length: DsParameterSet::LENGTH,
                    actual_length: ie_data.len(),
                },
            )?)),
            ErpInfo::ID => Ie::from(ErpInfo::new(ie_data.try_into().map_err(
                |ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: ErpInfo::NAME,
                    expected_length: ErpInfo::LENGTH,
                    actual_length: ie_data.len(),
                },
            )?)),
            ExtendedCapabilities::ID => Ie::from(ExtendedCapabilities::new(ie_data)),
            ExtendedSupportedRates::ID => Ie::from(ExtendedSupportedRates::new(ie_data)),
            HtCapabilities::ID => Ie::from(HtCapabilities::new(ie_data)?),
            HtOperation::ID => Ie::from(HtOperation::new(ie_data)?),
            IbssParameterSet::ID => Ie::from(IbssParameterSet::new(ie_data.try_into().map_err(
                |ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: IbssParameterSet::NAME,
                    expected_length: IbssParameterSet::LENGTH,
                    actual_length: ie_data.len(),
                },
            )?)),
            MeasurementPilotTransmission::ID => {
                Ie::from(MeasurementPilotTransmission::new(ie_data)?)
            }
            MeshId::ID => Ie::from(MeshId::new(ie_data)),
            OverlappingBssScanParams::ID => {
                Ie::from(OverlappingBssScanParams::new(ie_data.try_into().map_err(
                    |ie_data: Vec<u8>| IeError::InvalidLength {
                        ie_name: OverlappingBssScanParams::NAME,
                        expected_length: OverlappingBssScanParams::LENGTH,
                        actual_length: ie_data.len(),
                    },
                )?))
            }
            PowerConstraint::ID => Ie::from(PowerConstraint::new(ie_data.try_into().map_err(
                |ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: PowerConstraint::NAME,
                    expected_length: PowerConstraint::LENGTH,
                    actual_length: ie_data.len(),
                },
            )?)),
            RmEnabledCapabilities::ID => Ie::from(RmEnabledCapabilities::new(ie_data)?),
            Rsn::ID => Ie::from(Rsn::new(ie_data)),
            Ssid::ID => Ie::from(Ssid::new(ie_data)),
            SupportedRates::ID => Ie::from(SupportedRates::new(ie_data)),
            Tim::ID => Ie::from(Tim::new(ie_data)),
            TransmitPowerEnvelope::ID => Ie::from(TransmitPowerEnvelope::new(ie_data)?),
            VendorSpecific::ID => {
                if ie_data.starts_with(&Wpa::OUI) {
                    Ie::from(Wpa::new(ie_data))
                } else {
                    Ie::from(VendorSpecific::new(ie_data))
                }
            }
            VhtCapabilities::ID => Ie::from(VhtCapabilities::new(ie_data)?),
            VhtOperation::ID => Ie::from(VhtOperation::new(ie_data)?),
            MAX_IE_ID => match ie_id_ext {
                Some(HeCapabilities::ID_EXT) => Ie::from(HeCapabilities::new(ie_data)),
                Some(HeOperation::ID_EXT) => Ie::from(HeOperation::new(ie_data)),
                _ => Ie::from(Unknown::new(ie_data, ie_id, ie_id_ext)),
            },
            _ => Ie::from(Unknown::new(ie_data, ie_id, ie_id_ext)),
        })
    }
}

impl Display for Ie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for field in self.information_fields() {
            write!(f, "{}\r\n", field)?;
        }

        Ok(())
    }
}

impl PartialEq for Ie {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.id_ext() == other.id_ext() && self.bytes() == other.bytes()
    }
}

#[derive(Error, Debug)]
pub enum IeError {
    #[error("Invalid IE length")]
    InvalidLength {
        ie_name: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<Ie>, IeError> {
    let mut bytes = Cursor::new(bytes);

    let mut ies = Vec::new();

    loop {
        // The first byte of the IE is the ID
        // Break out of the loop if reading the byte fails
        let ie_id = match bytes.read_u8() {
            Ok(ie_id) => ie_id,
            _ => break,
        };

        // The second byte of the IE is the number of bytes of data
        // Break out of the loop if reading the byte fails
        let ie_len = match bytes.read_u8() {
            Ok(ie_len) => ie_len,
            _ => break,
        };

        // If the element ID is 255 then the next byte is the element ID extension
        let ie_id_ext = match ie_id {
            MAX_IE_ID => bytes.read_u8().ok(),
            _ => None,
        };

        // Bytes [2..ie_len+2] or [3..ie_len+3] is the data
        let ie_data = {
            let mut ie_data = vec![0; ie_len as usize];
            match bytes.read_exact(&mut ie_data) {
                Ok(_) => ie_data,
                _ => break,
            }
        };

        // Using the IE's ID, try to create an information element and add it to the vector of IEs
        // If there's an error creating an information element, return the error
        ies.push(Ie::new(ie_data, ie_id, ie_id_ext)?);
    }

    Ok(ies)
}
