pub enum ProcessionLayer {
    PodDistanceTraveledProcessionLayer(Box<PodDistanceTraveledProcessionLayer>),
    PodMovementProcessionLayer(Box<PodMovementProcessionLayer>),
    PodOpacityProcessionLayer(Box<PodOpacityProcessionLayer>),
    SingleGraphicProcessionLayer(Box<SingleGraphicProcessionLayer>),
    CoverGraphicProcessionLayer(Box<CoverGraphicProcessionLayer>),
    TintProcessionLayer(Box<TintProcessionLayer>),
    PodAnimationProcessionLayer(Box<PodAnimationProcessionLayer>),
}
