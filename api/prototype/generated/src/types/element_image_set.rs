#[derive(serde::Deserialize)]
pub enum ElementImageSet {
    #[serde(untagged)]
    ElementImageSet {
        base: Option<crate::types::ElementImageSetLayer>,
        glow: Option<crate::types::ElementImageSetLayer>,
        shadow: Option<crate::types::ElementImageSetLayer>,
    },
    #[serde(untagged)]
    ElementImageSetLayer(crate::types::ElementImageSetLayer),
}
