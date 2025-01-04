pub struct BeaconGraphicsSet {
    animation_layer: RenderLayer,
    animation_list: Vec<AnimationElement>,
    animation_progress: f32,
    apply_module_tint: ModuleTint,
    base_layer: RenderLayer,
    draw_animation_when_idle: bool,
    draw_light_when_idle: bool,
    frozen_patch: Sprite,
    light: LightDefinition,
    module_icons_suppressed: bool,
    module_tint_mode: BeaconGraphicsSetModuleTintMode,
    module_visualisations: Vec<BeaconModuleVisualizations>,
    no_modules_tint: Color,
    random_animation_offset: bool,
    reset_animation_when_frozen: bool,
    top_layer: RenderLayer,
}
pub enum BeaconGraphicsSetModuleTintMode {
    SingleModule,
    Mix,
}
