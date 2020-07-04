mod country_info;
mod ds_parameter_set;
mod erp_info;
mod extended_capabilities;
mod ht_capabilities;
mod ht_operation;
mod power_constraint;
mod rsn;
mod ssid;
mod supported_rates;
mod tim;
mod vendor_specific;
mod vht_capabilities;
mod vht_operation;
mod wpa;

pub use country_info::CountryInfo;
pub use ds_parameter_set::DsParameterSet;
pub use enum_dispatch::enum_dispatch;
pub use erp_info::ErpInfo;
pub use extended_capabilities::ExtendedCapabilities;
pub use ht_capabilities::HtCapabilities;
pub use ht_operation::HtOperation;
pub use power_constraint::PowerConstraint;
pub use rsn::Rsn;
pub use ssid::Ssid;
pub use std::fmt::Display;
pub use supported_rates::SupportedRates;
pub use tim::Tim;
pub use vendor_specific::VendorSpecific;
pub use vht_capabilities::VhtCapabilities;
pub use vht_operation::VhtOperation;
pub use wpa::Wpa;

use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};
use thiserror::Error;

#[enum_dispatch]
pub trait InformationElement {
    fn name(&self) -> &'static str;
    fn id(&self) -> u8;
    fn bytes(&self) -> &[u8];
}

#[enum_dispatch(InformationElement, Display)]
#[derive(Debug)]
pub enum Ie {
    CountryInfo,
    DsParameterSet,
    ErpInfo,
    ExtendedCapabilities,
    HtCapabilities,
    HtOperation,
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

        // Bytes [2..ie_len+2] is the data
        let mut ie_data = vec![0; ie_len as usize];
        // Break out of the loop if reading the bytes fails
        if bytes.read_exact(&mut ie_data).is_err() {
            break;
        }

        // Using the IE's ID, try to create an information element and add it to the vector of IEs
        // If there's an error creating an information element, return the error
        match ie_id {
            CountryInfo::ID => ies.push(Ie::from(CountryInfo::new(ie_data))),
            DsParameterSet::ID => ies.push(Ie::from(DsParameterSet::new(ie_data)?)),
            ErpInfo::ID => ies.push(Ie::from(ErpInfo::new(ie_data)?)),
            ExtendedCapabilities::ID => ies.push(Ie::from(ExtendedCapabilities::new(ie_data))),
            HtCapabilities::ID => ies.push(Ie::from(HtCapabilities::new(ie_data)?)),
            HtOperation::ID => ies.push(Ie::from(HtOperation::new(ie_data)?)),
            PowerConstraint::ID => ies.push(Ie::from(PowerConstraint::new(ie_data)?)),
            Rsn::ID => ies.push(Ie::from(Rsn::new(ie_data))),
            Ssid::ID => ies.push(Ie::from(Ssid::new(ie_data))),
            SupportedRates::ID => ies.push(Ie::from(SupportedRates::new(ie_data))),
            Tim::ID => ies.push(Ie::from(Tim::new(ie_data))),
            VendorSpecific::ID if !ie_data.starts_with(&Wpa::OUI) => {
                ies.push(Ie::from(VendorSpecific::new(ie_data)))
            }
            VhtCapabilities::ID => ies.push(Ie::from(VhtCapabilities::new(ie_data)?)),
            VhtOperation::ID => ies.push(Ie::from(VhtOperation::new(ie_data)?)),
            Wpa::ID if ie_data.starts_with(&Wpa::OUI) => {
                ies.push(Ie::from(Wpa::new(ie_data).unwrap()))
            }
            _ => continue,
        }
    }

    Ok(ies)
}
