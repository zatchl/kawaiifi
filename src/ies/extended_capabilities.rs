use super::{Field, InformationElement};
use bitvec::prelude::*;
use std::fmt::Display;

pub enum MaxMsdus {
    NoLimit,
    ThirtyTwo,
    Sixteen,
    Eight,
}

impl Display for MaxMsdus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxMsdus::NoLimit => write!(f, "No Limit"),
            MaxMsdus::ThirtyTwo => write!(f, "32"),
            MaxMsdus::Sixteen => write!(f, "16"),
            MaxMsdus::Eight => write!(f, "8"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtendedCapabilities {
    bits: BitVec<LocalBits, u8>,
}

impl ExtendedCapabilities {
    pub const NAME: &'static str = "Extended Capabilities";
    pub const ID: u8 = 127;

    pub fn new(bytes: Vec<u8>) -> ExtendedCapabilities {
        ExtendedCapabilities {
            bits: BitVec::from_vec(bytes),
        }
    }

    pub fn twenty_forty_bss_coexistence_management_support(&self) -> Option<bool> {
        self.bits.get(0).as_deref().cloned()
    }

    pub fn extended_channel_switching(&self) -> Option<bool> {
        self.bits.get(2).as_deref().cloned()
    }

    pub fn psmp_capability(&self) -> Option<bool> {
        self.bits.get(4).as_deref().cloned()
    }

    pub fn spsmp_support(&self) -> Option<bool> {
        self.bits.get(6).as_deref().cloned()
    }

    pub fn event(&self) -> Option<bool> {
        self.bits.get(7).as_deref().cloned()
    }

    pub fn diagnostics(&self) -> Option<bool> {
        self.bits.get(8).as_deref().cloned()
    }

    pub fn multicast_diagnostics(&self) -> Option<bool> {
        self.bits.get(9).as_deref().cloned()
    }

    pub fn location_tracking(&self) -> Option<bool> {
        self.bits.get(10).as_deref().cloned()
    }

    pub fn fms(&self) -> Option<bool> {
        self.bits.get(11).as_deref().cloned()
    }

    pub fn proxy_arp_service(&self) -> Option<bool> {
        self.bits.get(12).as_deref().cloned()
    }

    pub fn collocated_interference_reporting(&self) -> Option<bool> {
        self.bits.get(13).as_deref().cloned()
    }

    pub fn civic_location(&self) -> Option<bool> {
        self.bits.get(14).as_deref().cloned()
    }

    pub fn geospatial_location(&self) -> Option<bool> {
        self.bits.get(15).as_deref().cloned()
    }

    pub fn tfs(&self) -> Option<bool> {
        self.bits.get(16).as_deref().cloned()
    }

    pub fn wnm_sleep_mode(&self) -> Option<bool> {
        self.bits.get(17).as_deref().cloned()
    }

    pub fn tim_broadcast(&self) -> Option<bool> {
        self.bits.get(18).as_deref().cloned()
    }

    pub fn bss_transition(&self) -> Option<bool> {
        self.bits.get(19).as_deref().cloned()
    }

    pub fn qos_traffic_capability(&self) -> Option<bool> {
        self.bits.get(20).as_deref().cloned()
    }

    pub fn ac_station_count(&self) -> Option<bool> {
        self.bits.get(21).as_deref().cloned()
    }

    pub fn multiple_bssid(&self) -> Option<bool> {
        self.bits.get(22).as_deref().cloned()
    }

    pub fn timing_measurement(&self) -> Option<bool> {
        self.bits.get(23).as_deref().cloned()
    }

    pub fn channel_usage(&self) -> Option<bool> {
        self.bits.get(24).as_deref().cloned()
    }

    pub fn ssid_list(&self) -> Option<bool> {
        self.bits.get(25).as_deref().cloned()
    }

    pub fn dms(&self) -> Option<bool> {
        self.bits.get(26).as_deref().cloned()
    }

    pub fn utc_tsf_offset(&self) -> Option<bool> {
        self.bits.get(27).as_deref().cloned()
    }

    pub fn tpu_buffer_sta_support(&self) -> Option<bool> {
        self.bits.get(28).as_deref().cloned()
    }

    pub fn tdls_peer_psm_support(&self) -> Option<bool> {
        self.bits.get(29).as_deref().cloned()
    }

    pub fn tdls_channel_switching(&self) -> Option<bool> {
        self.bits.get(30).as_deref().cloned()
    }

    pub fn interworking(&self) -> Option<bool> {
        self.bits.get(31).as_deref().cloned()
    }

    pub fn qos_map(&self) -> Option<bool> {
        self.bits.get(32).as_deref().cloned()
    }

    pub fn ebr(&self) -> Option<bool> {
        self.bits.get(33).as_deref().cloned()
    }

    pub fn sspn_interface(&self) -> Option<bool> {
        self.bits.get(34).as_deref().cloned()
    }

    pub fn msgcf_capability(&self) -> Option<bool> {
        self.bits.get(36).as_deref().cloned()
    }

    pub fn tdls_support(&self) -> Option<bool> {
        self.bits.get(37).as_deref().cloned()
    }

    pub fn tdls_prohibited(&self) -> Option<bool> {
        self.bits.get(38).as_deref().cloned()
    }

    pub fn tdls_channel_switching_prohibited(&self) -> Option<bool> {
        self.bits.get(39).as_deref().cloned()
    }

    pub fn reject_unadmitted_frame(&self) -> Option<bool> {
        self.bits.get(40).as_deref().cloned()
    }

    pub fn service_interval_granularity_ms(&self) -> Option<u8> {
        if let Some(bit_slice) = self.bits.get(41..=43) {
            match bit_slice.load::<u8>() {
                0 => Some(5),
                1 => Some(10),
                2 => Some(15),
                3 => Some(20),
                4 => Some(25),
                5 => Some(30),
                6 => Some(35),
                7 => Some(40),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn identifier_location(&self) -> Option<bool> {
        self.bits.get(44).as_deref().cloned()
    }

    pub fn uapsd_coexistence(&self) -> Option<bool> {
        self.bits.get(45).as_deref().cloned()
    }

    pub fn wnm_notification(&self) -> Option<bool> {
        self.bits.get(46).as_deref().cloned()
    }

    pub fn qab_capability(&self) -> Option<bool> {
        self.bits.get(47).as_deref().cloned()
    }

    pub fn utf8_ssid(&self) -> Option<bool> {
        self.bits.get(48).as_deref().cloned()
    }

    pub fn qmf_activated(&self) -> Option<bool> {
        self.bits.get(49).as_deref().cloned()
    }

    pub fn qmf_reconfiguration_activated(&self) -> Option<bool> {
        self.bits.get(50).as_deref().cloned()
    }

    pub fn robust_av_streaming(&self) -> Option<bool> {
        self.bits.get(51).as_deref().cloned()
    }

    pub fn advanced_gcr(&self) -> Option<bool> {
        self.bits.get(52).as_deref().cloned()
    }

    pub fn mesh_gcr(&self) -> Option<bool> {
        self.bits.get(53).as_deref().cloned()
    }

    pub fn scs(&self) -> Option<bool> {
        self.bits.get(54).as_deref().cloned()
    }

    pub fn qload_report(&self) -> Option<bool> {
        self.bits.get(55).as_deref().cloned()
    }

    pub fn alternate_edca(&self) -> Option<bool> {
        self.bits.get(56).as_deref().cloned()
    }

    pub fn unprotected_txop_negotiation(&self) -> Option<bool> {
        self.bits.get(57).as_deref().cloned()
    }

    pub fn protected_txop_negotiation(&self) -> Option<bool> {
        self.bits.get(58).as_deref().cloned()
    }

    pub fn protected_qload_report(&self) -> Option<bool> {
        self.bits.get(60).as_deref().cloned()
    }

    pub fn tdls_wider_bandwidth(&self) -> Option<bool> {
        self.bits.get(61).as_deref().cloned()
    }

    pub fn operating_mode_notification(&self) -> Option<bool> {
        self.bits.get(62).as_deref().cloned()
    }

    pub fn max_msdus_in_amsdu(&self) -> Option<MaxMsdus> {
        if let Some(bit_slice) = self.bits.get(63..=64) {
            match bit_slice.load::<u8>() {
                0 => Some(MaxMsdus::NoLimit),
                1 => Some(MaxMsdus::ThirtyTwo),
                2 => Some(MaxMsdus::Sixteen),
                3 => Some(MaxMsdus::Eight),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn channel_schedule_management(&self) -> Option<bool> {
        self.bits.get(65).as_deref().cloned()
    }

    pub fn geodatabase_inband_enabling_signal(&self) -> Option<bool> {
        self.bits.get(66).as_deref().cloned()
    }

    pub fn network_channel_control(&self) -> Option<bool> {
        self.bits.get(67).as_deref().cloned()
    }

    pub fn white_space_map(&self) -> Option<bool> {
        self.bits.get(68).as_deref().cloned()
    }

    pub fn channel_availability_query(&self) -> Option<bool> {
        self.bits.get(69).as_deref().cloned()
    }

    pub fn fine_timing_measurement_responder(&self) -> Option<bool> {
        self.bits.get(70).as_deref().cloned()
    }

    pub fn fine_timing_measurement_initiator(&self) -> Option<bool> {
        self.bits.get(71).as_deref().cloned()
    }

    pub fn extended_spectrum_management_capable(&self) -> Option<bool> {
        self.bits.get(73).as_deref().cloned()
    }

    pub fn future_channel_guidance(&self) -> Option<bool> {
        self.bits.get(74).as_deref().cloned()
    }
}

impl InformationElement for ExtendedCapabilities {
    fn name(&self) -> &'static str {
        ExtendedCapabilities::NAME
    }

    fn id(&self) -> u8 {
        ExtendedCapabilities::ID
    }

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();

        if let Some(twenty_forty_coexistence) =
            self.twenty_forty_bss_coexistence_management_support()
        {
            fields.push(Field::new(
                "20/40 BSS Coexistence Management Support",
                twenty_forty_coexistence,
            ));
        }

        if let Some(extended_channel_switching) = self.extended_channel_switching() {
            fields.push(Field::new(
                "Extended Channel Switching",
                extended_channel_switching,
            ));
        }

        if let Some(psmp_capability) = self.psmp_capability() {
            fields.push(Field::new("PSMP Capability", psmp_capability));
        }

        if let Some(spsmp_support) = self.spsmp_support() {
            fields.push(Field::new("S-PSMP Support", spsmp_support));
        }

        if let Some(event) = self.event() {
            fields.push(Field::new("Event", event));
        }

        if let Some(diagnostics) = self.diagnostics() {
            fields.push(Field::new("Diagnostics", diagnostics));
        }

        if let Some(multicast_diagnostics) = self.multicast_diagnostics() {
            fields.push(Field::new("Multicast Diagnostics", multicast_diagnostics));
        }

        if let Some(location_tracking) = self.location_tracking() {
            fields.push(Field::new("Location Tracking", location_tracking));
        }

        if let Some(fms) = self.fms() {
            fields.push(Field::new("FMS", fms));
        }

        if let Some(proxy_arp_service) = self.proxy_arp_service() {
            fields.push(Field::new("Proxy ARP Service", proxy_arp_service));
        }

        if let Some(collocated_interference_reporting) = self.collocated_interference_reporting() {
            fields.push(Field::new(
                "Collocated Interference Reporting",
                collocated_interference_reporting,
            ));
        }

        if let Some(civic_location) = self.civic_location() {
            fields.push(Field::new("Civic Location", civic_location));
        }

        if let Some(geospatial_location) = self.geospatial_location() {
            fields.push(Field::new("Geospatial Location", geospatial_location));
        }

        if let Some(tfs) = self.tfs() {
            fields.push(Field::new("TFS", tfs));
        }

        if let Some(wnm_sleep_mode) = self.wnm_sleep_mode() {
            fields.push(Field::new("WNM Sleep Mode", wnm_sleep_mode));
        }

        if let Some(tim_broadcast) = self.tim_broadcast() {
            fields.push(Field::new("TIM Broadcast", tim_broadcast));
        }

        if let Some(bss_transition) = self.bss_transition() {
            fields.push(Field::new("BSS Transition", bss_transition));
        }

        if let Some(qos_traffic_capability) = self.qos_traffic_capability() {
            fields.push(Field::new("QoS Traffic Capability", qos_traffic_capability));
        }

        if let Some(ac_station_count) = self.ac_station_count() {
            fields.push(Field::new("AC Station Count", ac_station_count));
        }

        if let Some(multiple_bssid) = self.multiple_bssid() {
            fields.push(Field::new("Multiple BSSID", multiple_bssid));
        }

        if let Some(timing_measurement) = self.timing_measurement() {
            fields.push(Field::new("Timing Measurement", timing_measurement));
        }

        if let Some(channel_usage) = self.channel_usage() {
            fields.push(Field::new("Channel Usage", channel_usage));
        }

        if let Some(ssid_list) = self.ssid_list() {
            fields.push(Field::new("SSID List", ssid_list));
        }

        if let Some(dms) = self.dms() {
            fields.push(Field::new("DMS", dms));
        }

        if let Some(utc_tsf_offset) = self.utc_tsf_offset() {
            fields.push(Field::new("UTC TSF Offset", utc_tsf_offset));
        }

        if let Some(tpu_buffer_sta_support) = self.tpu_buffer_sta_support() {
            fields.push(Field::new("TPU Buffer STA Support", tpu_buffer_sta_support));
        }

        if let Some(tdls_peer_psm_support) = self.tdls_peer_psm_support() {
            fields.push(Field::new("TDLS Peer PSM Support", tdls_peer_psm_support));
        }

        if let Some(tdls_channel_switching) = self.tdls_channel_switching() {
            fields.push(Field::new("TDLS Channel Switching", tdls_channel_switching));
        }

        if let Some(interworking) = self.interworking() {
            fields.push(Field::new("Interworking", interworking));
        }

        if let Some(qos_map) = self.qos_map() {
            fields.push(Field::new("QoS Map", qos_map));
        }

        if let Some(ebr) = self.ebr() {
            fields.push(Field::new("EBR", ebr));
        }

        if let Some(sspn_interface) = self.sspn_interface() {
            fields.push(Field::new("SSPN Interface", sspn_interface));
        }

        if let Some(msgcf_capability) = self.msgcf_capability() {
            fields.push(Field::new("MSGCF Capability", msgcf_capability));
        }

        if let Some(tdls_support) = self.tdls_support() {
            fields.push(Field::new("TDLS Support", tdls_support));
        }

        if let Some(tdls_prohibited) = self.tdls_prohibited() {
            fields.push(Field::new("TDLS Prohibited", tdls_prohibited));
        }

        if let Some(tdls_channel_switching_prohibited) = self.tdls_channel_switching_prohibited() {
            fields.push(Field::new(
                "TDLS Channel Switching Prohibited",
                tdls_channel_switching_prohibited,
            ));
        }

        if let Some(reject_unadmitted_frame) = self.reject_unadmitted_frame() {
            fields.push(Field::new(
                "Reject Unadmitted Frame",
                reject_unadmitted_frame,
            ));
        }

        if let Some(service_interval_granularity_ms) = self.service_interval_granularity_ms() {
            fields.push(Field::new(
                "Service Interval Granularity",
                format!("{} ms", service_interval_granularity_ms),
            ));
        }

        if let Some(identifier_location) = self.identifier_location() {
            fields.push(Field::new("Identifier Location", identifier_location));
        }

        if let Some(uapsd_coexistence) = self.uapsd_coexistence() {
            fields.push(Field::new("U-APSD Coexistence", uapsd_coexistence));
        }

        if let Some(wnm_notification) = self.wnm_notification() {
            fields.push(Field::new("WNM Notification", wnm_notification));
        }

        if let Some(qab_capability) = self.qab_capability() {
            fields.push(Field::new("QAB Capability", qab_capability));
        }

        if let Some(utf8_ssid) = self.utf8_ssid() {
            fields.push(Field::new("UTF-8 SSID", utf8_ssid));
        }

        if let Some(qmf_activated) = self.qmf_activated() {
            fields.push(Field::new("QMF Activated", qmf_activated));
        }

        if let Some(qmf_reconfiguration_activated) = self.qmf_reconfiguration_activated() {
            fields.push(Field::new(
                "QMF Reconfiguration Activated",
                qmf_reconfiguration_activated,
            ));
        }

        if let Some(robust_av_streaming) = self.robust_av_streaming() {
            fields.push(Field::new("Robust AV Streaming", robust_av_streaming));
        }

        if let Some(advanced_gcr) = self.advanced_gcr() {
            fields.push(Field::new("Advanced GCR", advanced_gcr));
        }

        if let Some(mesh_gcr) = self.mesh_gcr() {
            fields.push(Field::new("Mesh GCR", mesh_gcr));
        }

        if let Some(scs) = self.scs() {
            fields.push(Field::new("SCS", scs));
        }

        if let Some(qload_report) = self.qload_report() {
            fields.push(Field::new("QLoad Report", qload_report));
        }

        if let Some(alternate_edca) = self.alternate_edca() {
            fields.push(Field::new("Alternate EDCA", alternate_edca));
        }

        if let Some(unprotected_txop_negotiation) = self.unprotected_txop_negotiation() {
            fields.push(Field::new(
                "Unprotected TXOP Negotiation",
                unprotected_txop_negotiation,
            ));
        }

        if let Some(protected_txop_negotiation) = self.protected_txop_negotiation() {
            fields.push(Field::new(
                "Protected TXOP Negotiation",
                protected_txop_negotiation,
            ));
        }

        if let Some(protected_qload_report) = self.protected_qload_report() {
            fields.push(Field::new("Protected QLoad Report", protected_qload_report));
        }

        if let Some(tdls_wider_bandwidth) = self.tdls_wider_bandwidth() {
            fields.push(Field::new("TDLS Wider Bandwidth", tdls_wider_bandwidth));
        }

        if let Some(operating_mode_notification) = self.operating_mode_notification() {
            fields.push(Field::new(
                "Operating Mode Notification",
                operating_mode_notification,
            ));
        }

        if let Some(max_msdus_in_amsdu) = self.max_msdus_in_amsdu() {
            fields.push(Field::new(
                "Max Number Of MSDUs In A-MSDU",
                max_msdus_in_amsdu,
            ));
        }

        if let Some(channel_schedule_management) = self.channel_schedule_management() {
            fields.push(Field::new(
                "Channel Schedule Management",
                channel_schedule_management,
            ));
        }

        if let Some(geodatabase_inband_enabling_signal) = self.geodatabase_inband_enabling_signal()
        {
            fields.push(Field::new(
                "Geodatabase Inband Enabling Signal",
                geodatabase_inband_enabling_signal,
            ));
        }

        if let Some(network_channel_control) = self.network_channel_control() {
            fields.push(Field::new(
                "Network Channel Control",
                network_channel_control,
            ));
        }

        if let Some(white_space_map) = self.white_space_map() {
            fields.push(Field::new("White Space Map", white_space_map));
        }

        if let Some(channel_availability_query) = self.channel_availability_query() {
            fields.push(Field::new(
                "Channel Availability Query",
                channel_availability_query,
            ));
        }

        if let Some(fine_timing_measurement_responder) = self.fine_timing_measurement_responder() {
            fields.push(Field::new(
                "Fine Timing Measurement Responder",
                fine_timing_measurement_responder,
            ));
        }

        if let Some(fine_timing_measurement_initiator) = self.fine_timing_measurement_initiator() {
            fields.push(Field::new(
                "Fine Timing Measurement Initiator",
                fine_timing_measurement_initiator,
            ));
        }

        if let Some(extended_spectrum_management_capable) =
            self.extended_spectrum_management_capable()
        {
            fields.push(Field::new(
                "Extended Spectrum Management Capable",
                extended_spectrum_management_capable,
            ));
        }

        if let Some(future_channel_guidance) = self.future_channel_guidance() {
            fields.push(Field::new(
                "Future Channel Guidance",
                future_channel_guidance,
            ));
        }

        fields
    }
}
