pub struct StreamAttackParameters {
    fluid_consumption: crate::types::FluidAmount,
    fluids: Vec<StreamFluidProperties>,
    gun_barrel_length: f32,
    gun_center_shift: StreamAttackParametersGunCenterShift,
    projectile_creation_parameters: crate::types::CircularProjectileCreationSpecification,
    type_: String,
}
pub struct StreamFluidProperties {
    damage_modifier: f64,
    type_: crate::types::FluidID,
}
pub enum StreamAttackParametersGunCenterShift {
    Vector(crate::types::Vector),
    GunShift4Way(Box<GunShift4Way>),
}
pub struct GunShift4Way {
    east: crate::types::Vector,
    north: crate::types::Vector,
    south: crate::types::Vector,
    west: crate::types::Vector,
}
