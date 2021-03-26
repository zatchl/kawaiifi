use super::{Field, IeError, InformationElement};
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementPilotTransmission {
    bytes: Vec<u8>,
}

impl MeasurementPilotTransmission {
    pub const MIN_LENGTH: usize = 1;

    pub fn new(bytes: Vec<u8>) -> Result<MeasurementPilotTransmission, IeError> {
        if bytes.len() >= Self::MIN_LENGTH {
            Ok(MeasurementPilotTransmission { bytes })
        } else {
            Err(IeError::InvalidLength {
                ie_name: Self::NAME,
                expected_length: Self::MIN_LENGTH,
                actual_length: bytes.len(),
            })
        }
    }

    pub fn measurement_pilot_interval_tu(&self) -> u8 {
        self.bytes[0]
    }

    pub fn subelements(&self) -> Vec<(u8, u8, Vec<u8>)> {
        let mut bytes = Cursor::new(&self.bytes[1..]);

        let mut subelements = Vec::new();

        loop {
            // The first byte of the subelement is the ID
            let se_id = match bytes.read_u8() {
                Ok(se_id) => se_id,
                _ => break,
            };

            // The second byte of the subelement is the number of bytes of data
            let se_len = match bytes.read_u8() {
                Ok(se_len) => se_len,
                _ => break,
            };

            // Bytes [2..se_len+2] is the subelement data
            let se_data = {
                let mut se_data = vec![0; se_len as usize];
                match bytes.read_exact(&mut se_data) {
                    Ok(_) => se_data,
                    _ => break,
                }
            };

            subelements.push((se_id, se_len, se_data));
        }

        subelements
    }
}

impl InformationElement for MeasurementPilotTransmission {
    const NAME: &'static str = "Measurement Pilot Transmission";
    const ID: u8 = 66;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        let mut fields = vec![Field::new(
            "Measurement Pilot Interval",
            format!("{} TU", self.measurement_pilot_interval_tu()),
        )];

        let subelements = self.subelements();
        if !subelements.is_empty() {
            fields.push(Field::with_subfields(
                "Subelements",
                format!("{:?}", &self.bytes[1..]),
                subelements
                    .iter()
                    .map(|se| Field::new(format!("ID {}", se.0), format!("{:?}", se.2)))
                    .collect(),
            ));
        }

        fields
    }
}

impl_display_for_ie!(MeasurementPilotTransmission);
