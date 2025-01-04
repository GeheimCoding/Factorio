pub struct StreamAttackParameters {
    fluid_consumption: crate::types::FluidAmount,
    fluids: Vec<crate::types::StreamFluidProperties>,
    gun_barrel_length: f32,
    gun_center_shift: StreamAttackParametersGunCenterShift,
    projectile_creation_parameters: crate::types::CircularProjectileCreationSpecification,
    type_: Stream,
}
pub enum StreamAttackParametersGunCenterShift {
    Vector(crate::types::Vector),
    GunShift4Way(Box<crate::types::GunShift4Way>),
}
