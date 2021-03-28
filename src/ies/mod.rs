pub trait InformationElement {
    const NAME: &'static str;
    const ID: u8;
    const ID_EXT: Option<u8> = None;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn id(&self) -> u8 {
        Self::ID
    }

    fn id_ext(&self) -> Option<u8> {
        Self::ID_EXT
    }

    fn bytes(&self) -> &[u8];

    fn information_fields(&self) -> Vec<Field>;
}

macro_rules! impl_display_for_ie {
    ($ie_name:ident) => {
        impl std::fmt::Display for $ie_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for field in self.information_fields() {
                    std::write!(f, "{}\r\n", field)?;
                }

                Ok(())
            }
        }
    };
}

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
mod mesh_configuration;
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
pub use erp_info::ErpInfo;
pub use extended_capabilities::ExtendedCapabilities;
pub use ht_capabilities::HtCapabilities;
pub use ht_operation::HtOperation;
pub use ibss_parameter_set::IbssParameterSet;
pub use measurement_pilot_transmission::MeasurementPilotTransmission;
pub use mesh_configuration::MeshConfiguration;
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
use std::fmt::Display;
use std::io::{Cursor, Read};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ie {
    Antenna(Antenna),
    BssLoad(BssLoad),
    Country(Country),
    DsParameterSet(DsParameterSet),
    ErpInfo(ErpInfo),
    ExtendedCapabilities(ExtendedCapabilities),
    ExtendedSupportedRates(ExtendedSupportedRates),
    HeCapabilities(HeCapabilities),
    HeOperation(HeOperation),
    HtCapabilities(HtCapabilities),
    HtOperation(HtOperation),
    IbssParameterSet(IbssParameterSet),
    MeasurementPilotTransmission(MeasurementPilotTransmission),
    MeshConfiguration(MeshConfiguration),
    MeshId(MeshId),
    OverlappingBssScanParams(OverlappingBssScanParams),
    PowerConstraint(PowerConstraint),
    RmEnabledCapabilities(RmEnabledCapabilities),
    Rsn(Rsn),
    Ssid(Ssid),
    SupportedRates(SupportedRates),
    Tim(Tim),
    TransmitPowerEnvelope(TransmitPowerEnvelope),
    Unknown(Unknown),
    VendorSpecific(VendorSpecific),
    VhtCapabilities(VhtCapabilities),
    VhtOperation(VhtOperation),
    Wpa(Wpa),
}

macro_rules! match_inner_ie {
    ($ie:ident, $inner_ie:ident, $output:expr) => {
        match $ie {
            Ie::Antenna($inner_ie) => $output,
            Ie::BssLoad($inner_ie) => $output,
            Ie::Country($inner_ie) => $output,
            Ie::DsParameterSet($inner_ie) => $output,
            Ie::ErpInfo($inner_ie) => $output,
            Ie::ExtendedCapabilities($inner_ie) => $output,
            Ie::ExtendedSupportedRates($inner_ie) => $output,
            Ie::HeCapabilities($inner_ie) => $output,
            Ie::HeOperation($inner_ie) => $output,
            Ie::HtCapabilities($inner_ie) => $output,
            Ie::HtOperation($inner_ie) => $output,
            Ie::IbssParameterSet($inner_ie) => $output,
            Ie::MeasurementPilotTransmission($inner_ie) => $output,
            Ie::MeshConfiguration($inner_ie) => $output,
            Ie::MeshId($inner_ie) => $output,
            Ie::OverlappingBssScanParams($inner_ie) => $output,
            Ie::PowerConstraint($inner_ie) => $output,
            Ie::RmEnabledCapabilities($inner_ie) => $output,
            Ie::Rsn($inner_ie) => $output,
            Ie::Ssid($inner_ie) => $output,
            Ie::SupportedRates($inner_ie) => $output,
            Ie::Tim($inner_ie) => $output,
            Ie::TransmitPowerEnvelope($inner_ie) => $output,
            Ie::Unknown($inner_ie) => $output,
            Ie::VendorSpecific($inner_ie) => $output,
            Ie::VhtCapabilities($inner_ie) => $output,
            Ie::VhtOperation($inner_ie) => $output,
            Ie::Wpa($inner_ie) => $output,
        }
    };
}

impl Ie {
    fn new(ie_data: Vec<u8>, ie_id: u8, ie_id_ext: Option<u8>) -> Result<Ie, IeError> {
        Ok(match ie_id {
            Antenna::ID => Ie::Antenna(Antenna::new(ie_data)?),
            BssLoad::ID => Ie::BssLoad(BssLoad::new(ie_data)?),
            Country::ID => Ie::Country(Country::new(ie_data)?),
            DsParameterSet::ID => Ie::DsParameterSet(DsParameterSet::new(ie_data)?),
            ErpInfo::ID => Ie::ErpInfo(ErpInfo::new(ie_data)?),
            ExtendedCapabilities::ID => {
                Ie::ExtendedCapabilities(ExtendedCapabilities::new(ie_data))
            }
            ExtendedSupportedRates::ID => {
                Ie::ExtendedSupportedRates(ExtendedSupportedRates::new(ie_data))
            }
            HtCapabilities::ID => Ie::HtCapabilities(HtCapabilities::new(ie_data)?),
            HtOperation::ID => Ie::HtOperation(HtOperation::new(ie_data)?),
            IbssParameterSet::ID => Ie::IbssParameterSet(IbssParameterSet::new(ie_data)?),
            MeasurementPilotTransmission::ID => {
                Ie::MeasurementPilotTransmission(MeasurementPilotTransmission::new(ie_data)?)
            }
            MeshConfiguration::ID => Ie::MeshConfiguration(MeshConfiguration::new(ie_data)?),
            MeshId::ID => Ie::MeshId(MeshId::new(ie_data)),
            OverlappingBssScanParams::ID => {
                Ie::OverlappingBssScanParams(OverlappingBssScanParams::new(ie_data)?)
            }
            PowerConstraint::ID => Ie::PowerConstraint(PowerConstraint::new(ie_data)?),
            RmEnabledCapabilities::ID => {
                Ie::RmEnabledCapabilities(RmEnabledCapabilities::new(ie_data)?)
            }
            Rsn::ID => Ie::Rsn(Rsn::new(ie_data)),
            Ssid::ID => Ie::Ssid(Ssid::new(ie_data)),
            SupportedRates::ID => Ie::SupportedRates(SupportedRates::new(ie_data)),
            Tim::ID => Ie::Tim(Tim::new(ie_data)),
            TransmitPowerEnvelope::ID => {
                Ie::TransmitPowerEnvelope(TransmitPowerEnvelope::new(ie_data)?)
            }
            VendorSpecific::ID => {
                if ie_data.starts_with(&Wpa::OUI) {
                    Ie::Wpa(Wpa::new(ie_data))
                } else {
                    Ie::VendorSpecific(VendorSpecific::new(ie_data))
                }
            }
            VhtCapabilities::ID => Ie::VhtCapabilities(VhtCapabilities::new(ie_data)?),
            VhtOperation::ID => Ie::VhtOperation(VhtOperation::new(ie_data)?),
            u8::MAX => match ie_id_ext {
                HeCapabilities::ID_EXT => Ie::HeCapabilities(HeCapabilities::new(ie_data)),
                HeOperation::ID_EXT => Ie::HeOperation(HeOperation::new(ie_data)),
                _ => Ie::Unknown(Unknown::new(ie_data, ie_id, ie_id_ext)),
            },
            _ => Ie::Unknown(Unknown::new(ie_data, ie_id, ie_id_ext)),
        })
    }

    pub fn name(&self) -> &'static str {
        match_inner_ie!(self, ie, ie.name())
    }

    pub fn id(&self) -> u8 {
        match_inner_ie!(self, ie, ie.id())
    }

    pub fn id_ext(&self) -> Option<u8> {
        match_inner_ie!(self, ie, ie.id_ext())
    }

    pub fn bytes(&self) -> &[u8] {
        match_inner_ie!(self, ie, ie.bytes())
    }

    pub fn information_fields(&self) -> Vec<Field> {
        match_inner_ie!(self, ie, ie.information_fields())
    }
}

impl Display for Ie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match_inner_ie!(self, ie, ie.fmt(f))
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
            u8::MAX => bytes.read_u8().ok(),
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
