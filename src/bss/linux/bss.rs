use super::{BssStatus, Nl80211Bss, ScanWidth};
use crate::{
    bss::CapabilityInfo,
    ies::{self, Ie},
    Channel, SecurityProtocols, WifiProtocols,
};
use macaddr::MacAddr6;
use neli::{attr::Attribute, genl::Nlattr, types::Buffer};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Display,
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct Bss {
    bssid: MacAddr6,
    frequency_mhz: u32,
    signal_dbm: i32,
    beacon_interval_tu: u16,
    capability_info: CapabilityInfo,
    status: BssStatus,
    ies: Vec<Ie>,
    is_from_probe_response: bool,
    parent_bssid: Option<MacAddr6>,
    parent_tsf: Option<u64>,
    tsf: Option<u64>,
    beacon_tsf: Option<u64>,
    frequency_offset_khz: Option<u32>,
    signal_percent: Option<u8>,
    beacon_ies: Option<Vec<Ie>>,
    scan_width: Option<ScanWidth>,
    last_seen_boottime: Option<u64>,
    seen_ms_ago: Option<u32>,
}

impl Bss {
    pub fn bssid(&self) -> MacAddr6 {
        self.bssid
    }

    pub fn frequency_mhz(&self) -> u32 {
        self.frequency_mhz
    }

    pub fn signal_dbm(&self) -> i32 {
        self.signal_dbm
    }

    pub fn beacon_interval_tu(&self) -> u16 {
        self.beacon_interval_tu
    }

    pub fn beacon_interval_ms(&self) -> f64 {
        self.beacon_interval_tu as f64 * 1.024
    }

    pub fn capability_info(&self) -> CapabilityInfo {
        self.capability_info.clone()
    }

    pub fn status(&self) -> BssStatus {
        self.status
    }

    pub fn ies(&self) -> &[Ie] {
        &self.ies
    }

    pub fn is_from_probe_response(&self) -> bool {
        self.is_from_probe_response
    }

    pub fn parent_bssid(&self) -> Option<MacAddr6> {
        self.parent_bssid
    }

    pub fn parent_tsf(&self) -> Option<u64> {
        self.parent_tsf
    }

    pub fn tsf(&self) -> Option<u64> {
        self.tsf
    }

    pub fn beacon_tsf(&self) -> Option<u64> {
        self.beacon_tsf
    }

    pub fn frequency_offset_khz(&self) -> Option<u32> {
        self.frequency_offset_khz
    }

    pub fn signal_percent(&self) -> Option<u8> {
        self.signal_percent
    }

    pub fn beacon_ies(&self) -> Option<&[Ie]> {
        self.beacon_ies.as_deref()
    }

    pub fn scan_width(&self) -> Option<ScanWidth> {
        self.scan_width
    }

    pub fn last_seen_boottime(&self) -> Option<u64> {
        self.last_seen_boottime
    }

    pub fn seen_ms_ago(&self) -> Option<u32> {
        self.seen_ms_ago
    }

    pub fn ssid(&self) -> Option<&str> {
        self.ies.iter().find_map(|ie| {
            if let Ie::Ssid(ssid) = ie {
                ssid.as_str().ok()
            } else {
                None
            }
        })
    }

    pub fn channel(&self) -> Channel {
        Channel::from(&self.ies)
    }

    pub fn security_protocols(&self) -> SecurityProtocols {
        SecurityProtocols::from(self.ies.as_slice())
    }

    pub fn wifi_protocols(&self) -> WifiProtocols {
        WifiProtocols::from(self.ies.as_slice())
    }

    pub fn max_rate_mbps(&self) -> f64 {
        let mut max_rate = 0.0;

        for ie in self.ies.iter() {
            match ie {
                Ie::SupportedRates(supported_rates) => {
                    let data_rates = supported_rates.rates();
                    if max_rate < data_rates.iter().max().unwrap().value() {
                        max_rate = data_rates.iter().max().unwrap().value();
                    }
                }
                Ie::HtOperation(_) => continue,
                Ie::VhtOperation(_) => continue,
                _ => continue,
            }
        }
        0.0
    }
}

impl Hash for Bss {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bssid.hash(state);
    }
}

impl PartialEq for Bss {
    fn eq(&self, other: &Self) -> bool {
        self.bssid == other.bssid
    }
}

impl Eq for Bss {}

impl TryFrom<&[Nlattr<Nl80211Bss, Buffer>]> for Bss {
    type Error = ();

    fn try_from(bss_attrs: &[Nlattr<Nl80211Bss, Buffer>]) -> Result<Self, Self::Error> {
        let bss_attrs: HashMap<_, _> = bss_attrs.iter().map(|attr| (attr.nla_type, attr)).collect();

        Ok(Bss {
            bssid: bss_attrs
                .get(&Nl80211Bss::Bssid)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .map(|bssid_bytes: [u8; 6]| MacAddr6::from(bssid_bytes))
                .ok_or(())?,
            frequency_mhz: bss_attrs
                .get(&Nl80211Bss::Frequency)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            signal_dbm: bss_attrs
                .get(&Nl80211Bss::SignalMbm)
                .and_then(|attr| attr.get_payload_as::<i32>().ok())
                .ok_or(())?
                / 100,
            beacon_interval_tu: bss_attrs
                .get(&Nl80211Bss::BeaconInterval)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            capability_info: bss_attrs
                .get(&Nl80211Bss::Capability)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .map(|payload| CapabilityInfo::new(payload))
                .ok_or(())?,
            status: bss_attrs
                .get(&Nl80211Bss::Status)
                .and_then(|attr| attr.get_payload_as::<u32>().ok())
                .and_then(|payload| BssStatus::try_from(payload).ok())
                .unwrap_or(BssStatus::NotAssociated),
            ies: bss_attrs
                .get(&Nl80211Bss::InformationElements)
                .and_then(|attr| ies::from_bytes(attr.payload().as_ref()).ok())
                .ok_or(())?,
            is_from_probe_response: bss_attrs.contains_key(&Nl80211Bss::PrespData),
            parent_bssid: bss_attrs
                .get(&Nl80211Bss::ParentBssid)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .map(|parent_bssid_bytes: [u8; 6]| MacAddr6::from(parent_bssid_bytes)),
            parent_tsf: bss_attrs
                .get(&Nl80211Bss::ParentTsf)
                .and_then(|attr| attr.get_payload_as().ok()),
            tsf: bss_attrs
                .get(&Nl80211Bss::Tsf)
                .and_then(|attr| attr.get_payload_as().ok()),
            beacon_tsf: bss_attrs
                .get(&Nl80211Bss::BeaconTsf)
                .and_then(|attr| attr.get_payload_as().ok()),
            frequency_offset_khz: bss_attrs
                .get(&Nl80211Bss::FrequencyOffset)
                .and_then(|attr| attr.get_payload_as().ok()),
            signal_percent: bss_attrs
                .get(&Nl80211Bss::SignalUnspec)
                .and_then(|attr| attr.get_payload_as().ok()),
            beacon_ies: bss_attrs
                .get(&Nl80211Bss::BeaconIes)
                .and_then(|attr| ies::from_bytes(attr.payload().as_ref()).ok()),
            scan_width: bss_attrs.get(&Nl80211Bss::ChanWidth).and_then(|attr| {
                ScanWidth::try_from(attr.get_payload_as::<u32>().unwrap_or_default()).ok()
            }),
            last_seen_boottime: bss_attrs
                .get(&Nl80211Bss::LastSeenBoottime)
                .and_then(|attr| attr.get_payload_as().ok()),
            seen_ms_ago: bss_attrs
                .get(&Nl80211Bss::SeenMsAgo)
                .and_then(|attr| attr.get_payload_as().ok()),
        })
    }
}

impl Display for Bss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _r = writeln!(
            f,
            "BSSID: {}\r\nSSID: {}\r\nRSSI: {} dBm\r\nChannel Number: {}\r\nChannel Width: {}\r\nWi-Fi Protocols: {}",
            self.bssid,
            self.ssid().unwrap_or_default(),
            self.signal_dbm,
            self.channel().number(),
            self.channel().width(),
            self.wifi_protocols()
        );

        let _b = writeln!(f, "{}", self.capability_info);

        for ie in self.ies.iter() {
            let _b = writeln!(f, "{}", ie);
        }

        Ok(())
    }
}
