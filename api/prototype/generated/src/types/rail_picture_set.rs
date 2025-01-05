pub struct RailPictureSet {
    back_rail_endings: crate::types::Sprite16Way,
    east: crate::types::RailPieceLayers,
    fog_mask: RailsFogMaskDefinitions,
    front_rail_endings: crate::types::Sprite16Way,
    north: crate::types::RailPieceLayers,
    northeast: crate::types::RailPieceLayers,
    northwest: crate::types::RailPieceLayers,
    rail_endings: crate::types::Sprite16Way,
    render_layers: crate::types::RailRenderLayers,
    secondary_render_layers: crate::types::RailRenderLayers,
    segment_visualisation_endings: crate::types::RotatedAnimation,
    slice_origin: RailsSliceOffsets,
    south: crate::types::RailPieceLayers,
    southeast: crate::types::RailPieceLayers,
    southwest: crate::types::RailPieceLayers,
    west: crate::types::RailPieceLayers,
}
pub struct RailsFogMaskDefinitions {
    east: crate::types::FogMaskShapeDefinition,
    north: crate::types::FogMaskShapeDefinition,
    south: crate::types::FogMaskShapeDefinition,
    west: crate::types::FogMaskShapeDefinition,
}
pub struct RailsSliceOffsets {
    east: crate::types::Vector,
    north: crate::types::Vector,
    south: crate::types::Vector,
    west: crate::types::Vector,
}
