pub struct AsteroidPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    emissions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    graphics_set: crate::types::AsteroidGraphicsSet,
    mass: f64,
}
