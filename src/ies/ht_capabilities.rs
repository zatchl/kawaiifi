use super::{Field, IeError, InformationElement};
use crate::{ChannelWidth, ChannelWidths};
use bitvec::prelude::*;
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SmPowerSave {
    Static = 0,
    Dynamic = 1,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RxStbc {
    NotSupported,
    OneSpatialStream,
    OneAndTwoSpatialStreams,
    OneTwoAndThreeSpatialStreams,
}

impl Display for RxStbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RxStbc::NotSupported => write!(f, "Not Supported"),
            RxStbc::OneSpatialStream => write!(f, "One Spatial Stream"),
            RxStbc::OneAndTwoSpatialStreams => write!(f, "One and Two Spatial Streams"),
            RxStbc::OneTwoAndThreeSpatialStreams => {
                write!(f, "One, Two, and Three Spatial Streams")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MpduStartSpacing {
    NoRestriction,
    QuarterMicrosecond,
    HalfMicrosecond,
    OneMicrosecond,
    TwoMicroseconds,
    FourMicroseconds,
    EightMicroseconds,
    SixteenMicroseconds,
}

impl Display for MpduStartSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MpduStartSpacing::NoRestriction => write!(f, "No Restriction"),
            MpduStartSpacing::QuarterMicrosecond => write!(f, "1/4 μs"),
            MpduStartSpacing::HalfMicrosecond => write!(f, "1/2 μs"),
            MpduStartSpacing::OneMicrosecond => write!(f, "1 μs"),
            MpduStartSpacing::TwoMicroseconds => write!(f, "2 μs"),
            MpduStartSpacing::FourMicroseconds => write!(f, "4 μs"),
            MpduStartSpacing::EightMicroseconds => write!(f, "8 μs"),
            MpduStartSpacing::SixteenMicroseconds => write!(f, "16 μs"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PcoTransitionTime {
    NoTransition,
    FourHundredMicroseconds,
    OneAndAHalfMilliseconds,
    FiveMilliseconds,
}

impl Display for PcoTransitionTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcoTransitionTime::NoTransition => write!(f, "No Transition"),
            PcoTransitionTime::FourHundredMicroseconds => write!(f, "400 μs"),
            PcoTransitionTime::OneAndAHalfMilliseconds => write!(f, "1.5 ms"),
            PcoTransitionTime::FiveMilliseconds => write!(f, "5 ms"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum McsFeedback {
    NoMfb,
    UnsolicitedMfb = 2,
    ResponseOrUnsolicitedMfb,
}

impl Display for McsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McsFeedback::NoMfb => write!(f, "No MFB"),
            McsFeedback::UnsolicitedMfb => write!(f, "Unsolicited MFB"),
            McsFeedback::ResponseOrUnsolicitedMfb => {
                write!(f, "Response (Delayed/Immediate) or Unsolicited MFB")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Calibration {
    NotSupported,
    Respond,
    InitiateAndRespond = 3,
}

impl Display for Calibration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calibration::NotSupported => write!(f, "Not Supported"),
            Calibration::Respond => write!(f, "Respond to Calibration Request"),
            Calibration::InitiateAndRespond => {
                write!(f, "Initiate and Respond to Calibration Request")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum BeamformingFeedback {
    NotSupported,
    Delayed,
    Immediate,
    DelayedAndImmediate,
}

impl Display for BeamformingFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeamformingFeedback::NotSupported => write!(f, "Not Supported"),
            BeamformingFeedback::Delayed => write!(f, "Delayed"),
            BeamformingFeedback::Immediate => write!(f, "Immediate"),
            BeamformingFeedback::DelayedAndImmediate => write!(f, "Delayed and Immediate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MinimalGrouping {
    One,
    OneOrTwo,
    OneOrFour,
    OneOrTwoOrFour,
}

impl Display for MinimalGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinimalGrouping::One => write!(f, "1"),
            MinimalGrouping::OneOrTwo => write!(f, "1 or 2"),
            MinimalGrouping::OneOrFour => write!(f, "1 or 4"),
            MinimalGrouping::OneOrTwoOrFour => write!(f, "1, 2, or 4"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtCapabilities {
    bits: BitVec<LocalBits, u8>,
}

impl HtCapabilities {
    pub const MIN_LENGTH: usize = 26;

    pub fn new(bytes: Vec<u8>) -> Result<HtCapabilities, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(HtCapabilities {
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

    // HT Capability Information

    pub fn ldpc_coding_capability(&self) -> bool {
        self.bits[0]
    }

    pub fn supported_channel_width_set(&self) -> ChannelWidths {
        if self.bits[1] {
            (ChannelWidth::TwentyMhz | ChannelWidth::FortyMhz).into()
        } else {
            ChannelWidth::TwentyMhz.into()
        }
    }

    pub fn sm_power_save(&self) -> SmPowerSave {
        SmPowerSave::try_from(self.bits[2..=3].load::<u8>()).unwrap_or(SmPowerSave::None)
    }

    pub fn ht_greenfield(&self) -> bool {
        self.bits[4]
    }

    pub fn short_gi_twenty_mhz(&self) -> bool {
        self.bits[5]
    }

    pub fn short_gi_forty_mhz(&self) -> bool {
        self.bits[6]
    }

    pub fn tx_stbc(&self) -> bool {
        self.bits[7]
    }

    pub fn rx_stbc(&self) -> RxStbc {
        RxStbc::try_from(self.bits[8..=9].load::<u8>()).unwrap_or(RxStbc::NotSupported)
    }

    pub fn ht_delayed_block_ack(&self) -> bool {
        self.bits[10]
    }

    pub fn max_amsdu_length(&self) -> usize {
        if self.bits[11] {
            7935
        } else {
            3839
        }
    }

    pub fn dsss_cck_mode_forty_mhz(&self) -> bool {
        self.bits[12]
    }

    pub fn forty_mhz_intolerant(&self) -> bool {
        self.bits[14]
    }

    pub fn lsig_txop_protection_support(&self) -> bool {
        self.bits[15]
    }

    // A-MPDU Parameters

    pub fn max_ampdu_length_exponent(&self) -> u8 {
        self.bits[16..=17].load::<u8>()
    }

    pub fn max_ampdu_length(&self) -> u32 {
        2u32.pow(13 + u32::from(self.max_ampdu_length_exponent())) - 1
    }

    pub fn min_mpdu_start_spacing(&self) -> MpduStartSpacing {
        MpduStartSpacing::try_from(self.bits[18..=20].load::<u8>())
            .unwrap_or(MpduStartSpacing::NoRestriction)
    }

    // Supported MCS Set

    // HT Extended Capabilities

    pub fn pco(&self) -> bool {
        self.bits[152]
    }

    pub fn pco_transition_time(&self) -> PcoTransitionTime {
        PcoTransitionTime::try_from(self.bits[153..=154].load::<u8>())
            .unwrap_or(PcoTransitionTime::NoTransition)
    }

    pub fn mcs_feedback(&self) -> McsFeedback {
        McsFeedback::try_from(self.bits[160..=161].load::<u8>()).unwrap_or(McsFeedback::NoMfb)
    }

    pub fn htc_ht_support(&self) -> bool {
        self.bits[162]
    }

    pub fn rd_responder(&self) -> bool {
        self.bits[163]
    }

    // Transmit Beamforming Capabilities

    pub fn implicit_tx_beamforming_rx_capable(&self) -> bool {
        self.bits[168]
    }

    pub fn rx_staggered_sounding_capable(&self) -> bool {
        self.bits[169]
    }

    pub fn tx_staggered_sounding_capable(&self) -> bool {
        self.bits[170]
    }

    pub fn rx_ndp_capable(&self) -> bool {
        self.bits[171]
    }

    pub fn tx_ndp_capable(&self) -> bool {
        self.bits[172]
    }

    pub fn implicit_tx_beamforming_capable(&self) -> bool {
        self.bits[173]
    }

    pub fn calibration(&self) -> Calibration {
        Calibration::try_from(self.bits[174..=175].load::<u8>())
            .unwrap_or(Calibration::NotSupported)
    }

    pub fn explicit_csi_tx_beamforming_capable(&self) -> bool {
        self.bits[176]
    }

    pub fn explicit_noncompressed_steering_capable(&self) -> bool {
        self.bits[177]
    }

    pub fn explicit_compressed_steering_capable(&self) -> bool {
        self.bits[178]
    }

    pub fn explicit_tx_beamforming_csi_feedback(&self) -> BeamformingFeedback {
        BeamformingFeedback::try_from(self.bits[179..=180].load::<u8>())
            .unwrap_or(BeamformingFeedback::NotSupported)
    }

    pub fn explicit_noncompressed_beamforming_feedback_capable(&self) -> BeamformingFeedback {
        BeamformingFeedback::try_from(self.bits[181..=182].load::<u8>())
            .unwrap_or(BeamformingFeedback::NotSupported)
    }

    pub fn explicit_compressed_beamforming_feedback_capable(&self) -> BeamformingFeedback {
        BeamformingFeedback::try_from(self.bits[183..=184].load::<u8>())
            .unwrap_or(BeamformingFeedback::NotSupported)
    }

    pub fn minimal_grouping(&self) -> MinimalGrouping {
        MinimalGrouping::try_from(self.bits[185..=186].load::<u8>()).unwrap_or(MinimalGrouping::One)
    }

    pub fn csi_number_of_beamformer_antennas_supported(&self) -> u8 {
        match self.bits[187..=188].load::<u8>() {
            num @ 0..=3 => num + 1,
            _ => 0,
        }
    }

    pub fn noncompressed_steering_number_of_beamformer_antennas_supported(&self) -> u8 {
        match self.bits[189..=190].load::<u8>() {
            num @ 0..=3 => num + 1,
            _ => 0,
        }
    }

    pub fn compressed_steering_number_of_beamformer_antennas_supported(&self) -> u8 {
        match self.bits[191..=192].load::<u8>() {
            num @ 0..=3 => num + 1,
            _ => 0,
        }
    }

    pub fn csi_max_number_of_rows_beamformer_supported(&self) -> u8 {
        match self.bits[193..=194].load::<u8>() {
            num @ 0..=3 => num + 1,
            _ => 0,
        }
    }

    pub fn channel_estimation_capability(&self) -> u8 {
        match self.bits[195..=196].load::<u8>() {
            num @ 0..=3 => num + 1,
            _ => 0,
        }
    }

    // ASEL Capabilities

    pub fn antenna_selection_capable(&self) -> bool {
        self.bits[200]
    }

    pub fn explicit_csi_feedback_based_tx_asel_capable(&self) -> bool {
        self.bits[201]
    }

    pub fn antenna_indices_feedback_based_tx_asel_capable(&self) -> bool {
        self.bits[202]
    }

    pub fn explicit_csi_feedback_capable(&self) -> bool {
        self.bits[203]
    }

    pub fn antenna_indices_feedback_capable(&self) -> bool {
        self.bits[204]
    }

    pub fn rx_asel_capable(&self) -> bool {
        self.bits[205]
    }

    pub fn tx_sounding_ppdus_capable(&self) -> bool {
        self.bits[206]
    }
}

impl InformationElement for HtCapabilities {
    const NAME: &'static str = "HT Capabilities";
    const ID: u8 = 45;

    fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![
            Field::with_subfields(
                "HT Capability Information",
                format!("{:02?}", &self.bits.as_raw_slice()[0..=1]),
                vec![
                    Field::new("LDPC Coding Capability", self.ldpc_coding_capability()),
                    Field::new(
                        "Supported Channel Width Set",
                        self.supported_channel_width_set(),
                    ),
                    Field::new("SM Power Save", format!("{:?}", self.sm_power_save())),
                    Field::new("HT-Greenfield", self.ht_greenfield()),
                    Field::new("Short GI for 20 MHz", self.short_gi_twenty_mhz()),
                    Field::new("Short GI for 40 MHz", self.short_gi_forty_mhz()),
                    Field::new("Tx STBC", self.tx_stbc()),
                    Field::new("Rx STBC", self.rx_stbc()),
                    Field::new("HT-delayed Block Ack", self.ht_delayed_block_ack()),
                    Field::new("Maximum A-MSDU Length", self.max_amsdu_length()),
                    Field::new("DSSS/CCK Mode in 40 MHz", self.dsss_cck_mode_forty_mhz()),
                    Field::new("Forty MHz Intolerant", self.forty_mhz_intolerant()),
                    Field::new(
                        "L-SIG TXOP Protection Support",
                        self.lsig_txop_protection_support(),
                    ),
                ],
            ),
            Field::with_subfields(
                "A-MPDU Parameters",
                format!("{:02?}", self.bits.as_raw_slice()[2]),
                vec![
                    Field::new(
                        "Maximum A-MPDU Length Exponent",
                        format!(
                            "{} ({} octets)",
                            self.max_ampdu_length_exponent(),
                            self.max_ampdu_length()
                        ),
                    ),
                    Field::new("Minimum MPDU Start Spacing", self.min_mpdu_start_spacing()),
                ],
            ),
            Field::with_subfields(
                "Supported MCS Set",
                format!("{:02?}", &self.bits.as_raw_slice()[3..=18]),
                Vec::new(),
            ),
            Field::with_subfields(
                "HT Extended Capabilities",
                format!("{:02?}", &self.bits.as_raw_slice()[19..=20]),
                vec![
                    Field::new("PCO", self.pco()),
                    Field::new("PCO Transition Time", self.pco_transition_time()),
                    Field::new("MCS Feedback", self.mcs_feedback()),
                    Field::new("+HTC-HT Support", self.htc_ht_support()),
                    Field::new("RD Responder", self.rd_responder()),
                ],
            ),
            Field::with_subfields(
                "Transmit Beamforming Capabilities",
                format!("{:02?}", &self.bits.as_raw_slice()[21..=24]),
                vec![
                    Field::new(
                        "Implicit Transmit Beamforming Receiving Capable",
                        self.implicit_tx_beamforming_rx_capable(),
                    ),
                    Field::new(
                        "Receive Staggered Sounding Capable",
                        self.rx_staggered_sounding_capable(),
                    ),
                    Field::new(
                        "Transmit Staggered Sounding Capable",
                        self.tx_staggered_sounding_capable(),
                    ),
                    Field::new("Receive NDP Capable", self.rx_ndp_capable()),
                    Field::new("Transmit NDP Capable", self.tx_ndp_capable()),
                    Field::new(
                        "Implicit Transmit Beamforming Capable",
                        self.implicit_tx_beamforming_capable(),
                    ),
                    Field::new("Calibration", self.calibration()),
                    Field::new(
                        "Explicit CSI Transmit Beamforming Capable",
                        self.explicit_csi_tx_beamforming_capable(),
                    ),
                    Field::new(
                        "Explicit Noncompressed Steering Capable",
                        self.explicit_noncompressed_steering_capable(),
                    ),
                    Field::new(
                        "Explicit Compressed Steering Capable",
                        self.explicit_compressed_steering_capable(),
                    ),
                    Field::new(
                        "Explicit Transmit Beamforming CSI Feedback",
                        self.explicit_tx_beamforming_csi_feedback(),
                    ),
                    Field::new(
                        "Explicit Noncompressed Beamforming Feedback Capable",
                        self.explicit_noncompressed_beamforming_feedback_capable(),
                    ),
                    Field::new(
                        "Explicit Compressed Beamforming Feedback Capable",
                        self.explicit_compressed_beamforming_feedback_capable(),
                    ),
                    Field::new("Minimal Grouping", self.minimal_grouping()),
                    Field::new(
                        "CSI Number of Beamformer Antennas Supported",
                        self.csi_number_of_beamformer_antennas_supported(),
                    ),
                    Field::new(
                        "Noncompressed Steering Number of Beamformer Antennas Supported",
                        self.noncompressed_steering_number_of_beamformer_antennas_supported(),
                    ),
                    Field::new(
                        "Compressed Steering Number of Beamformer Antennas Supported",
                        self.compressed_steering_number_of_beamformer_antennas_supported(),
                    ),
                    Field::new(
                        "CSI Max Number of Rows Beamformer Supported",
                        self.csi_max_number_of_rows_beamformer_supported(),
                    ),
                    Field::new(
                        "Channel Estimation Capability",
                        self.channel_estimation_capability(),
                    ),
                ],
            ),
            Field::with_subfields(
                "ASEL Capabilities",
                format!("{:02?}", &self.bits.as_raw_slice()[25]),
                vec![
                    Field::new(
                        "Antenna Selection Capable",
                        self.antenna_selection_capable(),
                    ),
                    Field::new(
                        "Explicit CSI Feedback Based Transmit ASEL Capable",
                        self.explicit_csi_feedback_based_tx_asel_capable(),
                    ),
                    Field::new(
                        "Antenna Indices Feedback Based Transmit ASEL Capable",
                        self.antenna_indices_feedback_based_tx_asel_capable(),
                    ),
                    Field::new(
                        "Explicit CSI Feedback Capable",
                        self.explicit_csi_feedback_capable(),
                    ),
                    Field::new(
                        "Antenna Indices Feedback Capable",
                        self.antenna_indices_feedback_capable(),
                    ),
                    Field::new("Receive ASEL Capable", self.rx_asel_capable()),
                    Field::new(
                        "Transmit Sounding PPDUs Capable",
                        self.tx_sounding_ppdus_capable(),
                    ),
                ],
            ),
        ]
    }
}

impl_display_for_ie!(HtCapabilities);
