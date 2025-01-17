#[derive(serde::Deserialize)]
pub struct CranePartDyingEffect {
    explosion: crate::types::ExplosionDefinition,
    explosion_linear_distance_step: f32,
    particle_effect_linear_distance_step: f32,
    particle_effects: Vec<crate::types::CreateParticleTriggerEffectItem>,
}
