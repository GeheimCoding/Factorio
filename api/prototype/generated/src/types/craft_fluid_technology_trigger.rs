#[derive(serde::Deserialize)]
pub struct CraftFluidTechnologyTrigger {
    amount: f64,
    fluid: crate::types::FluidID,
    type_: String,
}
