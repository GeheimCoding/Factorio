#[derive(serde::Deserialize)]
pub struct BeamAnimationSet {
    body: crate::types::AnimationVariations,
    ending: crate::types::Animation,
    head: crate::types::Animation,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    start: crate::types::Animation,
    tail: crate::types::Animation,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Projectile
}
