use crate::{
    ies::{DataRate, SupportedRates},
    Ie,
};
use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{bitflags, BitFlags};
use std::fmt::Display;

#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
#[repr(u16)]
pub enum WifiProtocol {
    A = 1 << 0,
    B = 1 << 1,
    G = 1 << 2,
    N = 1 << 3,
    AC = 1 << 4,
    AX = 1 << 5,
}

impl Display for WifiProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WifiProtocol::A => write!(f, "a"),
            WifiProtocol::B => write!(f, "b"),
            WifiProtocol::G => write!(f, "g"),
            WifiProtocol::N => write!(f, "n"),
            WifiProtocol::AC => write!(f, "ac"),
            WifiProtocol::AX => write!(f, "ax"),
        }
    }
}

// Use the Newtype pattern to create a type alias (WifiProtocols) and implement the From trait
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
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
pub struct WifiProtocols(BitFlags<WifiProtocol>);

impl PartialEq<BitFlags<WifiProtocol, u16>> for WifiProtocols {
    fn eq(&self, other: &BitFlags<WifiProtocol, u16>) -> bool {
        self.0.eq(other)
    }
}

impl Ord for WifiProtocols {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits().cmp(&other.bits())
    }
}

impl PartialOrd for WifiProtocols {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&SupportedRates> for WifiProtocols {
    fn from(supported_rates: &SupportedRates) -> Self {
        let mut protocols = WifiProtocols(BitFlags::empty());

        for rate in supported_rates.rates() {
            match rate {
                DataRate::OneMbps(_) | DataRate::TwoMbps(_) | DataRate::FivePointFiveMbps(_) => {
                    protocols.insert(WifiProtocol::B)
                }
                DataRate::SixMbps(_)
                | DataRate::NineMbps(_)
                | DataRate::TwelveMbps(_)
                | DataRate::EighteenMbps(_)
                | DataRate::TwentyFourMbps(_)
                | DataRate::ThirtySixMbps(_)
                | DataRate::FortyEightMbps(_)
                | DataRate::FiftyFourMbps(_) => protocols.insert(WifiProtocol::G),
                _ => continue,
            }
        }

        protocols
    }
}

impl From<&[Ie]> for WifiProtocols {
    fn from(ies: &[Ie]) -> Self {
        let mut protocols = WifiProtocols(BitFlags::empty());

        for ie in ies {
            match ie {
                Ie::SupportedRates(supported_rates) => {
                    protocols.insert(*WifiProtocols::from(supported_rates));
                }
                Ie::HtCapabilities(_) => protocols.insert(WifiProtocol::N),
                Ie::VhtCapabilities(_) => protocols.insert(WifiProtocol::AC),
                Ie::HeCapabilities(_) => protocols.insert(WifiProtocol::AX),
                _ => continue,
            }
        }

        protocols
    }
}

impl Display for WifiProtocols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|protocol| protocol.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}
