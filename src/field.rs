use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub(crate) title: String,
    pub(crate) value: String,
    pub(crate) subfields: Option<Vec<Field>>,
}

impl Field {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn subfields(&self) -> Option<&Vec<Field>> {
        self.subfields.as_ref()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.value)
    }
}
