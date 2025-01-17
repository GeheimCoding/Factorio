#[derive(serde::Deserialize)]
pub enum ThrusterPerformancePoint {
    #[serde(untagged)]
    ThrusterPerformancePoint {
        effectivity: f64,
        fluid_usage: crate::types::FluidAmount,
        fluid_volume: f64,
    },
    #[serde(untagged)]
    F64F64F64((f64, f64, f64)),
}
