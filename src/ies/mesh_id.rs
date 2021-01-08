use super::{Field, InformationElement};

#[derive(Debug, Clone)]
pub struct MeshId {
    bytes: Vec<u8>,
}

impl MeshId {
    pub const NAME: &'static str = "Mesh ID";
    pub const ID: u8 = 114;

    pub fn new(bytes: Vec<u8>) -> MeshId {
        MeshId { bytes }
    }

    pub fn mesh_id(&self) -> &[u8] {
        &self.bytes
    }
}

impl InformationElement for MeshId {
    fn name(&self) -> &'static str {
        MeshId::NAME
    }

    fn id(&self) -> u8 {
        MeshId::ID
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field {
            title: "Mesh ID".to_string(),
            value: format!("{:X?}", self.mesh_id()),
            subfields: None,
        }]
    }
}
