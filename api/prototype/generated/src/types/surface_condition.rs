#[derive(serde::Deserialize)]
pub struct SurfaceCondition {
    // default: Max double
    max: f64,
    // default: Lowest double
    min: f64,
    property: crate::types::SurfacePropertyID,
}
