pub struct ElectricEnergyInterfacePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    allow_copy_paste: bool,
    animation: crate::types::Animation,
    animations: crate::types::Animation4Way,
    continuous_animation: bool,
    energy_production: crate::types::Energy,
    energy_source: crate::types::ElectricEnergySource,
    energy_usage: crate::types::Energy,
    gui_mode: ElectricEnergyInterfacePrototypeGuiMode,
    light: crate::types::LightDefinition,
    picture: crate::types::Sprite,
    pictures: crate::types::Sprite4Way,
    render_layer: crate::types::RenderLayer,
}
#[derive(serde::Deserialize)]
pub enum ElectricEnergyInterfacePrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
