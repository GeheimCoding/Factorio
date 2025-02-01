#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ProcessionLayer {
    #[serde(rename = "pod-distance-traveled")]
    PodDistanceTraveledProcessionLayer(Box<crate::types::PodDistanceTraveledProcessionLayer>),
    #[serde(rename = "pod-movement")]
    PodMovementProcessionLayer(Box<crate::types::PodMovementProcessionLayer>),
    #[serde(rename = "pod-opacity")]
    PodOpacityProcessionLayer(Box<crate::types::PodOpacityProcessionLayer>),
    #[serde(rename = "single-graphic")]
    SingleGraphicProcessionLayer(Box<crate::types::SingleGraphicProcessionLayer>),
    #[serde(rename = "cover-graphic")]
    CoverGraphicProcessionLayer(Box<crate::types::CoverGraphicProcessionLayer>),
    #[serde(rename = "tint")]
    TintProcessionLayer(Box<crate::types::TintProcessionLayer>),
    #[serde(rename = "pod-animation")]
    PodAnimationProcessionLayer(Box<crate::types::PodAnimationProcessionLayer>),
}
