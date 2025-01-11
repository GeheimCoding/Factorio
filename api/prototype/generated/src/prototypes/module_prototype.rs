pub struct ModulePrototype {
    art_style: String,
    beacon_tint: BeaconVisualizationTints,
    category: crate::types::ModuleCategoryID,
    effect: crate::types::Effect,
    requires_beacon_alt_mode: bool,
    tier: u32,
}
pub struct BeaconVisualizationTints {
    primary: crate::types::Color,
    quaternary: crate::types::Color,
    secondary: crate::types::Color,
    tertiary: crate::types::Color,
}
