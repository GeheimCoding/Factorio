#[derive(serde::Deserialize)]
pub struct FusionGeneratorGraphicsSet {
    east_graphics_set: crate::types::FusionGeneratorDirectionGraphicsSet,
    // default: `{1, 1, 1}`
    glow_color: crate::types::Color,
    light: crate::types::LightDefinition,
    north_graphics_set: crate::types::FusionGeneratorDirectionGraphicsSet,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    south_graphics_set: crate::types::FusionGeneratorDirectionGraphicsSet,
    west_graphics_set: crate::types::FusionGeneratorDirectionGraphicsSet,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
