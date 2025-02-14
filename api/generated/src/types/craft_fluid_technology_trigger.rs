#[derive(Debug, serde::Deserialize)]
pub struct CraftFluidTechnologyTrigger {
    #[serde(default = "default_amount")]
    amount: f64,
    fluid: crate::types::FluidID,
}
fn default_amount() -> f64 {
    0.0
}
