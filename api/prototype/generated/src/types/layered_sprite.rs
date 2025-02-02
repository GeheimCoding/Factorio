#[derive(Debug, serde::Deserialize)]
pub enum LayeredSprite {
    #[serde(untagged)]
    LayeredSprite {
        #[serde(flatten)]
        base_: crate::types::Sprite,
        render_layer: crate::types::RenderLayer,
    },
    #[serde(untagged)]
    VecLayeredSprite(Vec<crate::types::LayeredSprite>),
}
