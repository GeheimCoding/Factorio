#[derive(serde::Deserialize)]
pub struct BeaconModuleVisualization {
    #[serde(default = "default_apply_module_tint")]
    apply_module_tint: crate::types::ModuleTint,
    #[serde(default = "default_has_empty_slot")]
    has_empty_slot: bool,
    pictures: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
}
fn default_apply_module_tint() -> crate::types::ModuleTint {
    crate::types::ModuleTint::None
}
fn default_has_empty_slot() -> bool {
    false
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_secondary_draw_order() -> i8 {
    0
}
