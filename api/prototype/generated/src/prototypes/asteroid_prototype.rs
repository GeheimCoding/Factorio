#[derive(Debug, serde::Deserialize)]
pub struct AsteroidPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    emissions_per_second: Option<std::collections::HashMap<crate::types::AirbornePollutantID, f64>>,
    graphics_set: Option<crate::types::AsteroidGraphicsSet>,
    #[serde(default = "default_mass")]
    mass: f64,
}
fn default_mass() -> f64 {
    1.0
}
