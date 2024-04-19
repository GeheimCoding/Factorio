use api::ResourceEntityPrototype;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub category: String,
    pub infinite: bool,
    // TODO: extend
}

pub type ResourcesByName = HashMap<String, Resource>;

pub struct Resources {
    pub by_name: ResourcesByName,
    pub by_category: HashMap<String, ResourcesByName>,
}

impl From<&ResourceEntityPrototype> for Resource {
    fn from(prototype: &ResourceEntityPrototype) -> Self {
        Resource {
            name: prototype.parent_.parent_.name.clone(),
            category: prototype
                .category
                .clone()
                .unwrap_or("basic-solid".to_string()),
            infinite: prototype.infinite.unwrap_or(false),
        }
    }
}
