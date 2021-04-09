use crate::Ie;
use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{bitflags, BitFlags};
use std::{convert::From, fmt::Display};

#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
#[repr(u8)]
pub enum ChannelWidth {
    TwentyMhz = 1 << 0,
    TwentyTwoMhz = 1 << 1,
    FortyMhz = 1 << 2,
    EightyMhz = 1 << 3,
    EightyPlusEightyMhz = 1 << 4,
    OneSixtyMhz = 1 << 5,
}

impl From<&[Ie]> for ChannelWidth {
    // From Table 11-24 in IEEE Std 802.11-2016
    fn from(ies: &[Ie]) -> Self {
        let ht_channel_widths = ies
            .iter()
            .find_map(|ie| match ie {
                Ie::HtOperation(ht_operation) => Some(ht_operation.sta_channel_width()),
                _ => None,
            })
            .unwrap_or(ChannelWidth::TwentyMhz.into());

        let (vht_channel_widths, channel_center_segment_zero, channel_center_segment_one) = ies
            .iter()
            .find_map(|ie| match ie {
                Ie::VhtOperation(vht_operation) => Some((
                    vht_operation.channel_width(),
                    vht_operation.channel_center_freq_segment_zero(),
                    vht_operation.channel_center_freq_segment_one(),
                )),
                _ => None,
            })
            .unwrap_or((ChannelWidth::TwentyMhz.into(), 0, 0));

        if ht_channel_widths == ChannelWidths::from(ChannelWidth::TwentyMhz) {
            return ChannelWidth::TwentyMhz;
        }

        if vht_channel_widths == ChannelWidth::TwentyMhz | ChannelWidth::FortyMhz {
            return ChannelWidth::FortyMhz;
        }

        if vht_channel_widths
            == ChannelWidth::EightyMhz
                | ChannelWidth::EightyPlusEightyMhz
                | ChannelWidth::OneSixtyMhz
            && channel_center_segment_one == 0
        {
            return ChannelWidth::EightyMhz;
        }

        if vht_channel_widths
            == ChannelWidth::EightyMhz
                | ChannelWidth::EightyPlusEightyMhz
                | ChannelWidth::OneSixtyMhz
            && channel_center_segment_one > 0
            && (channel_center_segment_one - channel_center_segment_zero) == 8
        {
            return ChannelWidth::OneSixtyMhz;
        }

        if vht_channel_widths.0
            == ChannelWidth::EightyMhz
                | ChannelWidth::EightyPlusEightyMhz
                | ChannelWidth::OneSixtyMhz
            && channel_center_segment_one > 0
            && (channel_center_segment_one - channel_center_segment_zero) > 16
        {
            return ChannelWidth::EightyPlusEightyMhz;
        }

        if vht_channel_widths == ChannelWidths::from(ChannelWidth::EightyPlusEightyMhz) {
            return ChannelWidth::EightyPlusEightyMhz;
        }

        if vht_channel_widths == ChannelWidths::from(ChannelWidth::OneSixtyMhz) {
            return ChannelWidth::OneSixtyMhz;
        }

        ChannelWidth::TwentyMhz
    }
}

impl Display for ChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelWidth::TwentyMhz => write!(f, "20 MHz"),
            ChannelWidth::TwentyTwoMhz => write!(f, "22 MHz"),
            ChannelWidth::FortyMhz => write!(f, "40 MHz"),
            ChannelWidth::EightyMhz => write!(f, "80 MHz"),
            ChannelWidth::EightyPlusEightyMhz => write!(f, "80+80 MHz"),
            ChannelWidth::OneSixtyMhz => write!(f, "160 MHz"),
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Deref,
    DerefMut,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    From,
    Not,
)]
#[from(forward)]
pub struct ChannelWidths(BitFlags<ChannelWidth>);

impl PartialEq<BitFlags<ChannelWidth, u8>> for ChannelWidths {
    fn eq(&self, other: &BitFlags<ChannelWidth, u8>) -> bool {
        self.0.eq(other)
    }
}

impl Display for ChannelWidths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut channel_widths = Vec::new();

        for channel_width in [
            ChannelWidth::TwentyMhz,
            ChannelWidth::TwentyTwoMhz,
            ChannelWidth::FortyMhz,
            ChannelWidth::EightyMhz,
            ChannelWidth::EightyPlusEightyMhz,
            ChannelWidth::OneSixtyMhz,
        ]
        .iter()
        {
            if self.contains(*channel_width) {
                channel_widths.push(channel_width.to_string());
            }
        }

        write!(f, "{}", channel_widths.join(", "))
    }
}
