#[derive(serde::Deserialize)]
pub struct ProjectileAttackParameters {
    base_: crate::types::BaseAttackParameters,
    #[serde(default = "default_apply_projection_to_projectile_creation_position")]
    apply_projection_to_projectile_creation_position: bool,
    // default: `{0, 0}`
    projectile_center: crate::types::Vector,
    #[serde(default = "default_projectile_creation_distance")]
    projectile_creation_distance: f32,
    projectile_creation_offsets: Vec<crate::types::Vector>,
    projectile_creation_parameters: crate::types::CircularProjectileCreationSpecification,
    #[serde(default = "default_projectile_orientation_offset")]
    projectile_orientation_offset: crate::types::RealOrientation,
    shell_particle: crate::types::CircularParticleCreationSpecification,
    #[serde(rename = "type")]
    type_: String,
}
fn default_apply_projection_to_projectile_creation_position() -> bool {
    true
}
fn default_projectile_creation_distance() -> f32 {
    0.0
}
fn default_projectile_orientation_offset() -> crate::types::RealOrientation {
    0.0
}
