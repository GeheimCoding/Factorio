#[derive(serde::Deserialize)]
pub struct CraftFluidTechnologyTrigger {
    amount: f64,
    fluid: crate::types::FluidID,
    #[serde(rename = "type")]
    type_: String,
}
