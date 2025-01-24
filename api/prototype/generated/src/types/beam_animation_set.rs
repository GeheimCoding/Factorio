#[derive(serde::Deserialize)]
pub struct BeamAnimationSet {
    body: Option<crate::types::AnimationVariations>,
    ending: Option<crate::types::Animation>,
    head: Option<crate::types::Animation>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    start: Option<crate::types::Animation>,
    tail: Option<crate::types::Animation>,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Projectile
}
