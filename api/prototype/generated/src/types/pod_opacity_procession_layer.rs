pub struct PodOpacityProcessionLayer {
    frames: Vec<PodOpacityProcessionBezierControlPoint>,
    lut: ColorLookupTable,
    type_: PodOpacity,
}
