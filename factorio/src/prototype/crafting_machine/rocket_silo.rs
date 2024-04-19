use crate::prototype::crafting_machine::assembling_machine::AssemblingMachineData;
use api::RocketSiloPrototype;

#[derive(Debug)]
pub struct RocketSiloData {
    pub parent: AssemblingMachineData,
    // TODO: extend
}

impl From<&RocketSiloPrototype> for RocketSiloData {
    fn from(rocket_silo: &RocketSiloPrototype) -> Self {
        Self {
            parent: (&rocket_silo.parent_).into(),
        }
    }
}
