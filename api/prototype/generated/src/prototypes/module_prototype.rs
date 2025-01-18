#[derive(serde::Deserialize)]
pub struct ModulePrototype {
    base_: crate::prototypes::ItemPrototype,
    art_style: String,
    beacon_tint: BeaconVisualizationTints,
    category: crate::types::ModuleCategoryID,
    effect: crate::types::Effect,
    #[serde(default = "default_requires_beacon_alt_mode")]
    requires_beacon_alt_mode: bool,
    tier: u32,
}
#[derive(serde::Deserialize)]
pub struct BeaconVisualizationTints {
    // default: no color
    primary: crate::types::Color,
    // default: no color
    quaternary: crate::types::Color,
    // default: no color
    secondary: crate::types::Color,
    // default: no color
    tertiary: crate::types::Color,
}
fn default_requires_beacon_alt_mode() -> bool {
    true
}
