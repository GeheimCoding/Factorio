use api::AssemblingMachinePrototype;

#[derive(Debug)]
pub struct AssemblingMachineData {
    pub ingredient_count: u8,
}

impl From<&AssemblingMachinePrototype> for AssemblingMachineData {
    fn from(assembling_machine: &AssemblingMachinePrototype) -> Self {
        Self {
            ingredient_count: assembling_machine.ingredient_count.unwrap_or(u8::MAX),
        }
    }
}
