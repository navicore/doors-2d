pub struct IntegrationResource {
    pub name: String,
    pub kind: String,
    pub owner: Option<Box<Self>>,
    pub containers: Vec<String>,
}

impl IntegrationResource {
    pub fn new(name: String, kind: String, owner: Option<Self>, containers: Vec<String>) -> Self {
        Self {
            name,
            kind,
            owner: owner.map(Box::new),
            containers,
        }
    }
}
