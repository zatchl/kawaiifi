use super::{Display, InformationElement};

#[derive(Debug)]
pub struct CountryInfo {
    bytes: Vec<u8>,
}

impl CountryInfo {
    pub const ID: u8 = 7;
    pub const NAME: &'static str = "Country Info";

    pub fn new(bytes: Vec<u8>) -> CountryInfo {
        CountryInfo { bytes }
        // if bytes.len() < 5 {
        //     Err(String::from("CountryInfo IE must have at least 5 bytes"))
        // } else if bytes.len() % 2 != 0 {
        //     Err(String::from(
        //         "CountryInfo IE must have an even number of bytes",
        //     ))
        // } else if bytes[0] != CountryInfo::ID {
        //     Err(format!(
        //         "CountryInfo IE ID component must be {}",
        //         CountryInfo::ID
        //     ))
        // } else if usize::from(bytes[1]) != bytes.len() - 2 {
        //     Err(format!(
        //         "CountryInfo IE length component must be {}",
        //         bytes.len() - 2
        //     ))
        // } else {
        //     Ok(CountryInfo {
        //         bytes: bytes.to_vec(),
        //     })
        // }
    }
}

impl InformationElement for CountryInfo {
    fn name(&self) -> &'static str {
        CountryInfo::NAME
    }

    fn id(&self) -> u8 {
        CountryInfo::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Display for CountryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", Self::NAME, self.bytes)
    }
}
