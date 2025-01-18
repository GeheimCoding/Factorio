#[derive(serde::Deserialize)]
pub struct BeaconGraphicsSet {
    #[serde(default = "default_animation_layer")]
    animation_layer: crate::types::RenderLayer,
    animation_list: Vec<crate::types::AnimationElement>,
    #[serde(default = "default_animation_progress")]
    animation_progress: f32,
    #[serde(default = "default_apply_module_tint")]
    apply_module_tint: crate::types::ModuleTint,
    #[serde(default = "default_base_layer")]
    base_layer: crate::types::RenderLayer,
    #[serde(default = "default_draw_animation_when_idle")]
    draw_animation_when_idle: bool,
    #[serde(default = "default_draw_light_when_idle")]
    draw_light_when_idle: bool,
    frozen_patch: crate::types::Sprite,
    light: crate::types::LightDefinition,
    #[serde(default = "default_module_icons_suppressed")]
    module_icons_suppressed: bool,
    #[serde(default = "default_module_tint_mode")]
    module_tint_mode: BeaconGraphicsSetModuleTintMode,
    module_visualisations: Vec<crate::types::BeaconModuleVisualizations>,
    // default: no color
    no_modules_tint: crate::types::Color,
    #[serde(default = "default_random_animation_offset")]
    random_animation_offset: bool,
    #[serde(default = "default_reset_animation_when_frozen")]
    reset_animation_when_frozen: bool,
    #[serde(default = "default_top_layer")]
    top_layer: crate::types::RenderLayer,
}
fn default_animation_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_animation_progress() -> f32 {
    1.0
}
fn default_apply_module_tint() -> crate::types::ModuleTint {
    crate::types::ModuleTint::None
}
fn default_base_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_draw_animation_when_idle() -> bool {
    true
}
fn default_draw_light_when_idle() -> bool {
    false
}
fn default_module_icons_suppressed() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum BeaconGraphicsSetModuleTintMode {
    #[serde(rename = "single_module")]
    SingleModule,
    #[serde(rename = "mix")]
    Mix,
}
fn default_module_tint_mode() -> BeaconGraphicsSetModuleTintMode {
    BeaconGraphicsSetModuleTintMode::SingleModule
}
fn default_random_animation_offset() -> bool {
    false
}
fn default_reset_animation_when_frozen() -> bool {
    false
}
fn default_top_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
