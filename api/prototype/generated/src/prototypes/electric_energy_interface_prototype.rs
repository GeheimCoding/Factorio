#[derive(Debug, serde::Deserialize)]
pub struct ElectricEnergyInterfacePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_allow_copy_paste")]
    allow_copy_paste: bool,
    animation: Option<crate::types::Animation>,
    animations: Option<crate::types::Animation4Way>,
    #[serde(default = "default_continuous_animation")]
    continuous_animation: bool,
    #[serde(default = "default_energy_production")]
    energy_production: crate::types::Energy,
    energy_source: crate::types::ElectricEnergySource,
    #[serde(default = "default_energy_usage")]
    energy_usage: crate::types::Energy,
    #[serde(default = "default_gui_mode")]
    gui_mode: ElectricEnergyInterfacePrototypeGuiMode,
    light: Option<crate::types::LightDefinition>,
    picture: Option<crate::types::Sprite>,
    pictures: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
}
fn default_allow_copy_paste() -> bool {
    false
}
fn default_continuous_animation() -> bool {
    false
}
fn default_energy_production() -> crate::types::Energy {
    String::from("0")
}
fn default_energy_usage() -> crate::types::Energy {
    String::from("0")
}
#[derive(Debug, serde::Deserialize)]
pub enum ElectricEnergyInterfacePrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
fn default_gui_mode() -> ElectricEnergyInterfacePrototypeGuiMode {
    ElectricEnergyInterfacePrototypeGuiMode::None
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
