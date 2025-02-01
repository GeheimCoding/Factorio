#[derive(Debug, serde::Deserialize)]
pub struct StreamAttackParameters {
    base_: crate::types::BaseAttackParameters,
    #[serde(default = "default_fluid_consumption")]
    fluid_consumption: crate::types::FluidAmount,
    fluids: Option<Vec<StreamFluidProperties>>,
    #[serde(default = "default_gun_barrel_length")]
    gun_barrel_length: f32,
    gun_center_shift: Option<StreamAttackParametersGunCenterShift>,
    projectile_creation_parameters: Option<crate::types::CircularProjectileCreationSpecification>,
}
fn default_fluid_consumption() -> crate::types::FluidAmount {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub struct StreamFluidProperties {
    #[serde(default = "default_damage_modifier")]
    damage_modifier: f64,
    #[serde(rename = "type")]
    type_: crate::types::FluidID,
}
fn default_damage_modifier() -> f64 {
    1.0
}
fn default_gun_barrel_length() -> f32 {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub enum StreamAttackParametersGunCenterShift {
    #[serde(untagged)]
    Vector(crate::types::Vector),
    #[serde(untagged)]
    GunShift4Way(Box<GunShift4Way>),
}
#[derive(Debug, serde::Deserialize)]
pub struct GunShift4Way {
    east: Option<crate::types::Vector>,
    north: crate::types::Vector,
    south: Option<crate::types::Vector>,
    west: Option<crate::types::Vector>,
}
