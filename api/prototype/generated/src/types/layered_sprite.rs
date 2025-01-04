pub enum LayeredSprite {
    LayeredSprite {
        render_layer: crate::types::RenderLayer,
    },
    VecLayeredSprite(Vec<crate::types::LayeredSprite>),
}
