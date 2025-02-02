#[derive(Debug, serde::Deserialize)]
pub struct FootprintParticle {
    particle_name: Option<crate::types::ParticleID>,
    tiles: crate::vec::Vec<crate::types::TileID>,
    #[serde(default = "default_use_as_default")]
    use_as_default: bool,
}
fn default_use_as_default() -> bool {
    false
}
