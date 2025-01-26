#[derive(Debug, serde::Deserialize)]
pub struct ModulePrototype {
    base_: crate::prototypes::ItemPrototype,
    art_style: Option<String>,
    beacon_tint: Option<BeaconVisualizationTints>,
    category: crate::types::ModuleCategoryID,
    effect: crate::types::Effect,
    #[serde(default = "default_requires_beacon_alt_mode")]
    requires_beacon_alt_mode: bool,
    tier: u32,
}
#[derive(Debug, serde::Deserialize)]
pub struct BeaconVisualizationTints {
    // default: no color
    primary: Option<crate::types::Color>,
    // default: no color
    quaternary: Option<crate::types::Color>,
    // default: no color
    secondary: Option<crate::types::Color>,
    // default: no color
    tertiary: Option<crate::types::Color>,
}
fn default_requires_beacon_alt_mode() -> bool {
    true
}
