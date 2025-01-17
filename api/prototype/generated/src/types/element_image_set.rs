#[derive(serde::Deserialize)]
pub enum ElementImageSet {
    #[serde(untagged)]
    ElementImageSet {
        base: crate::types::ElementImageSetLayer,
        glow: crate::types::ElementImageSetLayer,
        shadow: crate::types::ElementImageSetLayer,
    },
    #[serde(untagged)]
    ElementImageSetLayer(crate::types::ElementImageSetLayer),
}
