#[derive(Debug, serde::Deserialize)]
pub struct LocomotivePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RollingStockPrototype,
    #[serde(default = "default_darkness_to_render_light_animation")]
    darkness_to_render_light_animation: f32,
    energy_source: LocomotivePrototypeEnergySource,
    front_light: Option<crate::types::LightDefinition>,
    front_light_pictures: Option<crate::types::RollingStockRotatedSlopedGraphics>,
    max_power: crate::types::Energy,
    #[serde(default = "default_max_snap_to_train_stop_distance")]
    max_snap_to_train_stop_distance: f32,
    reversing_power_modifier: f64,
}
fn default_darkness_to_render_light_animation() -> f32 {
    0.3
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum LocomotivePrototypeEnergySource {
    #[serde(rename = "burner")]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_max_snap_to_train_stop_distance() -> f32 {
    3.0
}
