pub struct ProjectileAttackParameters {
    apply_projection_to_projectile_creation_position: bool,
    projectile_center: Vector,
    projectile_creation_distance: f32,
    projectile_creation_offsets: Vec<Vector>,
    projectile_creation_parameters: CircularProjectileCreationSpecification,
    projectile_orientation_offset: RealOrientation,
    shell_particle: CircularParticleCreationSpecification,
    type_: Projectile,
}
