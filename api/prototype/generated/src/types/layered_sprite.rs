pub enum LayeredSprite {
    LayeredSprite {
        base_: crate::types::Sprite,
        render_layer: crate::types::RenderLayer,
    },
    VecLayeredSprite(Vec<crate::types::LayeredSprite>),
}
