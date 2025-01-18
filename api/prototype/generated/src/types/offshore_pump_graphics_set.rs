#[derive(serde::Deserialize)]
pub struct OffshorePumpGraphicsSet {
    animation: crate::types::Animation4Way,
    base_pictures: crate::types::Sprite4Way,
    #[serde(default = "default_base_render_layer")]
    base_render_layer: crate::types::RenderLayer,
    fluid_animation: crate::types::Animation4Way,
    glass_pictures: crate::types::Sprite4Way,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
    underwater_pictures: crate::types::Sprite4Way,
}
fn default_base_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::GroundPatch
}
fn default_underwater_layer_offset() -> i8 {
    1
}
