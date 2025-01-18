#[derive(serde::Deserialize)]
pub struct StreamAttackParameters {
    base_: crate::types::BaseAttackParameters,
    fluid_consumption: crate::types::FluidAmount,
    fluids: Vec<StreamFluidProperties>,
    gun_barrel_length: f32,
    gun_center_shift: StreamAttackParametersGunCenterShift,
    projectile_creation_parameters: crate::types::CircularProjectileCreationSpecification,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct StreamFluidProperties {
    damage_modifier: f64,
    #[serde(rename = "type")]
    type_: crate::types::FluidID,
}
#[derive(serde::Deserialize)]
pub enum StreamAttackParametersGunCenterShift {
    #[serde(untagged)]
    Vector(crate::types::Vector),
    #[serde(untagged)]
    GunShift4Way(Box<GunShift4Way>),
}
#[derive(serde::Deserialize)]
pub struct GunShift4Way {
    east: crate::types::Vector,
    north: crate::types::Vector,
    south: crate::types::Vector,
    west: crate::types::Vector,
}
