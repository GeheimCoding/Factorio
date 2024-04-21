use api::{Prototype, ResourceEntityPrototype};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub category: String,
    pub infinite: bool,
    // TODO: extend
}

pub type ResourcesByName = HashMap<String, Resource>;

#[derive(Debug)]
pub struct Resources {
    pub by_name: ResourcesByName,
    pub by_category: HashMap<String, ResourcesByName>,
}

impl From<&ResourceEntityPrototype> for Resource {
    fn from(prototype: &ResourceEntityPrototype) -> Self {
        Self {
            name: prototype.parent_.parent_.name.clone(),
            category: prototype
                .category
                .clone()
                .unwrap_or("basic-solid".to_string()),
            infinite: prototype.infinite.unwrap_or(false),
        }
    }
}

impl From<&Vec<Prototype>> for Resources {
    fn from(prototypes: &Vec<Prototype>) -> Self {
        let resources = prototypes
            .iter()
            .filter_map(|prototype| prototype.as_resource_entity_prototype())
            .map(Resource::from)
            .collect();
        Self {
            by_name: map_by_name(&resources),
            by_category: resources
                .iter()
                .group_by(|resource| resource.category.clone())
                .into_iter()
                .map(|(category, resources)| (category, map_by_name(&resources.cloned().collect())))
                .collect(),
        }
    }
}

// TODO: make generic (impl Iter)
fn map_by_name(resources: &Vec<Resource>) -> ResourcesByName {
    resources
        .iter()
        .map(|resource| (resource.name.clone(), resource.clone()))
        .collect()
}
