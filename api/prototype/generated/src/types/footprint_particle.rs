#[derive(serde::Deserialize)]
pub struct FootprintParticle {
    particle_name: crate::types::ParticleID,
    tiles: Vec<crate::types::TileID>,
    use_as_default: bool,
}
