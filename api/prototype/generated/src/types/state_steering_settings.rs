#[derive(serde::Deserialize)]
pub struct StateSteeringSettings {
    force_unit_fuzzy_goto_behavior: bool,
    radius: f64,
    separation_factor: f64,
    separation_force: f64,
}
