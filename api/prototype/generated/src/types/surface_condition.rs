#[derive(Debug, serde::Deserialize)]
pub struct SurfaceCondition {
    // default: Max double
    max: Option<f64>,
    // default: Lowest double
    min: Option<f64>,
    property: crate::types::SurfacePropertyID,
}
