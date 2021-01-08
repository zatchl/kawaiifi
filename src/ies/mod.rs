mod antenna;
mod bss_load;
mod country;
mod ds_parameter_set;
mod erp_info;
mod extended_capabilities;
mod ht_capabilities;
mod ht_operation;
mod mesh_id;
mod power_constraint;
mod rsn;
mod ssid;
mod supported_rates;
mod tim;
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
pub use mesh_id::MeshId;
pub use power_constraint::PowerConstraint;
pub use rsn::Rsn;
pub use ssid::Ssid;
pub use supported_rates::SupportedRates;
pub use tim::Tim;
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

#[enum_dispatch(InformationElement)]
#[derive(Debug, Clone)]
pub enum Ie {
    Antenna,
    BssLoad,
    Country,
    DsParameterSet,
    ErpInfo,
    ExtendedCapabilities,
    HtCapabilities,
    HtOperation,
    MeshId,
    PowerConstraint,
    Rsn,
    Ssid,
    SupportedRates,
    Tim,
    VendorSpecific,
    VhtCapabilities,
    VhtOperation,
    Wpa,
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
        let ie_id_ext = {
            if ie_id == 255 {
                match bytes.read_u8() {
                    Ok(ie_ext_id) => Some(ie_ext_id),
                    _ => break,
                }
            } else {
                None
            }
        };

        // Bytes [2..ie_len+2] is the data
        let mut ie_data = vec![0; ie_len as usize];
        // Break out of the loop if reading the bytes fails
        if bytes.read_exact(&mut ie_data).is_err() {
            break;
        }

        // Using the IE's ID, try to create an information element and add it to the vector of IEs
        // If there's an error creating an information element, return the error
        match (ie_id, ie_id_ext) {
            (Antenna::ID, _) => ies.push(Ie::from(Antenna::new(ie_data)?)),
            (BssLoad::ID, _) => ies.push(Ie::from(BssLoad::new(ie_data)?)),
            (Country::ID, _) => ies.push(Ie::from(Country::new(ie_data)?)),
            (DsParameterSet::ID, _) => {
                ies.push(Ie::from(DsParameterSet::new(ie_data.try_into().map_err(
                    |ie_data: Vec<u8>| IeError::InvalidLength {
                        ie_name: DsParameterSet::NAME,
                        expected_length: DsParameterSet::LENGTH,
                        actual_length: ie_data.len(),
                    },
                )?)))
            }
            (ErpInfo::ID, _) => ies.push(Ie::from(ErpInfo::new(ie_data.try_into().map_err(
                |ie_data: Vec<u8>| IeError::InvalidLength {
                    ie_name: ErpInfo::NAME,
                    expected_length: ErpInfo::LENGTH,
                    actual_length: ie_data.len(),
                },
            )?))),
            (ExtendedCapabilities::ID, _) => ies.push(Ie::from(ExtendedCapabilities::new(ie_data))),
            (HeCapabilities::ID, Some(HeCapabilities::ID_EXT)) => {
                ies.push(Ie::from(HeCapabilities::new(ie_data)))
            }
            (HeOperation::ID, Some(HeOperation::ID_EXT)) => {
                ies.push(Ie::from(HeOperation::new(ie_data)?))
            }
            (HtCapabilities::ID, _) => ies.push(Ie::from(HtCapabilities::new(ie_data)?)),
            (HtOperation::ID, _) => ies.push(Ie::from(HtOperation::new(ie_data)?)),
            (MeshId::ID, _) => ies.push(Ie::from(MeshId::new(ie_data))),
            (PowerConstraint::ID, _) => {
                ies.push(Ie::from(PowerConstraint::new(ie_data.try_into().map_err(
                    |ie_data: Vec<u8>| IeError::InvalidLength {
                        ie_name: PowerConstraint::NAME,
                        expected_length: PowerConstraint::LENGTH,
                        actual_length: ie_data.len(),
                    },
                )?)))
            }
            (Rsn::ID, _) => ies.push(Ie::from(Rsn::new(ie_data))),
            (Ssid::ID, _) => ies.push(Ie::from(Ssid::new(ie_data))),
            (SupportedRates::ID, _) => ies.push(Ie::from(SupportedRates::new(ie_data))),
            (Tim::ID, _) => ies.push(Ie::from(Tim::new(ie_data))),
            (VendorSpecific::ID, _) if !ie_data.starts_with(&Wpa::OUI) => {
                ies.push(Ie::from(VendorSpecific::new(ie_data)))
            }
            (VhtCapabilities::ID, _) => ies.push(Ie::from(VhtCapabilities::new(ie_data)?)),
            (VhtOperation::ID, _) => ies.push(Ie::from(VhtOperation::new(ie_data)?)),
            (Wpa::ID, _) if ie_data.starts_with(&Wpa::OUI) => {
                ies.push(Ie::from(Wpa::new(ie_data).unwrap()))
            }
            _ => continue,
        }
    }

    Ok(ies)
}
