pub struct StreamAttackParameters {
    fluid_consumption: FluidAmount,
    fluids: Vec<StreamFluidProperties>,
    gun_barrel_length: f32,
    gun_center_shift: StreamAttackParametersGunCenterShift,
    projectile_creation_parameters: CircularProjectileCreationSpecification,
    type_: Stream,
}
pub enum StreamAttackParametersGunCenterShift {
    Vector(Vector),
    GunShift4Way(GunShift4Way),
}
