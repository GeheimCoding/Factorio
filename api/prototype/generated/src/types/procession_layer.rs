#[derive(Debug, serde::Deserialize)]
pub enum ProcessionLayer {
    #[serde(untagged)]
    PodDistanceTraveledProcessionLayer(Box<crate::types::PodDistanceTraveledProcessionLayer>),
    #[serde(untagged)]
    PodMovementProcessionLayer(Box<crate::types::PodMovementProcessionLayer>),
    #[serde(untagged)]
    PodOpacityProcessionLayer(Box<crate::types::PodOpacityProcessionLayer>),
    #[serde(untagged)]
    SingleGraphicProcessionLayer(Box<crate::types::SingleGraphicProcessionLayer>),
    #[serde(untagged)]
    CoverGraphicProcessionLayer(Box<crate::types::CoverGraphicProcessionLayer>),
    #[serde(untagged)]
    TintProcessionLayer(Box<crate::types::TintProcessionLayer>),
    #[serde(untagged)]
    PodAnimationProcessionLayer(Box<crate::types::PodAnimationProcessionLayer>),
}
