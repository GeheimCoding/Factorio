pub struct FluidPrototype {
    base_: crate::prototypes::Prototype,
    auto_barrel: bool,
    base_color: crate::types::Color,
    default_temperature: f32,
    emissions_multiplier: f64,
    flow_color: crate::types::Color,
    fuel_value: crate::types::Energy,
    gas_temperature: f32,
    heat_capacity: crate::types::Energy,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    max_temperature: f32,
    visualization_color: crate::types::Color,
}
