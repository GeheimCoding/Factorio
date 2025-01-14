pub struct LocomotivePrototype {
    base_: crate::prototypes::RollingStockPrototype,
    darkness_to_render_light_animation: f32,
    energy_source: LocomotivePrototypeEnergySource,
    front_light: crate::types::LightDefinition,
    front_light_pictures: crate::types::RollingStockRotatedSlopedGraphics,
    max_power: crate::types::Energy,
    max_snap_to_train_stop_distance: f32,
    reversing_power_modifier: f64,
}
pub enum LocomotivePrototypeEnergySource {
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
