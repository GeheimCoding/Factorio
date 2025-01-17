#[derive(serde::Deserialize)]
pub struct BeaconGraphicsSet {
    animation_layer: crate::types::RenderLayer,
    animation_list: Vec<crate::types::AnimationElement>,
    animation_progress: f32,
    apply_module_tint: crate::types::ModuleTint,
    base_layer: crate::types::RenderLayer,
    draw_animation_when_idle: bool,
    draw_light_when_idle: bool,
    frozen_patch: crate::types::Sprite,
    light: crate::types::LightDefinition,
    module_icons_suppressed: bool,
    module_tint_mode: BeaconGraphicsSetModuleTintMode,
    module_visualisations: Vec<crate::types::BeaconModuleVisualizations>,
    no_modules_tint: crate::types::Color,
    random_animation_offset: bool,
    reset_animation_when_frozen: bool,
    top_layer: crate::types::RenderLayer,
}
#[derive(serde::Deserialize)]
pub enum BeaconGraphicsSetModuleTintMode {
    #[serde(rename = "single_module")]
    SingleModule,
    #[serde(rename = "mix")]
    Mix,
}
