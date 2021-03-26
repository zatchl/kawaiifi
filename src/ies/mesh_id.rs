use super::{Field, InformationElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshId {
    bytes: Vec<u8>,
}

impl MeshId {
    pub fn new(bytes: Vec<u8>) -> MeshId {
        MeshId { bytes }
    }

    pub fn mesh_id(&self) -> &[u8] {
        &self.bytes
    }
}

impl InformationElement for MeshId {
    const NAME: &'static str = "Mesh ID";
    const ID: u8 = 114;

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn information_fields(&self) -> Vec<Field> {
        vec![Field::new("Mesh ID", format!("{:X?}", self.mesh_id()))]
    }
}

impl_display_for_ie!(MeshId);
