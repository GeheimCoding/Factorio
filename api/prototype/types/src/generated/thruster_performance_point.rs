pub enum ThrusterPerformancePoint {
    ThrusterPerformancePoint {
        effectivity: f64,
        fluid_usage: FluidAmount,
        fluid_volume: f64,
    },
    F64F64F64((f64, f64, f64)),
}
