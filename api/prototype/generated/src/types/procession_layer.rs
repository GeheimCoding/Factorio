pub enum ProcessionLayer {
    PodDistanceTraveledProcessionLayer(Box<crate::types::PodDistanceTraveledProcessionLayer>),
    PodMovementProcessionLayer(Box<crate::types::PodMovementProcessionLayer>),
    PodOpacityProcessionLayer(Box<crate::types::PodOpacityProcessionLayer>),
    SingleGraphicProcessionLayer(Box<crate::types::SingleGraphicProcessionLayer>),
    CoverGraphicProcessionLayer(Box<crate::types::CoverGraphicProcessionLayer>),
    TintProcessionLayer(Box<crate::types::TintProcessionLayer>),
    PodAnimationProcessionLayer(Box<crate::types::PodAnimationProcessionLayer>),
}
