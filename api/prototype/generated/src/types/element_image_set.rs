#[derive(Debug, serde::Deserialize)]
pub enum ElementImageSet {
    #[serde(untagged)]
    ElementImageSet {
        base: Option<Box<crate::types::ElementImageSetLayer>>,
        glow: Option<Box<crate::types::ElementImageSetLayer>>,
        shadow: Option<Box<crate::types::ElementImageSetLayer>>,
    },
    #[serde(untagged)]
    ElementImageSetLayer(Box<crate::types::ElementImageSetLayer>),
}
