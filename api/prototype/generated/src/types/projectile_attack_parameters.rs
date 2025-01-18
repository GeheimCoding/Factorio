#[derive(serde::Deserialize)]
pub struct ProjectileAttackParameters {
    base_: crate::types::BaseAttackParameters,
    apply_projection_to_projectile_creation_position: bool,
    projectile_center: crate::types::Vector,
    projectile_creation_distance: f32,
    projectile_creation_offsets: Vec<crate::types::Vector>,
    projectile_creation_parameters: crate::types::CircularProjectileCreationSpecification,
    projectile_orientation_offset: crate::types::RealOrientation,
    shell_particle: crate::types::CircularParticleCreationSpecification,
    #[serde(rename = "type")]
    type_: String,
}
