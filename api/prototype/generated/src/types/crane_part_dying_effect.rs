#[derive(serde::Deserialize)]
pub struct CranePartDyingEffect {
    explosion: Option<crate::types::ExplosionDefinition>,
    #[serde(default = "default_explosion_linear_distance_step")]
    explosion_linear_distance_step: f32,
    #[serde(default = "default_particle_effect_linear_distance_step")]
    particle_effect_linear_distance_step: f32,
    particle_effects: Option<Vec<crate::types::CreateParticleTriggerEffectItem>>,
}
fn default_explosion_linear_distance_step() -> f32 {
    0.5
}
fn default_particle_effect_linear_distance_step() -> f32 {
    0.2
}
