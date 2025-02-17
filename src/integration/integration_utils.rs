#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationResource {
    pub name: String,
    pub kind: String,
    pub parent: Option<Box<Self>>,
    pub children: Vec<IntegrationResource>,
}

impl IntegrationResource {
    pub fn new(
        name: String,
        kind: String,
        parent: Option<Self>,
        children: Vec<IntegrationResource>,
    ) -> Self {
        Self {
            name,
            kind,
            parent: parent.map(Box::new),
            children,
        }
    }
}
